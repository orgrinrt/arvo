//! Affinity-ordered first-fit bin packing.
//!
//! Pre-sort item indices by total pairwise affinity (sum of `affinity`
//! from an item to every other item), then first-fit each item into
//! the earliest bin whose remaining capacity allows it. Opens a new
//! bin when no existing bin fits, up to the `B` upper bound.
//!
//! The numeric bound is `Add + TotalOrd + Copy + FromConstant`; no
//! subtraction. Bin state is tracked as "used weight", and fit checks
//! compare `used + w` against the capacity directly.
//!
//! Returns the bin assignment per item in original input order; items
//! that do not fit any bin leave the sentinel index `USize(0)`.

use core::cmp::Ordering;
use core::ops::Add;

use arvo::newtype::{Cap, USize};
use arvo::traits::{FromConstant, TotalOrd};
use arvo_bitmask::cap_size;

/// First-fit bin packing with affinity-based pre-ordering.
///
/// `items` is a fixed-size array; `weight_of` maps each to a weight;
/// `capacity` is the per-bin budget; `affinity` is the symmetric
/// proximity score used to order placement (high-affinity items go
/// first to keep co-firing pairs in the same bin).
///
/// Returns `(bin_count, bin_id_per_item)`. `bin_id_per_item[i]` is the
/// bin index for `items[i]`, keyed by original order. `bin_count` is
/// the number of bins that received at least one item (never greater
/// than `B`).
pub fn bin_pack<const N: Cap, const B: Cap, T, W>(
    items: &[T; cap_size(N)],
    capacity: W,
    weight_of: impl Fn(&T) -> W,
    affinity: impl Fn(&T, &T) -> W,
) -> (USize, [USize; cap_size(N)])
where
    [(); cap_size(N)]:,
    [(); cap_size(B)]:,
    W: Add<Output = W> + TotalOrd + Copy + FromConstant,
{
    let zero = <W as FromConstant>::from_constant(0);
    let mut bins_of_items: [USize; cap_size(N)] = [USize(0); cap_size(N)];

    if cap_size(N) == 0 || cap_size(B) == 0 {
        return (USize(0), bins_of_items);
    }

    // Total-affinity score per item: sum over all other items.
    let mut score: [W; cap_size(N)] = [zero; cap_size(N)];
    let mut a: usize = 0;
    while a < cap_size(N) {
        let mut s = zero;
        let mut b: usize = 0;
        while b < cap_size(N) {
            if a != b {
                s = s + affinity(&items[a], &items[b]);
            }
            b += 1;
        }
        score[a] = s;
        a += 1;
    }

    // Index array sorted by `score` descending via insertion sort.
    // Bounded N, O(N^2) in the const size, no alloc.
    let mut order: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    let mut i: usize = 0;
    while i < cap_size(N) {
        order[i] = USize(i);
        i += 1;
    }
    let mut j: usize = 1;
    while j < cap_size(N) {
        let cur = order[j];
        let cur_score = score[cur.0];
        let mut k: usize = j;
        while k > 0 {
            let prev = order[k - 1];
            let prev_score = score[prev.0];
            // Descending: move cur left while its score exceeds the
            // predecessor.
            if matches!(cur_score.total_cmp(&prev_score), Ordering::Greater) {
                order[k] = prev;
                k -= 1;
            } else {
                break;
            }
        }
        order[k] = cur;
        j += 1;
    }

    // Bin used-weight vector. All start at zero. A fit check compares
    // `used[b] + w` against `capacity` using `TotalOrd::total_cmp`,
    // which avoids requiring `Sub` on `W`.
    let mut used: [W; cap_size(B)] = [zero; cap_size(B)];
    let mut opened: usize = 0;

    // Walk items in affinity-descending order and place first-fit.
    let mut p: usize = 0;
    while p < cap_size(N) {
        let idx = order[p].0;
        let w = weight_of(&items[idx]);

        // Try existing opened bins in index order.
        let mut placed = false;
        let mut b: usize = 0;
        while b < opened {
            let after = used[b] + w;
            // Fit if `after <= capacity`, i.e. not Greater.
            if !matches!(after.total_cmp(&capacity), Ordering::Greater) {
                used[b] = after;
                bins_of_items[idx] = USize(b);
                placed = true;
                break;
            }
            b += 1;
        }

        // Open a new bin if capacity allows and we are under `B`.
        if !placed && opened < cap_size(B) {
            if !matches!(w.total_cmp(&capacity), Ordering::Greater) {
                used[opened] = w;
                bins_of_items[idx] = USize(opened);
                opened += 1;
            }
        }

        // Items that do not fit a fresh bin (weight > capacity) or
        // that arrive after `B` bins are already full remain at the
        // sentinel `USize(0)`. Consumers detect via `bin_count` and a
        // weight recheck.

        p += 1;
    }

    (USize(opened), bins_of_items)
}
