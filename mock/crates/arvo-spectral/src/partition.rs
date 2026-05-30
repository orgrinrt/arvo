//! Spectral bisection and recursive k-way partitioning.
//!
//! `spectral_bisection` splits an `N`-node set into two classes by the
//! sign of the Fiedler vector. The return shape is
//! `(class_count, C::Array<USize>)` where `class[i]` is the
//! class id of node `i`. Class IDs: `0` for positive, `1` for
//! non-positive. Ties go to the negative class; `F::from_constant(0)`
//! compared strictly-greater-than via `TotalOrd`. `class_count` is
//! always `2`.
//!
//! `k_way_partition` runs recursive spectral bisection. A
//! stack-allocated fixed-capacity work stack holds partition IDs awaiting
//! a split. Each pop bisects one partition by recomputing the Fiedler
//! vector on the full operator and partitioning by sign within the
//! membership filter. Output is `(partition_count, per_node_partition_id)`.
//! `partition_count` can be less than `K` if a component becomes
//! unsplittable before the budget is exhausted.
//!
//! Both routines are operator-generic over `LinearOperator<F, C>`.
//! Callers wrap their Laplacian (dense `Matrix<F, C>` or
//! `SparseLaplacian<...>`) once and reuse for both eigenvector and
//! partition stages. The capacity is a TYPE (`C: Capacity`); the
//! partition budget is a second capacity `K: Capacity`.

use core::cmp::Ordering;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo::{Identity, USize};
use arvo_tensor::{Capacity, cap_size};

use crate::fiedler::fiedler_vector;
use crate::operator::{LinearOperator, SparseLaplacian};

/// Partition `N` nodes into two classes by the sign of their Fiedler
/// component.
///
/// Returns `(class_count, per_node_class_id)`. `class_id[i]` is `0`
/// when `fiedler[i] > 0` (strict, per `TotalOrd`) and `1` otherwise.
/// `class_count` is always `2`. Ties and negative values go to class
/// `1`. Only the first `live_n` entries are classified; entries at or
/// beyond `live_n` keep class `1` (a loose graph's slack rows are not
/// real nodes).
#[inline]
pub fn spectral_bisection<C: Capacity, F>(
    fiedler: &C::Array<F>,
    live_n: USize,
) -> (USize, C::Array<USize>)
where
    F: TotalOrd + Copy + FromConstant,
{
    let n = live_n.0;
    let zero = F::from_constant::<{ USize(0) }>();
    let mut class: C::Array<USize> = C::filled(USize(1));
    let fs = fiedler.as_ref();
    let cs = class.as_mut();
    let mut i = 0usize;
    while i < n {
        if let Ordering::Greater = fs[i].total_cmp(zero) {
            cs[i] = USize(0);
        }
        i += 1;
    }
    (USize(2), class)
}

