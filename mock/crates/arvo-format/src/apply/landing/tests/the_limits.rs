//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the four limbs bound: a difference far finer than both its terms, a
//! denominator past the host's integer, a point past the 128-bit index, and the
//! nothing past the remainder's limit.

use super::*;
use crate::overflow::Wrap;

/// A falling law of 300 magnitudes from `2^0`, so its finest step is `2^-299`.
struct Deep;

impl Quantum for Deep {
    const BASE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(300);
    const SLOPE: Exponent = Exponent::of(-1);
}

struct DeepPhased;

impl Format for DeepPhased {
    type Ambient = BinaryRationals;
    type Quantum = Deep;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::halves(1);
}

#[test]
fn a_point_a_hair_off_a_phase_lands_at_the_finest_magnitude_holding_it() {
    // `(2^126 + 1) / (2^127 + 1)` is `1/2 + 1/(2^128 + 2)`, so on step `2^-135` it
    // sits at `128 - e`, slot 127, and on `2^-136` it is past. Both terms of the
    // position leave the four limbs long before magnitude 135; their difference
    // does not.
    let point = Scaled::of((1 << 126) + 1, nz((1 << 127) + 1), Exponent::ZERO);
    let Maybe::Is(got) = land::<Under<DeepPhased, TowardZero>>(point, Dither::UNUSED) else {
        panic!("magnitude 135 holds the point");
    };
    assert_eq!(got.magnitude(), Magnitude::at(135));
    assert!(!got.exact().is_on_grid().get());
    assert_eq!(
        adapt::<Under<DeepPhased, TowardZero>>(got.exact(), Dither::UNUSED),
        Slot::at(127)
    );
}

#[test]
fn past_the_remainder_s_limit_nothing_lands() {
    // On the integer grid `2^-k` is slot zero with the remainder `1 / 2^k`, whose
    // denominator the four limbs hold up to `2^255`.
    let at = |power| {
        land::<Under<Integer<8>, TowardZero>>(
            Scaled::of(1, nz(1), Exponent::of(power)),
            Dither::UNUSED,
        )
    };
    assert!(matches!(at(-255), Maybe::Is(l) if l.exact().slot() == Slot::ZERO));
    assert!(matches!(at(-256), Maybe::Isnt));
    assert!(matches!(at(-300), Maybe::Isnt));
}

#[test]
fn past_the_index_a_wrap_reads_the_point_s_residue() {
    // `2^128 + 4` is past the 128-bit index, and modulo the 16 slots of
    // `Integer<4>` it is 4; its negation is -4. Reading the index's end instead
    // gives -1 and 0.
    type Wrapped = Signature<Integer<4>, Adapt<TowardZero, Wrap>>;
    for (sign, slot) in [(1, 4), (-1, -4)] {
        let point = Scaled::of(sign * ((1 << 126) + 1), nz(1), Exponent::of(2));
        let Maybe::Is(got) = land::<Wrapped>(point, Dither::UNUSED) else {
            panic!("a wrapped point past the index has a residue");
        };
        assert_eq!(
            adapt::<Wrapped>(got.exact(), Dither::UNUSED),
            Slot::at(slot)
        );
    }
    // `2^130` is a multiple of 16, so it wraps to slot 0.
    let Maybe::Is(got) = land::<Wrapped>(Scaled::of(1, nz(1), Exponent::of(130)), Dither::UNUSED)
    else {
        panic!("a wrapped point past the index has a residue");
    };
    assert_eq!(adapt::<Wrapped>(got.exact(), Dither::UNUSED), Slot::ZERO);
}

#[test]
fn a_point_whose_denominator_passes_128_bits_still_lands() {
    // `7 / (2^128 - 1)` at `2^-20` on the integer grid is `D = (2^128 - 1) 2^20`,
    // and `7` shares no factor with it, so the remainder keeps a denominator past
    // the host's 128-bit integer: slot zero, a little above it.
    let point = Scaled::of(7, nz(u128::MAX), Exponent::of(-20));
    let Maybe::Is(down) = land::<Under<Integer<8>, TowardZero>>(point, Dither::UNUSED) else {
        panic!("the point is in range at the one magnitude");
    };
    let Maybe::Is(up) = land::<Under<Integer<8>, Ceil>>(point, Dither::UNUSED) else {
        panic!("the point is in range at the one magnitude");
    };
    assert_eq!(down.exact().slot(), Slot::ZERO);
    assert!(!down.exact().is_on_grid().get());
    assert_eq!(
        adapt::<Under<Integer<8>, TowardZero>>(down.exact(), Dither::UNUSED),
        Slot::ZERO
    );
    assert_eq!(
        adapt::<Under<Integer<8>, Ceil>>(up.exact(), Dither::UNUSED),
        Slot::at(1)
    );
    // The pair is `7 / D` in lowest terms, and the accessors hand out neither
    // half, since the denominator has no host integer to be read back in.
    let Maybe::Is(scale) = Wide::power(2, 20) else {
        panic!("the power is formed in the four limbs");
    };
    let Maybe::Is(den) = Wide::of_unsigned(u128::MAX).times(scale) else {
        panic!("the denominator is formed in the four limbs");
    };
    let remainder = down.exact().part;
    assert_eq!(remainder, Remainder::reduced(Wide::of(7), den));
    assert!(matches!(remainder.numerator(), Maybe::Isnt));
    assert!(matches!(remainder.denominator(), Maybe::Isnt));
}
