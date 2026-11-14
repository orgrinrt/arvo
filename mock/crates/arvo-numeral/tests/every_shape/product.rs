//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

// The product sweep's body, included by one test file a radix.

use arvo_numeral::Sole;
use common::shapes::{SIGNATURES_OF_A_BAND, Shapes, minus_one, pairs, widest};
use common::{oracle, value};

/// The widest product of two members, as a magnitude.
const fn widest_product(w: u32, signed: bool) -> u128 {
    let m = widest(w, signed) as u128;
    m * m
}

const HELD: u128 = i128::MAX as u128;

/// The highest exponent up to one at which the widest product is still a
/// position a slot index holds.
const fn top(w: u32, signed: bool, radix: i128) -> i32 {
    let widest = widest_product(w, signed);
    match widest.checked_mul(radix as u128) {
        Some(scaled) if scaled <= HELD => 1,
        _ => whole(w, signed, radix),
    }
}

/// The highest exponent up to zero at which it is.
const fn whole(w: u32, signed: bool, _radix: i128) -> i32 {
    if widest_product(w, signed) <= HELD { 0 } else { -1 }
}

/// Minus half the width, rounded up.
const fn half(w: u32, _signed: bool, _radix: i128) -> i32 {
    -((w as i32 + 1) / 2)
}

/// Minus the width, or the deepest exponent whose power of the radix an exact
/// position can carry a remainder over, which is an unsigned 128-bit word.
const fn deep(w: u32, _signed: bool, radix: i128) -> i32 {
    let mut power = 1u128;
    let mut k = 0;
    while k < w as i32 {
        match power.checked_mul(radix as u128) {
            Some(next) => power = next,
            None => break,
        }
        k += 1;
    }
    -k
}

/// Every pair the sweep takes of `S`.
fn products<S: Sole>(shapes: &mut Shapes) {
    shapes.signatures += 1;
    for (a, b) in pairs::<S>() {
        let (va, vb) = (value::<S>(a), value::<S>(b));
        let expected = oracle::product::<S>(a, b);
        shapes.checked += 1;
        if va.mul(vb).slot().index() != expected.slot {
            shapes.wrong += 1;
        }
        assert_eq!(
            va.fits_mul(vb).get(),
            expected.fits,
            "`fits_mul` at {a} * {b}"
        );
    }
}

/// One format's products under every adaptation.
macro_rules! products_of {
    ($sign:ident, $format:ty, $w:literal, $radix:literal, $e:expr, ($shapes:expr)) => {
        every_adaptation!($format, products($shapes))
    };
}

/// A band of sixteen widths of one radix family.
macro_rules! band {
    ($name:ident, $widths:ident, $at:ident) => {
        #[test]
        fn $name() {
            let mut shapes = Shapes::default();
            $widths!($at!(
                [top, whole, minus_one, half, deep],
                products_of,
                (&mut shapes)
            ));
            shapes.agree("`mul`", SIGNATURES_OF_A_BAND);
        }
    };
}

/// The four width bands of one radix family, each a test.
macro_rules! bands {
    ($at:ident, $a:ident, $b:ident, $c:ident, $d:ident) => {
        band!($a, widths_1_to_16, $at);
        band!($b, widths_17_to_32, $at);
        band!($c, widths_33_to_48, $at);
        band!($d, widths_49_to_64, $at);
    };
}
