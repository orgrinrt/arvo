//! Strategy semantics tests.
//!
//! Asserts the arithmetic behaviour each strategy promises:
//! - Hot / Warm: wrapping (single-op container overflow wraps modulo container width).
//! - Cold: wrapping at container width (widen-narrow lands in a later round — current L0 Cold
//!   uses the same wrapping surface as Hot).
//! - Precise: saturating (clamps to container MAX / MIN on overflow, clamps on div-by-zero).

#![no_std]

use arvo::ifixed::IFixed;
use arvo::newtype::{FBits, IBits};
use arvo::strategy::{Cold, Hot, Precise, Warm};
use arvo::ufixed::UFixed;

#[test]
fn hot_u8_add_wraps() {
    // UFixed<8, 0, Hot> -> u8 container. 200 + 100 = 300 wraps to 44.
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    let a = U::from_raw(200);
    let b = U::from_raw(100);
    let sum = a + b;
    assert_eq!(sum.to_raw(), 44u8);
}

#[test]
fn hot_u8_sub_wraps() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    let a = U::from_raw(10);
    let b = U::from_raw(20);
    let diff = a - b;
    // 10 - 20 = -10 wraps to 246 in u8.
    assert_eq!(diff.to_raw(), 246u8);
}

#[test]
fn hot_u8_mul_wraps() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    let a = U::from_raw(20);
    let b = U::from_raw(20);
    let prod = a * b;
    // 400 wraps to 144 in u8.
    assert_eq!(prod.to_raw(), 144u8);
}

#[test]
fn warm_u16_add_does_not_wrap_single_op() {
    // UFixed<8, 0, Warm> -> u16 container (2x of the 8-bit logical width).
    // 200 + 100 = 300 fits in u16.
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    let a = U::from_raw(200);
    let b = U::from_raw(100);
    let sum = a + b;
    assert_eq!(sum.to_raw(), 300u16);
}

#[test]
fn warm_mul_safe_for_8bit_operands() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    let a = U::from_raw(200);
    let b = U::from_raw(200);
    let prod = a * b;
    // 40000 fits in u16.
    assert_eq!(prod.to_raw(), 40000u16);
}

#[test]
fn cold_u8_uses_wrapping_container_for_now() {
    // Cold and Hot share container widths at L0; widen-narrow lands later.
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Cold>;
    let a = U::from_raw(200);
    let b = U::from_raw(100);
    let sum = a + b;
    assert_eq!(sum.to_raw(), 44u8);
}

#[test]
fn precise_u16_saturates_on_add_overflow() {
    // UFixed<8, 0, Precise> -> u16 container. Max is u16::MAX, but
    // u8 + u8 cannot overflow u16 in a single op — use a larger
    // width to see saturation.
    type U = UFixed<{ IBits(16) }, { FBits::ZERO }, Precise>;
    let a = U::from_raw(u32::MAX - 10);
    let b = U::from_raw(100);
    // Wait: U32 container, so u32::MAX - 10 + 100 saturates to u32::MAX.
    let sum = a + b;
    assert_eq!(sum.to_raw(), u32::MAX);
}

#[test]
fn precise_u16_saturates_on_sub_underflow() {
    type U = UFixed<{ IBits(16) }, { FBits::ZERO }, Precise>;
    let a = U::from_raw(5);
    let b = U::from_raw(10);
    let diff = a - b;
    // Saturating sub on unsigned clamps to 0.
    assert_eq!(diff.to_raw(), 0u32);
}

#[test]
fn precise_div_by_zero_clamps_to_max() {
    type U = UFixed<{ IBits(16) }, { FBits::ZERO }, Precise>;
    let a = U::from_raw(42);
    let b = U::from_raw(0);
    let q = a / b;
    assert_eq!(q.to_raw(), u32::MAX);
}

#[test]
fn ifixed_hot_i8_add_wraps() {
    // IFixed<7, 0, Hot> has 1 + 7 = 8 logical bits -> i8 container.
    type I = IFixed<{ IBits(7) }, { FBits::ZERO }, Hot>;
    let a = I::from_raw(100);
    let b = I::from_raw(50);
    let sum = a + b;
    // 150 wraps to -106 in i8.
    assert_eq!(sum.to_raw(), -106i8);
}

#[test]
fn ifixed_precise_saturates_on_add() {
    // IFixed<15, 0, Precise> -> 1 + 15 = 16 logical bits -> i32 container (Precise 2x).
    type I = IFixed<{ IBits(15) }, { FBits::ZERO }, Precise>;
    let a = I::from_raw(i32::MAX - 10);
    let b = I::from_raw(100);
    let sum = a + b;
    assert_eq!(sum.to_raw(), i32::MAX);
}

#[test]
fn repr_transparent_size_matches_container() {
    // UFixed<8, 0, Hot> should be same size as u8.
    use core::mem::size_of;
    type U8Hot = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    type U8Warm = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    assert_eq!(size_of::<U8Hot>(), size_of::<u8>());
    assert_eq!(size_of::<U8Warm>(), size_of::<u16>());
}
