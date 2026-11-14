//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The shipped applied map and the broken ones the laws have to report.
//!
//! Each broken map differs from the shipped one in one place, so a law that
//! reports it has seen that place, and one that does not says where its reach
//! ends. They are kept as maps rather than as edits somebody made once and
//! reverted, so the laws stay checked against them. Two broken wraps reduce the
//! four-limb floor as the shipped one does and differ only in how the lowest
//! slot is reduced; the one that subtracts first works in the index's own
//! integer and reads the floor narrowed to it. The oracle sweep reports each of
//! the three at a position the index holds too. Three maps differ from the
//! shipped one only past the index, each the shape a step that held its floor
//! in the index's integer would take: a wrap over the narrowed floor, a wrap
//! that saturates there, and a tie whose parity is read from the narrowed
//! floor.
//!
//! The planted tie rules are broken in the rounding region rather than the
//! completion, and each differs from the shipped map at a tie only, under the one
//! mode it plants. The first is the rule this crate shipped for `HalfUp` before
//! the canon settled what `half_up` denotes, so a sweep that reports it is one
//! that would have caught that defect.

use core::cmp::Ordering;

use notko::Maybe;

use crate::apply::{Dither, Exact, Rounded, complete_slot, round_slot};
use crate::overflow::Policy;
use crate::rounding::Mode;
use crate::slots::Slot;
use crate::wide::Wide;

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

/// The floor narrowed to the index's own integer, pinned at the index's end on
/// its own side where it is past it.
fn narrowed(r: Rounded) -> i128 {
    match r.down.index() {
        Maybe::Is(at) => at,
        Maybe::Isnt if r.down.is_negative().get() => i128::MIN,
        Maybe::Isnt => i128::MAX,
    }
}

/// Where the floor sits against the index: `Less` below it, `Equal` in it,
/// `Greater` above it.
fn side(r: Rounded) -> Ordering {
    match r.down.index() {
        Maybe::Is(_) => Ordering::Equal,
        Maybe::Isnt if r.down.is_negative().get() => Ordering::Less,
        Maybe::Isnt => Ordering::Greater,
    }
}

/// Whether the floor is the slot one under the index.
fn one_under(r: Rounded) -> bool {
    r.down == Wide::of_sum(i128::MIN, -1)
}

/// A wrap that subtracts the lowest slot before reducing, in the index's own
/// integer, wrapping where the difference leaves it, which is what the naive
/// subtraction does in a build without overflow checks. The floor is narrowed
/// to the index first, since the subtraction is in its integer.
pub(super) fn subtracts_first() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy != Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let span = hi - lo + 1;
        Slot::at(
            lo + narrowed(r)
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
        Slot::at(lo + (r.down.residue(span) + r.step()).rem_euclid(span))
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
        let offset = r.down.residue(span) - lo.rem_euclid(256) + r.step();
        Slot::at(lo + offset.rem_euclid(span))
    }
    Map {
        round: round_slot,
        complete,
        leaves: shipped_leaves,
    }
}

/// The shipped wrap over the floor narrowed to the index's own integer.
///
/// Inside the index it is the shipped wrap. Past it the floor is pinned at the
/// index's end, so every position past the index on one side wraps to the one
/// slot that end reduces to, which is the completion of a step that held its
/// floor in the index's integer and kept no distance past it.
pub(super) fn narrows_the_floor() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy != Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let span = hi - lo + 1;
        let offset = narrowed(r).rem_euclid(span) - lo.rem_euclid(span) + r.step();
        Slot::at(lo + offset.rem_euclid(span))
    }
    Map {
        round: round_slot,
        complete,
        leaves: shipped_leaves,
    }
}

/// A wrap that saturates a floor past the index instead of reducing it.
///
/// Inside the index it is the shipped wrap. Past it the floor answers the
/// range's end on its own side, which is the refusal to read a floor the index
/// does not hold, turned into an answer.
pub(super) fn saturates_past_the_index() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        match (policy, side(r)) {
            (Policy::Wrap, Ordering::Less) => min,
            (Policy::Wrap, Ordering::Greater) => max,
            _ => complete_slot(policy, r, min, max),
        }
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
        let (lo, hi) = (Wide::of(min.index()), Wide::of(max.index()));
        let from_below = !matches!(r.down.ordering(lo), Ordering::Less);
        let from_above = match r.down.ordering(hi) {
            Ordering::Less => true,
            Ordering::Equal => !r.up.get(),
            Ordering::Greater => false,
        };
        !(from_below && from_above)
    }
    Map {
        round: round_slot,
        complete: complete_slot,
        leaves,
    }
}

