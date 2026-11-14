//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The remainder's two comparisons at the top of its 128-bit coordinate.
//!
//! The rounding region reads a remainder twice: against the midpoint, and
//! against the dither. Both are checked against the exact rational answer formed
//! in the shared wide oracle, at denominators from two to `2^128 - 1` and at
//! numerators on both sides of half and at the ends, and then through the map,
//! every mode at those positions. Beside each sits the comparison a narrower
//! reading would make, doubling the numerator or cross-multiplying in the host's
//! own word, which has to disagree with the oracle somewhere in this set: the
//! set reaches the region where the coordinate's width is what decides.

use core::cmp::Ordering;

use crate::apply::{self, Dither, Exact, Fraction, Remainder};
use crate::rounding::{ALL_MODES, Mode};
use crate::slots::Slot;
use crate::tests::big::Big;
use crate::wide::Wide;

/// Denominators from the smallest past one to the top of the coordinate.
fn denominators() -> [u128; 8] {
    [
        2,
        3,
        i64::MAX as u128,
        (1 << 64) + 1,
        1 << 127,
        (1 << 127) + 1,
        u128::MAX - 1,
        u128::MAX,
    ]
}

/// Numerators below `den`: the ends, and each side of half.
fn numerators(den: u128) -> impl Iterator<Item = u128> {
    let half = den / 2;
    let all = [0, 1, half.saturating_sub(1), half, half + 1, den - 1];
    (0 .. all.len())
        .filter(move |&i| all[i] < den && !all[.. i].contains(&all[i]))
        .map(move |i| all[i])
}

/// Dithers through both signs, at and past one, and at the ends of the ratio.
fn dithers() -> [Fraction; 9] {
    [
        Fraction::of(-1, 2),
        Fraction::ZERO,
        Fraction::of(1, i64::MAX),
        Fraction::of(1, 3),
        Fraction::HALF,
        Fraction::of(i64::MAX - 1, i64::MAX),
        Fraction::of(i64::MAX, i64::MAX),
        Fraction::of(5, 1),
        Fraction::of(i64::MAX / 2 + 1, i64::MAX),
    ]
}

/// The sign of `x`, read off the oracle.
fn sign(x: Big) -> Ordering {
    if x == Big::of(0) {
        Ordering::Equal
    } else if x.is_negative() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

/// `num / den` against one half, exactly: the sign of `2 num - den`.
fn against_half(num: u128, den: u128) -> Ordering {
    let twice = Big::of_unsigned(num).plus(Big::of_unsigned(num));
    sign(twice.plus(Big::of_unsigned(den).times(Big::of(-1))))
}

/// Whether `dither` is below `num / den`, exactly: the sign of
/// `num * dither.den - dither.num * den`.
fn dither_is_below(num: u128, den: u128, dither: Fraction) -> bool {
    let right = Big::of_unsigned(num).times(Big::of(i128::from(dither.denominator())));
    let left = Big::of(i128::from(dither.numerator())).times(Big::of_unsigned(den));
    sign(right.plus(left.times(Big::of(-1)))) == Ordering::Greater
}

#[test]
fn the_midpoint_test_is_the_exact_comparison_with_half_at_every_width() {
    let mut checked = 0;
    let mut narrow_wrong = 0;
    for den in denominators() {
        for num in numerators(den) {
            let want = against_half(num, den);
            assert_eq!(
                Remainder::of(num, den).against_half(),
                want,
                "{num} / {den}"
            );
            // The narrow reading: `2 num` in the coordinate's own word.
            if num.wrapping_mul(2).cmp(&den) != want {
                narrow_wrong += 1;
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 2 + 3 + 6 * 6);
    assert!(
        narrow_wrong > 0,
        "doubling in the host's word never wrapped here"
    );
}

#[test]
fn the_dither_test_is_the_exact_comparison_at_every_width() {
    let mut checked = 0;
    let mut narrow_wrong = 0;
    for den in denominators() {
        for num in numerators(den) {
            for dither in dithers() {
                let want = dither_is_below(num, den, dither);
                assert_eq!(
                    Remainder::of(num, den).is_above(dither),
                    want,
                    "{dither:?} below {num} / {den}"
                );
                // The narrow reading: the cross product in the host's signed
                // 128-bit word, wrapping where it leaves it.
                let narrow = (dither.numerator() as i128).wrapping_mul(den as i128)
                    < (num as i128).wrapping_mul(i128::from(dither.denominator()));
                if narrow != want {
                    narrow_wrong += 1;
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, (2 + 3 + 6 * 6) * 9);
    assert!(
        narrow_wrong > 0,
        "the host's word held every cross product here"
    );
}

#[test]
fn every_mode_rounds_a_wide_remainder_by_its_exact_value() {
    // Through the map: down is the slot, up the slot and one more, and which
    // one is the mode's rule read against the exact comparisons above. The
    // slot is odd so the even rule steps up at a tie, and negative so toward
    // zero does too.
    let slot = Slot::at(-7);
    let mut stepped = 0;
    for den in denominators() {
        for num in numerators(den) {
            for dither in dithers() {
                let exact = Exact::at(slot, Remainder::of(num, den));
                for &mode in ALL_MODES.iter() {
                    let rounded = apply::round_slot(mode, exact, Dither::at(dither));
                    assert_eq!(rounded.down, Wide::of(-7));
                    let half = against_half(num, den);
                    let up = num != 0
                        && match mode {
                            Mode::Floor => false,
                            Mode::Ceil | Mode::TowardZero => true,
                            Mode::HalfUp => half != Ordering::Less,
                            Mode::HalfEven => half != Ordering::Less,
                            Mode::Stochastic => dither_is_below(num, den, dither),
                        };
                    assert_eq!(rounded.step(), i128::from(up), "{mode:?} at {num} / {den}");
                    stepped += i128::from(up);
                }
            }
        }
    }
    assert!(stepped > 0);
}

#[test]
fn a_remainder_equals_the_fraction_it_was_built_from_and_no_other() {
    let exact = Exact::between(Slot::ZERO, Fraction::of(3, 7));
    assert_eq!(exact.part, Fraction::of(3, 7));
    // The pair, not the ratio, the way `Fraction`'s own equality reads.
    assert_ne!(exact.part, Fraction::of(6, 14));
    assert_ne!(exact.part, Fraction::of(-3, 7));
    assert_eq!(exact.part, Remainder::of(3, 7));
    // A remainder past what a `Fraction` carries equals none of them.
    assert_ne!(Remainder::of(1, 1 << 64), Fraction::of(1, i64::MAX));
    // A numerator at or past the denominator keeps its residue, and a zero
    // denominator reads as nothing, so every value is in `[0, 1)`.
    assert_eq!(Remainder::of(9, 4), Remainder::of(1, 4));
    assert_eq!(
        Remainder::of(u128::MAX, u128::MAX),
        Remainder::of(0, u128::MAX)
    );
    assert!(Remainder::of(5, 0).is_zero());
}
