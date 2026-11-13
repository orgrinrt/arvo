//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The conversion onto another signature, against the format's `convert`.
//!
//! Every member of each source is converted onto each target under both
//! policies and the five modes of the target: between integer widths in both
//! directions, across the two slot families, from an integer onto a
//! fixed-point grid and back, and between two fixed-point grids of different
//! exponents. A few conversions are named beside the sweep with their slots
//! written out, so that the suite does not rest on the oracle alone.

mod common;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, Integer, UFixed};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::{
    Adapt,
    DeclaredSignature,
    Dither,
    Signature,
    Slot,
    convert,
    converted_position,
    panic_on_overflow,
};
use arvo_numeral::{Num, Quiet, Sole};
use common::{range, value};

#[derive(Default)]
struct Counts {
    checked: u64,
    wrong:   u64,
}

/// Every member of `S` onto `T`.
fn converts<S: Sole, T: Sole>(counts: &mut Counts) {
    let (min, max) = range::<S>();
    for a in min ..= max {
        let v = value::<S>(a);
        counts.checked += 1;
        let there: Num<T, Quiet> = v.convert::<T>();
        if there.slot() != convert::<S::Format, T>(Slot::at(a), Dither::UNUSED) {
            counts.wrong += 1;
        }
        let position = converted_position::<S::Format, T::Format>(Slot::at(a));
        let leaves = panic_on_overflow::<T>(position, Dither::UNUSED).get();
        assert_eq!(
            v.fits_convert::<T>().get(),
            !leaves,
            "`fits_convert` disagrees with `panic_on_overflow` at {a}"
        );
    }
}

/// One source point onto one target point, under both policies and the five
/// modes of the target. The source's adaptation takes no part in a conversion.
macro_rules! onto {
    ($from:ty => $to:ty; $counts:expr) => {{
        onto!(@modes $from, $to, Wrap, $counts);
        onto!(@modes $from, $to, Saturate, $counts);
    }};
    (@modes $from:ty, $to:ty, $policy:ty, $counts:expr) => {
        onto!(@one $from, $to, TowardZero, $policy, $counts);
        onto!(@one $from, $to, Floor, $policy, $counts);
        onto!(@one $from, $to, Ceil, $policy, $counts);
        onto!(@one $from, $to, HalfUp, $policy, $counts);
        onto!(@one $from, $to, HalfEven, $policy, $counts);
    };
    (@one $from:ty, $to:ty, $mode:ty, $policy:ty, $counts:expr) => {
        converts::<Signature<$from, Adapt<TowardZero, Wrap>>, Signature<$to, Adapt<$mode, $policy>>>(
            $counts,
        );
    };
}

fn agree(counts: &Counts, members: u64) {
    assert_eq!(
        counts.wrong, 0,
        "`convert` disagrees with the oracle on {} of {} members",
        counts.wrong, counts.checked
    );
    assert_eq!(counts.checked, members * 10, "the sweep's size");
}

#[test]
fn an_integer_converts_across_widths_and_signs() {
    let mut c = Counts::default();
    onto!(Integer<8> => Integer<4>; &mut c);
    onto!(Integer<4> => Integer<8>; &mut c);
    onto!(Integer<8> => UFixed<8, 0>; &mut c);
    onto!(UFixed<8, 0> => Integer<8>; &mut c);
    onto!(UFixed<8, 0> => UFixed<5, 0>; &mut c);
    onto!(Integer<16> => Integer<8>; &mut c);
    onto!(UFixed<8, 0> => Integer<64>; &mut c);
    agree(&c, 256 + 16 + 256 + 256 + 256 + 65536 + 256);
}

#[test]
fn an_integer_converts_onto_a_fixed_point_grid_and_back() {
    let mut c = Counts::default();
    onto!(Integer<8> => Fixed<6, -3>; &mut c);
    onto!(Fixed<6, -3> => Integer<8>; &mut c);
    onto!(UFixed<8, 0> => UFixed<6, 3>; &mut c);
    onto!(UFixed<6, 3> => UFixed<8, 0>; &mut c);
    agree(&c, 256 + 64 + 256 + 64);
}

#[test]
fn a_fixed_point_member_converts_between_two_exponents() {
    let mut c = Counts::default();
    onto!(Fixed<8, -5> => Fixed<6, -3>; &mut c);
    onto!(Fixed<6, -3> => Fixed<8, -5>; &mut c);
    onto!(UFixed<6, -3> => UFixed<6, 3>; &mut c);
    agree(&c, 256 + 64 + 64);
}

/// Every member of `S` onto a signature over its own format, beside `under`.
fn as_under<S, T>(checked: &mut u64)
where
    S: Sole,
    T: Sole + DeclaredSignature<Format = S::Format>,
{
    let (min, max) = range::<S>();
    for a in min ..= max {
        let v = value::<S>(a);
        assert_eq!(
            v.convert::<T>(),
            v.under::<T>(),
            "a conversion over one format is not the crossing at {a}"
        );
        *checked += 1;
    }
}

