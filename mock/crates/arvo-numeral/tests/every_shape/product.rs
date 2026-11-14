//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

// The product sweep's body, included by one test file a radix.

use arvo_numeral::Sole;
use common::edges::{product_deepest as deep, product_highest as top};
use common::shapes::{SIGNATURES_OF_A_BAND, Shapes, minus_one, pairs};
use common::{oracle, value};

/// The highest exponent up to zero the product admits.
const fn whole(w: u32, signed: bool, radix: i128) -> i32 {
    let top = top(w, signed, radix);
    if top < 0 { top } else { 0 }
}

/// Minus half the width, rounded up.
const fn half(w: u32, _signed: bool, _radix: i128) -> i32 {
    -((w as i32 + 1) / 2)
}

/// Both ends of the family at one width and radix, held against the format's
/// verdict on both ranges: admitted at each, refused one past it.
macro_rules! product_edges_at {
    ($w:literal, $radix:literal, $signed:ident, $unsigned:ident) => {{
        $crate::edge!(
            is_multipliable,
            $signed,
            $w,
            top($w, true, $radix),
            top($w, true, $radix) + 1,
            "the product's highest"
        );
        $crate::edge!(
            is_multipliable,
            $unsigned,
            $w,
            top($w, false, $radix),
            top($w, false, $radix) + 1,
            "the product's highest"
        );
        $crate::edge!(
            is_multipliable,
            $signed,
            $w,
            deep($w, true, $radix),
            deep($w, true, $radix) - 1,
            "the product's deepest"
        );
        $crate::edge!(
            is_multipliable,
            $unsigned,
            $w,
            deep($w, false, $radix),
            deep($w, false, $radix) - 1,
            "the product's deepest"
        );
    }};
}

/// The family's ends at every width of one radix family, one test.
macro_rules! product_edges {
    ($name:ident, $radix:literal, $signed:ident, $unsigned:ident) => {
        #[test]
        fn $name() {
            widths_1_to_16!(product_edges_at!($radix, $signed, $unsigned));
            widths_17_to_32!(product_edges_at!($radix, $signed, $unsigned));
            widths_33_to_48!(product_edges_at!($radix, $signed, $unsigned));
            widths_49_to_64!(product_edges_at!($radix, $signed, $unsigned));
        }
    };
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
