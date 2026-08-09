//! Probe 1: what the aarch64 ISA actually defines for the two division corner cells,
//! read from the silicon through inline asm rather than assumed from a manual.
//!
//! WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates the two corner cells of integer
//! division that the fork's discussion has been treating as one kind: `x/0` (no exact
//! result exists) and `INT_MIN / -1` (an exact result exists and is out of range). The
//! distinction is nonvacuous exactly because the ISA defines both and the value layer
//! defines only the second: `ReduceModulo` has an answer for `INT_MIN / -1` (the wrapped
//! `INT_MIN`) and no answer for `x/0`. A probe reading only `x/0` could not show that the
//! ISA-agrees-with-the-value-layer cell and the ISA-is-the-only-author cell are different
//! cells.
//!
//! CLAIM A: aarch64 `sdiv`/`udiv` with a zero divisor return 0, for positive, negative,
//!   and zero dividends. Deterministic, no trap. (ARM DDI 0487 documents this; the probe
//!   reads the silicon.)
//! CLAIM B: aarch64 `sdiv` at `INT_MIN / -1` returns `INT_MIN`, which IS the value-layer
//!   `ReduceModulo` answer (2^63 mod 2^64, re-signed). So this cell is a stated value the
//!   target happens to give away free, not a target-defined value.
//!
//! Build: rustc --edition 2021 -O probe_1_what_the_isa_actually_defines.rs --out-dir out
//! Run: ./out/probe_1_what_the_isa_actually_defines
//! Outcome: WORKS. Both claims hold.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), aarch64-apple-darwin.

use core::arch::asm;

#[inline(never)]
fn sdiv(x: i64, d: i64) -> i64 {
    let out: i64;
    // the raw instruction, opaque to LLVM, so no UB reasoning and no inserted checks
    unsafe { asm!("sdiv {0}, {1}, {2}", out(reg) out, in(reg) x, in(reg) d) };
    out
}

#[inline(never)]
fn udiv(x: u64, d: u64) -> u64 {
    let out: u64;
    unsafe { asm!("udiv {0}, {1}, {2}", out(reg) out, in(reg) x, in(reg) d) };
    out
}

fn main() {
    // CLAIM A: x/0 is the ISA's own cell; aarch64 defines it as 0, every dividend.
    assert_eq!(sdiv(5, 0), 0, "sdiv 5/0");
    assert_eq!(sdiv(-5, 0), 0, "sdiv -5/0");
    assert_eq!(sdiv(0, 0), 0, "sdiv 0/0");
    assert_eq!(sdiv(i64::MIN, 0), 0, "sdiv MIN/0");
    assert_eq!(udiv(u64::MAX, 0), 0, "udiv MAX/0");

    // CLAIM B: INT_MIN / -1 is the value layer's cell; the ISA merely agrees with it.
    // ReduceModulo of the exact quotient 2^63 into 64 signed bits is -2^63 = INT_MIN.
    assert_eq!(sdiv(i64::MIN, -1), i64::MIN, "sdiv MIN/-1 wraps to MIN");

    println!("CLAIM A holds: aarch64 sdiv/udiv x/0 -> 0, deterministic, no trap");
    println!("CLAIM B holds: aarch64 sdiv MIN/-1 -> MIN, which equals ReduceModulo's own answer");
}
