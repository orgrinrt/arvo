// The licence, half 1: does a real, stable-track, per-call-site mechanism
// exist that grants LLVM reassociation permission on exactly the
// expression a proof covers, rather than on the whole compilation unit
// the way `-ffast-math` would? `f32::algebraic_add` (feature
// `float_algebraic`, tracking issue rust-lang/rust#136469, stabilization
// PR rust-lang/rust#157029 already open) is the candidate. This probe
// reproduces file 50's own measurement (49:4.6, 50:439-453: five scalar
// `fadd`, LLVM refuses to reassociate a float reduction) and shows the
// algebraic form vectorises to the same shape as the integer reduction
// LLVM already reassociates freely.

#![allow(dead_code)]
#![feature(float_algebraic)]

#[inline(never)]
#[no_mangle]
pub fn sum_plain(xs: &[f32; 8]) -> f32 {
    let mut acc = 0.0f32;
    let mut i = 0;
    while i < 8 {
        acc = acc + xs[i];
        i += 1;
    }
    acc
}

#[inline(never)]
#[no_mangle]
pub fn sum_algebraic(xs: &[f32; 8]) -> f32 {
    let mut acc = 0.0f32;
    let mut i = 0;
    while i < 8 {
        acc = acc.algebraic_add(xs[i]);
        i += 1;
    }
    acc
}

#[inline(never)]
#[no_mangle]
pub fn sum_int(xs: &[i32; 8]) -> i32 {
    let mut acc = 0i32;
    let mut i = 0;
    while i < 8 {
        acc = acc + xs[i];
        i += 1;
    }
    acc
}

// build: rustc +nightly-2026-05-28 --edition 2021 --crate-type lib
//          -C opt-level=3 --emit asm,llvm-ir probe_4_licence_reassoc_vectorizes.rs
//
// measured, aarch64-apple-darwin:
//   sum_plain:      8x scalar `fadd s0, s0, sN`, no vector instruction at all.
//   sum_algebraic:  `fadd.4s v0, v1, v0` (one vector add across all 8 lanes)
//                   then `faddp.4s` / `faddp.2s` (pairwise horizontal reduction).
//   sum_int:        `add.4s v0, v1, v0` then `addv.4s` (the vector reduce
//                   intrinsic), the identical two-instruction shape.
//
// llvm-ir for sum_algebraic's reduction call:
//   %1 = tail call reassoc nsz arcp contract float
//          @llvm.vector.reduce.fadd.v8f32(float 0.0, <8 x float> %0)
//
// `nnan` and `ninf` are absent. sum_plain's llvm-ir has no fast-math
// flags on its scalar `fadd` calls at all.
