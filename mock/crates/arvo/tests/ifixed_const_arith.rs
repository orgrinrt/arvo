//! Const-context smoke test for `IFixed` const composition.
//!
//! Closes audit Finding 28 (IFixed half) (round 202605041051,
//! task #324). Round 202605021400 lifted IFixed arithmetic to
//! `impl const`. Round 306 added blanket `Identity` and
//! `SignedIdentity::MINUS_ONE` projections through inner `Bits`.
//! Round 202605041128 (#325) added blanket `Bounded` MIN/MAX
//! projections through inner signed `Bits`; this file extends to
//! assert MIN/MAX const-callable surface plus runtime parity.

#![feature(adt_const_params)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo::ifixed::IFixed;
use arvo::strategy::{Hot, Precise, Warm};
use arvo::traits::FromConstant;
use arvo::{FBits, USize, ibits};
use arvo_strategy::{Bounded, Identity};

// IFixed<7, 0, Hot> = 1 sign + 7 = 8 bits = i8.
type I8Hot = IFixed<{ ibits(7) }, { FBits::ZERO }, Hot>;
type I16Warm = IFixed<{ ibits(15) }, { FBits::ZERO }, Warm>;
type I32Precise = IFixed<{ ibits(31) }, { FBits::ZERO }, Precise>;

const _I8_ZERO: I8Hot = <I8Hot as Identity>::ZERO;
const _I8_ONE: I8Hot = <I8Hot as Identity>::ONE;
const _I16_ZERO: I16Warm = <I16Warm as Identity>::ZERO;
const _I32_ZERO: I32Precise = <I32Precise as Identity>::ZERO;

// Bounded MIN / MAX projections through the inner signed Bits Bounded
// blanket (Round 7, #325). Round 6 had to skip these per deviation 3.
const _I8_MIN: I8Hot = <I8Hot as Bounded>::MIN;
const _I8_MAX: I8Hot = <I8Hot as Bounded>::MAX;
const _I16_MIN: I16Warm = <I16Warm as Bounded>::MIN;
const _I16_MAX: I16Warm = <I16Warm as Bounded>::MAX;
const _I32_MIN: I32Precise = <I32Precise as Bounded>::MIN;
const _I32_MAX: I32Precise = <I32Precise as Bounded>::MAX;

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

#[test]
fn ifixed_bounded_runtime_parity() {
    assert_eq!(_I8_MIN.to_raw(), i8::MIN);
    assert_eq!(_I8_MAX.to_raw(), i8::MAX);
    // I16Warm container is i32 (Warm bumps for >8 bits per Round 6 evidence).
    assert_eq!(_I16_MIN.to_raw() as i64, i32::MIN as i64);
    assert_eq!(_I16_MAX.to_raw() as i64, i32::MAX as i64);
    // I32Precise container is i64 (Precise doubles to fit logical width).
    assert_eq!(_I32_MIN.to_raw() as i128, i64::MIN as i128);
    assert_eq!(_I32_MAX.to_raw() as i128, i64::MAX as i128);
}