/// The Saturate/Clamp arm with a floor under the index sent below the range
/// only where the range's own lowest slot is `i128::MIN`.
///
/// `complete_slot` sends every floor under the range's lowest slot to that
/// slot, wherever the range sits. This narrows the floors under the index to
/// ranges whose own bottom is `i128::MIN`, so a position under the index, fed
/// into a range whose bottom is not `i128::MIN`, answers the range's highest
/// slot instead of its lowest. A test feeding such a position only into ranges
/// that do start at `i128::MIN` cannot see this, on the grid or off it.
pub(super) fn past_the_bottom_pins_high_off_the_bottom() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let lo = min.index();
        if policy == Policy::Wrap || r.lands_within(lo, max.index()) {
            return complete_slot(policy, r, min, max);
        }
        let low = match side(r) {
            Ordering::Less => lo == i128::MIN,
            Ordering::Equal => narrowed(r) < lo,
            Ordering::Greater => false,
        };
        if low { min } else { max }
    }
    Map {
        round: round_slot,
        complete,
        leaves: shipped_leaves,
    }
}

/// The mirror of `past_the_bottom_pins_high_off_the_bottom`: a floor over the
/// index sent below the range wherever the range's own top is not `i128::MAX`.
///
/// A position over the index, fed into a range whose own top is not
/// `i128::MAX`, answers the range's lowest slot instead of its highest.
pub(super) fn past_the_top_pins_low_off_the_top() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let (lo, hi) = (min.index(), max.index());
        if policy == Policy::Wrap || r.lands_within(lo, hi) {
            return complete_slot(policy, r, min, max);
        }
        let low = match side(r) {
            Ordering::Less => true,
            Ordering::Equal => narrowed(r) < lo,
            Ordering::Greater => hi != i128::MAX,
        };
        if low { min } else { max }
    }
    Map {
        round: round_slot,
        complete,
        leaves: shipped_leaves,
    }
}

/// The Saturate/Clamp arm's guard under the index, narrowed a second,
/// independent way: conditional on the rounding not having stepped up, rather
/// than on where the fed range's own bottom sits.
///
/// `complete_slot` reads only the floor, never the step, to tell the two ends
/// apart. This sends a floor under the index whose rounding stepped up, fed
/// into a range whose own bottom is not `i128::MIN`, to the range's highest
/// slot instead of its lowest. An on-grid position never steps up, so a matrix
/// fed only whole-slot positions cannot see this one, where
/// `past_the_bottom_pins_high_off_the_bottom` is wrong on the grid as well.
pub(super) fn a_step_up_past_the_bottom_pins_high() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        let lo = min.index();
        if policy == Policy::Wrap || r.lands_within(lo, max.index()) {
            return complete_slot(policy, r, min, max);
        }
        let low = match side(r) {
            Ordering::Less => lo == i128::MIN || !r.up.get(),
            Ordering::Equal => narrowed(r) < lo,
            Ordering::Greater => false,
        };
        if low { min } else { max }
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
/// lowest slot is above it, so the shipped arm sends it to that lowest slot.
/// This sends exactly that step to the highest slot instead. It is wrong only
/// one slot under the index and only when the rounding steps up, which is the
/// band a feed starting two slots under the index never reaches.
pub(super) fn the_step_onto_the_bottom_pins_high() -> Map {
    fn complete(policy: Policy, r: Rounded, min: Slot, max: Slot) -> Slot {
        if policy != Policy::Wrap
            && one_under(r)
            && r.up.get()
            && !r.lands_within(min.index(), max.index())
        {
            return max;
        }
        complete_slot(policy, r, min, max)
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
/// The slot half is the shipped one; only the verdict takes that one step as
/// landing, which is the shape a verdict that tests the index's bottom rather
/// than the range's produces.
pub(super) fn no_lo_guard_onto_the_lowest() -> Map {
    fn leaves(r: Rounded, min: Slot, max: Slot) -> bool {
        if one_under(r) && r.up.get() {
            return false;
        }
        !r.lands_within(min.index(), max.index())
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

/// A planted `HalfUp` that sends a tie away from zero, down below zero and up
/// above it.
///
/// The wrong rule this crate shipped before the ruling. It differs from the
/// shipped map at a negative tie and nowhere else, so a sweep reports it only
/// where it feeds one.
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

/// `HalfEven` with a tie's parity read from the floor narrowed to the index.
///
/// Inside the index it is the shipped rounding. Past it the narrowed floor is
/// the index's end, odd at the top and even at the bottom, so a tie whose own
/// floor has the other parity is sent to the wrong neighbour. It is kept out of
/// `planted_tie_rules`, whose sweep feeds no position past the index.
pub(super) fn half_even_reads_a_narrowed_parity() -> Map {
    fn round(mode: Mode, e: Exact, d: Dither) -> Rounded {
        if mode == Mode::HalfEven && e.is_tie().get() && matches!(e.floor().index(), Maybe::Isnt) {
            // `i128::MAX` is odd, so its tie goes up; `i128::MIN` is even.
            let odd = !e.floor().is_negative().get();
            return round_slot(if odd { Mode::Ceil } else { Mode::Floor }, e, d);
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
