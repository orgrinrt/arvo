//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A range placed at the very top of the slot index, adapted.
//!
//! `AtTheTop` is admitted by the slot range's obligation, so every verb owes it
//! the answers a range at zero gets. The rounding step is where it could not
//! have them: the slot above a position just under one past `i128::MAX` is a
//! position the index does not hold, so a step that forms it has to saturate
//! or overflow, and either answer is wrong under wrapping and in the overflow
//! verdict. These arms feed exactly those positions.

use crate::adapt::{Adapt, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow};
use crate::overflow::{Clamp, Saturate, Wrap};
use crate::quantum::Constant;
use crate::rounding::{Ceil, Floor, HalfEven, HalfUp, Stochastic, TowardZero};
use crate::slots::Slot;
use crate::tests::grid::Grid;
use crate::tests::the_inventory::{AtTheBottom, AtTheTop};

/// A plain integer format over the range at the top of the index.
type Top = Grid<BinaryRationals, Constant<0>, AtTheTop, 0, 1>;

/// The same format over the range at the bottom of the index.
type Bottom = Grid<BinaryRationals, Constant<0>, AtTheBottom, 0, 1>;

/// The lowest slot of `AtTheTop`, written down rather than read from the range.
const fn top_lowest() -> Slot {
    Slot::at(i128::MAX - 255)
}

/// The highest slot of `AtTheTop`.
const fn top_highest() -> Slot {
    Slot::at(i128::MAX)
}

/// A position a quarter of the way from the top of the index to one past it.
fn quarter_past_the_top() -> Exact {
    Exact::between(Slot::at(i128::MAX), Fraction::of(1, 4))
}

#[test]
fn ceil_just_under_one_past_the_top_wraps_to_the_range_s_lowest_slot() {
    // The position is between `i128::MAX` and one past it, so `Ceil` names one
    // past it, and one past the top of a range ending at `i128::MAX` wraps to
    // the range's lowest slot. Worked by hand: the range is 256 slots, one past
    // its top is 256 above its lowest, and 256 mod 256 is zero.
    type Up = Signature<Top, Adapt<Ceil, Wrap>>;
    assert_eq!(
        adapt::<Up>(quarter_past_the_top(), Dither::UNUSED),
        top_lowest()
    );
    assert_ne!(
        adapt::<Up>(quarter_past_the_top(), Dither::UNUSED),
        top_highest(),
        "the step up was pinned at the top of the index rather than taken"
    );
}

#[test]
fn every_mode_answers_at_the_top_as_it_answers_by_hand() {
    // Down is the top of the index, up is one past it and wraps to the lowest
    // slot. `i128::MAX` is odd, so the even neighbour of a tie is the one above.
    macro_rules! wraps {
        ($mode:ty, $part:expr, $want:expr) => {{
            type S = Signature<Top, Adapt<$mode, Wrap>>;
            let e = Exact::between(Slot::at(i128::MAX), $part);
            assert_eq!(
                adapt::<S>(e, Dither::UNUSED),
                $want,
                "{} at {:?}",
                stringify!($mode),
                $part
            );
        }};
    }
    let (lo, hi) = (top_lowest(), top_highest());
    let quarter = Fraction::of(1, 4);
    let half = Fraction::HALF;
    let three = Fraction::of(3, 4);

    wraps!(Floor, quarter, hi);
    wraps!(Floor, three, hi);
    wraps!(Ceil, quarter, lo);
    wraps!(Ceil, three, lo);
    wraps!(TowardZero, quarter, hi);
    wraps!(TowardZero, three, hi);
    wraps!(HalfUp, quarter, hi);
    wraps!(HalfUp, half, lo);
    wraps!(HalfUp, three, lo);
    wraps!(HalfEven, quarter, hi);
    wraps!(HalfEven, half, lo);
    wraps!(HalfEven, three, lo);

    // The stochastic mode steps up exactly when the dither is below the offset.
    type Dithered = Signature<Top, Adapt<Stochastic, Wrap>>;
    let e = quarter_past_the_top();
    assert_eq!(adapt::<Dithered>(e, Dither::at(Fraction::of(1, 8))), lo);
    assert_eq!(adapt::<Dithered>(e, Dither::at(Fraction::of(3, 8))), hi);
}

#[test]
fn saturating_and_clamping_pin_a_step_past_the_top_to_the_top() {
    // The control on the policy: where the step is taken, saturation answers the
    // range's highest slot, which is also where a saturated step would have
    // landed. So these two pass either way, and the wrapping arms above are the
    // ones that separate the two.
    type Sat = Signature<Top, Adapt<Ceil, Saturate>>;
    type Pin = Signature<Top, Adapt<Ceil, Clamp>>;
    assert_eq!(
        adapt::<Sat>(quarter_past_the_top(), Dither::UNUSED),
        top_highest()
    );
    assert_eq!(
        adapt::<Pin>(quarter_past_the_top(), Dither::UNUSED),
        top_highest()
    );
}

