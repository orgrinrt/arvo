//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The edges of the slot index, which every arm in the parent stays away from.
//!
//! Thirty references to one small window meant the arithmetic could never leave
//! the index, so the breaking path was never entered and five review passes found
//! what the suite did not. These arms feed the index's own ends, and the ends of
//! the integer one size down, which is where the index used to stop and where a
//! position is now an ordinary one.

use super::{MAX5, MIN5};
use crate::adapt::{Adapt, Signature};
use crate::apply::{Dither, Exact, Fraction, adapt};
use crate::overflow::{Saturate, Wrap};
use crate::points::Integer;
use crate::rounding::{Ceil, Floor, Stochastic};
use crate::slots::Slot;

/// The wrapping answer, computed without the index's own subtraction.
///
/// The distance from the lowest slot to the position is taken as an unsigned
/// magnitude, which holds every distance between two indices, so the oracle
/// never overflows and never shares the separate reductions the crate performs.
fn wrap_of(slot: i128, min: Slot, max: Slot) -> Slot {
    let lo = min.index();
    let span = max.index().abs_diff(lo) + 1;
    let offset = if slot >= lo {
        slot.abs_diff(lo) % span
    } else {
        (span - slot.abs_diff(lo) % span) % span
    };
    Slot::at(lo + offset as i128)
}

/// Signature over the slot range `[-4, 3]`, span 8, which `Integer<3>` declares.
type Edge = Signature<Integer<3>, Adapt<Floor, Wrap>>;
const EDGE_MIN: Slot = Slot::at(-4);
const EDGE_MAX: Slot = Slot::at(3);

/// The positions fed at the edges: both ends of the index and their neighbours,
/// both ends of the integer one size down and the positions just past them, and
/// three nearer zero for contrast.
///
/// A function rather than an item constant, for the reason `the_ratio_coordinate`
/// gives: a const here is a coordinate spelled in the host's own type.
fn edges() -> [i128; 13] {
    [
        i128::MAX,
        i128::MIN,
        i128::MAX - 1,
        i128::MIN + 1,
        i64::MAX as i128,
        i64::MIN as i128,
        i64::MAX as i128 + 1,
        i64::MIN as i128 - 1,
        u64::MAX as i128,
        u64::MAX as i128 + 1,
        0,
        7,
        -9,
    ]
}

#[test]
fn the_control_the_edges_are_outside_the_window_the_other_arms_use() {
    // If the ends of the index were inside the window these arms would be
    // testing the in-range path under another name.
    assert!(i128::MAX > EDGE_MAX.index() && i128::MIN < EDGE_MIN.index());
    assert!(i128::MAX > MAX5.index() && i128::MIN < MIN5.index());
    let outside = edges()
        .iter()
        .filter(|&&index| index < EDGE_MIN.index() || index > EDGE_MAX.index())
        .count();
    assert_eq!(outside, edges().len() - 1, "only 0 was meant to sit inside");
}

#[test]
fn the_oracle_agrees_with_positions_worked_out_by_hand() {
    // The oracle is checked before anything is checked against it. Every value
    // here is reduced on paper: `i128::MAX` is seven modulo eight and
    // `i128::MIN` is zero, `i64::MAX` is seven and `u64::MAX` is seven.
    assert_eq!(wrap_of(i128::MAX, EDGE_MIN, EDGE_MAX), Slot::at(-1));
    assert_eq!(wrap_of(i128::MIN, EDGE_MIN, EDGE_MAX), Slot::ZERO);
    assert_eq!(wrap_of(i64::MAX as i128, EDGE_MIN, EDGE_MAX), Slot::at(-1));
    assert_eq!(
        wrap_of(i64::MAX as i128 + 1, EDGE_MIN, EDGE_MAX),
        Slot::ZERO
    );
    assert_eq!(wrap_of(u64::MAX as i128, EDGE_MIN, EDGE_MAX), Slot::at(-1));
    assert_eq!(wrap_of(-5, EDGE_MIN, EDGE_MAX), Slot::at(3));
    assert_eq!(wrap_of(4, EDGE_MIN, EDGE_MAX), Slot::at(-4));
    // And over a window nowhere near zero, every slot a span either side.
    for slot in -40i128 ..= 40 {
        let want = Slot::at(-4 + (slot + 4).rem_euclid(8));
        assert_eq!(wrap_of(slot, EDGE_MIN, EDGE_MAX), want, "{slot}");
    }
}

