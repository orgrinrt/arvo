//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A point no magnitude holds lands at the top magnitude, where the policy reads it.

use super::*;
use crate::overflow::{Clamp, Wrap};

type Clamped<F, R> = Signature<F, Adapt<R, Clamp>>;
type Wrapped<F, R> = Signature<F, Adapt<R, Wrap>>;

/// The magnitude and the slot `adapt` reads off a landing, or nothing.
fn read<S: DeclaredSignature>(num: i128, power: i32) -> Maybe<(Magnitude, Slot)> {
    match land::<S>(Scaled::of(num, nz(1), Exponent::of(power)), Dither::UNUSED) {
        Maybe::Is(l) => Maybe::Is((l.magnitude(), adapt::<S>(l.exact(), Dither::UNUSED))),
        Maybe::Isnt => Maybe::Isnt,
    }
}

#[test]
fn past_the_top_magnitude_a_point_lands_on_the_end_it_is_past() {
    // The top magnitude's step is `2^1` and its slots run from -8 to 7, so toward
    // zero 15 and -17 are the last points a range holds, and 16 and -18 are past.
    type F = Floating<4, -6, 8>;
    let top = Magnitude::at(7);
    assert_eq!(
        read::<Under<F, TowardZero>>(15, 0),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<F, TowardZero>>(-17, 0),
        Maybe::Is((top, Slot::at(-8)))
    );
    assert_eq!(
        read::<Under<F, TowardZero>>(16, 0),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<F, TowardZero>>(-18, 0),
        Maybe::Is((top, Slot::at(-8)))
    );
    assert_eq!(
        read::<Clamped<F, TowardZero>>(16, 0),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Clamped<F, Ceil>>(-18, 0),
        Maybe::Is((top, Slot::at(-8)))
    );
    // Far past, at a position the four limbs cannot form, the side still decides.
    assert_eq!(
        read::<Under<F, HalfEven>>(i128::MAX, 200),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<F, HalfEven>>(i128::MIN, 200),
        Maybe::Is((top, Slot::at(-8)))
    );
}

#[test]
fn a_falling_law_s_top_magnitude_is_its_first() {
    // Magnitude zero carries the step `2^1`, so 14 is the largest member and -16
    // the smallest.
    let top = Magnitude::at(0);
    assert_eq!(
        read::<Under<FallingFour, TowardZero>>(15, 0),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<FallingFour, TowardZero>>(16, 0),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<FallingFour, Floor>>(-17, 0),
        Maybe::Is((top, Slot::at(-8)))
    );
    assert_eq!(
        read::<Under<FallingFour, Floor>>(-1000, 3),
        Maybe::Is((top, Slot::at(-8)))
    );
}

#[test]
fn a_phased_law_s_ends_are_its_top_magnitude_s() {
    // The phase is `1/32`, the top step `2^1`, so the ends are `1/32 + 14` and
    // `1/32 - 16`: 15 rounds down onto the first, and 16 is past it.
    let top = Magnitude::at(5);
    assert_eq!(
        read::<Under<PhasedFour, TowardZero>>(15, 0),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<PhasedFour, Ceil>>(17, 0),
        Maybe::Is((top, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<PhasedFour, Floor>>(-17, 0),
        Maybe::Is((top, Slot::at(-8)))
    );
}

#[test]
fn past_a_constant_quantum_s_range_the_wrap_is_the_grid_s_own() {
    // `Integer<4>` holds -8 to 7, so 9 wraps to -7 and -9 to 7.
    let only = Magnitude::at(0);
    assert_eq!(
        read::<Wrapped<Integer<4>, TowardZero>>(9, 0),
        Maybe::Is((only, Slot::at(-7)))
    );
    assert_eq!(
        read::<Wrapped<Integer<4>, TowardZero>>(-9, 0),
        Maybe::Is((only, Slot::at(7)))
    );
    assert_eq!(
        read::<Under<Integer<4>, TowardZero>>(9, 0),
        Maybe::Is((only, Slot::at(7)))
    );
}
