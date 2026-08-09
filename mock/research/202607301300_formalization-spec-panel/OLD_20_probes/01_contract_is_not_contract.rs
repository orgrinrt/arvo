//! PROBE 1: is `f64::mul_add` the `contract` liberty, or a different operation
//! wearing its name.
//!
//! File 17 section 8 lists `contract` among the fast-math liberties that are
//! source-expressible, gives `a.mul_add(b, c)` as the source form, and states
//! it "lowers to `llvm.fmuladd`". `llvm.fmuladd` and `llvm.fma` are different
//! contracts:
//!
//!   llvm.fmuladd  the backend MAY fuse. A licence. The result is whichever of
//!                 the two forms the target picked, so it varies by target, and
//!                 it is never slower than a multiply and an add.
//!   llvm.fma      the backend MUST produce the single-rounded result. An
//!                 operation, with the exact IEEE 754-2019 `fusedMultiplyAdd`
//!                 definition, which on a target with no FMA unit is a libm
//!                 call.
//!
//! Part A0 checks whether the source form is available in arvo's environment at
//! all. Part A reads which intrinsic it emits. Parts B to D read what it costs
//! on a target that has an FMA unit and on one that does not.
//!
//! Run: ./01_run.sh

// This file is compiled twice by the runner, once with `--cfg nostd` to stand
// in for arvo's own crate roots (every arvo crate is `#![no_std]`).
#![cfg_attr(nostd, no_std)]
#![crate_type = "lib"]

#[no_mangle]
pub fn via_mul_add(a: f64, b: f64, c: f64) -> f64 {
    a.mul_add(b, c)
}

#[no_mangle]
pub fn via_source(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}
