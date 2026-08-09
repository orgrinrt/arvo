//! PROBE D: what each delivery of a refusal costs in EMITTED CODE.
//! Four shapes of the same summation over a u16 lane with logical bound 4095.
//!
//! NO TIMING IS TAKEN. The artifact is the instruction sequence, read from
//! `cargo rustc --release -- --emit asm -C opt-level=3` on aarch64-apple-darwin
//! under nightly-2026-05-28. A timing claim would be a bench and belongs in
//! `mock/benches/` per bench-and-sketch-discipline.md.
//!
//! LOOP BODIES AS EMITTED (aarch64):
//!
//! sum_saturate, 8 instructions, no branch but the back-edge:
//!     ldrh w11,[x0,x9] / add w8,w11,w8,uxth / cmp w8,#4095 /
//!     csel w8,w8,w10,lo / add x9,x9,#2 / cmp x9,#128 / b.ne
//!
//! sum_poison, 10 instructions, no branch but the back-edge, branchless via
//! the ccmp/csinv chain:
//!     ldrh w11,[x0,x9] / bic w12,w10,w8 / add w8,w11,w8 /
//!     and w13,w8,#0xffff / cmp w13,#4095 / ccmp w11,w10,#4,ls /
//!     ccmp w12,#0,#4,ne / csinv w8,w8,wzr,ne / add x9,x9,#2 / cmp / b.ne
//!
//! sum_outcome, 11 instructions AND TWO CONDITIONAL EXITS from the loop
//! (`b.lo LBB1_5`, `b.hs LBB1_5`). Two early exits per element. That control
//! flow is the short circuit, and it is what forecloses restructuring.
//!
//! sum_flag, LLVM unrolled it 4x with FOUR INDEPENDENT `orr` accumulators
//! (w9, w10, w11, w12) combined by three `orr` after the loop. The flag chain
//! is an independent reduction, which is why it unrolls and why the Outcome
//! version structurally cannot.
#![allow(dead_code)]
use notko::Outcome;

pub const HI: u16 = 4095;
#[derive(Clone, Copy)]
pub struct OutOfRange;

/// Delivery 1: refusal as control. Short-circuits per element.
#[unsafe(no_mangle)]
pub fn sum_outcome(xs: &[u16; 64]) -> Outcome<u16, OutOfRange> {
    let mut acc: u16 = 0;
    let mut i = 0;
    while i < 64 {
        match acc.checked_add(xs[i]) {
            Some(v) if v <= HI => acc = v,
            _ => return Outcome::Err(OutOfRange),
        }
        i += 1;
    }
    Outcome::Ok(acc)
}

/// Delivery 2: refusal as an accumulated flag, read once at the end.
#[unsafe(no_mangle)]
pub fn sum_flag(xs: &[u16; 64]) -> (u16, bool) {
    let mut acc: u32 = 0;
    let mut bad: u32 = 0;
    let mut i = 0;
    while i < 64 {
        acc += xs[i] as u32;
        bad |= (acc > HI as u32) as u32;
        i += 1;
    }
    (acc as u16, bad != 0)
}

/// Delivery 3: refusal as an absorbing value in the numeral's spare patterns.
pub const POISON: u16 = u16::MAX;
#[unsafe(no_mangle)]
pub fn sum_poison(xs: &[u16; 64]) -> u16 {
    let mut acc: u16 = 0;
    let mut i = 0;
    while i < 64 {
        let s = acc.wrapping_add(xs[i]);
        let bad = (s > HI) | (acc == POISON) | (xs[i] == POISON);
        acc = if bad { POISON } else { s };
        i += 1;
    }
    acc
}

/// Baseline: no refusal notion at all, clamping.
#[unsafe(no_mangle)]
pub fn sum_saturate(xs: &[u16; 64]) -> u16 {
    let mut acc: u16 = 0;
    let mut i = 0;
    while i < 64 {
        let s = acc.saturating_add(xs[i]);
        acc = if s > HI { HI } else { s };
        i += 1;
    }
    acc
}
