//! IFixed arithmetic correctness per strategy.
//!
//! Signed variant: sign behaviour on `abs`, overflow semantics on add.
//! Cross-width arithmetic deferred (see ifixed.rs TODO). Only
//! same-strategy same-width is covered here.

#![no_std]

use arvo::ifixed::IFixed;
use arvo::{FBits, IBits, ibits, fbits, USize};
use arvo::strategy::{Hot, Precise, Warm};
use arvo::traits::{Abs, FromConstant, TotalOrd};

#[test]
fn hot_i8_arith_positive() {
    // IFixed<7, 0, Hot>: 1 + 7 = 8 bits -> i8.
    type I = IFixed<{ ibits(7) }, { FBits::ZERO }, Hot>;
    let a = I::from_constant::<{ USize(5) }>();
    let b = I::from_constant::<{ USize(3) }>();
    assert_eq!((a + b).to_raw(), 8i8);
    assert_eq!((a - b).to_raw(), 2i8);
    assert_eq!((a * b).to_raw(), 15i8);
    assert_eq!((a / b).to_raw(), 1i8);
}

#[test]
fn hot_i8_negative_arith() {
    type I = IFixed<{ ibits(7) }, { FBits::ZERO }, Hot>;
    let neg5 = I::from_raw(-5);
    let three = I::from_raw(3);
    assert_eq!((neg5 + three).to_raw(), -2i8);
    assert_eq!((neg5 * three).to_raw(), -15i8);
}

#[test]
fn abs_of_negative_is_positive() {
    type I = IFixed<{ ibits(7) }, { FBits::ZERO }, Hot>;
    let neg42 = I::from_raw(-42);
    assert_eq!(neg42.abs().to_raw(), 42i8);
}

#[test]
fn abs_at_min_wraps_hot() {
    // Hot uses wrapping_abs: i8::MIN.wrapping_abs() = i8::MIN.
    type I = IFixed<{ ibits(7) }, { FBits::ZERO }, Hot>;
    let min_val = I::from_raw(i8::MIN);
    assert_eq!(min_val.abs().to_raw(), i8::MIN);
}

#[test]
fn abs_at_min_saturates_precise() {
    // Precise uses saturating_abs: i16::MIN -> i16::MAX (Precise 2x at 8 bits -> i16).
    type I = IFixed<{ ibits(7) }, { FBits::ZERO }, Precise>;
    let min_val = I::from_raw(i16::MIN);
    assert_eq!(min_val.abs().to_raw(), i16::MAX);
}

#[test]
fn warm_i8_add_no_wrap() {
    // IFixed<7, 0, Warm>: 1 + 7 = 8 bits -> i16 container.
    type I = IFixed<{ ibits(7) }, { FBits::ZERO }, Warm>;
    let a = I::from_raw(100);
    let b = I::from_raw(100);
    let sum = a + b;
    // 200 fits in i16.
    assert_eq!(sum.to_raw(), 200i16);
}

#[test]
fn precise_i8_add_saturates() {
    // Use a wider IFixed so we can see the saturation against the Precise container.
    // IFixed<15, 0, Precise>: 1 + 15 = 16 -> i32 container.
    type I = IFixed<{ ibits(15) }, { FBits::ZERO }, Precise>;
    let a = I::from_raw(i32::MAX - 5);
    let b = I::from_raw(100);
    let sum = a + b;
    assert_eq!(sum.to_raw(), i32::MAX);

    let c = I::from_raw(i32::MIN + 5);
    let d = I::from_raw(-100);
    let diff = c + d;
    assert_eq!(diff.to_raw(), i32::MIN);
}

#[test]
fn total_cmp_orders_ifixed() {
    use core::cmp::Ordering;
    type I = IFixed<{ ibits(7) }, { FBits::ZERO }, Hot>;
    let neg = I::from_raw(-10);
    let pos = I::from_raw(10);
    assert_eq!(neg.total_cmp(pos), Ordering::Less);
}

#[test]
fn from_constant_places_at_integer_position_signed() {
    // IFixed<7, 1, Hot>: 1 + 7 + 1 = 9 bits -> i16. from_constant(1) = 1 << 1 = 2.
    type I = IFixed<{ ibits(7) }, { fbits(1) }, Hot>;
    let one = I::from_constant::<{ USize(1) }>();
    assert_eq!(one.to_raw(), 2i16);
}
