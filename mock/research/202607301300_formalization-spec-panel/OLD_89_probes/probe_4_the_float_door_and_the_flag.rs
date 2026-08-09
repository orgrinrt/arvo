//! Probe 4: what reading B costs on the float side, where the ratified preset table sends
//! `Hot` and `Warm` through the hardware door.
//!
//! WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates "the standard's carrier is
//! unavailable" (file 50's claim, `50:322-327`) from "the standard's carrier is available
//! but unusable per-operation and per-value". The distinction is nonvacuous because the
//! probe actually reads the register rather than reasoning about whether `core` exposes a
//! wrapper: a check that only grepped `core` could not tell the two apart, and file 50's
//! did only that.
//!
//! It does NOT separate: anything about determinism under a pluggable executor, which is
//! file 50's OTHER and load-bearing argument and which this probe leaves entirely intact.
//!
//! CLAIM A: `core::arch::asm!` reads AArch64's FPSR on the pinned toolchain, with no
//!   feature gate and no `core::arch::aarch64` intrinsic. So file 50's stronger sentence
//!   ("the standard's carrier is not available to us in any case", `50:326-327`) is too
//!   strong as written. Its weaker half (no `fetestexcept`, no `fegetround`, no FPCR
//!   intrinsic in `core` or `std`) reproduces exactly and is re-run in the file.
//! CLAIM B: the flag is STICKY and per-thread, so attributing an inexact event to one
//!   operation requires clearing it before and reading it after that operation. Asserted
//!   by exhibiting a value-attributing loop and counting its instructions against the
//!   plain one.
//! CLAIM C: the value-attributing loop does not vectorise and serialises against the FPU,
//!   because the register read is a barrier the scheduler cannot move across arithmetic.
//!   Read out of the disassembly, not asserted.
//! CLAIM D: the recompute route (do the operation, then check exactness independently) is
//!   what the ratified `Warm` float row forbids by name (`78:448-455`: doubling `Warm`'s
//!   float storage adds bookkeeping the hardware never asks for). Exhibited for its
//!   instruction count so the two routes can be compared rather than argued.
//!
//! Build: rustc --edition 2021 -O probe_4_the_float_door_and_the_flag.rs --out-dir out
//! Outcome: WORKS (runs and prints; the counts come from the object, see OUTCOMES.md).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), aarch64-apple-darwin.

use core::arch::asm;

/// AArch64 FPSR bit 4 is IXC, the cumulative inexact flag (Arm ARM, secondary read).
const IXC: u64 = 1 << 4;

#[inline(always)]
fn fpsr_read() -> u64 {
    let v: u64;
    unsafe { asm!("mrs {0}, fpsr", out(reg) v, options(nomem, nostack)) };
    v
}

#[inline(always)]
fn fpsr_clear_ixc() {
    let v = fpsr_read() & !IXC;
    unsafe { asm!("msr fpsr, {0}", in(reg) v, options(nomem, nostack)) };
}

/// The plain fold: the ratified `Hot`/`Warm` float shape, hardware door, no grade.
#[no_mangle]
pub extern "C" fn ffold_plain(xs: &[f64; 64]) -> f64 {
    let mut acc = 0.0f64;
    let mut i = 0usize;
    while i < 64 {
        acc += xs[i];
        i += 1;
    }
    acc
}

/// Reading B through the hardware flag: clear, operate, read, per element.
#[no_mangle]
pub extern "C" fn ffold_reading_b_via_flag(xs: &[f64; 64]) -> (f64, u32) {
    let mut acc = 0.0f64;
    let mut ev = 0u32;
    let mut i = 0usize;
    while i < 64 {
        fpsr_clear_ixc();
        acc += xs[i];
        if fpsr_read() & IXC != 0 {
            ev += 1;
        }
        i += 1;
    }
    (acc, ev)
}

