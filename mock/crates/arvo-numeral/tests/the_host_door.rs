//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The door for a host integer, against `adapted`.
//!
//! `from_index` is checked to be `adapted(Exact::on_grid(Slot::at(host)))` for
//! each of the ten host integers, at every width from 1 to 64, both ranges,
//! both policies and every mode: every value of the 8-bit ones, every value of
//! the wider ones within 1024 of zero, both ends of each host integer, and both
//! ends of the signature's range with their neighbours. `index` returns what
//! `from_index` was given. Where the `Abort` check aborts on a host integer is
//! `the_debug_check`'s to show. A few doors are named beside the sweep with their
//! slots written out, so that the suite does not rest on the oracle alone.

mod common;

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{self, Fixed, Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};
use arvo_numeral::{Int, Num, Quiet, Sole, Uint};
use common::hosts::{Value, hosts};
use common::{lattice, value};

/// The values of one host integer the sweep takes for the signature `S`,
/// through the door, against the public door it stands for.
fn through<S: Sole, H: Value>(doors: &mut u64) {
    for host in hosts::<S, H>(1024) {
        let index = host.value();
        assert_eq!(
            Num::<S, Quiet>::from_index(host),
            value::<S>(index),
            "`from_index` is not `adapted` at {index}"
        );
        *doors += 1;
    }
}

/// Every host integer into `S`.
fn doors<S: Sole>(doors: &mut u64) {
    through::<S, i8>(doors);
    through::<S, u8>(doors);
    through::<S, i16>(doors);
    through::<S, u16>(doors);
    through::<S, i32>(doors);
    through::<S, u32>(doors);
    through::<S, i64>(doors);
    through::<S, u64>(doors);
    through::<S, isize>(doors);
    through::<S, usize>(doors);
}

/// Every signature of the integer points at one width.
macro_rules! doors_at {
    ($w:literal, $n:expr) => {
        every_signature_at!($w, doors, $n)
    };
}

/// A band of sixteen widths: at least every `i8` and `u8` into each of its
/// 320 signatures.
macro_rules! band {
    ($name:ident, $widths:ident) => {
        #[test]
        fn $name() {
            let mut n = 0u64;
            $widths!(doors_at!(&mut n));
            assert!(n >= 320 * 512, "the sweep took {n} doors");
        }
    };
}

band!(
    every_host_integer_comes_in_as_adapted_at_widths_1_to_16,
    widths_1_to_16
);
band!(
    every_host_integer_comes_in_as_adapted_at_widths_17_to_32,
    widths_17_to_32
);
band!(
    every_host_integer_comes_in_as_adapted_at_widths_33_to_48,
    widths_33_to_48
);
band!(
    every_host_integer_comes_in_as_adapted_at_widths_49_to_64,
    widths_49_to_64
);

#[test]
fn every_host_integer_comes_in_as_adapted_at_the_platform_and_fixed_points() {
    let mut n = 0u64;
    macro_rules! at {
        ($($point:ty),+) => {
            $(
                doors::<Signature<$point, Adapt<TowardZero, Wrap>>>(&mut n);
                doors::<Signature<$point, Adapt<TowardZero, Saturate>>>(&mut n);
            )+
        };
    }
    at!(points::ISize, points::USize, Fixed<6, -3>, UFixed<6, 3>, Fixed<40, -8>);
    assert!(n >= 10 * 512, "the sweep took {n} doors");
}

/// Every member of the lattice of `S` out as an `H` and back in.
fn out_and_back<S: Sole, H: Value>(out: impl Fn(Num<S, Quiet>) -> H, trips: &mut u64) {
    for &a in lattice::<S>().as_slice() {
        let v = value::<S>(a);
        let host = out(v);
        assert_eq!(
            host.value(),
            a,
            "`index` does not give the member's slot index"
        );
        assert_eq!(
            Num::<S, Quiet>::from_index(host),
            v,
            "the index does not come back in"
        );
        *trips += 1;
    }
}

/// `index` in each host integer listed, which is each the range fits.
macro_rules! out_as {
    ($point:ty; $($h:ty),+; $trips:expr) => {
        $( out_and_back::<Signature<$point, Adapt<TowardZero, Wrap>>, $h>(|v| v.index::<$h>(), $trips); )+
    };
}

