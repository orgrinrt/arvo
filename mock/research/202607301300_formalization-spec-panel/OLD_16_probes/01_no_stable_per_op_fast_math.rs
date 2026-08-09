// Probe 01: does Rust (this pinned nightly, nightly-2026-05-28) expose ANY
// stable, per-operation lever for LLVM fast-math flags, distinct from the
// one named, stable, per-call-site fusion op (`f32::mul_add`)?
//
// This is not a law probe. It tests a claim about the TOOLCHAIN, which the
// whole "derived fact -> backend license" question in this dispatch stands
// or falls on: is there anywhere for a per-composition fact to attach to a
// backend decision at finer granularity than a whole-build codegen flag?
//
// Two builds. Build A (this file, unmodified) must FAIL to compile, because
// `core::intrinsics::fadd_fast` requires `#![feature(core_intrinsics)]`,
// which `unstable-features.md` forbids outright ("Compiler-internal
// surface... not intended for general use... unlikely to ever be
// stabilized"). Build B (same file, feature-gated) must SUCCEED, proving
// the intrinsic exists and is real, just forbidden. `mul_add` needs no
// gate at all: it is fully stable and has been since Rust 1.0-adjacent.
//
// Run:
//   rustc +nightly-2026-05-28 --edition 2024 01_no_stable_per_op_fast_math.rs -o /tmp/nofast 2>&1
//     (expect E0658, unstable feature, on the fadd_fast call)
//   rustc +nightly-2026-05-28 --edition 2024 --cfg gated 01_no_stable_per_op_fast_math.rs -o /tmp/gated 2>&1 && /tmp/gated
//     (expect a clean compile and a printed result, proving the intrinsic
//     is real and only the FEATURE GATE stands between arvo and it)

#![cfg_attr(gated, feature(core_intrinsics))]

fn stable_fma(a: f32, b: f32, c: f32) -> f32 {
    // stable since 1.0-era; lowers to `llvm.fmuladd.f32`, one rounding step.
    a.mul_add(b, c)
}

#[cfg(gated)]
fn unstable_fadd_fast(a: f32, b: f32) -> f32 {
    // per-instruction LLVM `fadd fast` flag. real, and the ONLY per-operation
    // (as opposed to whole-build) lever this toolchain has for fast-math on
    // an individual add. requires the forbidden `core_intrinsics` feature.
    unsafe { core::intrinsics::fadd_fast(a, b) }
}

#[cfg(not(gated))]
fn unstable_fadd_fast(a: f32, b: f32) -> f32 {
    // this call is the one that must fail to compile without the gate.
    unsafe { core::intrinsics::fadd_fast(a, b) }
}

fn main() {
    println!("mul_add(2.0, 3.0, 1.0) = {}", stable_fma(2.0, 3.0, 1.0));
    println!("fadd_fast(1.0, 2.0) = {}", unstable_fadd_fast(1.0, 2.0));
}