#[test]
fn a_conversion_over_one_format_is_the_crossing() {
    let mut n = 0u64;
    type From8 = Signature<Integer<8>, Adapt<TowardZero, Wrap>>;
    as_under::<From8, Signature<Integer<8>, Adapt<HalfEven, Saturate>>>(&mut n);
    as_under::<From8, Signature<Integer<8>, Adapt<Floor, Wrap>>>(&mut n);
    type FromEighths = Signature<Fixed<6, -3>, Adapt<Ceil, Saturate>>;
    as_under::<FromEighths, Signature<Fixed<6, -3>, Adapt<HalfUp, Wrap>>>(&mut n);
    assert_eq!(n, 256 + 256 + 64);
}

type Sig<P, R, O> = Signature<P, Adapt<R, O>>;
type I8 = Num<Sig<Integer<8>, TowardZero, Wrap>, Quiet>;
type Eighths = Num<Sig<Fixed<6, -3>, TowardZero, Wrap>, Quiet>;

#[test]
fn the_named_conversions_sit_where_arithmetic_puts_them() {
    // A hundred onto four signed bits: past the top, so the policy completes it.
    let hundred = I8::constant::<100>();
    assert!(
        !hundred
            .fits_convert::<Sig<Integer<4>, TowardZero, Saturate>>()
            .get()
    );
    assert_eq!(
        hundred
            .convert::<Sig<Integer<4>, TowardZero, Saturate>>()
            .slot()
            .index(),
        7
    );
    assert_eq!(
        hundred
            .convert::<Sig<Integer<4>, TowardZero, Wrap>>()
            .slot()
            .index(),
        4
    );
    // Minus one onto an unsigned byte.
    let minus_one = I8::constant::<-1>();
    assert_eq!(
        minus_one
            .convert::<Sig<UFixed<8, 0>, TowardZero, Saturate>>()
            .slot()
            .index(),
        0
    );
    assert_eq!(
        minus_one
            .convert::<Sig<UFixed<8, 0>, TowardZero, Wrap>>()
            .slot()
            .index(),
        255
    );

    // Three is twenty-four eighths, and five is forty, past the top at 31.
    let three = I8::constant::<3>();
    assert!(
        three
            .fits_convert::<Sig<Fixed<6, -3>, HalfEven, Saturate>>()
            .get()
    );
    assert_eq!(
        three
            .convert::<Sig<Fixed<6, -3>, HalfEven, Saturate>>()
            .slot()
            .index(),
        24
    );
    let five = I8::constant::<5>();
    assert!(
        !five
            .fits_convert::<Sig<Fixed<6, -3>, HalfEven, Saturate>>()
            .get()
    );
    assert_eq!(
        five.convert::<Sig<Fixed<6, -3>, HalfEven, Saturate>>()
            .slot()
            .index(),
        31
    );

    // One and a half, twelve eighths, onto the integers.
    let above = Eighths::constant::<12>();
    assert_eq!(
        above
            .convert::<Sig<Integer<8>, TowardZero, Wrap>>()
            .slot()
            .index(),
        1
    );
    assert_eq!(
        above
            .convert::<Sig<Integer<8>, Floor, Wrap>>()
            .slot()
            .index(),
        1
    );
    assert_eq!(
        above
            .convert::<Sig<Integer<8>, Ceil, Wrap>>()
            .slot()
            .index(),
        2
    );
    assert_eq!(
        above
            .convert::<Sig<Integer<8>, HalfUp, Wrap>>()
            .slot()
            .index(),
        2
    );
    assert_eq!(
        above
            .convert::<Sig<Integer<8>, HalfEven, Wrap>>()
            .slot()
            .index(),
        2
    );
    // Minus one and a half, a tie below zero.
    let below = Eighths::constant::<-12>();
    assert_eq!(
        below
            .convert::<Sig<Integer<8>, TowardZero, Wrap>>()
            .slot()
            .index(),
        -1
    );
    assert_eq!(
        below
            .convert::<Sig<Integer<8>, Floor, Wrap>>()
            .slot()
            .index(),
        -2
    );
    assert_eq!(
        below
            .convert::<Sig<Integer<8>, Ceil, Wrap>>()
            .slot()
            .index(),
        -1
    );
    assert_eq!(
        below
            .convert::<Sig<Integer<8>, HalfUp, Wrap>>()
            .slot()
            .index(),
        -1
    );
    assert_eq!(
        below
            .convert::<Sig<Integer<8>, HalfEven, Wrap>>()
            .slot()
            .index(),
        -2
    );
}

#[test]
fn the_conversion_is_a_constant_where_its_operand_is() {
    const THREE: Num<Sig<Fixed<6, -3>, HalfEven, Saturate>, Quiet> = I8::constant::<3>().convert();
    assert_eq!(THREE.slot().index(), 24);
}
