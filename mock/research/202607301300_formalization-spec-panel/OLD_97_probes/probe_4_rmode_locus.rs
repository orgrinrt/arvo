//! Probe 4. The linked-library test classifies per lowering, not per fact,
//! and one target suffices to show it: on aarch64, the SAME abstract fact
//! (rounding mode) is ambient for `fadd` (reads FPCR.RMode, a linked library
//! can change it) and a lowering decision for the `as` cast (`fcvtzs`
//! encodes round-toward-zero in the instruction; FPCR cannot touch it).
//!
//! Executed: write FPCR.RMode to round-toward-plus-infinity, observe the
//! addition's result move and the cast's result not move, restore FPCR.
//! Caveat, stated: LLVM assumes the default FP environment, so a program
//! that perturbs FPCR is outside the toolchain's guarantees (file 93 probe 3
//! established what that licence does elsewhere); black_box fences every
//! operand so the operations execute on silicon, and the probe is offered as
//! a silicon read of the ISA-level distinction, with FPCR restored before
//! exit.
//!
//! Build: rustc --edition 2021 -O probe_4_rmode_locus.rs -o out/probe_4
//! Run:   ./out/probe_4  (aarch64 only)
//! Asm:   rustc --edition 2021 --crate-type=lib -O --emit=asm \
//!          -o out/probe_4_aarch64.s probe_4_rmode_locus.rs

use std::hint::black_box;

#[cfg(target_arch = "aarch64")]
fn read_fpcr() -> u64 {
    let v: u64;
    unsafe { core::arch::asm!("mrs {0}, fpcr", out(reg) v, options(nomem, nostack)) };
    v
}

#[cfg(target_arch = "aarch64")]
fn write_fpcr(v: u64) {
    unsafe { core::arch::asm!("msr fpcr, {0}", in(reg) v, options(nomem, nostack)) };
}

/// The cast lowering under scrutiny; emitted asm carries `fcvtzs` with the
/// rounding baked into the opcode.
#[no_mangle]
pub extern "C" fn cast_trunc(x: f64) -> i64 {
    x as i64
}

#[cfg(target_arch = "aarch64")]
fn main() {
    const RMODE_RP: u64 = 0b01 << 22; // round toward +infinity

    let one = black_box(1.0f64);
    let tiny = black_box(1e-300f64);

    let saved = read_fpcr();

    // baseline: round-to-nearest (IEEE default assumed by the toolchain)
    let add_rn = black_box(one) + black_box(tiny);
    let cast_rn = cast_trunc(black_box(2.7f64));
    let castn_rn = cast_trunc(black_box(-2.7f64));

    // perturb the ambient state the way a linked library could
    write_fpcr((saved & !(0b11 << 22)) | RMODE_RP);
    let add_rp = black_box(one) + black_box(tiny);
    let cast_rp = cast_trunc(black_box(2.7f64));
    let castn_rp = cast_trunc(black_box(-2.7f64));
    write_fpcr(saved); // restore before any further FP work

    println!("fadd 1.0 + 1e-300 under RN: == 1.0 ? {}", add_rn == 1.0);
    println!("fadd 1.0 + 1e-300 under RP: >  1.0 ? {}", add_rp > 1.0);
    println!("fadd moved with the ambient state: {}", add_rn != add_rp);
    println!("cast 2.7 / -2.7 under RN: {cast_rn} / {castn_rn}; under RP: {cast_rp} / {castn_rp}");
    println!(
        "cast moved with the ambient state: {}",
        cast_rn != cast_rp || castn_rn != castn_rp
    );
    println!("fpcr restored: {}", read_fpcr() == saved);
}

#[cfg(not(target_arch = "aarch64"))]
fn main() {
    println!("aarch64 only");
}
