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
//! Returns the bin assignment per item in original input order. Items
//! that do not fit any bin map to `NUSize::NONE`; items that placed
//! map to `NUSize::some(bin_id)`. The prior overloaded `USize(0)`
//! sentinel for "did not fit" is replaced by `NUSize::NONE` so logical
//! 0 stays a valid bin index distinct from the absent marker.

use core::cmp::Ordering;
use core::ops::Add;

use arvo::{Identity, Bool, NUSize, USize};
use arvo::traits::{FromConstant, TotalOrd};
use arvo_tensor::{Array, Capacity, cap_size};

/// First-fit bin packing with affinity-based pre-ordering.
///
/// `items` is a fixed-size `Array`; `weight_of` maps each to a weight;
/// `capacity` is the per-bin budget; `affinity` is the symmetric
/// proximity score used to order placement (high-affinity items go
/// first to keep co-firing pairs in the same bin).
///
/// Returns `(bin_count, bin_id_per_item)`. `bin_id_per_item` is an
/// `Array<NUSize, N>` keyed by original input order; consumers read
/// per-item placement via `into_maybe()` on each `NUSize` cell.
/// `bin_count` is the number of bins that received at least one item
/// (never greater than `B`).
pub fn bin_pack<N: Capacity, B: Capacity, T, W>(
    items: &Array<T, N>,
    capacity: W,
    weight_of: impl Fn(&T) -> W,
    affinity: impl Fn(&T, &T) -> W,
) -> (USize, Array<NUSize, N>)
where
    W: Add<Output = W> + TotalOrd + Copy + FromConstant,
{
    let zero = <W as FromConstant>::from_constant::<{ USize(0) }>();
    let mut bins_of_items: Array<NUSize, N> = Array::filled(NUSize::NONE);

    if cap_size(N::CAP) == 0 || cap_size(B::CAP) == 0 {
        return (USize(0), bins_of_items);
    }

    // Total-affinity score per item: sum over all other items.
    let mut score: Array<W, N> = Array::filled(zero);
    for a in 0..cap_size(N::CAP) {
        let mut s = zero;
        for b in 0..cap_size(N::CAP) {
            if a != b {
                s = s + affinity(items.get(USize(a)), items.get(USize(b)));
            }
        }
        score.set(USize(a), s);
    }

    // Index array sorted by `score` descending via insertion sort.
    // Bounded N, O(N^2) in the const size, no alloc.
    let mut order: Array<USize, N> = Array::from_fn(|i| i);
    for j in 1..cap_size(N::CAP) {
        let cur = *order.get(USize(j));
        let cur_score = *score.get(cur);
        let mut k = j;
        while k > 0 {
            let prev = *order.get(USize(k - 1));
            let prev_score = *score.get(prev);
            // Descending: move cur left while its score exceeds the
            // predecessor.
            if matches!(cur_score.total_cmp(prev_score), Ordering::Greater) {
                order.set(USize(k), prev);
                k -= 1;
            } else {
                break;
            }
        }
        order.set(USize(k), cur);
    }

    // Bin used-weight vector. All start at zero. A fit check compares
    // `used[b] + w` against `capacity` using `TotalOrd::total_cmp`,
    // which avoids requiring `Sub` on `W`.
    let mut used: Array<W, B> = Array::filled(zero);
    let mut opened = USize(0);

    // Walk items in affinity-descending order and place first-fit.
    for p in 0..cap_size(N::CAP) {
        let idx = *order.get(USize(p));
        let w = weight_of(items.get(idx));

        // Try existing opened bins in index order.
        let mut placed = Bool::FALSE;
        for b in 0..opened.0 {
            let after = *used.get(USize(b)) + w;
            // Fit if `after <= capacity`, i.e. not Greater.
            if !matches!(after.total_cmp(capacity), Ordering::Greater) {
                used.set(USize(b), after);
                bins_of_items.set(idx, NUSize::some(USize(b)));
                placed = Bool::TRUE;
                break;
            }
        }

        // Open a new bin if capacity allows and we are under `B`.
        if !placed.0 && opened.0 < cap_size(B::CAP) {
            if !matches!(w.total_cmp(capacity), Ordering::Greater) {
                used.set(opened, w);
                bins_of_items.set(idx, NUSize::some(opened));
                opened = opened + USize::ONE;
            }
        }

        // Items that do not fit a fresh bin (weight > capacity) or
        // that arrive after `B` bins are already full stay at
        // `NUSize::NONE`. Consumers detect via `into_maybe()` on the
        // per-item cell, distinct from any logical bin index.
    }

    (opened, bins_of_items)
}
