//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A position at one end of the index adapted into a range at the other.
//!
//! The difference between the slot below such a position and the range's lowest
//! slot is about `2^128` and leaves the index, which is why the wrap reduces the
//! two separately before subtracting. Every answer here is worked by hand.
//!
//! The ends of the index are `-2^127` and `2^127 - 1`, both congruent to a
//! range's own end modulo any power of two, so at spans of 256 and `2^64` a wrap
//! that subtracted first and let the difference wrap in the index's integer
//! would land on the same slot by accident. The ranges of 200 slots are what
//! can tell the two apart.

use super::the_broken_maps::{shipped, subtracts_first};
use super::the_translation_law::{WideBottom, WideTop, range_of};
use crate::adapt::{Adapt, Signature};
use crate::ambient::BinaryRationals;
use crate::apply::{Dither, Exact, Fraction, adapt, panic_on_overflow, round_slot};
use crate::overflow::{Clamp, Policy, SHIPPED_POLICIES, Saturate, Wrap};
use crate::quantum::Constant;
use crate::rounding::{Ceil, Floor, Mode, TowardZero};
use crate::slots::{Slot, Slots};
use crate::tests::grid::Grid;
use crate::tests::the_inventory::{AtTheBottom, AtTheTop};
use crate::width::Width;

/// 200 slots ending at the top of the index. Its lowest slot is 128 modulo 200.
pub(super) struct TopOf200;

impl Slots for TopOf200 {
    const MAX: Slot = Slot::at(i128::MAX);
    const MIN: Slot = Slot::at(i128::MAX - 199);
    const WIDTH: Width = Width::bits(8);
}

/// 200 slots from the bottom of the index. Its lowest slot is 72 modulo 200.
pub(super) struct BottomOf200;

impl Slots for BottomOf200 {
    const MAX: Slot = Slot::at(i128::MIN + 199);
    const MIN: Slot = Slot::at(i128::MIN);
    const WIDTH: Width = Width::bits(8);
}

/// A plain integer format over a slot range.
type Over<S> = Grid<BinaryRationals, Constant<0>, S, 0, 1>;

/// `2^64`, the widest span an admitted range has.
const fn span_of_64_bits() -> i128 {
    1 << 64
}

/// The bottom of the index, on the grid.
fn at_the_bottom() -> Exact {
    Exact::on_grid(Slot::at(i128::MIN))
}

/// A quarter above the bottom of the index, which `Floor` names as the bottom and
/// `Ceil` and `TowardZero`, the position being negative, as one above it.
fn quarter_above_the_bottom() -> Exact {
    Exact::between(Slot::at(i128::MIN), Fraction::of(1, 4))
}

/// A quarter above the top of the index, which `Floor` names as the top and
/// `Ceil` as one past it.
fn quarter_above_the_top() -> Exact {
    Exact::between(Slot::at(i128::MAX), Fraction::of(1, 4))
}

/// One position into one range under one signature, and the slot it must land on.
macro_rules! lands {
    ($range:ty, $exact:expr, $mode:ty, $policy:ty, $want:expr) => {{
        type S = Signature<Over<$range>, Adapt<$mode, $policy>>;
        assert_eq!(
            adapt::<S>($exact, Dither::UNUSED),
            Slot::at($want),
            "{} {} into {}",
            stringify!($mode),
            stringify!($policy),
            stringify!($range)
        );
        assert!(
            panic_on_overflow::<S>($exact, Dither::UNUSED).get(),
            "a position at the far end was reported inside {}",
            stringify!($range)
        );
    }};
}

#[test]
fn the_bottom_of_the_index_wraps_into_a_range_at_the_top() {
    // `-2^127` is zero modulo 256 and modulo `2^64`, and so is each range's
    // lowest slot, so the wrap lands on the lowest slot plus the step.
    let lo = i128::MAX - 255;
    lands!(AtTheTop, at_the_bottom(), Floor, Wrap, lo);
    lands!(AtTheTop, quarter_above_the_bottom(), Floor, Wrap, lo);
    lands!(AtTheTop, quarter_above_the_bottom(), Ceil, Wrap, lo + 1);
    lands!(
        AtTheTop,
        quarter_above_the_bottom(),
        TowardZero,
        Wrap,
        lo + 1
    );
    lands!(AtTheTop, quarter_above_the_bottom(), Ceil, Saturate, lo);
    lands!(AtTheTop, quarter_above_the_bottom(), Ceil, Clamp, lo);

    let lo = i128::MAX - span_of_64_bits() + 1;
    lands!(WideTop, at_the_bottom(), Floor, Wrap, lo);
    lands!(WideTop, quarter_above_the_bottom(), Ceil, Wrap, lo + 1);
    lands!(
        WideTop,
        quarter_above_the_bottom(),
        TowardZero,
        Wrap,
        lo + 1
    );
    lands!(WideTop, quarter_above_the_bottom(), Floor, Saturate, lo);
    lands!(WideTop, quarter_above_the_bottom(), Ceil, Clamp, lo);

    // `-2^127` is 72 modulo 200 and the lowest slot is 128, so the wrap lands
    // 144 above the lowest slot, 55 below the top, plus the step.
    lands!(TopOf200, at_the_bottom(), Floor, Wrap, i128::MAX - 55);
    lands!(
        TopOf200,
        quarter_above_the_bottom(),
        Ceil,
        Wrap,
        i128::MAX - 54
    );
    lands!(
        TopOf200,
        quarter_above_the_bottom(),
        TowardZero,
        Wrap,
        i128::MAX - 54
    );
    lands!(
        TopOf200,
        quarter_above_the_bottom(),
        Floor,
        Saturate,
        i128::MAX - 199
    );
    lands!(
        TopOf200,
        quarter_above_the_bottom(),
        Ceil,
        Clamp,
        i128::MAX - 199
    );
}

