//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The door for a host integer, against `adapted`.
//!
//! `from_index` is checked to be `adapted(Exact::on_grid(Slot::at(host)))` for
//! each of the ten host integers: every value of the 8-bit ones, every value of
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
use arvo_numeral::{HostInteger, Int, Num, Quiet, Sole, Uint};
use common::{lattice, range, value};

/// A host integer's value, wide enough for any of the ten.
trait Value: HostInteger + TryFrom<i128> {
    const MIN: Self;
    const MAX: Self;
    fn value(self) -> i128;
}

macro_rules! host_values {
    ($($t:ty),+) => {
        $(
            impl Value for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
                fn value(self) -> i128 {
                    self as i128
                }
            }
        )+
    };
}

host_values!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

/// One host value through the door, against the public door it stands for.
fn door<S: Sole, H: Value>(host: H, doors: &mut u64) {
    let index = host.value();
    assert_eq!(
        Num::<S, Quiet>::from_index(host),
        value::<S>(index),
        "`from_index` is not `adapted` at {index}"
    );
    *doors += 1;
}

/// The values of one host integer the sweep takes for the signature `S`.
fn through<S: Sole, H: Value>(doors: &mut u64) {
    let (min, max) = range::<S>();
    let near_zero = -1024i128 ..= 1024;
    let ends = [
        H::MIN.value(),
        H::MIN.value() + 1,
        H::MAX.value() - 1,
        H::MAX.value(),
        min - 1,
        min,
        min + 1,
        max - 1,
        max,
        max + 1,
    ];
    for index in near_zero.chain(ends) {
        if let Ok(host) = H::try_from(index) {
            door::<S, H>(host, doors);
        }
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

#[test]
fn every_host_integer_comes_in_as_adapted_at_widths_one_to_eight() {
    let mut n = 0u64;
    every_signature_at!(1, doors, &mut n);
    every_signature_at!(2, doors, &mut n);
    every_signature_at!(3, doors, &mut n);
    every_signature_at!(4, doors, &mut n);
    every_signature_at!(5, doors, &mut n);
    every_signature_at!(6, doors, &mut n);
    every_signature_at!(7, doors, &mut n);
    every_signature_at!(8, doors, &mut n);
    // At least every `i8` and `u8` into each of the 160 signatures.
    assert!(n >= 160 * 512, "the sweep took {n} doors");
}

#[test]
fn every_host_integer_comes_in_as_adapted_at_the_wider_widths() {
    let mut n = 0u64;
    macro_rules! at {
        ($($point:ty),+) => {
            $(
                doors::<Signature<$point, Adapt<TowardZero, Wrap>>>(&mut n);
                doors::<Signature<$point, Adapt<TowardZero, Saturate>>>(&mut n);
            )+
        };
    }
    at!(
        Integer<9>, UFixed<9, 0>, Integer<13>, UFixed<13, 0>, Integer<16>, UFixed<16, 0>,
        Integer<31>, UFixed<31, 0>, Integer<32>, UFixed<32, 0>, Integer<33>, UFixed<33, 0>,
        Integer<63>, UFixed<63, 0>, Integer<64>, UFixed<64, 0>, points::ISize, points::USize,
        Fixed<6, -3>, UFixed<6, 3>, Fixed<40, -8>
    );
    assert!(n >= 42 * 10 * 1024, "the sweep took {n} doors");
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

#[test]
fn the_index_goes_out_in_every_host_integer_the_range_fits() {
    let mut n = 0u64;
    out_as!(Integer<8>; i8, i16, i32, i64, isize; &mut n);
    out_as!(UFixed<8, 0>; u8, i16, u16, i32, u32, i64, u64, isize, usize; &mut n);
    out_as!(UFixed<7, 0>; i8, u8; &mut n);
    out_as!(Integer<13>; i16, i32, i64; &mut n);
    out_as!(UFixed<13, 0>; i16, u16, u32, i64; &mut n);
    out_as!(Integer<32>; i32, i64; &mut n);
    out_as!(UFixed<32, 0>; u32, i64, u64; &mut n);
    out_as!(Integer<64>; i64; &mut n);
    out_as!(UFixed<64, 0>; u64; &mut n);
    out_as!(UFixed<63, 0>; i64, u64; &mut n);
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
