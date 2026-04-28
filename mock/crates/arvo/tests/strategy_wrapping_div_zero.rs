//! Wrapping strategies return the numerator on div-by-zero.
//!
//! Pins the post-review fix (F1) semantic: Hot / Warm / Cold `u_div`
//! and `i_div` MUST NOT panic when `b == 0`. They return `a` (the
//! numerator) as a cheap, defined fallback — wrapping math has no
//! identity for zero.

#![no_std]

use arvo::ifixed::IFixed;
use arvo::{FBits, IBits, ibits, fbits};
use arvo::strategy::{Cold, Hot, Warm};
use arvo::ufixed::UFixed;

#[test]
fn u_div_by_zero_returns_numerator_hot() {
    type U = UFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;
    let a = U::from_raw(42);
    let b = U::from_raw(0);
    assert_eq!((a / b).to_raw(), 42u8);
}

#[test]
fn u_div_by_zero_returns_numerator_warm() {
    // Warm u8 uses u16 container (2x widened).
    type U = UFixed<{ ibits(8) }, { FBits::ZERO }, Warm>;
    let a = U::from_raw(42);
    let b = U::from_raw(0);
    assert_eq!((a / b).to_raw(), 42u16);
}

#[test]
fn u_div_by_zero_returns_numerator_cold() {
    // Cold u8 uses u8 container (shares Hot width in L0).
    type U = UFixed<{ ibits(8) }, { FBits::ZERO }, Cold>;
    let a = U::from_raw(42);
    let b = U::from_raw(0);
    assert_eq!((a / b).to_raw(), 42u8);
}

#[test]
fn i_div_by_zero_returns_numerator_hot() {
    // Hot ibits(8) uses i16 container.
    type I = IFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;
    let a = I::from_raw(-42);
    let b = I::from_raw(0);
    assert_eq!((a / b).to_raw(), -42i16);
}

#[test]
fn i_div_by_zero_returns_numerator_warm() {
    // Warm ibits(8) uses i32 container (2x widened over Hot).
    type I = IFixed<{ ibits(8) }, { FBits::ZERO }, Warm>;
    let a = I::from_raw(-42);
    let b = I::from_raw(0);
    assert_eq!((a / b).to_raw(), -42i32);
}

#[test]
fn i_div_by_zero_returns_numerator_cold() {
    // Cold ibits(8) uses i16 container (shares Hot width in L0).
    type I = IFixed<{ ibits(8) }, { FBits::ZERO }, Cold>;
    let a = I::from_raw(-42);
    let b = I::from_raw(0);
    assert_eq!((a / b).to_raw(), -42i16);
}