#[test]
fn a_step_past_the_top_of_the_index_wraps_into_a_range_at_the_bottom() {
    // `2^127 - 1` is one below zero modulo 256 and modulo `2^64`, so the slot
    // below lands on the highest slot and the step past it on the lowest.
    lands!(
        AtTheBottom,
        quarter_above_the_top(),
        Floor,
        Wrap,
        i128::MIN + 255
    );
    lands!(AtTheBottom, quarter_above_the_top(), Ceil, Wrap, i128::MIN);
    lands!(
        AtTheBottom,
        quarter_above_the_top(),
        TowardZero,
        Wrap,
        i128::MIN + 255
    );
    lands!(
        AtTheBottom,
        quarter_above_the_top(),
        Ceil,
        Saturate,
        i128::MIN + 255
    );
    lands!(
        AtTheBottom,
        quarter_above_the_top(),
        Ceil,
        Clamp,
        i128::MIN + 255
    );

    let hi = i128::MIN + span_of_64_bits() - 1;
    lands!(WideBottom, quarter_above_the_top(), Floor, Wrap, hi);
    lands!(WideBottom, quarter_above_the_top(), Ceil, Wrap, i128::MIN);
    lands!(WideBottom, quarter_above_the_top(), Ceil, Saturate, hi);
    lands!(WideBottom, quarter_above_the_top(), Floor, Clamp, hi);

    // `2^127 - 1` is 127 modulo 200 and the lowest slot is 72, so the slot below
    // lands 55 above the lowest and the step past it one further.
    lands!(
        BottomOf200,
        quarter_above_the_top(),
        Floor,
        Wrap,
        i128::MIN + 55
    );
    lands!(
        BottomOf200,
        quarter_above_the_top(),
        Ceil,
        Wrap,
        i128::MIN + 56
    );
    lands!(
        BottomOf200,
        quarter_above_the_top(),
        Ceil,
        Saturate,
        i128::MIN + 199
    );
    lands!(
        BottomOf200,
        quarter_above_the_top(),
        Floor,
        Clamp,
        i128::MIN + 199
    );
}

/// Every far-end position this file feeds, into every range at the other end.
fn far_end_cases() -> [(Exact, (Slot, Slot), &'static str); 9] {
    [
        (at_the_bottom(), range_of::<AtTheTop>(), "256 at the top"),
        (
            quarter_above_the_bottom(),
            range_of::<AtTheTop>(),
            "256 at the top",
        ),
        (
            quarter_above_the_bottom(),
            range_of::<WideTop>(),
            "2^64 at the top",
        ),
        (at_the_bottom(), range_of::<TopOf200>(), "200 at the top"),
        (
            quarter_above_the_bottom(),
            range_of::<TopOf200>(),
            "200 at the top",
        ),
        (
            quarter_above_the_top(),
            range_of::<AtTheBottom>(),
            "256 at the bottom",
        ),
        (
            quarter_above_the_top(),
            range_of::<WideBottom>(),
            "2^64 at the bottom",
        ),
        (
            quarter_above_the_top(),
            range_of::<BottomOf200>(),
            "200 at the bottom",
        ),
        (
            at_the_bottom(),
            range_of::<BottomOf200>(),
            "200 at the bottom, in range",
        ),
    ]
}

#[test]
fn a_wrap_that_subtracts_first_goes_wrong_only_where_the_span_is_not_a_power_of_two() {
    // The negative control for the wrap's separate reductions, as a map the
    // suite keeps rather than an edit made once and reverted. It agrees with the
    // shipped map at every position inside one end of the index, which is why
    // the translation law cannot report it, and at the far end over the
    // power-of-two spans, where the wrapped difference is congruent to the
    // true one. It disagrees over 200 slots at both ends.
    let (good, bad) = (shipped(), subtracts_first());
    let mut disagreed = [false; 9];
    for (i, (exact, (min, max), _)) in far_end_cases().into_iter().enumerate() {
        for mode in [Mode::Floor, Mode::Ceil, Mode::TowardZero] {
            for policy in SHIPPED_POLICIES {
                let rounded = round_slot(mode, exact, Dither::UNUSED);
                if (good.complete)(policy, rounded, min, max)
                    != (bad.complete)(policy, rounded, min, max)
                {
                    assert_eq!(policy, Policy::Wrap, "only the wrap was changed");
                    disagreed[i] = true;
                }
            }
        }
    }
    for (i, (_, _, which)) in far_end_cases().into_iter().enumerate() {
        let want = which.starts_with("200") && !which.ends_with("in range");
        assert_eq!(disagreed[i], want, "case {i}, {which}");
    }
}
