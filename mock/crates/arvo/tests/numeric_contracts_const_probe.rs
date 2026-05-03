//! Const-context probe for the numeric-contracts impl const sweep.
//!
//! Validates that `Sqrt` / `Recip` / `Abs` / `FromConstant` /
//! `TotalOrd` are all callable from `const fn` context after the
//! round 202605021800 sweep. Bodies for `total_cmp` (which depended
//! on the not-yet-const-stable `core::cmp::Ord::cmp` and
//! `f32::total_cmp`) route through `arvo_storage::ConstOrd::const_cmp`
//! plus a const-callable `ConstOrdering -> Ordering` bridge.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use core::cmp::Ordering;

use arvo::{FastFloat, IFixed, StrictFloat, UFixed, fbits, ibits};
use arvo::strategy::Hot;
use arvo::traits::{Abs, FromConstant, Recip, Sqrt, TotalOrd};
use arvo_storage::USize;

type U16 = UFixed<{ ibits(16) }, { fbits(0) }, Hot>;
type I8 = IFixed<{ ibits(7) }, { fbits(0) }, Hot>;

// FromConstant projects a USize const into a typed value. Must work
// at const time.
const _U16_FROM: U16 = <U16 as FromConstant>::from_constant::<{ USize(42) }>();
const _I8_FROM: I8 = <I8 as FromConstant>::from_constant::<{ USize(7) }>();

// TotalOrd routes through ConstOrd::const_cmp.
const _U16_CMP_LT: Ordering = {
    let a = <U16 as FromConstant>::from_constant::<{ USize(3) }>();
    let b = <U16 as FromConstant>::from_constant::<{ USize(5) }>();
    <U16 as TotalOrd>::total_cmp(a, b)
};
const _U16_CMP_EQ: Ordering = {
    let a = <U16 as FromConstant>::from_constant::<{ USize(7) }>();
    let b = <U16 as FromConstant>::from_constant::<{ USize(7) }>();
    <U16 as TotalOrd>::total_cmp(a, b)
};

// Sqrt on integer UFixed projects through u*::isqrt which is const.
const _U16_SQRT: U16 = {
    let n = <U16 as FromConstant>::from_constant::<{ USize(64) }>();
    <U16 as Sqrt>::sqrt(n)
};

// Abs on UFixed is identity; on IFixed routes through wrapping_abs / saturating_abs.
const _U16_ABS: U16 = <U16 as Abs>::abs(<U16 as FromConstant>::from_constant::<{ USize(3) }>());

// Float Sqrt / Recip / Abs / TotalOrd at const time.
const _F32_SQRT: FastFloat<f32> = <FastFloat<f32> as Sqrt>::sqrt(FastFloat(64.0));
const _F32_RECIP: FastFloat<f32> = <FastFloat<f32> as Recip>::recip(FastFloat(2.0));
const _F32_ABS: FastFloat<f32> = <FastFloat<f32> as Abs>::abs(FastFloat(-3.0));
const _F32_TC: Ordering = <FastFloat<f32> as TotalOrd>::total_cmp(FastFloat(1.0), FastFloat(2.0));

const _F64_SQRT: StrictFloat<f64> = <StrictFloat<f64> as Sqrt>::sqrt(StrictFloat(100.0));
const _F64_TC: Ordering = <StrictFloat<f64> as TotalOrd>::total_cmp(StrictFloat(2.0), StrictFloat(1.0));

#[test]
fn const_projections_resolve_at_runtime() {
    assert_eq!(_U16_CMP_LT, Ordering::Less);
    assert_eq!(_U16_CMP_EQ, Ordering::Equal);
    // sqrt(64) at integer width is 8.
    assert_eq!(_U16_SQRT.to_raw(), 8);
    assert_eq!(_U16_ABS.to_raw(), 3);
    // sqrt(64) under fast-math NR is within 7 ULP of 8.0.
    let s = _F32_SQRT.0;
    assert!((s - 8.0).abs() < 1e-3, "sqrt(64) ~= 8: got {}", s);
    assert_eq!(_F32_RECIP.0, 0.5);
    assert_eq!(_F32_ABS.0, 3.0);
    assert_eq!(_F32_TC, Ordering::Less);
    let s64 = _F64_SQRT.0;
    assert!((s64 - 10.0).abs() < 1e-6, "sqrt(100) ~= 10: got {}", s64);
    assert_eq!(_F64_TC, Ordering::Greater);
    // Verify FromConstant lands at the integer-bit position for IFixed
    // (F = 0 means raw == n).
    assert_eq!(_I8_FROM.to_raw(), 7);
}

#[test]
fn const_total_cmp_handles_nan_correctly() {
    // The XOR-trick total_cmp orders NaN as less-than positive infinity
    // bit-pattern wise. Test reflexivity on NaN bits.
    let nan = FastFloat(f32::NAN);
    let nan_cmp = <FastFloat<f32> as TotalOrd>::total_cmp(nan, nan);
    // Different NaN bit-patterns may compare differently; same NaN
    // value compares equal under bit-reinterpret.
    assert_eq!(nan_cmp, Ordering::Equal);
}
