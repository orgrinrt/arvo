//! Const-context smoke test for round 202605021200 Layer A.
//!
//! Validates that `USize` and `Cap` carry const-trait arithmetic so
//! consumers can compute on them at const time without falling back
//! to `.0` field-access.

#![feature(const_trait_impl)]
#![feature(const_ops)]

use arvo_storage::{Cap, USize};
use arvo_strategy::{Bounded, Identity};

const _USIZE_ZERO_PLUS_ONE: USize = USize::ZERO + USize::ONE;
const _USIZE_ADD: USize = USize(5) + USize(3);
const _USIZE_SUB: USize = USize(10) - USize(4);
const _USIZE_MUL: USize = USize(6) * USize(7);
const _USIZE_DIV: USize = USize(20) / USize(4);
const _USIZE_REM: USize = USize(13) % USize(5);
const _USIZE_SHL: USize = USize(1) << USize(4);
const _USIZE_SHR: USize = USize(64) >> USize(2);
const _USIZE_AND: USize = USize(0xFF) & USize(0x0F);
const _USIZE_OR: USize = USize(0xF0) | USize(0x0F);
const _USIZE_XOR: USize = USize(0xFF) ^ USize(0xAA);
const _USIZE_NOT: USize = !USize(0);

const _CAP_ZERO_PLUS_ONE: Cap = Cap::ZERO + Cap::ONE;
const _CAP_SUM: Cap = Cap(USize(10)) + Cap(USize(5));

// Chained const-trait composition: validates round-202605021400 step 2
// const lift on USize Add doesn't break under repeated invocation in
// a single const expression.
const _USIZE_CHAIN: USize = USize(1) + USize(2) + USize(3) + USize(4);

// Round 202605021600: Bounded + Identity blanket on bare primitives.
const _U8_MIN: u8 = <u8 as Bounded>::MIN;
const _U64_MAX: u64 = <u64 as Bounded>::MAX;
const _I32_NEG: i32 = <i32 as Identity>::ONE;

#[test]
fn const_evaluations_match_runtime() {
    assert_eq!(_USIZE_ZERO_PLUS_ONE.0, 1);
    assert_eq!(_USIZE_ADD.0, 8);
    assert_eq!(_USIZE_SUB.0, 6);
    assert_eq!(_USIZE_MUL.0, 42);
    assert_eq!(_USIZE_DIV.0, 5);
    assert_eq!(_USIZE_REM.0, 3);
    assert_eq!(_USIZE_SHL.0, 16);
    assert_eq!(_USIZE_SHR.0, 16);
    assert_eq!(_USIZE_AND.0, 0x0F);
    assert_eq!(_USIZE_OR.0, 0xFF);
    assert_eq!(_USIZE_XOR.0, 0x55);
    assert_eq!(_CAP_SUM.0.0, 15);
    assert_eq!(_USIZE_CHAIN.0, 10);
}
