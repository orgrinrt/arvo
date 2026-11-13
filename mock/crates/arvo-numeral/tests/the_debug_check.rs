//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The debug check, observed at each site that carries it: `add`, `sub`, `neg`,
//! `mul`, `div`, `convert`, `adapted`, `from_index` and an accumulator's
//! `finish`, under `Wrap` and under `Saturate`. Under `Abort` a debug build aborts when a
//! result leaves the range, so each of those is a `should_panic` test; each has
//! a twin under `Quiet` that takes the same input and completes it by the
//! policy. The check is no part of the signature, so the twins hold the same
//! member the same way.

#![cfg(debug_assertions)]

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::Integer;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Exact, Signature, Slot};
use arvo_numeral::{Abort, Accumulator, Divisor, Int, Quiet};

type TwoBitsWrap = Signature<Integer<2>, Adapt<TowardZero, Wrap>>;
type TwoBitsSaturate = Signature<Integer<2>, Adapt<TowardZero, Saturate>>;

type WrapAborts = Int<4, Adapt<TowardZero, Wrap>>;
type SaturateAborts = Int<4, Adapt<TowardZero, Saturate>, Abort>;
type WrapQuiet = Int<4, Adapt<TowardZero, Wrap>, Quiet>;
type SaturateQuiet = Int<4, Adapt<TowardZero, Saturate>, Quiet>;

/// One past the top of a 4-bit signed range.
const PAST_THE_TOP: i128 = 8;

#[test]
#[should_panic(expected = "the sum leaves the signature's range")]
fn add_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::constant::<7>().add(WrapAborts::constant::<1>());
}

#[test]
#[should_panic(expected = "the sum leaves the signature's range")]
fn add_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::constant::<7>().add(SaturateAborts::constant::<1>());
}

#[test]
#[should_panic(expected = "the position leaves the signature's range")]
fn adapted_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
}

#[test]
#[should_panic(expected = "the position leaves the signature's range")]
fn adapted_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
}

#[test]
#[should_panic(expected = "the difference leaves the signature's range")]
fn sub_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::constant::<-8>().sub(WrapAborts::constant::<1>());
}

#[test]
#[should_panic(expected = "the difference leaves the signature's range")]
fn sub_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::constant::<-8>().sub(SaturateAborts::constant::<1>());
}

#[test]
#[should_panic(expected = "the negation leaves the signature's range")]
fn neg_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::constant::<-8>().neg();
}

#[test]
#[should_panic(expected = "the negation leaves the signature's range")]
fn neg_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::constant::<-8>().neg();
}

#[test]
#[should_panic(expected = "the product leaves the signature's range")]
fn mul_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::constant::<4>().mul(WrapAborts::constant::<-3>());
}

#[test]
#[should_panic(expected = "the product leaves the signature's range")]
fn mul_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::constant::<4>().mul(SaturateAborts::constant::<-3>());
}

#[test]
#[should_panic(expected = "the quotient leaves the signature's range")]
fn div_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::constant::<-8>().div(Divisor::constant::<-1>());
}

#[test]
#[should_panic(expected = "the quotient leaves the signature's range")]
fn div_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::constant::<-8>().div(Divisor::constant::<-1>());
}

#[test]
#[should_panic(expected = "the member leaves the target signature's range")]
fn convert_aborts_onto_a_wrapping_target_with_the_default_check() {
    let _ = WrapAborts::constant::<7>().convert::<TwoBitsWrap>();
}

#[test]
#[should_panic(expected = "the member leaves the target signature's range")]
fn convert_aborts_onto_a_saturating_target_with_abort_named() {
    let _ = SaturateAborts::constant::<7>().convert::<TwoBitsSaturate>();
}

#[test]
#[should_panic(expected = "the host integer names a slot index outside the signature's range")]
fn from_index_aborts_under_wrap_with_the_default_check() {
    let _ = WrapAborts::from_index(8i8);
}

#[test]
#[should_panic(expected = "the host integer names a slot index outside the signature's range")]
fn from_index_aborts_under_saturate_with_abort_named() {
    let _ = SaturateAborts::from_index(8u64);
}

type FourWrap = Signature<Integer<4>, Adapt<TowardZero, Wrap>>;
type FourSaturate = Signature<Integer<4>, Adapt<TowardZero, Saturate>>;
type SixBits = Signature<Integer<6>, Adapt<TowardZero, Wrap>>;

#[test]
#[should_panic(expected = "the member leaves the target signature's range")]
fn finish_aborts_under_wrap_with_the_default_check() {
    let sevens = [WrapAborts::constant::<7>(); 2];
    let _ = Accumulator::<FourWrap, SixBits, 2>::of(&sevens).finish();
}

#[test]
#[should_panic(expected = "the member leaves the target signature's range")]
fn finish_aborts_under_saturate_with_abort_named() {
    let sevens = [SaturateAborts::constant::<7>(); 2];
    let _ = Accumulator::<FourSaturate, SixBits, 2, Abort>::of(&sevens).finish();
}

