//! UFixed arithmetic correctness per strategy.
//!
//! Exercises Add / Sub / Mul / Div at representative widths for each
//! strategy. Cross-width arithmetic is deferred to a later round (see
//! ufixed.rs TODO); these tests only cover same-strategy same-width.

#![no_std]

use arvo::markers::BitPresentation;
use arvo::{FBits, IBits, USize};
use arvo::strategy::{Cold, Hot, Precise, Warm};
use arvo::traits::{Abs, FromConstant, TotalOrd};
use arvo::ufixed::UFixed;

#[test]
fn hot_u8_arith() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    let a = U::from_constant(USize(5));
    let b = U::from_constant(USize(3));
    assert_eq!((a + b).to_raw(), 8u8);
    assert_eq!((a - b).to_raw(), 2u8);
    assert_eq!((a * b).to_raw(), 15u8);
    assert_eq!((a / b).to_raw(), 1u8); // 5 / 3 = 1 integer.
}

#[test]
fn warm_u8_arith() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    let a = U::from_constant(USize(5));
    let b = U::from_constant(USize(3));
    assert_eq!((a + b).to_raw(), 8u16);
    assert_eq!((a - b).to_raw(), 2u16);
    assert_eq!((a * b).to_raw(), 15u16);
    assert_eq!((a / b).to_raw(), 1u16);
}

#[test]
fn cold_u8_arith() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Cold>;
    let a = U::from_constant(USize(5));
    let b = U::from_constant(USize(3));
    assert_eq!((a + b).to_raw(), 8u8);
    assert_eq!((a - b).to_raw(), 2u8);
}

#[test]
fn precise_u8_arith() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Precise>;
    let a = U::from_constant(USize(5));
    let b = U::from_constant(USize(3));
    assert_eq!((a + b).to_raw(), 8u16);
    assert_eq!((a * b).to_raw(), 15u16);
}

#[test]
fn abs_is_identity_on_ufixed() {
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    let a = U::from_raw(42);
    assert_eq!(a.abs().to_raw(), 42u8);
}

#[test]
fn total_cmp_orders_ufixed() {
    use core::cmp::Ordering;
    type U = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    let a = U::from_raw(10);
    let b = U::from_raw(20);
    assert_eq!(a.total_cmp(&b), Ordering::Less);
    assert_eq!(b.total_cmp(&a), Ordering::Greater);
    assert_eq!(a.total_cmp(&a), Ordering::Equal);
}

#[test]
fn from_constant_places_at_integer_bit_position() {
    // UFixed<8, 8, Warm>: 8.8 fixed point. from_constant(1) = 1 << 8 = 256.
    type U = UFixed<{ IBits(8) }, { FBits(8) }, Warm>;
    let one = U::from_constant(USize(1));
    assert_eq!(one.to_raw(), 1u32 << 8);
}

// Note: IntegerLike / FractionLike trait-selection tests are
// intentionally omitted. Their generic impls rely on the same
// `{ FBits::ZERO }` and `[(); 1 / is_fractional(F)]:` const-expression
// patterns that currently trigger a normalizer cycle when called from
// downstream test code on nightly. The marker impls themselves compile
// and `BitPresentation` dispatch below exercises the same shape. Full
// trait-selection coverage returns when the const-expr machinery
// stabilises further.

#[test]
fn bit_presentation_logical_width() {
    type U = UFixed<{ IBits(8) }, { FBits(4) }, Warm>;
    assert_eq!(<U as BitPresentation>::LOGICAL_WIDTH.0, 12);
}