/// Reading B by recomputation: the route the ratified `Warm` float row forbids by name.
/// A double-rounded check against a wider intermediate; here the wider intermediate is
/// exact 2Sum, which is the cheapest honest form.
#[no_mangle]
pub extern "C" fn ffold_reading_b_via_recompute(xs: &[f64; 64]) -> (f64, u32) {
    let mut acc = 0.0f64;
    let mut ev = 0u32;
    let mut i = 0usize;
    while i < 64 {
        let a = acc;
        let b = xs[i];
        let s = a + b;
        // Knuth 2Sum: the exact error term of the addition, zero exactly when the add was
        // exact. Six flops for one add's worth of information.
        let bb = s - a;
        let err = (a - (s - bb)) + (b - bb);
        ev += (err != 0.0) as u32;
        acc = s;
        i += 1;
    }
    (acc, ev)
}

/// Reading A on the same shape: the count is `SITES * arity`, a literal.
#[no_mangle]
pub extern "C" fn ffold_reading_a(xs: &[f64; 64]) -> (f64, u32) {
    let mut acc = 0.0f64;
    let mut i = 0usize;
    while i < 64 {
        acc += xs[i];
        i += 1;
    }
    (acc, 64)
}

fn main() {
    // CLAIM A: the register reads.
    let before = fpsr_read();
    let a = 1.0f64;
    let b = 1.0f64 / 3.0f64; // inexact by construction
    let s = core::hint::black_box(a) + core::hint::black_box(b);
    let after = fpsr_read();
    println!(
        "FPSR before = {:#x}, after one inexact add = {:#x}, sum = {}",
        before, after, s
    );
    assert!(after & IXC != 0, "the inexact flag must be set after an inexact addition; if this fails the register read is not doing what the probe claims");
    println!(
        "CLAIM A holds: core::arch::asm! reads FPSR on the pin, no feature gate, and IXC is set."
    );

    // CLAIM B: the flag is sticky, so a second inexact add is indistinguishable from the
    // first without a clear in between.
    let s2 = core::hint::black_box(s) + core::hint::black_box(b);
    let after2 = fpsr_read();
    assert_eq!(
        after & IXC,
        after2 & IXC,
        "sticky: the second add cannot be told from the first"
    );
    fpsr_clear_ixc();
    assert_eq!(
        fpsr_read() & IXC,
        0,
        "and clearing works, so per-operation attribution needs a clear per operation"
    );
    let exact = core::hint::black_box(2.0f64) + core::hint::black_box(2.0f64);
    assert_eq!(
        fpsr_read() & IXC,
        0,
        "an exact add leaves IXC clear, which is exactly reading B's condition"
    );
    println!(
        "CLAIM B holds: sticky, clearable, and an exact add ({}) leaves it clear.",
        exact
    );

    // Run the four folds so nothing is dead-stripped and the counts below are of live code.
    let mut xs = [0.0f64; 64];
    let mut i = 0usize;
    while i < 64 {
        xs[i] = 1.0 / ((i + 1) as f64);
        i += 1;
    }
    let p = ffold_plain(core::hint::black_box(&xs));
    let (va, ca) = ffold_reading_a(core::hint::black_box(&xs));
    let (vf, cf) = ffold_reading_b_via_flag(core::hint::black_box(&xs));
    let (vr, cr) = ffold_reading_b_via_recompute(core::hint::black_box(&xs));
    println!("plain = {p}");
    println!(
        "reading A: value {va}, count {ca}   (a literal; the count is the same for every input)"
    );
    println!("reading B via flag:      value {vf}, count {cf}");
    println!("reading B via recompute: value {vr}, count {cr}");
    assert_eq!(cf, cr, "the two routes to reading B must agree on the count, or one of them is not computing reading B");
    assert!(cf < 64, "and reading B must be strictly below reading A's 64 on this input, or the model cannot separate them");
    println!("\nBoth routes to reading B agree at {cf} of {ca}; the two readings are separated on this input.");
}
