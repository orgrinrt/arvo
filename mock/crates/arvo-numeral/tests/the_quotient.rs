//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quotient through the value type, against the tests' own arithmetic.
//!
//! The value's `div` is the format's `div` called on slots, so the format's
//! function is not what it is compared with here. The expected slot is the
//! dividend's slot over the divisor's, scaled onto the grid, rounded by the
//! mode and completed by the policy, in `common::oracle`.
//!
//! A divisor is a constant of its format, so each format names its divisors:
//! every member but zero at widths 1 to 4, and a named set above that, both
//! ends of the range, one, minus one, an even one and an odd one. Every dividend
//! is taken against each up to eight bits, and the edge lattice with seeded
//! dividends above that, under both policies and the five modes. A few quotients are
//! named beside the sweep with their slots written out, so that the suite does
//! not rest on the oracle alone.

mod common;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, Integer, UFixed};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Divisor, Int, Num, Quiet, Sole};
use common::{SEEDED_PAIRS, SplitMix, lattice, oracle, range, value};

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

/// One quotient of the wide sweep against the oracle.
fn one_wide<S: Sole>(a: i128, b: Divisor<S::Format>, counts: &mut Counts) {
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

/// The edge lattice of `S` and seeded members of it over each divisor given,
/// where the range is too wide to take every dividend.
fn wide_quotients<S: Sole>(by: &[Divisor<S::Format>], counts: &mut Counts) {
    let (min, max) = range::<S>();
    let edges = lattice::<S>();
    for &b in by {
        for &a in edges.as_slice() {
            one_wide::<S>(a, b, counts);
        }
        let mut rng = SplitMix::seeded();
        for _ in 0 .. SEEDED_PAIRS {
            one_wide::<S>(rng.member(min, max), b, counts);
        }
    }
}

/// One point's divisors under both policies and the five modes. `wide` takes
/// the lattice and seeded dividends where the plain form takes every one.
macro_rules! over {
    (wide $point:ty; [$($b:expr),+]; $counts:expr) => {
        over!(@go wide_quotients, $point; [$($b),+]; $counts)
    };
    ($point:ty; [$($b:expr),+]; $counts:expr) => {
        over!(@go quotients, $point; [$($b),+]; $counts)
    };
    (@go $body:ident, $point:ty; [$($b:expr),+]; $counts:expr) => {{
        let by = [$(Divisor::<$point>::constant::<{ $b }>()),+];
        over!(@modes $body, $point, Wrap, &by, $counts);
        over!(@modes $body, $point, Saturate, &by, $counts);
    }};
    (@modes $body:ident, $point:ty, $policy:ty, $by:expr, $counts:expr) => {
        $body::<Signature<$point, Adapt<TowardZero, $policy>>>($by, $counts);
        $body::<Signature<$point, Adapt<Floor, $policy>>>($by, $counts);
        $body::<Signature<$point, Adapt<Ceil, $policy>>>($by, $counts);
        $body::<Signature<$point, Adapt<HalfUp, $policy>>>($by, $counts);
        $body::<Signature<$point, Adapt<HalfEven, $policy>>>($by, $counts);
    };
}

fn agree(counts: &Counts, expected: u64) {
    assert_eq!(
        counts.wrong, 0,
        "`div` disagrees with the oracle on {} of {} quotients",
        counts.wrong, counts.checked
    );
    assert_eq!(counts.checked, expected, "the sweep's size");
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
    // Divisors times dividends, a family, then the ten signatures.
    agree(&c, (4 + 24 + 112 + 480) * 10);
}

#[test]
fn every_member_divides_by_the_named_divisors_at_widths_five_to_eight() {
    let mut c = Counts::default();
    over!(Integer<5>; [-16, -1, 1, 2, 3, 15]; &mut c);
    over!(UFixed<5, 0>; [1, 2, 3, 31]; &mut c);
    over!(Integer<6>; [-32, -1, 1, 2, 3, 31]; &mut c);
    over!(UFixed<6, 0>; [1, 2, 3, 63]; &mut c);
    over!(Integer<7>; [-64, -1, 1, 2, 3, 63]; &mut c);
    over!(UFixed<7, 0>; [1, 2, 3, 127]; &mut c);
    over!(Integer<8>; [-128, -1, 1, 2, 3, 127]; &mut c);
    over!(UFixed<8, 0>; [1, 2, 3, 255]; &mut c);
    agree(&c, (6 + 4) * (32 + 64 + 128 + 256) * 10);
}

#[test]
fn a_fixed_point_quotient_is_rounded_by_the_mode() {
    let mut c = Counts::default();
    // Slot 8 is one, at eighths.
    over!(Fixed<6, -3>; [-32, -8, -1, 1, 3, 8, 31]; &mut c);
    over!(UFixed<6, -3>; [1, 3, 8, 63]; &mut c);
    agree(&c, (7 + 4) * 64 * 10);
}

/// Above eight bits a range has too many members to take each, so the
/// dividends are the edge lattice and seeded members. The divisors are the
/// ends of the range, one and minus one, two, three and a large odd one.
#[test]
fn the_quotient_is_the_oracles_on_the_lattice_and_seeded_dividends_up_to_64_bits() {
    let mut c = Counts::default();
    over!(wide Integer<13>; [-4096, -3, -2, -1, 1, 2, 3, 4095]; &mut c);
    over!(wide UFixed<13, 0>; [1, 2, 3, 8191]; &mut c);
    over!(wide Integer<32>; [-2147483648, -3, -2, -1, 1, 2, 3, 2147483647]; &mut c);
    over!(wide UFixed<32, 0>; [1, 2, 3, 4294967295]; &mut c);
    over!(wide Integer<64>;
        [-9223372036854775808, -3, -2, -1, 1, 2, 3, 9223372036854775807]; &mut c);
    over!(wide UFixed<64, 0>; [1, 2, 3, 18446744073709551615]; &mut c);
    // One is slot 256 at two hundred and fifty-sixths, and 65536 at 65536ths.
    over!(wide Fixed<16, -8>; [-32768, -256, -3, -1, 1, 2, 3, 256, 32767]; &mut c);
    over!(wide UFixed<16, -8>; [1, 2, 3, 256, 65535]; &mut c);
    over!(wide Fixed<32, -16>; [-2147483648, -65536, -1, 1, 2, 3, 65536, 2147483647]; &mut c);
    over!(wide Fixed<64, -32>;
        [-9223372036854775808, -4294967296, -1, 1, 2, 3, 4294967296, 9223372036854775807];
        &mut c);
    assert_eq!(
        c.wrong, 0,
        "`div` disagrees with the oracle on {} of {} quotients",
        c.wrong, c.checked
    );
    // Divisors, the seeded dividends alone, and the ten signatures of a point.
    let divisors = 8 + 4 + 8 + 4 + 8 + 4 + 9 + 5 + 8 + 8;
    assert!(c.checked > divisors * SEEDED_PAIRS as u64 * 10);
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