/// Both integer points at each width listed, out in the host integers their
/// ranges fit. The lists are whole: a host integer left out of one is one the
/// build refuses for that range, which `tests/ui` holds.
macro_rules! out_at {
    ([$($w:literal),+]; signed $signed:tt; unsigned $unsigned:tt; $trips:expr) => {
        $(
            out_at!(@point Integer<$w>; $signed; $trips);
            out_at!(@point UFixed<$w, 0>; $unsigned; $trips);
        )+
    };
    (@point $point:ty; [$($h:ty),+]; $trips:expr) => {
        out_as!($point; $($h),+; $trips)
    };
}

#[test]
fn the_index_goes_out_in_every_host_integer_the_range_fits_at_every_width() {
    let mut n = 0u64;
    out_at!([1, 2, 3, 4, 5, 6, 7];
        signed [i8, i16, i32, i64, isize];
        unsigned [i8, u8, i16, u16, i32, u32, i64, u64, isize, usize]; &mut n);
    out_at!([8];
        signed [i8, i16, i32, i64, isize];
        unsigned [u8, i16, u16, i32, u32, i64, u64, isize, usize]; &mut n);
    out_at!([9, 10, 11, 12, 13, 14, 15];
        signed [i16, i32, i64, isize];
        unsigned [i16, u16, i32, u32, i64, u64, isize, usize]; &mut n);
    out_at!([16];
        signed [i16, i32, i64, isize];
        unsigned [u16, i32, u32, i64, u64, isize, usize]; &mut n);
    out_at!([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
        signed [i32, i64, isize];
        unsigned [i32, u32, i64, u64, isize, usize]; &mut n);
    out_at!([32];
        signed [i32, i64, isize];
        unsigned [u32, i64, u64, isize, usize]; &mut n);
    #[cfg(target_pointer_width = "64")]
    {
        out_at!([33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
                 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63];
            signed [i64, isize];
            unsigned [i64, u64, isize, usize]; &mut n);
        out_at!([64]; signed [i64, isize]; unsigned [u64, usize]; &mut n);
    }
    out_as!(points::ISize; isize, i64; &mut n);
    out_as!(points::USize; usize, u64; &mut n);
    out_as!(Fixed<6, -3>; i8, i64; &mut n);
    assert!(n > 0);
}

type U8<O> = Uint<8, Adapt<TowardZero, O>, Quiet>;
type I8<O> = Int<8, Adapt<TowardZero, O>, Quiet>;
type U64<O> = Uint<64, Adapt<TowardZero, O>, Quiet>;
type I64<O> = Int<64, Adapt<TowardZero, O>, Quiet>;

#[test]
fn the_named_doors_sit_where_the_policy_puts_them() {
    // A negative host integer into an unsigned range.
    assert_eq!(U8::<Saturate>::from_index(-1i8).slot().index(), 0);
    assert_eq!(U8::<Saturate>::from_index(i64::MIN).slot().index(), 0);
    assert_eq!(U8::<Wrap>::from_index(-1i8).slot().index(), 255);
    // Past the top.
    assert_eq!(U8::<Saturate>::from_index(300i16).slot().index(), 255);
    assert_eq!(U8::<Wrap>::from_index(300i16).slot().index(), 44);
    // An unsigned host integer above the signed 64-bit maximum into a signed
    // range.
    assert_eq!(I8::<Saturate>::from_index(u64::MAX).slot().index(), 127);
    assert_eq!(I8::<Wrap>::from_index(u64::MAX).slot().index(), -1);
    assert_eq!(
        I64::<Saturate>::from_index(u64::MAX).slot().index(),
        i128::from(i64::MAX)
    );
    assert_eq!(I64::<Wrap>::from_index(u64::MAX).slot().index(), -1);
    // The 64-bit ranges at their own width.
    assert_eq!(U64::<Saturate>::from_index(-1i64).slot().index(), 0);
    assert_eq!(
        U64::<Wrap>::from_index(-1i64).slot().index(),
        i128::from(u64::MAX)
    );
    assert_eq!(
        U64::<Saturate>::from_index(u64::MAX).slot().index(),
        i128::from(u64::MAX)
    );
    assert_eq!(
        I64::<Saturate>::from_index(i64::MIN).slot().index(),
        i128::from(i64::MIN)
    );
}

#[test]
fn the_door_is_a_constant_where_the_host_integer_is() {
    const TOP: U8<Saturate> = U8::<Saturate>::from_index(1000u32);
    const OUT: u8 = TOP.index::<u8>();
    assert_eq!(OUT, 255);
}