/// Recursive spectral k-way partitioning over a Laplacian operator.
///
/// Starts with every node in partition `0`, then repeatedly picks a
/// partition with more than one node and bisects it. Each bisection
/// computes the Fiedler vector via `fiedler_vector(operator, sigma,
/// iterations)` on the full operator, then filters by membership in
/// the active partition. The positive half receives a new partition
/// id; the negative half keeps the existing id.
///
/// Returns `(partition_count, partition_id_per_node)`. `partition_id`
/// for node `i` is in `[0, partition_count)`. `partition_count <= K`;
/// it can be lower if no partition can be split further before reaching
/// `K`.
///
/// `sigma` is the upper bound on `lambda_max(L)` used by every Fiedler
/// step (the operator is reused, so one sigma serves the whole
/// recursion).
#[inline]
pub fn k_way_partition<Op, C: Capacity, K: Capacity, F>(
    operator: &Op,
    sigma: F,
    iterations: USize,
) -> (USize, C::Array<USize>)
where
    Op: LinearOperator<F, C>,
    C::Array<F>: Copy,
    F: Add<Output = F>
        + Sub<Output = F>
        + Mul<Output = F>
        + Sqrt<Output = F>
        + Recip<Output = F>
        + TotalOrd
        + Copy
        + FromConstant,
{
    // Live node count: a loose-CSR operator excludes its empty slack
    // rows, so they are never counted into a partition or split.
    let n = operator.live_dim().0;
    let k = cap_size(K::CAP);

    let mut partition_id: C::Array<USize> = C::filled(USize(0));
    if n <= 1 || k <= 1 {
        return (USize(if n == 0 { 0 } else { 1 }), partition_id);
    }

    // Work stack: partition IDs awaiting bisection. At most K - 1
    // bisections produce K partitions; stack depth bounded by K.
    let mut stack: K::Array<USize> = K::filled(USize(0));
    stack.as_mut()[0] = USize(0);
    let mut stack_len = USize(1);

    let mut partition_count = USize(1);
    let zero = F::from_constant::<{ USize(0) }>();

    while stack_len.0 > 0 && partition_count.0 < k {
        stack_len = stack_len - USize::ONE;
        let active = stack.as_ref()[stack_len.0];

        // Count active partition's nodes; singletons cannot be split.
        let mut active_count = 0usize;
        {
            let pid = partition_id.as_ref();
            let mut j = 0usize;
            while j < n {
                if pid[j] == active {
                    active_count += 1;
                }
                j += 1;
            }
        }
        if active_count <= 1 {
            continue;
        }

        // Fiedler on the full operator. Filter by partition membership
        // for the bisection. Recomputing per-pop is the documented
        // approximation per arvo-spectral BACKLOG; restricted-Laplacian
        // per-component eigen is a follow-up round's scope.
        let fiedler: C::Array<F> = fiedler_vector(operator, sigma, iterations);

        // Tally positive vs non-positive sides within the active set.
        let mut positive_count = 0usize;
        let mut negative_count = 0usize;
        {
            let pid = partition_id.as_ref();
            let fs = fiedler.as_ref();
            let mut j = 0usize;
            while j < n {
                if pid[j] == active {
                    if let Ordering::Greater = fs[j].total_cmp(zero) {
                        positive_count += 1;
                    } else {
                        negative_count += 1;
                    }
                }
                j += 1;
            }
        }

        // Degenerate bisection (one side empty): cannot split.
        if positive_count == 0 || negative_count == 0 {
            continue;
        }

        // Assign the new id to the positive half. Negative half keeps
        // the existing `active` id.
        let new_id = partition_count;
        partition_count = partition_count + USize::ONE;
        {
            let pid = partition_id.as_mut();
            let fs = fiedler.as_ref();
            let mut j = 0usize;
            while j < n {
                if pid[j] == active {
                    if let Ordering::Greater = fs[j].total_cmp(zero) {
                        pid[j] = new_id;
                    }
                }
                j += 1;
            }
        }

        // Push both halves back for further bisection.
        //
        // Guard both pushes under a single "room for two" test so we
        // don't silently drop the negative half after admitting the
        // positive half into the last slot. When only one slot is
        // left we push the larger half (best-effort degradation).
        let pos_big = positive_count > 1;
        let neg_big = negative_count > 1;
        let want = match (pos_big, neg_big) {
            (true, true) => 2usize,
            (false, false) => 0usize,
            _ => 1usize,
        };
        // invariant: stack_len.0 < k here (the loop guard ensures we
        // never enter the body with a full stack). The room-for-two
        // branch admits both halves; the room-for-one fallback picks
        // the larger half and drops the smaller (F2 fix).
        if want > 0 && stack_len.0 + want <= k {
            if pos_big {
                stack.as_mut()[stack_len.0] = new_id;
                stack_len = stack_len + USize::ONE;
            }
            if neg_big {
                stack.as_mut()[stack_len.0] = active;
                stack_len = stack_len + USize::ONE;
            }
        } else if want > 0 && stack_len.0 + 1 <= k {
            let pick = if positive_count >= negative_count { new_id } else { active };
            stack.as_mut()[stack_len.0] = pick;
            stack_len = stack_len + USize::ONE;
        }
    }

    (partition_count, partition_id)
}

/// Spectral bipartition contract.
///
/// Any type providing both the Laplacian-shaped operator and its
/// Gershgorin upper bound implements this trait; the default impl
/// composes `fiedler_vector` + `spectral_bisection`. Consumers with a
/// `Matrix<F, C>` representing a Laplacian or a `SparseLaplacian<'_,
/// ...>` get the bipartition without naming the algorithm.
///
/// The trait is plain `pub trait` rather than `pub const trait`
/// because the underlying Fiedler iteration uses floating-point
/// `Sqrt` / `Recip` operations that are not const-callable.
pub trait SpectralBipartitioner<C: Capacity, F>: LinearOperator<F, C>
where
    C::Array<F>: Copy,
    F: Add<Output = F>
        + Sub<Output = F>
        + Mul<Output = F>
        + Sqrt<Output = F>
        + Recip<Output = F>
        + TotalOrd
        + Copy
        + FromConstant,
{
    /// Return the operator's Gershgorin upper bound on
    /// `lambda_max(L)`. Used as the shift for Fiedler iteration.
    ///
    /// `SparseLaplacian` provides an inherent `gershgorin_lambda_max`
    /// with the same body; either spelling is fine at the call site.
    fn lambda_max_bound(&self) -> F;

    /// Run Fiedler iteration and sign-partition.
    ///
    /// Returns `(class_count, per_node_class_id)`. Default
    /// implementation composes the shipped `fiedler_vector` +
    /// `spectral_bisection` free functions.
    #[inline]
    fn bipartition(&self, iterations: USize) -> (USize, C::Array<USize>)
    where
        Self: Sized,
    {
        let sigma = <Self as SpectralBipartitioner<C, F>>::lambda_max_bound(self);
        let fiedler: C::Array<F> = fiedler_vector(self, sigma, iterations);
        spectral_bisection::<C, F>(&fiedler, self.live_dim())
    }
}

// Wire SparseLaplacian into the bipartitioner trait. The bound is the
// Gershgorin upper bound, reused as the Fiedler shift.
impl<'data, R: Capacity, NNZ: Capacity, W, F> SpectralBipartitioner<R, F>
    for SparseLaplacian<'data, R, NNZ, W, F>
where
    R::Array<F>: Copy,
    W: Copy + Into<F>,
    F: Add<Output = F>
        + Sub<Output = F>
        + Mul<Output = F>
        + Sqrt<Output = F>
        + Recip<Output = F>
        + TotalOrd
        + Copy
        + FromConstant,
{
    #[inline]
    fn lambda_max_bound(&self) -> F {
        SparseLaplacian::gershgorin_lambda_max(self)
    }
}
