//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient through the value type on every pair of the narrowest widths,
//! and at named quotients, against the tests' own arithmetic.
//!
//! The value's `div` is the format's `div` called on slots, so the format's
//! function is not what it is compared with here. The expected slot is the
//! dividend's slot over the divisor's, scaled onto the grid, rounded by the
//! mode and completed by the policy, in `common::oracle`.
//!
//! `the_quotient_at_every_shape.rs` takes every width, radix and exponent
//! against a named set of divisors. Here every member but zero is a divisor,
//! at widths 1 to 4, and a few quotients are named with their slots written
//! out, so that the suite does not rest on the oracle alone.

mod common;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, Integer, UFixed};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Divisor, Int, Num, Quiet, Sole};
use common::{oracle, range, value};

#[derive(Default)]
struct Counts {
    checked: u64,
    wrong:   u64,
}

/// Every dividend of `S` over each divisor given.
fn quotients<S: Sole>(by: &[Divisor<S::Format>], counts: &mut Counts) {
    let (min, max) = range::<S>();
    for &b in by {
        for a in min ..= max {
            let v = value::<S>(a);
            counts.checked += 1;
            let expected = oracle::quotient::<S>(a, b.slot().index());
            if v.div(b).slot().index() != expected.slot {
                counts.wrong += 1;
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

/// One point's divisors under both policies and the five modes.
macro_rules! over {
    ($point:ty; [$($b:expr),+]; $counts:expr) => {{
        let by = [$(Divisor::<$point>::constant::<{ $b }>()),+];
        every_adaptation!($point, quotients(&by, $counts));
    }};
}

#[test]
fn every_member_divides_by_every_member_but_zero_at_widths_one_to_four() {
    let mut c = Counts::default();
    over!(Integer<1>; [-1]; &mut c);
    over!(UFixed<1, 0>; [1]; &mut c);
    over!(Integer<2>; [-2, -1, 1]; &mut c);
    over!(UFixed<2, 0>; [1, 2, 3]; &mut c);
    over!(Integer<3>; [-4, -3, -2, -1, 1, 2, 3]; &mut c);
    over!(UFixed<3, 0>; [1, 2, 3, 4, 5, 6, 7]; &mut c);
    over!(Integer<4>; [-8, -7, -6, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6, 7]; &mut c);
    over!(UFixed<4, 0>; [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; &mut c);
    assert_eq!(
        c.wrong, 0,
        "`div` disagrees with the oracle on {} of {} quotients",
        c.wrong, c.checked
    );
    // Divisors times dividends, a family, then the ten signatures.
    assert_eq!(c.checked, (4 + 24 + 112 + 480) * 10, "the sweep's size");
}

type I8<R, O> = Int<8, Adapt<R, O>, Quiet>;
type Eighths<R> = Num<Signature<Fixed<6, -3>, Adapt<R, Saturate>>, Quiet>;

#[test]
fn the_named_quotients_sit_where_arithmetic_puts_them() {
    // Minus seven over two is minus three and a half.
    let two = Divisor::constant::<2>();
    assert_eq!(
        I8::<TowardZero, Wrap>::constant::<-7>()
            .div(two)
            .slot()
            .index(),
        -3
    );
    assert_eq!(
        I8::<Floor, Wrap>::constant::<-7>().div(two).slot().index(),
        -4
    );
    assert_eq!(
        I8::<Ceil, Wrap>::constant::<-7>().div(two).slot().index(),
        -3
    );
    assert_eq!(
        I8::<HalfUp, Wrap>::constant::<-7>().div(two).slot().index(),
        -3
    );
    assert_eq!(
        I8::<HalfEven, Wrap>::constant::<-7>()
            .div(two)
            .slot()
            .index(),
        -4
    );

    // The lowest member over minus one is one past the top.
    let minus_one = Divisor::constant::<-1>();
    let lowest = I8::<TowardZero, Saturate>::constant::<-128>();
    assert!(!lowest.fits_div(minus_one).get());
    assert_eq!(lowest.div(minus_one).slot().index(), 127);
    let lowest = I8::<TowardZero, Wrap>::constant::<-128>();
    assert!(!lowest.fits_div(minus_one).get());
    assert_eq!(lowest.div(minus_one).slot().index(), -128);
    assert!(
        I8::<TowardZero, Wrap>::constant::<-127>()
            .fits_div(minus_one)
            .get()
    );

    // One over three eighths is two and two thirds, twenty-one and a third
    // eighths.
    let three_eighths = Divisor::constant::<3>();
    assert_eq!(
        Eighths::<Floor>::constant::<8>()
            .div(three_eighths)
            .slot()
            .index(),
        21
    );
    assert_eq!(
        Eighths::<Ceil>::constant::<8>()
            .div(three_eighths)
            .slot()
            .index(),
        22
    );
    assert_eq!(
        Eighths::<HalfEven>::constant::<8>()
            .div(three_eighths)
            .slot()
            .index(),
        21
    );
    // Three over an eighth is twenty-four, past the top of the range.
    let an_eighth = Divisor::constant::<1>();
    let three = Eighths::<HalfEven>::constant::<24>();
    assert!(!three.fits_div(an_eighth).get());
    assert_eq!(three.div(an_eighth).slot().index(), 31);
}

#[test]
fn the_quotient_is_a_constant_where_its_operands_are() {
    const HALF: I8<HalfEven, Saturate> =
        I8::<HalfEven, Saturate>::constant::<100>().div(Divisor::constant::<8>());
    // Twelve and a half goes to the even neighbour.
    assert_eq!(HALF.slot().index(), 12);
}
