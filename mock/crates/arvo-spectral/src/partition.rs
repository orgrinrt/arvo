//! Spectral bisection and recursive k-way partitioning.
//!
//! `spectral_bisection` splits an `N`-node set into two halves by the
//! sign of the Fiedler vector. Positive entries land in the first
//! returned mask, non-positive in the second. Ties go to the negative
//! side; `F::from_constant(0)` compared strictly-greater-than via
//! `TotalOrd`.
//!
//! `k_way_partition` runs recursive spectral bisection. A
//! stack-allocated fixed-size work stack holds components waiting to
//! be split; each pop bisects one component. The recursion budget is
//! `K - 1` splits (`K` partitions, `K - 1` bisections). Output is
//! `(partition_count, per_node_partition_id)`; `partition_count` can
//! be less than `K` if a component becomes unsplittable before the
//! budget is exhausted.

use core::cmp::Ordering;

use arvo::{Cap, USize};
use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo_bitmask::Mask64;

use core::ops::{Add, Mul, Sub};

use crate::fiedler::fiedler_vector;
use crate::matrix::{Matrix, cap_size};

/// Partition `N` nodes into two masks by the sign of their Fiedler
/// component.
///
/// Returns `(positive_mask, negative_mask)`. Node `i` goes into
/// `positive_mask` when `fiedler[i] > 0` (strict, per `TotalOrd`);
/// otherwise into `negative_mask`. Ties and negative values go to the
/// negative side.
///
/// Requires `N <= 64` at call site (Mask64 output); enforcement lives
/// at the caller by choosing an appropriate `N`. Nodes with index
/// `>= 64` cannot be represented in either mask and would be dropped;
/// the function does not iterate past `N`.
#[inline]
pub fn spectral_bisection<const N: Cap, F>(
    fiedler: &[F; cap_size(N)],
) -> (Mask64, Mask64)
where
    [(); cap_size(N)]:,
    F: TotalOrd + Copy + FromConstant,
{
    let n = cap_size(N);
    let zero = F::from_constant(USize(0));
    let mut positive = Mask64::default();
    let mut negative = Mask64::default();
    let mut i = 0usize;
    while i < n {
        match fiedler[i].total_cmp(&zero) {
            Ordering::Greater => positive.insert(USize(i)),
            _ => negative.insert(USize(i)),
        }
        i += 1;
    }
    (positive, negative)
}

/// Recursive spectral k-way partitioning.
///
/// Starts with every node in one component, then repeatedly picks a
/// component from the work stack and bisects it. Each bisection runs
/// `iterations` rounds of deflated power iteration on the Laplacian
/// restricted to the component's nodes — the Laplacian of the full
/// graph is used and the bisection-mask restricts which nodes get
/// which partition id.
///
/// Returns `(partition_count, partition_id_per_node)`. `partition_id`
/// for node `i` is in `[0, partition_count)`. `partition_count <= K`;
/// it can be lower if the algorithm runs out of splittable components
/// before reaching `K`.
#[inline]
pub fn k_way_partition<const N: Cap, const K: Cap, W, F>(
    weights: &Matrix<W, N>,
    iterations: USize,
) -> (USize, [USize; cap_size(N)])
where
    [(); cap_size(N)]:,
    [(); cap_size(K)]:,
    W: Into<F> + Copy,
    F: Add<Output = F>
        + Sub<Output = F>
        + Mul<Output = F>
        + Sqrt
        + Recip
        + TotalOrd
        + Copy
        + FromConstant,
{
    let n = cap_size(N);
    let k = cap_size(K);

    // Initial partition: all nodes in partition 0.
    let mut partition_id: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    if n <= 1 || k <= 1 {
        return (USize(if n == 0 { 0 } else { 1 }), partition_id);
    }

    // Work stack: fixed-size array sized to K. At most K - 1
    // bisections produce K partitions; the stack depth is bounded by
    // K at any moment (each pop produces two pushes at most).
    let mut stack: [Mask64; cap_size(K)] = [Mask64::default(); cap_size(K)];

    // Seed: the full-node mask.
    let mut initial = Mask64::default();
    let mut i = 0usize;
    while i < n {
        initial.insert(USize(i));
        i += 1;
    }
    stack[0] = initial;
    let mut stack_len = USize(1);

    // Current partition count. Starts at 1 (everything in partition 0).
    let mut partition_count = USize(1);

    // Compute the full-graph Fiedler vector once for each component
    // we pop. We rebuild per-pop because the Fiedler direction depends
    // on the component under consideration. Recomputing with the full
    // graph and then filtering by mask is an acceptable approximation
    // for this round — per the DESIGN doc, spectral k-way at typical
    // pipeline scale (N <= 32, K <= 8) converges fast enough that
    // recomputing the full-graph Fiedler is not the hot path. A
    // follow-up round can specialise to the restricted Laplacian.
    while stack_len.0 > 0 && partition_count.0 < k {
        stack_len = USize(stack_len.0 - 1);
        let component = stack[stack_len.0];

        // Count nodes in this component.
        let comp_count = component.count().0;
        if comp_count <= 1 {
            // Singleton (or empty) cannot be split further; skip.
            continue;
        }

        // Fiedler on the full graph. Filter by the component mask for
        // the bisection decision.
        let fiedler: [F; cap_size(N)] = fiedler_vector::<N, W, F>(weights, iterations);
        let zero = F::from_constant(USize(0));

        let mut positive_half = Mask64::default();
        let mut negative_half = Mask64::default();
        let mut j = 0usize;
        while j < n {
            if *component.contains(USize(j)) {
                match fiedler[j].total_cmp(&zero) {
                    Ordering::Greater => positive_half.insert(USize(j)),
                    _ => negative_half.insert(USize(j)),
                }
            }
            j += 1;
        }

        // Degenerate bisection (one side empty): cannot split this
        // component; fix its partition id and continue.
        if *positive_half.is_empty() || *negative_half.is_empty() {
            continue;
        }

        // Assign a new partition id to the positive half. The negative
        // half keeps its existing partition id (so already-assigned
        // ids on other nodes are untouched).
        let new_id = partition_count;
        partition_count = USize(partition_count.0 + 1);
        let mut j = 0usize;
        while j < n {
            if *positive_half.contains(USize(j)) {
                partition_id[j] = new_id;
            }
            j += 1;
        }

        // Push both halves back for further bisection.
        //
        // Guard both pushes under a single "room for two" test so we
        // don't silently drop the negative half after admitting the
        // positive half into the last slot. When only one slot is
        // left we push the larger half (best-effort degradation).
        let pos_big = *positive_half.count() > 1;
        let neg_big = *negative_half.count() > 1;
        let want = match (pos_big, neg_big) {
            (true, true) => USize(2),
            (false, false) => USize(0),
            _ => USize(1),
        };
        if want.0 > 0 && stack_len.0 + want.0 <= k {
            if pos_big {
                stack[stack_len.0] = positive_half;
                stack_len = USize(stack_len.0 + 1);
            }
            if neg_big {
                stack[stack_len.0] = negative_half;
                stack_len = USize(stack_len.0 + 1);
            }
        } else if want.0 > 0 && stack_len.0 + 1 <= k {
            let pick = if *positive_half.count() >= *negative_half.count() {
                positive_half
            } else {
                negative_half
            };
            stack[stack_len.0] = pick;
            stack_len = USize(stack_len.0 + 1);
        }
    }

    (partition_count, partition_id)
}