#[test]
fn adapting_at_the_edges_of_the_index_gives_the_arithmetic_answer() {
    // Asserted against the oracle, which reaches the answer by another route.
    // Before the exact step moved to a wide carrier the crate returned a value
    // inside the range that was simply wrong, which no assertion comparing it to
    // itself could have caught.
    for index in edges() {
        let got = adapt::<Edge>(Exact::on_grid(Slot::at(index)), Dither::UNUSED);
        let want = wrap_of(index, EDGE_MIN, EDGE_MAX);
        assert_eq!(got, want, "adapting {index} disagreed with the arithmetic");
        assert!(
            got.is_within(EDGE_MIN, EDGE_MAX).get(),
            "adapting {index} left the declared range"
        );
    }
}

#[test]
fn the_edge_answers_are_the_ones_worked_out_by_hand() {
    // Two values written down rather than derived, so the test does not agree
    // with the code by construction.
    //
    // `i128::MAX` into `[-4, 3]`: (i128::MAX + 4) mod 8 = 3, so -4 + 3 = -1.
    // `i128::MIN` into `[-4, 3]`: (i128::MIN + 4) mod 8 = 4, so -4 + 4 = 0.
    assert_eq!(
        adapt::<Edge>(Exact::on_grid(Slot::at(i128::MAX)), Dither::UNUSED),
        Slot::at(-1)
    );
    assert_eq!(
        adapt::<Edge>(Exact::on_grid(Slot::at(i128::MIN)), Dither::UNUSED),
        Slot::ZERO
    );
}

#[test]
fn rounding_one_past_the_integer_one_size_down_lands_the_position_it_names() {
    // `Ceil` on an off-grid position at `i64::MAX` rounds to one past the top of
    // `i64`. That is an ordinary position of the index, so the completion lands
    // it where the arithmetic says rather than where a saturated step would.
    type Up = Signature<Integer<3>, Adapt<Ceil, Wrap>>;
    let e = Exact::between(Slot::at(i64::MAX as i128), Fraction::of(1, 4));
    let got = adapt::<Up>(e, Dither::UNUSED);
    assert_eq!(got, wrap_of(i64::MAX as i128 + 1, EDGE_MIN, EDGE_MAX));
    assert_eq!(got, Slot::ZERO);
}

#[test]
fn rounding_past_the_top_of_the_index_wraps_the_position_it_names() {
    // One past `i128::MAX` is a position the index does not hold, and rounding
    // names it anyway, as the slot below and a step. Wrapping it into `[-4, 3]`
    // is exact: `i128::MAX` is seven modulo eight, so one past it is zero modulo
    // eight, and it lands on -4 + 4 = 0. `i128::MAX` itself lands on -1, so the
    // step moved the answer by the one slot it names.
    type UpWrap = Signature<Integer<3>, Adapt<Ceil, Wrap>>;
    type UpSat = Signature<Integer<3>, Adapt<Ceil, Saturate>>;
    type DownWrap = Signature<Integer<3>, Adapt<Floor, Wrap>>;
    let e = Exact::between(Slot::at(i128::MAX), Fraction::of(1, 4));
    assert_eq!(adapt::<UpWrap>(e, Dither::UNUSED), Slot::ZERO);
    assert_ne!(
        adapt::<UpWrap>(e, Dither::UNUSED),
        Slot::at(-1),
        "the step up was pinned at the top of the index, so wrapping lost a slot"
    );
    assert_eq!(adapt::<DownWrap>(e, Dither::UNUSED), Slot::at(-1));
    assert_eq!(adapt::<UpSat>(e, Dither::UNUSED), EDGE_MAX);

    // A carry out of the fraction past either end is where the constructor pins
    // the slot at the index's end and keeps the distance past it, rather than
    // saturating: `Exact::between` is not on the list of things that saturate
    // (DESIGN:818), so the position that reaches the map is still the one named.
    // `the_ratio_coordinate.rs:453-479` pins the kept distance, and
    // `the_top_of_the_index.rs` pins what the map then does with the pinned pair.
    let carried = Exact::between(Slot::at(i128::MAX), Fraction::of(9, 4));
    assert_eq!(carried.slot(), Slot::at(i128::MAX));
    assert_eq!(carried.past, 2);
    let carried = Exact::between(Slot::at(i128::MIN), Fraction::of(-9, 4));
    assert_eq!(carried.slot(), Slot::at(i128::MIN));
    assert_eq!(carried.past, -3);
}

