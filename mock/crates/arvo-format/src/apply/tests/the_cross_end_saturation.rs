//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Positions past one end of the index adapted into `Integer<3>`, worked by hand.
//!
//! `Integer<3>`'s range, `[-4, 3]`, sits at neither end of the index, so a
//! position carried past either end is out of range on the side it left by. The
//! oracle sweep in `the_oracle_sweep.rs` covers these bands as a whole; the
//! points here are reduced by hand instead, so they are evidence arrived at
//! another way rather than a second copy of the sweep. Each answer is checked
//! against the shipped surface, and where a broken map answers differently at the
//! same point, that answer is pinned beside it.

use super::the_broken_maps::{
    past_the_bottom_pins_high_off_the_bottom,
    past_the_top_pins_low_off_the_top,
    shipped,
    the_step_onto_the_bottom_pins_high,
};
use crate::adapt::{Adapt, Signature};
use crate::apply::{Dither, Exact, Fraction, adapt, round_slot};
use crate::overflow::{Policy, Saturate, Wrap};
use crate::points::Integer;
use crate::rounding::{Ceil, Floor, Mode};
use crate::slots::Slot;

/// `Integer<3>`'s own range, `[-4, 3]`.
fn integer_3() -> (Slot, Slot) {
    (Slot::at(-4), Slot::at(3))
}

#[test]
fn wrap_residues_hand_reduced_off_grid_into_integer_3() {
    // `Exact::between(Slot::at(i128::MIN), Fraction::of(-6, 4))` names a
    // position 1.5 slots below `i128::MIN`. Under `Floor` this rounds down to
    // 2 whole slots below (`i128::MIN` stays the pinned slot, `past = -2`,
    // `step = 0`). Reduced by hand: `i128::MIN` is exactly divisible by the
    // span 8, so `i128::MIN.rem_euclid(8) = 0`; `(-2i128).rem_euclid(8) = 6`;
    // `(-4i128).rem_euclid(8) = 4` (the range's own lowest slot, `-4`); the
    // offset is `0 + 6 - 4 + 0 = 2`, and the answer is `lo + 2 = -2`.
    type Bottom = Signature<Integer<3>, Adapt<Floor, Wrap>>;
    let bottom = Exact::between(Slot::at(i128::MIN), Fraction::of(-6, 4));
    assert_eq!(adapt::<Bottom>(bottom, Dither::UNUSED), Slot::at(-2));

    // The mirror: `Fraction::of(6, 4)` past `i128::MAX`, 1.5 slots above it.
    // Under `Ceil` this rounds up (`past = 1`, `step = 1`). Reduced by hand:
    // `i128::MAX.rem_euclid(8) = 7` (`i128::MAX = i128::MIN - 1` in two's
    // complement, one below a multiple of 8); `1i128.rem_euclid(8) = 1`; the
    // range's own lowest slot reduces the same as above, `4`; the offset is
    // `7 + 1 - 4 + 1 = 5`, and the answer is `lo + 5 = 1`.
    type Top = Signature<Integer<3>, Adapt<Ceil, Wrap>>;
    let top = Exact::between(Slot::at(i128::MAX), Fraction::of(6, 4));
    assert_eq!(adapt::<Top>(top, Dither::UNUSED), Slot::at(1));
}

#[test]
fn two_and_a_quarter_past_either_end_saturates_on_that_side() {
    // Two whole slots past the bottom and a quarter, into `[-4, 3]`: below the
    // range, so `-4`. `past_the_bottom_pins_high_off_the_bottom` answers `3`.
    type S = Signature<Integer<3>, Adapt<Floor, Saturate>>;
    let exact = Exact::between(Slot::at(i128::MIN), Fraction::of(-9, 4));
    assert_eq!(adapt::<S>(exact, Dither::UNUSED), Slot::at(-4));

    let (min, max) = integer_3();
    let rounded = round_slot(Mode::Floor, exact, Dither::UNUSED);
    assert_eq!(
        (shipped().complete)(Policy::Saturate, rounded, min, max),
        Slot::at(-4)
    );
    assert_eq!(
        (past_the_bottom_pins_high_off_the_bottom().complete)(Policy::Saturate, rounded, min, max),
        Slot::at(3)
    );

    // And the mirror past the top: above the range, so `3`.
    type Mirror = Signature<Integer<3>, Adapt<Ceil, Saturate>>;
    let up = Exact::between(Slot::at(i128::MAX), Fraction::of(9, 4));
    assert_eq!(adapt::<Mirror>(up, Dither::UNUSED), Slot::at(3));
    let rounded_up = round_slot(Mode::Ceil, up, Dither::UNUSED);
    assert_eq!(
        (shipped().complete)(Policy::Saturate, rounded_up, min, max),
        Slot::at(3)
    );
    assert_eq!(
        (past_the_top_pins_low_off_the_top().complete)(Policy::Saturate, rounded_up, min, max),
        Slot::at(-4)
    );
}

#[test]
fn a_quarter_under_the_index_ceils_onto_its_bottom_and_saturates_low() {
    // A quarter under `i128::MIN`, rounded up, is `i128::MIN` itself: one step
    // from one slot under the index. That is below `[-4, 3]`, so saturation
    // answers `-4`. `the_step_onto_the_bottom_pins_high` answers `3` here and
    // nowhere a position two or more slots under the index reaches.
    type S = Signature<Integer<3>, Adapt<Ceil, Saturate>>;
    let exact = Exact::between(Slot::at(i128::MIN), Fraction::of(-1, 4));
    assert_eq!(adapt::<S>(exact, Dither::UNUSED), Slot::at(-4));

    let rounded = round_slot(Mode::Ceil, exact, Dither::UNUSED);
    assert_eq!(
        (rounded.past, rounded.down(), rounded.step()),
        (-1, i128::MIN, 1)
    );
    let (min, max) = integer_3();
    assert_eq!(
        (the_step_onto_the_bottom_pins_high().complete)(Policy::Saturate, rounded, min, max),
        Slot::at(3)
    );
    assert_eq!(
        (the_step_onto_the_bottom_pins_high().complete)(Policy::Clamp, rounded, min, max),
        Slot::at(3)
    );
}
