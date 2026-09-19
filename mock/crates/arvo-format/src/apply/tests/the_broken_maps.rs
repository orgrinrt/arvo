//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The shipped applied map and the broken ones the laws have to report.
//!
//! Each broken map differs from the shipped one in one place, so a law that
//! reports it has seen that place, and one that does not says where its reach
//! ends. They are kept as maps rather than as edits somebody made once and
//! reverted, so the laws stay checked against them. The broken wraps are asked
//! only about positions the index holds, so they read the slot below the
//! position and not the distance past the index.

use crate::apply::{Rounded, complete_slot};
use crate::overflow::Policy;
use crate::slots::Slot;

/// The two answers of one applied map the laws compare.
#[derive(Clone, Copy)]
pub(super) struct Map {
    pub(super) complete: fn(Policy, Rounded, Slot, Slot) -> Slot,
    pub(super) leaves:   fn(Rounded, Slot, Slot) -> bool,
}

/// The shipped verdict, as `panic_on_overflow` asks it.
fn shipped_leaves(r: Rounded, min: Slot, max: Slot) -> bool {
    !r.lands_within(min.index(), max.index())
}

/// The map this crate ships.
pub(super) fn shipped() -> Map {
    Map {
        complete: complete_slot,
        leaves:   shipped_leaves,
    }
}

/// A wrap that subtracts the lowest slot before reducing, in the index's own
/// integer, wrapping where the difference leaves it, which is what the naive
/// subtraction does in a build without overflow checks.
pub(super) fn subtracts_first() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy != Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let span = hi - lo + 1;
        Slot::at(
            lo + r
                .down()
                .wrapping_sub(lo)
                .wrapping_add(r.step())
                .rem_euclid(span),
        )
    }
    Map {
        complete,
        leaves: shipped_leaves,
    }
}

/// A wrap reduced from zero rather than from the lowest slot.
pub(super) fn anchored_at_zero() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy != Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let span = hi - lo + 1;
        Slot::at(lo + (r.down().rem_euclid(span) + r.step()).rem_euclid(span))
    }
    Map {
        complete,
        leaves: shipped_leaves,
    }
}

/// A wrap that reduces the lowest slot modulo 256 rather than modulo the span.
///
/// Over a span of 256 the two reductions are the same, so only a range of
/// another span can tell this from the shipped wrap.
pub(super) fn reduced_modulo_256() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy != Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let span = hi - lo + 1;
        let offset = r.down().rem_euclid(span) - lo.rem_euclid(256) + r.step();
        Slot::at(lo + offset.rem_euclid(span))
    }
    Map {
        complete,
        leaves: shipped_leaves,
    }
}

/// An overflow verdict whose in-range test has no step onto the lowest slot.
///
/// The slot half is the shipped one: out of range, every policy sends a step
/// onto the lowest slot to the lowest slot anyway, so only the verdict can show
/// the missing branch.
pub(super) fn no_step_onto_the_lowest() -> Map {
    fn leaves(r: Rounded, min: Slot, max: Slot) -> bool {
        let (lo, hi) = (min.index(), max.index());
        !(r.down() >= lo && (r.down() < hi || (r.down() == hi && !r.up.get())))
    }
    Map {
        complete: complete_slot,
        leaves,
    }
}

/// The Saturate/Clamp arm with the near-bottom case narrowed to ranges whose own
/// lowest slot is `i128::MIN`, the review's `M1`.
///
/// `complete_slot`'s own near-bottom test is `rounded.past < 0`, unconditional
/// on where the range sits: any position carried past the bottom of the index
/// pins to the range's own lowest slot under Saturate and Clamp, wherever that
/// range is. This narrows it to `rounded.past < 0 && lo == i128::MIN`, so a
/// position carried past the bottom, fed into a range whose own bottom is not
/// `i128::MIN`, falls through to the `else` arm and answers the range's highest
/// slot instead of its lowest. Every hand test the suite carried before this
/// round fed such a position only into ranges that do start at `i128::MIN`, so
/// the mutation passed unnoticed.
pub(super) fn m1_no_far_lo_guard() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if r.lands_within(lo, hi) {
            if r.past != 0 {
                return min;
            }
            return Slot::at(r.down() + r.step());
        }
        match policy {
            Policy::Wrap => complete_slot(policy, r, min, max),
            Policy::Saturate | Policy::Clamp => {
                if (r.past < 0 && lo == i128::MIN) || (r.past == 0 && r.down() < lo) {
                    min
                } else {
                    max
                }
            },
        }
    }
    Map {
        complete,
        leaves: shipped_leaves,
    }
}

/// The mirror of `m1_no_far_lo_guard`: the near-top case widened to answer
/// `min` for a range whose own top is not `i128::MAX`, the review's `M2`.
///
/// A position carried past the top of the index, fed into a range whose own
/// top is not `i128::MAX`, answers the range's lowest slot instead of its
/// highest.
pub(super) fn m2_no_far_hi_guard() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if r.lands_within(lo, hi) {
            if r.past != 0 {
                return min;
            }
            return Slot::at(r.down() + r.step());
        }
        match policy {
            Policy::Wrap => complete_slot(policy, r, min, max),
            Policy::Saturate | Policy::Clamp => {
                if r.past < 0 || (r.past == 0 && r.down() < lo) || (r.past > 0 && hi != i128::MAX) {
                    min
                } else {
                    max
                }
            },
        }
    }
    Map {
        complete,
        leaves: shipped_leaves,
    }
}

/// An overflow verdict whose step onto `i128::MIN` from just under the index is
/// in range for every range, not only for the one whose own lowest slot is
/// `i128::MIN`.
///
/// The slot half is the shipped one; only the verdict drops the `lo ==
/// i128::MIN` conjunct `lands_within` carries on its `past < 0` branch, which
/// is the shape a mutation that deletes that conjunct produces.
pub(super) fn no_lo_guard_onto_the_lowest() -> Map {
    fn leaves(r: Rounded, min: Slot, max: Slot) -> bool {
        let (lo, hi) = (min.index(), max.index());
        if r.past > 0 {
            return true;
        }
        if r.past < 0 {
            return !(r.past == -1 && r.up.get());
        }
        let from_below = r.down() >= lo || (r.up.get() && r.down() + 1 == lo);
        let from_above = r.down() < hi || (r.down() == hi && !r.up.get());
        !(from_below && from_above)
    }
    Map {
        complete: complete_slot,
        leaves,
    }
}
