//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The shipped applied map and the broken ones the laws have to report.
//!
//! Each broken map differs from the shipped one in one place, so a law that
//! reports it has seen that place, and one that does not says where its reach
//! ends. They are kept as maps rather than as edits somebody made once and
//! reverted, so the laws stay checked against them. The broken wraps read the
//! slot below the position and not the distance past the index, so past the
//! index they are wrong for that reason as well as their own, and the oracle
//! sweep reports each of them at a position the index holds too.
//!
//! The planted tie rules are broken in the rounding region rather than the
//! completion, and each differs from the shipped map at a tie only, under the one
//! mode it plants. The first is the rule this crate shipped for `HalfUp` before
//! the canon settled what `half_up` denotes, so a sweep that reports it is one
//! that would have caught that defect.

use crate::apply::{Dither, Exact, Rounded, complete_slot, round_slot};
use crate::overflow::Policy;
use crate::rounding::Mode;
use crate::slots::Slot;

/// The three answers of one applied map the laws compare.
#[derive(Clone, Copy)]
pub(super) struct Map {
    pub(super) round:    fn(Mode, Exact, Dither) -> Rounded,
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
        round:    round_slot,
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
        round: round_slot,
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
        round: round_slot,
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
        round: round_slot,
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
        round: round_slot,
        complete: complete_slot,
        leaves,
    }
}

/// The Saturate/Clamp arm with the near-bottom case narrowed to ranges whose own
/// lowest slot is `i128::MIN`.
///
/// `complete_slot`'s own near-bottom test is `rounded.past < 0`, unconditional
/// on where the range sits: any position carried past the bottom of the index
/// pins to the range's own lowest slot under Saturate and Clamp, wherever that
/// range is. This narrows it to `rounded.past < 0 && lo == i128::MIN`, so a
/// position carried past the bottom, fed into a range whose own bottom is not
/// `i128::MIN`, falls through to the `else` arm and answers the range's highest
/// slot instead of its lowest. A test feeding such a position only into ranges
/// that do start at `i128::MIN` cannot see this, on the grid or off it.
pub(super) fn past_the_bottom_pins_high_off_the_bottom() -> Map {
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
        round: round_slot,
        complete,
        leaves: shipped_leaves,
    }
}

/// The mirror of `past_the_bottom_pins_high_off_the_bottom`: the near-top case
/// widened to answer `min` for a range whose own top is not `i128::MAX`.
///
/// A position carried past the top of the index, fed into a range whose own
/// top is not `i128::MAX`, answers the range's lowest slot instead of its
/// highest.
pub(super) fn past_the_top_pins_low_off_the_top() -> Map {
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
        round: round_slot,
        complete,
        leaves: shipped_leaves,
    }
}

/// The Saturate/Clamp arm's near-bottom guard, narrowed a second, independent
/// way: conditional on the rounding not having stepped up, rather than on
/// where the fed range's own bottom sits.
///
/// `complete_slot`'s own near-bottom test is `rounded.past < 0`, which does
/// not read `rounded.up` at all. This narrows it to `rounded.past < 0 &&
/// (lo == i128::MIN || !rounded.up.get())`, so a position carried past the
/// bottom whose rounding stepped up, fed into a range whose own bottom is not
/// `i128::MIN`, falls through to the `else` arm and answers the range's
/// highest slot instead of its lowest. An on-grid position never sets
/// `rounded.up`, so a matrix fed only whole-slot positions cannot see this
/// one, where `past_the_bottom_pins_high_off_the_bottom` is wrong on the grid
/// as well.
pub(super) fn a_step_up_past_the_bottom_pins_high() -> Map {
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
                if (r.past < 0 && (lo == i128::MIN || !r.up.get()))
                    || (r.past == 0 && r.down() < lo)
                {
                    min
                } else {
                    max
                }
            },
        }
    }
    Map {
        round: round_slot,
        complete,
        leaves: shipped_leaves,
    }
}

