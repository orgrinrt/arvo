//! Const-context smoke test for `IFixed` const composition.
//!
//! Closes audit Finding 28 (IFixed half) (round 202605041051,
//! task #324). Round 202605021400 lifted IFixed arithmetic to
//! `impl const`. Round 306 added blanket `Identity` and
//! `SignedIdentity::MINUS_ONE` projections through inner `Bits`.
//! This file exercises const composition at signed widths covering
//! Hot, Warm, Precise.

#![feature(adt_const_params)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::ifixed::IFixed;
use arvo::strategy::{Hot, Precise, Warm};
use arvo::traits::FromConstant;
use arvo::{FBits, USize, ibits};
use arvo_strategy::Identity;

// IFixed<7, 0, Hot> = 1 sign + 7 = 8 bits = i8.
type I8Hot = IFixed<{ ibits(7) }, { FBits::ZERO }, Hot>;
type I16Warm = IFixed<{ ibits(15) }, { FBits::ZERO }, Warm>;
type I32Precise = IFixed<{ ibits(31) }, { FBits::ZERO }, Precise>;

const _I8_ZERO: I8Hot = <I8Hot as Identity>::ZERO;
const _I8_ONE: I8Hot = <I8Hot as Identity>::ONE;
const _I16_ZERO: I16Warm = <I16Warm as Identity>::ZERO;
const _I32_ZERO: I32Precise = <I32Precise as Identity>::ZERO;

const _I8_HOT_ADD: I8Hot = {
    let a = I8Hot::from_constant::<{ USize(5) }>();
    let b = I8Hot::from_constant::<{ USize(3) }>();
    a + b
};

const _I8_HOT_SUB: I8Hot = {
    let a = I8Hot::from_constant::<{ USize(10) }>();
    let b = I8Hot::from_constant::<{ USize(4) }>();
    a - b
};

const _I8_HOT_MUL: I8Hot = {
    let a = I8Hot::from_constant::<{ USize(6) }>();
    let b = I8Hot::from_constant::<{ USize(7) }>();
    a * b
};

const _I8_HOT_DIV: I8Hot = {
    let a = I8Hot::from_constant::<{ USize(20) }>();
    let b = I8Hot::from_constant::<{ USize(4) }>();
    a / b
};

const _I16_WARM_ADD: I16Warm = {
    let a = I16Warm::from_constant::<{ USize(100) }>();
    let b = I16Warm::from_constant::<{ USize(200) }>();
    a + b
};

const _I32_PRECISE_ADD: I32Precise = {
    let a = I32Precise::from_constant::<{ USize(1000) }>();
    let b = I32Precise::from_constant::<{ USize(2000) }>();
    a + b
};

#[test]
fn ifixed_const_runtime_parity_hot() {
    assert_eq!(_I8_HOT_ADD.to_raw(), 8i8);
    assert_eq!(_I8_HOT_SUB.to_raw(), 6i8);
    assert_eq!(_I8_HOT_MUL.to_raw(), 42i8);
    assert_eq!(_I8_HOT_DIV.to_raw(), 5i8);
}

#[test]
fn ifixed_const_runtime_parity_strategies() {
    assert_eq!(_I16_WARM_ADD.to_raw() as i32, 300);
    assert_eq!(_I32_PRECISE_ADD.to_raw() as i64, 3000);
}