#[test]
fn finish_completes_under_quiet_and_abort_passes_a_sum_that_fits() {
    let sevens = [WrapQuiet::constant::<7>(); 2];
    let acc = Accumulator::<FourWrap, SixBits, 2, Quiet>::of(&sevens);
    assert!(!acc.fits().get());
    assert_eq!(acc.finish().slot().index(), -2);
    let sevens = [SaturateQuiet::constant::<7>(); 2];
    let acc = Accumulator::<FourSaturate, SixBits, 2, Quiet>::of(&sevens);
    assert_eq!(acc.finish().slot().index(), 7);

    // Under `Abort` nothing aborts on the way: the terms pass the range and come
    // back, and only the finished sum is asked.
    let terms = [
        SaturateAborts::constant::<7>(),
        SaturateAborts::constant::<7>(),
        SaturateAborts::constant::<-8>(),
    ];
    let acc = Accumulator::<FourSaturate, SixBits, 3>::of(&terms);
    assert_eq!(acc.total().slot().index(), 6);
    assert_eq!(acc.finish().slot().index(), 6);
    let (block, rest) = Accumulator::<FourSaturate, SixBits, 2>::take(&terms);
    assert_eq!(block.total().slot().index(), 14);
    assert_eq!(rest.len(), 1);
}

#[test]
fn div_convert_and_from_index_complete_under_quiet() {
    let minus_one = Divisor::constant::<-1>();
    assert_eq!(
        WrapQuiet::constant::<-8>().div(minus_one).slot().index(),
        -8
    );
    assert_eq!(
        SaturateQuiet::constant::<-8>()
            .div(minus_one)
            .slot()
            .index(),
        7
    );
    // Seven onto two signed bits: the low two bits read as minus one, or the top.
    assert_eq!(
        WrapQuiet::constant::<7>()
            .convert::<TwoBitsWrap>()
            .slot()
            .index(),
        -1
    );
    assert_eq!(
        SaturateQuiet::constant::<7>()
            .convert::<TwoBitsSaturate>()
            .slot()
            .index(),
        1
    );
    assert_eq!(WrapQuiet::from_index(8i8).slot().index(), -8);
    assert_eq!(SaturateQuiet::from_index(8u64).slot().index(), 7);
}

#[test]
fn abort_does_not_abort_on_a_quotient_a_conversion_or_an_index_that_fits() {
    assert_eq!(
        WrapAborts::constant::<-7>()
            .div(Divisor::constant::<-1>())
            .slot()
            .index(),
        7
    );
    assert_eq!(
        SaturateAborts::constant::<1>()
            .convert::<TwoBitsSaturate>()
            .slot()
            .index(),
        1
    );
    assert_eq!(WrapAborts::from_index(-8i64).slot().index(), -8);
    assert_eq!(SaturateAborts::from_index(7u8).index::<i8>(), 7);
}

#[test]
fn sub_neg_and_mul_complete_under_quiet() {
    assert_eq!(
        WrapQuiet::constant::<-8>()
            .sub(WrapQuiet::constant::<1>())
            .slot()
            .index(),
        7
    );
    assert_eq!(
        SaturateQuiet::constant::<-8>()
            .sub(SaturateQuiet::constant::<1>())
            .slot()
            .index(),
        -8
    );
    assert_eq!(WrapQuiet::constant::<-8>().neg().slot().index(), -8);
    assert_eq!(SaturateQuiet::constant::<-8>().neg().slot().index(), 7);
    // Minus twelve, which wraps to four and saturates at the bottom.
    assert_eq!(
        WrapQuiet::constant::<4>()
            .mul(WrapQuiet::constant::<-3>())
            .slot()
            .index(),
        4
    );
    assert_eq!(
        SaturateQuiet::constant::<4>()
            .mul(SaturateQuiet::constant::<-3>())
            .slot()
            .index(),
        -8
    );
}

#[test]
fn abort_does_not_abort_on_a_difference_a_negation_or_a_product_that_fits() {
    assert_eq!(
        WrapAborts::constant::<-8>()
            .sub(WrapAborts::constant::<-1>())
            .slot()
            .index(),
        -7
    );
    assert_eq!(SaturateAborts::constant::<7>().neg().slot().index(), -7);
    assert_eq!(
        WrapAborts::constant::<-4>()
            .mul(WrapAborts::constant::<2>())
            .slot()
            .index(),
        -8
    );
    assert_eq!(
        SaturateAborts::constant::<-1>()
            .mul(SaturateAborts::constant::<-7>())
            .slot()
            .index(),
        7
    );
}

#[test]
fn add_completes_under_quiet() {
    let wrapped = WrapQuiet::constant::<7>().add(WrapQuiet::constant::<1>());
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = SaturateQuiet::constant::<7>().add(SaturateQuiet::constant::<1>());
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
fn adapted_completes_under_quiet() {
    let wrapped = WrapQuiet::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(wrapped.slot().index(), -8);
    let saturated = SaturateQuiet::adapted(Exact::on_grid(Slot::at(PAST_THE_TOP)));
    assert_eq!(saturated.slot().index(), 7);
}

#[test]
fn abort_does_not_abort_on_what_fits() {
    assert_eq!(
        WrapAborts::constant::<7>()
            .add(WrapAborts::constant::<-1>())
            .slot()
            .index(),
        6
    );
    assert_eq!(
        SaturateAborts::constant::<-8>()
            .add(SaturateAborts::constant::<1>())
            .slot()
            .index(),
        -7
    );
    assert_eq!(
        WrapAborts::adapted(Exact::on_grid(Slot::at(7)))
            .slot()
            .index(),
        7
    );
    assert_eq!(
        SaturateAborts::adapted(Exact::on_grid(Slot::at(-8)))
            .slot()
            .index(),
        -8
    );
}

#[test]
fn the_check_changes_neither_the_member_nor_its_carrier() {
    use core::mem::size_of;
    assert_eq!(size_of::<WrapAborts>(), size_of::<WrapQuiet>());
    assert_eq!(
        WrapAborts::constant::<-3>().slot(),
        WrapQuiet::constant::<-3>().slot()
    );
}
