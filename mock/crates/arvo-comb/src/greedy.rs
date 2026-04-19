//! Sequential greedy interval grouping.
//!
//! Walks items left-to-right, extending the current group while the
//! feasibility predicate holds. When the predicate fails, the current
//! group is emitted as a `Range` and a new group opens at the current
//! index. The output is a fixed-size `[Range; M]` plus a count; `M` is
//! the caller-supplied maximum group count.
//!
//! Used as the fallback under the DP threshold (small problems) and as
//! the default for pipeline grouping where the DP's quadratic table is
//! oversized.

use arvo::newtype::USize;

use crate::range::Range;

/// Walk `items` in order, accumulating into a single group while the
/// feasibility predicate holds and emitting groups on infeasibility.
///
/// - `feasible(acc, item)` returns `true` when `item` can extend the
///   current group whose accumulator is `acc`.
/// - `merge(acc, item)` folds `item` into the accumulator.
/// - `init()` produces the fresh accumulator for a new group.
///
/// The returned `(count, groups)` has `count <= M`; only the first
/// `count` entries of `groups` are meaningful. If the algorithm would
/// exceed `M`, it returns early with `count == M` and the tail of the
/// input is dropped from the result.
///
/// If a just-reset accumulator still reports an item as infeasible,
/// that item is skipped to avoid an infinite loop. A well-formed
/// predicate should always accept the first item of a fresh group.
pub fn greedy_group<const N: usize, const M: usize, A, T>(
    items: &[T; N],
    feasible: impl Fn(&A, &T) -> bool,
    merge: impl Fn(A, &T) -> A,
    init: impl Fn() -> A,
) -> (USize, [Range; M]) {
    let mut groups: [Range; M] = [Range::default(); M];
    let mut count: usize = 0;

    // Empty input: no groups.
    if N == 0 || M == 0 {
        return (USize(count), groups);
    }

    // Open the first group on item 0.
    let mut acc: A = init();
    let mut range_start: usize = 0;
    let mut open = false;
    let mut i: usize = 0;

    while i < N {
        let item = &items[i];

        if !open {
            // Open a new group. If the predicate rejects the first
            // item against a fresh accumulator, skip to keep the
            // walk terminating.
            if !feasible(&acc, item) {
                i += 1;
                continue;
            }
            acc = merge(acc, item);
            range_start = i;
            open = true;
            i += 1;
            continue;
        }

        if feasible(&acc, item) {
            acc = merge(acc, item);
            i += 1;
            continue;
        }

        // Close the open group at `[range_start, i)`.
        groups[count] = Range {
            start: USize(range_start),
            end: USize(i),
        };
        count += 1;
        if count == M {
            return (USize(count), groups);
        }

        // Start a fresh accumulator; retry the same item against it.
        acc = init();
        open = false;
        // Do not advance `i`; the outer loop retries with a fresh acc.
    }

    // Close the trailing open group.
    if open && count < M {
        groups[count] = Range {
            start: USize(range_start),
            end: USize(N),
        };
        count += 1;
    }

    (USize(count), groups)
}