/// The Saturate/Clamp arm with the one step from just under the index onto
/// `i128::MIN` sent to the range's highest slot.
///
/// The rounded position is then `i128::MIN` itself, below any range whose own
/// lowest slot is above it, so the shipped arm pins it at that lowest slot.
/// This reads `rounded.past < 0 && !(rounded.past == -1 && rounded.up.get())`
/// instead, so exactly that step falls through to the `else` arm. It is wrong
/// only one slot under the index and only when the rounding steps up, which
/// is the band a feed starting two slots under the index never reaches.
pub(super) fn the_step_onto_the_bottom_pins_high() -> Map {
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
                if (r.past < 0 && !(r.past == -1 && r.up.get())) || (r.past == 0 && r.down() < lo) {
                    min
                } else {
                    max
                }
            },
        }
    }
    Map {
        round: round_slot,
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
        round: round_slot,
        complete: complete_slot,
        leaves,
    }
}

// --- the planted tie rules ------------------------------------------------------

/// The shipped completion and verdict over a planted rounding.
fn planted(round: fn(Mode, Exact, Dither) -> Rounded) -> Map {
    Map {
        round,
        complete: complete_slot,
        leaves: shipped_leaves,
    }
}

/// `HalfUp` with a tie sent away from zero: down below zero, up above it.
///
/// The rule this crate shipped before the ruling. It differs from the shipped
/// map at a negative tie and nowhere else, so a sweep reports it only where it
/// feeds one.
pub(super) fn half_up_ties_away_from_zero() -> Map {
    fn round(mode: Mode, e: Exact, d: Dither) -> Rounded {
        if mode == Mode::HalfUp && e.is_tie().get() {
            let away = if e.is_negative() { Mode::Floor } else { Mode::Ceil };
            return round_slot(away, e, d);
        }
        round_slot(mode, e, d)
    }
    planted(round)
}

/// `HalfUp` with every tie sent down, which differs at every tie.
pub(super) fn half_up_ties_down() -> Map {
    fn round(mode: Mode, e: Exact, d: Dither) -> Rounded {
        if mode == Mode::HalfUp && e.is_tie().get() {
            return round_slot(Mode::Floor, e, d);
        }
        round_slot(mode, e, d)
    }
    planted(round)
}

/// `HalfUp` with a tie sent toward zero, which differs at a non-negative tie.
pub(super) fn half_up_ties_toward_zero() -> Map {
    fn round(mode: Mode, e: Exact, d: Dither) -> Rounded {
        if mode == Mode::HalfUp && e.is_tie().get() {
            return round_slot(Mode::TowardZero, e, d);
        }
        round_slot(mode, e, d)
    }
    planted(round)
}

/// The neighbour with an odd slot, at a tie.
fn to_odd(e: Exact, d: Dither) -> Rounded {
    round_slot(if e.is_even() { Mode::Ceil } else { Mode::Floor }, e, d)
}

/// `HalfUp` with a tie sent to the odd neighbour, which differs at a tie whose
/// slot below is odd.
pub(super) fn half_up_ties_to_odd() -> Map {
    fn round(mode: Mode, e: Exact, d: Dither) -> Rounded {
        if mode == Mode::HalfUp && e.is_tie().get() {
            return to_odd(e, d);
        }
        round_slot(mode, e, d)
    }
    planted(round)
}

/// `HalfEven` with a tie sent to the odd neighbour, which differs at every tie.
pub(super) fn half_even_ties_to_odd() -> Map {
    fn round(mode: Mode, e: Exact, d: Dither) -> Rounded {
        if mode == Mode::HalfEven && e.is_tie().get() {
            return to_odd(e, d);
        }
        round_slot(mode, e, d)
    }
    planted(round)
}

/// Every planted tie rule, by name.
pub(super) fn planted_tie_rules() -> [(&'static str, Map); 5] {
    [
        ("half_up_ties_away_from_zero", half_up_ties_away_from_zero()),
        ("half_up_ties_down", half_up_ties_down()),
        ("half_up_ties_toward_zero", half_up_ties_toward_zero()),
        ("half_up_ties_to_odd", half_up_ties_to_odd()),
        ("half_even_ties_to_odd", half_even_ties_to_odd()),
    ]
}
