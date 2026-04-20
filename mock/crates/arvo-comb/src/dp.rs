//! Matrix-chain interval DP.
//!
//! Classic `O(N^3)` interval DP: find the split `k` of each interval
//! `[i..j]` that minimises `cost(i, k) + cost(k+1, j)` subject to a
//! caller-supplied feasibility predicate. The DP table is stack-only:
//! `[[W; N]; N]` for costs plus a parallel `[[bool; N]; N]` feasibility
//! table and a `[[USize; N]; N]` split-point table.
//!
//! The returned split array records, for each interval length along
//! the root `[0..N-1]`, the split point chosen by the DP. Consumers
//! walk the array to reconstruct the grouping.

use core::cmp::Ordering;
use core::ops::Add;

use arvo::newtype::{Cap, USize};
use arvo::traits::{FromConstant, TotalOrd};
use arvo_bitmask::cap_size;

/// Minimise the total cost of splitting `[0..N]` into intervals.
///
/// - `cost(i, j)` is the cost of the interval `[i..j]` treated as a
///   leaf (inclusive `i`, inclusive `j`).
/// - `feasible(i, j)` is `true` when the interval `[i..j]` is a legal
///   leaf (i.e. it may be taken whole without further splitting).
///
/// Returns `(optimal_cost, splits)` where `splits[i]` is the chosen
/// split point for the interval that opens at index `i` along the
/// recorded root decomposition. For `N <= 1` the function returns
/// `(cost(0, 0), [USize(0); N])`.
///
/// If no feasible leaf exists for some sub-interval, the DP falls
/// back on composing feasible children; if even that fails, the
/// returned cost reflects the best feasible split discovered, and
/// `splits` contains `USize(0)` for unreachable entries.
pub fn matrix_chain_dp<const N: Cap, W>(
    cost: impl Fn(USize, USize) -> W,
    feasible: impl Fn(USize, USize) -> bool,
) -> (W, [USize; cap_size(N)])
where
    [(); cap_size(N)]:,
    W: Add<Output = W> + TotalOrd + Copy + FromConstant,
{
    let zero = <W as FromConstant>::from_constant(0);
    let mut splits: [USize; cap_size(N)] = [USize(0); cap_size(N)];

    if cap_size(N) == 0 {
        return (zero, splits);
    }

    let root_cost = cost(USize(0), USize(0));
    if cap_size(N) == 1 {
        return (root_cost, splits);
    }

    // dp[i][j] = best cost for interval [i..j] (inclusive, j >= i).
    // reachable[i][j] = true when dp[i][j] holds a meaningful value.
    // split[i][j] = chosen split point k in [i, j) when composed.
    let mut dp: [[W; cap_size(N)]; cap_size(N)] = [[zero; cap_size(N)]; cap_size(N)];
    let mut reachable: [[bool; cap_size(N)]; cap_size(N)] =
        [[false; cap_size(N)]; cap_size(N)];
    let mut split: [[USize; cap_size(N)]; cap_size(N)] =
        [[USize(0); cap_size(N)]; cap_size(N)];

    // Base case: single-element intervals. Reachable only when feasible
    // as leaves; leaf cost is `cost(i, i)`.
    let mut i: usize = 0;
    while i < cap_size(N) {
        if feasible(USize(i), USize(i)) {
            dp[i][i] = cost(USize(i), USize(i));
            reachable[i][i] = true;
        }
        i += 1;
    }

    // Fill intervals of increasing length. `len` is inclusive width
    // minus one, so `len = 1` is pairs, up to `N - 1` for the root.
    let mut len: usize = 1;
    while len < cap_size(N) {
        let mut lo: usize = 0;
        while lo + len < cap_size(N) {
            let hi = lo + len;

            // Option A: take the whole interval as a feasible leaf.
            let mut best_val = zero;
            let mut best_set = false;
            let mut best_split = USize(lo);
            if feasible(USize(lo), USize(hi)) {
                best_val = cost(USize(lo), USize(hi));
                best_set = true;
                best_split = USize(hi);
            }

            // Option B: compose two reachable children over splits
            // k in [lo, hi). A split k assigns [lo..k] + [k+1..hi].
            let mut k: usize = lo;
            while k < hi {
                if reachable[lo][k] && reachable[k + 1][hi] {
                    let candidate = dp[lo][k] + dp[k + 1][hi];
                    if !best_set
                        || matches!(candidate.total_cmp(&best_val), Ordering::Less)
                    {
                        best_val = candidate;
                        best_set = true;
                        best_split = USize(k);
                    }
                }
                k += 1;
            }

            if best_set {
                dp[lo][hi] = best_val;
                reachable[lo][hi] = true;
                split[lo][hi] = best_split;
            }

            lo += 1;
        }
        len += 1;
    }

    // Walk the split tree from the root to fill the returned array.
    // `splits[i]` is populated from a preorder traversal in visit
    // order; unreachable intervals leave `USize(0)` sentinels.
    let mut out_idx: usize = 0;
    fill_splits::<N>(
        &split,
        &reachable,
        0,
        cap_size(N) - 1,
        &mut splits,
        &mut out_idx,
    );

    let final_cost = if reachable[0][cap_size(N) - 1] {
        dp[0][cap_size(N) - 1]
    } else {
        zero
    };

    (final_cost, splits)
}

/// Preorder-walk the `[[split; N]; N]` decomposition table, writing
/// each visited split into the output array. Leaf intervals (whose
/// recorded split equals their right endpoint) are skipped.
fn fill_splits<const N: Cap>(
    split: &[[USize; cap_size(N)]; cap_size(N)],
    reachable: &[[bool; cap_size(N)]; cap_size(N)],
    lo: usize,
    hi: usize,
    out: &mut [USize; cap_size(N)],
    out_idx: &mut usize,
) where
    [(); cap_size(N)]:,
{
    if lo >= hi || *out_idx >= cap_size(N) || !reachable[lo][hi] {
        return;
    }
    let k = split[lo][hi].0;
    // Leaf: the DP chose to take the interval as a whole (k == hi).
    if k >= hi {
        return;
    }
    out[*out_idx] = USize(k);
    *out_idx += 1;
    fill_splits::<N>(split, reachable, lo, k, out, out_idx);
    fill_splits::<N>(split, reachable, k + 1, hi, out, out_idx);
}
