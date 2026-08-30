//! Probe 1: the x86 form of the receipt assertion, owed since `91:1011-1012`
//! ("a compiled probe parallel to `90_probes/probe_1`, before the annotate
//! shape's verifier claim is called portable").
//!
//! `90_probes/probe_1` built the aarch64 form: one `mrs`, one masked compare,
//! one branch, reading FPCR bits RMode [23:22] and FZ [24]. File 90 priced the
//! verifier at "three instructions" (`90:203`) and the ninth consolidation
//! carries that price into the trusted-base entry for `IeeeDefault`
//! (`91:861-863`). This probe asks whether the same sentence is true on the
//! other mainstream target.
//!
//! Separation statement per `86b`: this probe separates "the abstract IEEE
//! default environment" from "the control-register state that realises it on a
//! target". On aarch64 the two coincide in one register with two fields, which
//! is the instantiation file 90 measured. x86_64 is where the distinction is
//! nonvacuous, so it is where the claim has to be checked.
//!
//! x86_64 fields, secondary read (Intel SDM Vol. 1 10.2.3 for MXCSR,
//! Vol. 1 8.1.5 for the x87 FCW):
//!   MXCSR: RC bits [14:13] (0b00 = round to nearest even),
//!          FTZ bit [15] (flush denormal RESULTS to zero),
//!          DAZ bit [6]  (treat denormal INPUTS as zero).
//!   x87 FCW: RC bits [11:10], PC (precision control) bits [9:8].
//! IEEE 754 default environment: RNE, gradual underflow, so on x86 that is
//! RC = 0b00 AND FTZ = 0 AND DAZ = 0, three fields in two disjoint bit
//! positions plus one more field ten bits away, against aarch64's two.
//!
//! Build:
//!   rustc --target x86_64-apple-darwin --crate-type=lib -O --emit=asm \
//!         -o probe_1_x86.s probe_1_the_x86_receipt_is_not_the_same_receipt.rs
//!   rustc --target aarch64-apple-darwin --crate-type=lib -O --emit=asm \
//!         -o probe_1_aarch64.s probe_1_the_x86_receipt_is_not_the_same_receipt.rs

#![no_std]
#![allow(internal_features)]

// ---------------------------------------------------------------------------
// aarch64: file 90's shape, reproduced here so the two emit side by side.
// ---------------------------------------------------------------------------

#[cfg(target_arch = "aarch64")]
const A_RMODE_MASK: u64 = 0b11 << 22;
#[cfg(target_arch = "aarch64")]
const A_FZ_BIT: u64 = 1 << 24;

/// aarch64 receipt: the whole IEEE-default question is two fields of one
/// register, and the register is readable into a GPR by one instruction.
#[cfg(target_arch = "aarch64")]
#[inline(never)]
#[no_mangle]
pub extern "C" fn receipt_aarch64() -> bool {
    let fpcr: u64;
    unsafe { core::arch::asm!("mrs {0}, fpcr", out(reg) fpcr, options(nomem, nostack)) };
    (fpcr & (A_RMODE_MASK | A_FZ_BIT)) == 0
}

// ---------------------------------------------------------------------------
// x86_64, shape A: the naive "parallel to probe_1" translation. RC and FTZ
// only, which is the field-for-field transliteration of the aarch64 receipt.
// This is the form that would be written by someone porting the aarch64
// receipt without re-deriving what the environment is on this target, and it
// is WRONG: it passes with DAZ set, and DAZ set is not the IEEE default
// environment (denormal inputs are silently zeroed, so gradual underflow does
// not hold on the input side).
// ---------------------------------------------------------------------------

#[cfg(target_arch = "x86_64")]
const X_RC_MASK: u32 = 0b11 << 13;
#[cfg(target_arch = "x86_64")]
const X_FTZ_BIT: u32 = 1 << 15;
#[cfg(target_arch = "x86_64")]
const X_DAZ_BIT: u32 = 1 << 6;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn read_mxcsr() -> u32 {
    let mut csr: u32 = 0;
    unsafe { core::arch::asm!("stmxcsr [{0}]", in(reg) &mut csr, options(nostack)) };
    csr
}

/// The transliterated receipt. Compiles. Is not the receipt.
#[cfg(target_arch = "x86_64")]
#[inline(never)]
#[no_mangle]
pub extern "C" fn receipt_x86_transliterated() -> bool {
    let csr = read_mxcsr();
    (csr & (X_RC_MASK | X_FTZ_BIT)) == 0
}

/// The honest MXCSR receipt: RC, FTZ, and DAZ. Still not the whole story,
/// because it says nothing about the x87 control word (below).
#[cfg(target_arch = "x86_64")]
#[inline(never)]
#[no_mangle]
pub extern "C" fn receipt_x86_mxcsr() -> bool {
    let csr = read_mxcsr();
    (csr & (X_RC_MASK | X_FTZ_BIT | X_DAZ_BIT)) == 0
}

/// The x87 half. A second control register, with its own rounding-control
/// field AND a precision-control field that has no aarch64 analogue at all:
/// PC selects 24/53/64-bit significand for x87 arithmetic, so an x87 path can
/// round twice (to 64 bits then to 53) and produce a result that is not the
/// correctly-rounded double. Nothing in the aarch64 receipt has a counterpart
/// to this field, so "read the control register" is not one obligation here.
#[cfg(target_arch = "x86_64")]
const X87_RC_MASK: u16 = 0b11 << 10;
#[cfg(target_arch = "x86_64")]
const X87_PC_MASK: u16 = 0b11 << 8;
#[cfg(target_arch = "x86_64")]
const X87_PC_EXTENDED: u16 = 0b11 << 8;

#[cfg(target_arch = "x86_64")]
#[inline(never)]
#[no_mangle]
pub extern "C" fn receipt_x86_full() -> bool {
    let csr = read_mxcsr();
    let mxcsr_ok = (csr & (X_RC_MASK | X_FTZ_BIT | X_DAZ_BIT)) == 0;

    let mut fcw: u16 = 0;
    unsafe { core::arch::asm!("fnstcw [{0}]", in(reg) &mut fcw, options(nostack)) };
    // x87 default after FINIT is RC = 0b00 (RNE) and PC = 0b11 (extended,
    // 64-bit significand). "IEEE default" for a binary64 computation on the
    // x87 path is NOT PC = extended, since double rounding is then possible;
    // which of the two a declaration means is a design question, not a
    // register read. The probe records both and asserts neither.
    let x87_rne = (fcw & X87_RC_MASK) == 0;
    let x87_pc_is_extended = (fcw & X87_PC_MASK) == X87_PC_EXTENDED;

    mxcsr_ok && x87_rne && x87_pc_is_extended
}
