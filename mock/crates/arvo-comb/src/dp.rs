//! Matrix-chain interval DP.
//!
//! Classic `O(N^3)` interval DP: find the split `k` of each interval
//! `[i..j]` that minimises `cost(i, k) + cost(k+1, j)` subject to a
//! caller-supplied feasibility predicate. The DP tables are stack-only:
//! `Matrix<W, N>` for costs, `Matrix<Bool, N>` for reachability,
//! `Matrix<USize, N>` for the split-point decisions.
//!
//! The returned `Array<USize, N>` records, for each interval length
//! along the root `[0..N-1]`, the split point chosen by the DP.
//! Consumers walk the array to reconstruct the grouping.

use core::cmp::Ordering;
use core::ops::Add;

use arvo::predicate::Pred2;
use arvo::traits::{FromConstant, TotalOrd};
use arvo::{Additive, Bool, Identity, Multiplicative, USize};
use arvo_tensor::{cap_size, Array, Capacity, Matrix};

/// Minimise the total cost of splitting `[0..N]` into intervals.
///
/// - `cost(i, j)` is the cost of the interval `[i..j]` treated as a
///   leaf (inclusive `i`, inclusive `j`).
/// - `feasible(i, j)` is `Bool::TRUE` when the interval `[i..j]` is a
///   legal leaf (i.e. it may be taken whole without further splitting).
///
/// Returns `(optimal_cost, splits)` where `splits` is an
/// `Array<USize, N>` populated via preorder traversal of the chosen
/// split tree. For `N <= 1` the function returns `(cost(0, 0), zeros)`.
///
/// If no feasible leaf exists for some sub-interval, the DP falls
/// back on composing feasible children; if even that fails, the
/// returned cost reflects the best feasible split discovered, and
/// `splits` contains `USize(0)` for unreachable entries.
pub fn matrix_chain_dp<N: Capacity, W>(
    cost: impl Fn(USize, USize) -> W,
    feasible: impl Pred2<USize, USize>,
) -> (W, Array<USize, N>)
where
    W: Add<Output = W> + TotalOrd + Copy + FromConstant,
    N::Array<W>: Copy,
    N::Array<Bool>: Copy,
    N::Array<USize>: Copy,
{
    let zero = <W as FromConstant>::from_constant::<{ USize(0) }>();
    let mut splits: Array<USize, N> = Array::filled(USize(0));

    if cap_size(N::CAP) == 0 {
        return (zero, splits);
    }

    let root_cost = cost(USize(0), USize(0));
    if cap_size(N::CAP) == 1 {
        return (root_cost, splits);
    }

    // dp[i][j] = best cost for interval [i..j] (inclusive, j >= i).
    // reachable[i][j] = TRUE when dp[i][j] holds a meaningful value.
    // split[i][j] = chosen split point k in [i, j) when composed.
    let mut dp: Matrix<W, N> = Matrix::filled(zero);
    let mut reachable: Matrix<Bool, N> = Matrix::filled(Bool::FALSE);
    let mut split: Matrix<USize, N> = Matrix::filled(USize(0));

    // Base case: single-element intervals. Reachable only when feasible
    // as leaves; leaf cost is `cost(i, i)`.
    for i in 0..cap_size(N::CAP) {
        let iu = USize(i);
        if feasible.test(&iu, &iu).0 {
            dp.set(iu, iu, cost(iu, iu));
            reachable.set(iu, iu, Bool::TRUE);
        }
    }

    // Fill intervals of increasing length. `len` is inclusive width
    // minus one, so `len = 1` is pairs, up to `N - 1` for the root.
    for len in 1..cap_size(N::CAP) {
        let mut lo = 0usize;
        while lo + len < cap_size(N::CAP) {
            let hi = lo + len;
            let lou = USize(lo);
            let hiu = USize(hi);

            // Option A: take the whole interval as a feasible leaf.
            let mut best_val = zero;
            let mut best_set = Bool::FALSE;
            let mut best_split = lou;
            if feasible.test(&lou, &hiu).0 {
                best_val = cost(lou, hiu);
                best_set = Bool::TRUE;
                best_split = hiu;
            }

            // Option B: compose two reachable children over splits
            // k in [lo, hi). A split k assigns [lo..k] + [k+1..hi].
            for k in lo..hi {
                let ku = USize(k);
                let k1u = USize(k + 1);
                if reachable.get(lou, ku).0 && reachable.get(k1u, hiu).0 {
                    let candidate = dp.get(lou, ku) + dp.get(k1u, hiu);
                    if !best_set.0 || matches!(candidate.total_cmp(best_val), Ordering::Less) {
                        best_val = candidate;
                        best_set = Bool::TRUE;
                        best_split = ku;
                    }
                }
            }

            if best_set.0 {
                dp.set(lou, hiu, best_val);
                reachable.set(lou, hiu, Bool::TRUE);
                split.set(lou, hiu, best_split);
            }

            lo += 1;
        }
    }

    // Walk the split tree from the root to fill the returned array.
    // `splits[i]` is populated from a preorder traversal in visit
    // order; unreachable intervals leave `USize(0)` sentinels.
    let mut out_idx = USize(0);
    fill_splits::<N>(
        &split,
        &reachable,
        USize(0),
        USize(cap_size(N::CAP) - 1),
        &mut splits,
        &mut out_idx,
    );

    let root_end = USize(cap_size(N::CAP) - 1);
    let final_cost = if reachable
        .get(<USize as Identity<Additive>>::IDENTITY, root_end)
        .0
    {
        dp.get(<USize as Identity<Additive>>::IDENTITY, root_end)
    } else {
        zero
    };

    (final_cost, splits)
}

/// Preorder-walk the decomposition table, writing each visited split
/// into the output array. Leaf intervals (whose recorded split equals
/// their right endpoint) are skipped.
fn fill_splits<N: Capacity>(
    split: &Matrix<USize, N>,
    reachable: &Matrix<Bool, N>,
    lo: USize,
    hi: USize,
    out: &mut Array<USize, N>,
    out_idx: &mut USize,
) {
    if lo.0 >= hi.0 || out_idx.0 >= cap_size(N::CAP) || !reachable.get(lo, hi).0 {
        return;
    }
    let k = split.get(lo, hi);
    // Leaf: the DP chose to take the interval as a whole (k == hi).
    if k.0 >= hi.0 {
        return;
    }
    out.set(*out_idx, k);
    *out_idx = *out_idx + <USize as Identity<Multiplicative>>::IDENTITY;
    fill_splits::<N>(split, reachable, lo, k, out, out_idx);
    fill_splits::<N>(
        split,
        reachable,
        k + <USize as Identity<Multiplicative>>::IDENTITY,
        hi,
        out,
        out_idx,
    );
}
