//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

// The quotient sweep's body, included by one test file a radix.

use arvo_format::Format;
use arvo_numeral::{Divisor, Sole};
use common::shapes::{SIGNATURES_OF_A_BAND, Shapes, members, minus_one, one, widest, zero};
use common::{oracle, value};

/// The slot of the number one at a radix and an exponent, or one where the
/// number one is on no slot or past `max`.
const fn unit(radix: i128, exponent: i32, max: i128) -> i128 {
    if exponent >= 0 {
        return 1;
    }
    let mut slot = 1i128;
    let mut k = 0;
    while k < -exponent {
        if slot > max / radix {
            return 1;
        }
        slot *= radix;
        k += 1;
    }
    slot
}

/// The `k`th divisor of a signed range of `w` bits, never zero and always a
/// member, so narrow widths name some slot more than once.
const fn signed_divisor(w: u32, k: usize, radix: i128, exponent: i32) -> i128 {
    let max = (1i128 << (w - 1)) - 1;
    let min = -max - 1;
    let named = [min, -3, -2, -1, 1, 2, 3, max, (max / 3) | 1, unit(radix, exponent, max)];
    let at = if named[k] < min {
        min
    } else if named[k] > max {
        max
    } else {
        named[k]
    };
    if at == 0 { -1 } else { at }
}

/// The `k`th divisor of an unsigned range of `w` bits.
const fn unsigned_divisor(w: u32, k: usize, radix: i128, exponent: i32) -> i128 {
    let max = (1i128 << w) - 1;
    let named = [1, 2, 3, max, (max / 3) | 1, unit(radix, exponent, max)];
    if named[k] > max { max } else { named[k] }
}

/// Every dividend the sweep takes of `S` over each divisor given.
fn quotients<S: Sole>(by: &[Divisor<S::Format>], shapes: &mut Shapes) {
    shapes.signatures += 1;
    for a in members::<S>() {
        for &b in by {
            let v = value::<S>(a);
            let expected = oracle::quotient::<S>(a, b.slot().index());
            shapes.checked += 1;
            if v.div(b).slot().index() != expected.slot {
                shapes.wrong += 1;
            }
            assert_eq!(
                v.fits_div(b).get(),
                expected.fits,
                "`fits_div` at {a} / {}",
                b.slot().index()
            );
        }
    }
}

fn divisors<F: Format, const N: usize>(slots: [Divisor<F>; N]) -> [Divisor<F>; N] {
    slots
}

/// One signed format of `$w` bits at `$radix` and `$e`, under every adaptation.
macro_rules! signed {
    ($format:ty, $w:literal, $radix:literal, $e:expr, $shapes:expr) => {{
        let by = divisors::<$format, 10>([
            Divisor::constant::<{ signed_divisor($w, 0, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 1, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 2, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 3, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 4, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 5, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 6, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 7, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 8, $radix, $e) }>(),
            Divisor::constant::<{ signed_divisor($w, 9, $radix, $e) }>(),
        ]);
        every_adaptation!($format, quotients(&by, $shapes));
    }};
}

/// One unsigned format of `$w` bits at `$radix` and `$e`, under every adaptation.
macro_rules! unsigned {
    ($format:ty, $w:literal, $radix:literal, $e:expr, $shapes:expr) => {{
        let by = divisors::<$format, 6>([
            Divisor::constant::<{ unsigned_divisor($w, 0, $radix, $e) }>(),
            Divisor::constant::<{ unsigned_divisor($w, 1, $radix, $e) }>(),
            Divisor::constant::<{ unsigned_divisor($w, 2, $radix, $e) }>(),
            Divisor::constant::<{ unsigned_divisor($w, 3, $radix, $e) }>(),
            Divisor::constant::<{ unsigned_divisor($w, 4, $radix, $e) }>(),
            Divisor::constant::<{ unsigned_divisor($w, 5, $radix, $e) }>(),
        ]);
        every_adaptation!($format, quotients(&by, $shapes));
    }};
}

/// The deepest exponent the format's quotient admits at a range of `w` bits:
/// minus the largest `k` at which the widest member times the radix to the
/// `k` is still a position a slot index holds, since a quotient by the slot
/// one is the dividend's slot times the radix to the `k`.
const fn deep(w: u32, signed: bool, radix: i128) -> i32 {
    let mut at = widest(w, signed);
    let mut k = 0;
    while at <= i128::MAX / radix {
        at *= radix;
        k += 1;
    }
    -k
}

/// Minus half the width rounded up, or the deepest exponent where that is
/// nearer zero.
const fn half(w: u32, signed: bool, radix: i128) -> i32 {
    let half = -((w as i32 + 1) / 2);
    let deep = deep(w, signed, radix);
    if half > deep { half } else { deep }
}

/// One format's quotients under every adaptation.
macro_rules! quotients_of {
    (signed, $format:ty, $w:literal, $radix:literal, $e:expr, ($shapes:expr)) => {
        signed!($format, $w, $radix, $e, $shapes)
    };
    (unsigned, $format:ty, $w:literal, $radix:literal, $e:expr, ($shapes:expr)) => {
        unsigned!($format, $w, $radix, $e, $shapes)
    };
}

/// A band of sixteen widths of one radix family.
macro_rules! band {
    ($name:ident, $widths:ident, $at:ident) => {
        #[test]
        fn $name() {
            let mut shapes = Shapes::default();
            $widths!($at!(
                [zero, one, minus_one, half, deep],
                quotients_of,
                (&mut shapes)
            ));
            shapes.agree("`div`", SIGNATURES_OF_A_BAND);
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