#[test]
fn saturating_at_the_edges_pins_to_the_declared_ends() {
    type Sat = Signature<Integer<3>, Adapt<Floor, Saturate>>;
    for index in edges() {
        let got = adapt::<Sat>(Exact::on_grid(Slot::at(index)), Dither::UNUSED);
        let want = if index > EDGE_MAX.index() {
            EDGE_MAX
        } else if index < EDGE_MIN.index() {
            EDGE_MIN
        } else {
            Slot::at(index)
        };
        assert_eq!(got, want, "{index}");
    }
}

#[test]
fn a_dither_at_the_edges_still_selects_between_two_neighbours() {
    // The stochastic mode cross-multiplies, which is the other site that could
    // leave its integer, and this arm is what reaches it.
    //
    // Two things were wrong with the arm that stood here. It adapted through
    // `Edge`, which is a `Floor` signature and reads no dither at all, so the
    // path it is named for was never entered. And it asserted only that the two
    // answers sat inside the declared window, which under wrapping is true for
    // every input by construction, so the two dithers agreeing would not have
    // failed it.
    //
    // The position is one below the top of the index at a residue one part below
    // one, so both neighbours are slots the index holds, the upper one its very
    // top, and the products the mode compares are near two to the one hundred
    // and twenty-six.
    type EdgeStochastic = Signature<Integer<3>, Adapt<Stochastic, Wrap>>;
    let e = Exact::between(
        Slot::at(i128::MAX - 1),
        Fraction::of(i64::MAX - 1, i64::MAX),
    );
    let low = adapt::<EdgeStochastic>(e, Dither::at(Fraction::of(1, i64::MAX)));
    let high = adapt::<EdgeStochastic>(e, Dither::at(Fraction::of(i64::MAX - 1, i64::MAX)));

    for got in [low, high] {
        assert!(
            got.is_within(EDGE_MIN, EDGE_MAX).get(),
            "left the declared range"
        );
    }
    assert_ne!(
        low, high,
        "the two dithers picked the same neighbour, so the mode is not reading the dither"
    );

    // Each against the wrapping answer for the neighbour it should have picked.
    assert_eq!(low, wrap_of(i128::MAX, EDGE_MIN, EDGE_MAX));
    assert_eq!(high, wrap_of(i128::MAX - 1, EDGE_MIN, EDGE_MAX));

    // And both worked out on paper, so the arm does not rest on the oracle
    // alone. `i128::MAX` is seven modulo eight, so it gives (7 + 4) mod 8, which
    // is three and lands on -4 + 3; one below it gives two, landing on -2.
    assert_eq!(low, Slot::at(-1));
    assert_eq!(high, Slot::at(-2));

    // The control on the carrier: the products the mode compares do not fit the
    // fraction's own integer, so this arm reaches the one-wider comparison
    // rather than naming it.
    let offered = i128::from(i64::MAX - 1) * i128::from(i64::MAX);
    assert!(
        offered > i128::from(i64::MAX),
        "the comparison stays inside the fraction's integer, so the wide carrier is untested"
    );

    // The control on the signature: the same two dithers through `Edge`, which is
    // the `Floor` signature this arm used to adapt through, give one answer.
    assert_eq!(
        adapt::<Edge>(e, Dither::at(Fraction::of(1, i64::MAX))),
        adapt::<Edge>(e, Dither::at(Fraction::of(i64::MAX - 1, i64::MAX)))
    );
}