#[test]
fn a_step_past_the_top_is_reported_as_leaving_the_range() {
    type Up = Signature<Top, Adapt<Ceil, Wrap>>;
    type Down = Signature<Top, Adapt<Floor, Wrap>>;
    type Even = Signature<Top, Adapt<HalfEven, Saturate>>;
    assert!(
        panic_on_overflow::<Up>(quarter_past_the_top(), Dither::UNUSED).get(),
        "a position rounding to one past the index was reported as in range"
    );
    assert!(!panic_on_overflow::<Down>(quarter_past_the_top(), Dither::UNUSED).get());
    assert!(
        panic_on_overflow::<Even>(
            Exact::between(Slot::at(i128::MAX), Fraction::HALF),
            Dither::UNUSED
        )
        .get()
    );
    assert!(
        !panic_on_overflow::<Even>(
            Exact::between(Slot::at(i128::MAX), Fraction::of(1, 4)),
            Dither::UNUSED
        )
        .get()
    );

    // One slot lower the step lands on the top, which is in range.
    let under = Exact::between(Slot::at(i128::MAX - 1), Fraction::of(1, 4));
    assert!(!panic_on_overflow::<Up>(under, Dither::UNUSED).get());
    assert_eq!(adapt::<Up>(under, Dither::UNUSED), top_highest());

    // And a step from just under the lowest slot lands on it, in range.
    let below = Exact::between(Slot::at(i128::MAX - 256), Fraction::of(1, 4));
    assert!(!panic_on_overflow::<Up>(below, Dither::UNUSED).get());
    assert_eq!(adapt::<Up>(below, Dither::UNUSED), top_lowest());
    assert!(panic_on_overflow::<Down>(below, Dither::UNUSED).get());
}

#[test]
fn a_carry_past_the_index_pins_at_the_constructor_and_the_map_adapts_what_is_stored() {
    // `9/4` from the top names `i128::MAX + 9/4`, which the index cannot hold.
    // The constructor pins the slot at `i128::MAX` and keeps the quarter, so the
    // position stored is `i128::MAX + 1/4`, and that is what the map adapts:
    // `Ceil` names one past the top and wraps to the lowest slot. The named
    // position would have ceiled to three past the top and wrapped to two above
    // the lowest slot, which is the answer this construction does not give.
    let carried = Exact::between(Slot::at(i128::MAX), Fraction::of(9, 4));
    assert_eq!(carried.slot(), Slot::at(i128::MAX));
    assert_eq!(carried, quarter_past_the_top());
    type Up = Signature<Top, Adapt<Ceil, Wrap>>;
    type Down = Signature<Top, Adapt<Floor, Wrap>>;
    assert_eq!(adapt::<Up>(carried, Dither::UNUSED), top_lowest());
    assert_ne!(
        adapt::<Up>(carried, Dither::UNUSED),
        Slot::at(top_lowest().index() + 2),
        "the carry landed where the named position would, so it was not pinned"
    );
    assert_eq!(adapt::<Down>(carried, Dither::UNUSED), top_highest());
    assert!(panic_on_overflow::<Up>(carried, Dither::UNUSED).get());

    // What was stored is exactly what the same carry from two slots lower
    // names, and there the carry fits, so the pin is the constructor answering
    // for a different position. The control: from four slots lower the carry
    // lands exactly and `Ceil` names the slot the arithmetic says.
    let fits = Exact::between(Slot::at(i128::MAX - 2), Fraction::of(9, 4));
    assert_eq!(carried, fits);
    let lower = Exact::between(Slot::at(i128::MAX - 4), Fraction::of(9, 4));
    assert_eq!(lower.slot(), Slot::at(i128::MAX - 2));
    assert_eq!(adapt::<Up>(lower, Dither::UNUSED), Slot::at(i128::MAX - 1));

    // At the bottom, `-7/4` from `i128::MIN` names `i128::MIN - 7/4` and stores
    // `i128::MIN + 1/4`. `Floor` adapts the stored one onto `i128::MIN`; the
    // named one floors to two below it and would wrap to 254 above it.
    let under = Exact::between(Slot::at(i128::MIN), Fraction::of(-7, 4));
    assert_eq!(under.slot(), Slot::at(i128::MIN));
    assert_eq!(
        under,
        Exact::between(Slot::at(i128::MIN), Fraction::of(1, 4))
    );
    type Low = Signature<Bottom, Adapt<Floor, Wrap>>;
    assert_eq!(adapt::<Low>(under, Dither::UNUSED), Slot::at(i128::MIN));
    assert_ne!(
        adapt::<Low>(under, Dither::UNUSED),
        Slot::at(i128::MIN + 254),
        "the carry landed where the named position would, so it was not pinned"
    );
    assert!(!panic_on_overflow::<Low>(under, Dither::UNUSED).get());

    // What was stored is what the same carry from two slots up names, and the
    // control: from four slots up the carry fits and floors where the
    // arithmetic says.
    assert_eq!(
        under,
        Exact::between(Slot::at(i128::MIN + 2), Fraction::of(-7, 4))
    );
    let inside = Exact::between(Slot::at(i128::MIN + 4), Fraction::of(-7, 4));
    assert_eq!(inside.slot(), Slot::at(i128::MIN + 2));
    assert_eq!(
        adapt::<Low>(inside, Dither::UNUSED),
        Slot::at(i128::MIN + 2)
    );
}

#[test]
fn at_the_bottom_the_step_never_leaves_the_index() {
    // The control. The step up from any slot of a range at the bottom is a slot
    // the index holds, so this range was right before the repair and has to stay
    // right after it.
    type Up = Signature<Bottom, Adapt<Ceil, Wrap>>;
    let top_of_bottom = Exact::between(Slot::at(i128::MIN + 255), Fraction::of(1, 4));
    assert_eq!(
        adapt::<Up>(top_of_bottom, Dither::UNUSED),
        Slot::at(i128::MIN)
    );
    assert!(panic_on_overflow::<Up>(top_of_bottom, Dither::UNUSED).get());
    let first = Exact::between(Slot::at(i128::MIN), Fraction::of(1, 4));
    assert_eq!(adapt::<Up>(first, Dither::UNUSED), Slot::at(i128::MIN + 1));
    assert!(!panic_on_overflow::<Up>(first, Dither::UNUSED).get());
}
