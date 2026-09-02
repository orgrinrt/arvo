//! Probe 4: the whole codegen advantage the Door-defined cell would buy over a
//! consumer-defined cell, priced in emitted instructions on the friendliest target.
//!
//! WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates two implementations of a TOTAL
//! division whose x/0 answer is the value 0: (a) the value stated in the program
//! (`checked_div(..).unwrap_or(0)`, consumer-supplied fallback, target-independent,
//! provable-tier), and (b) the value left to the ISA (raw `sdiv` through asm, the
//! Door-defined cell on the one target where the ISA defines one). The distinction is
//! nonvacuous because both deliver identical values on every input on this target, so
//! the ONLY difference left is cost, which is exactly the claim under test: how much
//! "free" buys. A probe comparing (b) against a branching or panicking form would
//! overstate the Door's win by pricing the wrong alternative.
//!
//! CLAIM A: both loops deliver identical results over every element, including zero
//!   divisors. Asserted at runtime over a mixed 64-element input.
//! CLAIM B: the steady-state loop bodies differ by the zero-test only. Read from
//!   objdump: the consumer-defined body carries `sdiv` + `cmp` + `csel`, the asm body
//!   carries `sdiv` alone. No branch in either; NEON has no vector integer divide, so no
//!   vectorisation is at stake in either form.
//!
//! Build: rustc --edition 2021 -O -C llvm-args=-x86-asm-syntax=intel probe_4_the_cell_priced.rs --out-dir out
//! Inspect: objdump -d out/probe_4_the_cell_priced | sed -n '/consumer_defined/,/ret/p'
//! Outcome: WORKS. Counts in OUTCOMES.md.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), aarch64-apple-darwin.

use core::arch::asm;

const N: usize = 64;

/// (a) the consumer-defined cell: total division, x/0 -> 0, stated in the program.
#[inline(never)]
fn consumer_defined(xs: &[i64; N], ds: &[i64; N], out: &mut [i64; N]) {
    for i in 0..N {
        // the fallback is the consumer's own constant; 0 here to make (b) comparable.
        // wrapping_div, not checked_div: Hot's MIN/-1 cell is ReduceModulo -> MIN, which
        // wrapping_div states and sdiv happens to deliver; checked_div would fold that
        // cell into the fallback and the two forms would no longer compute one function.
        out[i] = if ds[i] == 0 {
            0
        } else {
            xs[i].wrapping_div(ds[i])
        };
    }
}

/// (b) the Door-defined cell: total division, x/0 -> whatever sdiv does (0 on aarch64),
/// reachable only through an asm-opaque instruction because llvm sdiv is UB at d == 0.
#[inline(never)]
fn door_defined(xs: &[i64; N], ds: &[i64; N], out: &mut [i64; N]) {
    for i in 0..N {
        let q: i64;
        unsafe { asm!("sdiv {0}, {1}, {2}", out(reg) q, in(reg) xs[i], in(reg) ds[i]) };
        out[i] = q;
    }
}

fn main() {
    let mut xs = [0i64; N];
    let mut ds = [0i64; N];
    for i in 0..N {
        xs[i] = (i as i64 - 31) * 7919;
        ds[i] = if i % 5 == 0 { 0 } else { i as i64 - 32 }; // zeros scattered through, plus negatives
    }
    let mut a = [0i64; N];
    let mut b = [0i64; N];
    consumer_defined(&xs, &ds, &mut a);
    door_defined(&xs, &ds, &mut b);
    // CLAIM A: identical values everywhere, so cost is the only remaining difference.
    assert_eq!(
        a, b,
        "consumer-defined 0 and aarch64's sdiv 0 agree on every element"
    );
    println!(
        "CLAIM A holds: {} elements identical, {} of them zero-divisor cells",
        N,
        ds.iter().filter(|d| **d == 0).count()
    );
}
