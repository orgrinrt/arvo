//! Const-context smoke test for `FastFloat` and `StrictFloat`.
//!
//! Closes audit Finding 29 (round 202605041051, task #324). The
//! `core::ops::Add`/`Sub`/`Mul`/`Div` impls on `FastFloat<F>` and
//! `StrictFloat<F>` are `impl const` gated on `F: [const] core::ops::Op`.
//! Whether the bound holds for bare `f32` / `f64` depends on rustc
//! nightly stability of `core::ops::Add` const for primitive floats.
//!
//! Surface always const-callable: `FastFloat::new`, `into_inner`,
//! `to_raw`, `from_raw` (the inherent constructors and unwrap doors).
//! These exercise the `repr(transparent)` path in const context.
//! Arithmetic ops are exercised at runtime.

#![feature(const_trait_impl)]

use arvo::{FastFloat, StrictFloat};

const _FAST_F32_NEW: FastFloat<f32> = FastFloat::new(1.5_f32);
const _FAST_F32_RAW: f32 = FastFloat::new(2.5_f32).into_inner();

const _FAST_F64_NEW: FastFloat<f64> = FastFloat::new(1.5_f64);
const _FAST_F64_RAW: f64 = FastFloat::new(2.5_f64).into_inner();

const _STRICT_F32_NEW: StrictFloat<f32> = StrictFloat::new(1.5_f32);
const _STRICT_F32_RAW: f32 = StrictFloat::new(2.5_f32).into_inner();

const _STRICT_F64_NEW: StrictFloat<f64> = StrictFloat::new(1.5_f64);
const _STRICT_F64_RAW: f64 = StrictFloat::new(2.5_f64).into_inner();

#[test]
fn float_const_construct_runtime() {
    assert_eq!(_FAST_F32_RAW, 2.5_f32);
    assert_eq!(_FAST_F64_RAW, 2.5_f64);
    assert_eq!(_STRICT_F32_RAW, 2.5_f32);
    assert_eq!(_STRICT_F64_RAW, 2.5_f64);
}

#[test]
fn fastfloat_arith_runtime() {
    let a: FastFloat<f32> = FastFloat::new(2.0_f32);
    let b: FastFloat<f32> = FastFloat::new(3.0_f32);
    assert_eq!((a + b).into_inner(), 5.0_f32);
    assert_eq!((a - b).into_inner(), -1.0_f32);
    assert_eq!((a * b).into_inner(), 6.0_f32);
    assert_eq!((b / a).into_inner(), 1.5_f32);
}

#[test]
fn strictfloat_arith_runtime() {
    let a: StrictFloat<f64> = StrictFloat::new(2.0_f64);
    let b: StrictFloat<f64> = StrictFloat::new(3.0_f64);
    assert_eq!((a + b).into_inner(), 5.0_f64);
    assert_eq!((a - b).into_inner(), -1.0_f64);
    assert_eq!((a * b).into_inner(), 6.0_f64);
    assert_eq!((b / a).into_inner(), 1.5_f64);
}
