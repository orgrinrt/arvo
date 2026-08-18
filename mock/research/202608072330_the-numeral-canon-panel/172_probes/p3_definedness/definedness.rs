// 172 P3. The licence's definedness clause: a partial interior operation is a
// binding-free distinguishing channel at EVERY profile.
//
// CLAIM UNDER TEST
//   171 section 3 bounds (L) by build profile: binding-free channels exist at
//   debug-assertions = on and none were found at off. Its alphabet was total.
//   This probe adds one partial operation (integer division) with the divisor an
//   INTERIOR intermediate, and asks whether two realisations of one stretch,
//   agreeing on final values wherever both are defined, are distinguishable at
//   debug-assertions = off with only the final value bound.
//
// THE CASE THAT MUST FAIL, declared before the run
//   The CONTROL arm keeps the same shape with the divisor an INPUT rather than an
//   intermediate. Its two realisations must agree on every swept input at off,
//   with no panic on either side. If the control distinguishes, the instrument is
//   detecting something other than the interior partiality and proves nothing.
//
// Build (per run.sh): rustc --edition 2021 -O -C debug-assertions=off
// The panic in the partial arm is expected at off; it is div-by-zero, which is
// not governed by debug-assertions.

#![allow(dead_code)]

use std::panic;

// The stretch: a / ((b + c) - d). Divisor is interior.
// Narrow realisation: the intermediate lives in i32, wrapping.
#[inline(never)]
fn partial_narrow(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let t = b.wrapping_add(c).wrapping_sub(d); // interior, unbound outside
    a / t
}

// Wide realisation: the intermediate lives in i64, exact.
#[inline(never)]
fn partial_wide(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let t = (b as i64) + (c as i64) - (d as i64);
    ((a as i64) / t) as i32
}

// CONTROL: same shape, divisor is an input. Two realisations of the (b+c)-d
// stretch feed a NON-divisor position; the divisor e is handed in.
#[inline(never)]
fn control_narrow(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let t = b.wrapping_add(c).wrapping_sub(d);
    (a / e).wrapping_add(t)
}

#[inline(never)]
fn control_wide(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let t = ((b as i64) + (c as i64) - (d as i64)) as i32; // same boundary value mod 2^32
    (a / e).wrapping_add(t)
}

fn outcome<F: Fn() -> i32 + panic::UnwindSafe>(f: F) -> Result<i32, ()> {
    panic::catch_unwind(f).map_err(|_| ())
}

fn main() {
    println!("debug_assertions = {}", cfg!(debug_assertions));

    // Constructed input: b + c - d wraps to exactly 0 in i32 while the exact
    // value is 2^32 (nonzero). b = c = 2^30, d = -(2^31) gives b+c-d = 2^31+2^31 = 2^32.
    // a = 0 makes the witness a PURE definedness difference: 0 / t = 0 for every
    // nonzero t, so on the a = 0 slice the two arms agree on every input where
    // both are defined, and differ only in WHERE they are defined.
    let (a, b, c, d): (i32, i32, i32, i32) = (0, 1 << 30, 1 << 30, i32::MIN);
    let exact_t = (b as i64) + (c as i64) - (d as i64);
    println!(
        "constructed divisor: exact = {} (nonzero), wrapped i32 = {}",
        exact_t,
        b.wrapping_add(c).wrapping_sub(d)
    );

    let narrow = outcome(|| partial_narrow(a, b, c, d));
    let wide = outcome(|| partial_wide(a, b, c, d));
    println!("partial arm, narrow realisation : {:?}", narrow);
    println!("partial arm, wide realisation   : {:?}", wide);
    let distinguished = narrow.is_err() != wide.is_err();
    println!("binding-free channel at this profile: {}", distinguished);

    // Sweep where both are defined: they must agree (extensional agreement on the
    // shared definedness domain).
    let mut checked = 0u32;
    let mut disagreements = 0u32;
    let mut skipped_undefined = 0u32;
    let mut x: i64 = 0x243F6A8885A308D3u64 as i64;
    for _ in 0..200_000 {
        // xorshift
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let a = (x >> 32) as i32;
        let b = (x >> 16) as i32;
        let c = (x >> 8) as i32;
        let d = x as i32;
        let n = outcome(|| partial_narrow(a, b, c, d));
        let w = outcome(|| partial_wide(a, b, c, d));
        match (n, w) {
            (Ok(nv), Ok(wv)) => {
                checked += 1;
                // agreement on the shared definedness domain is in the RAW sense:
                // both compute a/t with the same t value when neither wraps NOR
                // divides by zero; where narrow's t wrapped but stayed nonzero the
                // quotients legitimately differ (different t), so restrict to
                // non-wrapping t:
                let exact_t = (b as i64) + (c as i64) - (d as i64);
                if exact_t == (exact_t as i32) as i64 && nv != wv {
                    disagreements += 1;
                }
            }
            _ => skipped_undefined += 1,
        }
    }
    println!(
        "sweep: {} both-defined non-wrapping checked, {} disagreements, {} with a side undefined",
        checked, disagreements, skipped_undefined
    );

    // CONTROL: divisor is an input; must agree everywhere (e != 0), no panics.
    let mut ctl_checked = 0u32;
    let mut ctl_disagreements = 0u32;
    let mut ctl_panics = 0u32;
    let mut y: i64 = 0x452821E638D01377u64 as i64;
    for _ in 0..200_000 {
        y ^= y << 13;
        y ^= y >> 7;
        y ^= y << 17;
        let a = (y >> 32) as i32;
        let b = (y >> 16) as i32;
        let c = (y >> 8) as i32;
        let d = y as i32;
        let e = (y >> 24) as i32 | 1; // never zero
        let n = outcome(|| control_narrow(a, b, c, d, e));
        let w = outcome(|| control_wide(a, b, c, d, e));
        match (n, w) {
            (Ok(nv), Ok(wv)) => {
                ctl_checked += 1;
                if nv != wv {
                    ctl_disagreements += 1;
                }
            }
            _ => ctl_panics += 1,
        }
    }
    println!(
        "CONTROL (divisor is an input): {} checked, {} disagreements, {} panics",
        ctl_checked, ctl_disagreements, ctl_panics
    );
    println!(
        "CONTROL verdict: {}",
        if ctl_disagreements == 0 && ctl_panics == 0 {
            "PASS (control does not distinguish)"
        } else {
            "FAIL"
        }
    );

    // The trap this probe formalises: a value-only equivalence checker that skips
    // inputs where a side panics (which is what a catch_unwind harness naturally
    // does) certifies this pair from the random sweep above, and the constructed
    // input refutes the certificate. Stated as the two verdicts side by side:
    println!(
        "value-only checker over the random sweep : {} disagreements -> would certify",
        disagreements
    );
    println!(
        "definedness checker at the witness       : narrow undefined, wide defined -> refuses"
    );
}
