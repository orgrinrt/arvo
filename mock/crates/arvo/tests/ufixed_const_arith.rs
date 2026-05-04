//! Const-context smoke test for `UFixed` const composition.
//!
//! Closes audit Finding 28 (UFixed half) (round 202605041051,
//! task #324). Round 202605021400 lifted UFixed `Add`/`Sub`/`Mul`/
//! `Div` impls to `impl const`. Round 306 added blanket `Identity`
//! ZERO/ONE projections through inner `Bits`. This file exercises
//! the const composition surface in const blocks across
//! representative width tuples covering Hot, Warm, Cold, Precise.

#![feature(adt_const_params)]
#![feature(const_ops)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::strategy::{Cold, Hot, Precise, Warm};
use arvo::traits::FromConstant;
use arvo::ufixed::UFixed;
use arvo::{USize, fbits, ibits};
use arvo_strategy::Identity;

type U8Hot = UFixed<{ ibits(8) }, { fbits(0) }, Hot>;
type U16Warm = UFixed<{ ibits(16) }, { fbits(0) }, Warm>;
type U32Cold = UFixed<{ ibits(32) }, { fbits(0) }, Cold>;
type U64Precise = UFixed<{ ibits(64) }, { fbits(0) }, Precise>;

const _U8_ZERO: U8Hot = <U8Hot as Identity>::ZERO;
const _U8_ONE: U8Hot = <U8Hot as Identity>::ONE;

const _U16_ZERO: U16Warm = <U16Warm as Identity>::ZERO;
const _U16_ONE: U16Warm = <U16Warm as Identity>::ONE;

const _U32_ZERO: U32Cold = <U32Cold as Identity>::ZERO;
const _U32_ONE: U32Cold = <U32Cold as Identity>::ONE;

const _U64_ZERO: U64Precise = <U64Precise as Identity>::ZERO;
const _U64_ONE: U64Precise = <U64Precise as Identity>::ONE;

const _U8_HOT_ADD_ZERO_ONE: U8Hot = {
    let z = <U8Hot as Identity>::ZERO;
    let o = <U8Hot as Identity>::ONE;
    z + o
};

const _U8_HOT_ADD_FROM_CONSTANT: U8Hot = {
    let a = U8Hot::from_constant::<{ USize(5) }>();
    let b = U8Hot::from_constant::<{ USize(3) }>();
    a + b
};

const _U8_HOT_SUB: U8Hot = {
    let a = U8Hot::from_constant::<{ USize(10) }>();
    let b = U8Hot::from_constant::<{ USize(4) }>();
    a - b
};

const _U8_HOT_MUL: U8Hot = {
    let a = U8Hot::from_constant::<{ USize(6) }>();
    let b = U8Hot::from_constant::<{ USize(7) }>();
    a * b
};

const _U8_HOT_DIV: U8Hot = {
    let a = U8Hot::from_constant::<{ USize(20) }>();
    let b = U8Hot::from_constant::<{ USize(4) }>();
    a / b
};

const _U16_WARM_ADD: U16Warm = {
    let a = U16Warm::from_constant::<{ USize(100) }>();
    let b = U16Warm::from_constant::<{ USize(200) }>();
    a + b
};

const _U32_COLD_ADD: U32Cold = {
    let a = U32Cold::from_constant::<{ USize(1000) }>();
    let b = U32Cold::from_constant::<{ USize(2000) }>();
    a + b
};

const _U64_PRECISE_ADD: U64Precise = {
    let a = U64Precise::from_constant::<{ USize(10) }>();
    let b = U64Precise::from_constant::<{ USize(20) }>();
    a + b
};

#[test]
fn ufixed_const_runtime_parity_hot() {
    assert_eq!(_U8_HOT_ADD_FROM_CONSTANT.to_raw(), 8u8);
    assert_eq!(_U8_HOT_SUB.to_raw(), 6u8);
    assert_eq!(_U8_HOT_MUL.to_raw(), 42u8);
    assert_eq!(_U8_HOT_DIV.to_raw(), 5u8);
}

#[test]
fn ufixed_const_runtime_parity_strategies() {
    assert_eq!(_U16_WARM_ADD.to_raw(), 300u32);
    assert_eq!(_U32_COLD_ADD.to_raw(), 3000u32);
    assert_eq!(_U64_PRECISE_ADD.to_raw(), 30u128);
}
