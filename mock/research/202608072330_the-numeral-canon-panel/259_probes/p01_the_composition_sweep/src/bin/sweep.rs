//! Step 01. The fused realisation against the natural stepwise composition, at
//! one declared signature, exhaustively over every triple.
//!
//! The arms are declared here, in the source, before the run. Each names what it
//! must return, and three of them must fail, which is what makes the zeros in the
//! others worth anything.
//!
//! A1  must be zero.     floor under wrap, both signednesses, every W and F.
//! A2  must be non-zero. signed saturating, every mode, every F, F = 0 included.
//! A3  must be zero.     unsigned saturating at floor, ceil, toward_zero, half_up.
//! A4  must be zero at F = 0 and non-zero at F >= 1. half_even under wrap.
//! A5  must be non-zero at F >= 1. half_up under signed wrap. This one contradicts
//!     a committed law row, and it is stated here before the run rather than read
//!     off the output afterwards: the shipped `HalfUp` breaks a tie away from
//!     zero, so it reads the sign of the position and cannot be equivariant on a
//!     domain that has negatives in it.
//! A6  must be non-zero. The cross-pairing control: fused at floor against
//!     stepwise at ceil. If that agreed, the instrument would not be reading the
//!     mode at all and every zero above would be worthless.
//! A7  must be non-zero on all four counters. The domain control: the sweep has
//!     to reach off-grid product positions, exact ties, products that leave the
//!     range upward, and products that leave it downward. A cell that reaches
//!     none of those has measured nothing.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::slots::Slot;
use arvo_format::standards::{Fi, Ufi};
use p01_the_composition_sweep::{Cell, the_grid};

fn main() {
    println!("# step 01: fused against the natural stepwise composition");
    println!("# every adaptation is arvo_format::apply::adapt");
    println!("# rounding modes: the five deterministic members of the ratified six");
    println!("# stochastic is excluded by name: it is not a function of the value,");
    println!("# so equality of two realisations is not a well posed question for it");
    println!();

    let mut rows: Vec<Cell> = Vec::new();
    the_grid!(rows);
    for row in &rows {
        row.print();
    }

    println!();
    println!("# --- the arms ---");
    arm_a1(&rows);
    arm_a2(&rows);
    arm_a3(&rows);
    arm_a4(&rows);
    arm_a5(&rows);
    arm_a6();
    arm_a7();

    println!();
    println!("# --- the summary table, agreement per cell ---");
    for signedness in ["unsigned", "signed"] {
        for policy in ["wrap", "saturate"] {
            for mode in ["floor", "ceil", "toward_zero", "half_up", "half_even"] {
                let selected: Vec<&Cell> = rows
                    .iter()
                    .filter(|r| r.signedness == signedness && r.policy == policy && r.mode == mode)
                    .collect();
                let agreeing: Vec<u32> = selected
                    .iter()
                    .filter(|r| r.agrees())
                    .map(|r| r.fraction)
                    .collect();
                let failing: Vec<u32> = selected
                    .iter()
                    .filter(|r| !r.agrees())
                    .map(|r| r.fraction)
                    .collect();
                println!(
                    "summary {signedness} {policy} {mode}: cells={} agreeing_at_F={:?} failing_at_F={:?}",
                    selected.len(),
                    dedup(agreeing),
                    dedup(failing),
                );
            }
        }
    }
}

fn dedup(mut v: Vec<u32>) -> Vec<u32> {
    v.sort_unstable();
    v.dedup();
    v
}

fn verdict(name: &str, expectation: &str, held: bool) {
    println!(
        "{name}: {expectation} -> {}",
        if held { "HELD" } else { "BROKEN" }
    );
}

fn arm_a1(rows: &[Cell]) {
    let offenders: Vec<&Cell> = rows
        .iter()
        .filter(|r| r.mode == "floor" && r.policy == "wrap" && !r.agrees())
        .collect();
    for cell in &offenders {
        cell.print();
    }
    verdict(
        "A1",
        "floor under wrap must agree at every triple of every cell",
        offenders.is_empty(),
    );
}

fn arm_a2(rows: &[Cell]) {
    let cells: Vec<&Cell> = rows
        .iter()
        .filter(|r| r.signedness == "signed" && r.policy == "saturate")
        .collect();
    let agreeing: Vec<&&Cell> = cells.iter().filter(|r| r.agrees()).collect();
    for cell in &agreeing {
        cell.print();
    }
    println!(
        "A2 detail: {} signed saturating cells, {} of them disagree somewhere",
        cells.len(),
        cells.len() - agreeing.len()
    );
    let at_zero: Vec<&&Cell> = cells.iter().filter(|r| r.fraction == 0).collect();
    println!(
        "A2 detail at F = 0: {} cells, all disagreeing: {}",
        at_zero.len(),
        at_zero.iter().all(|r| !r.agrees())
    );
    verdict(
        "A2",
        "every signed saturating cell must disagree, F = 0 included",
        agreeing.is_empty() && !cells.is_empty(),
    );
}

fn arm_a3(rows: &[Cell]) {
    let cells: Vec<&Cell> = rows
        .iter()
        .filter(|r| {
            r.signedness == "unsigned"
                && r.policy == "saturate"
                && matches!(r.mode, "floor" | "ceil" | "toward_zero" | "half_up")
        })
        .collect();
    let offenders: Vec<&&Cell> = cells.iter().filter(|r| !r.agrees()).collect();
    for cell in &offenders {
        cell.print();
    }
    verdict(
        "A3",
        "unsigned saturating at the four modes equivariant on non-negatives must agree",
        offenders.is_empty() && !cells.is_empty(),
    );
}

fn arm_a4(rows: &[Cell]) {
    let at_zero: Vec<&Cell> = rows
        .iter()
        .filter(|r| r.mode == "half_even" && r.policy == "wrap" && r.fraction == 0)
        .collect();
    let above: Vec<&Cell> = rows
        .iter()
        .filter(|r| r.mode == "half_even" && r.policy == "wrap" && r.fraction >= 1)
        .collect();
    println!(
        "A4 detail: half_even wrap, {} cells at F = 0 all agreeing: {}; {} cells at F >= 1, {} disagreeing",
        at_zero.len(),
        at_zero.iter().all(|r| r.agrees()),
        above.len(),
        above.iter().filter(|r| !r.agrees()).count(),
    );
    verdict(
        "A4",
        "half_even under wrap must agree at F = 0 and disagree at every F >= 1",
        at_zero.iter().all(|r| r.agrees())
            && above.iter().all(|r| !r.agrees())
            && !at_zero.is_empty()
            && !above.is_empty(),
    );
}

fn arm_a5(rows: &[Cell]) {
    let above: Vec<&Cell> = rows
        .iter()
        .filter(|r| {
            r.signedness == "signed" && r.mode == "half_up" && r.policy == "wrap" && r.fraction >= 1
        })
        .collect();
    for cell in &above {
        cell.print();
    }
    verdict(
        "A5",
        "half_up under signed wrap must disagree at every F >= 1",
        above.iter().all(|r| !r.agrees()) && !above.is_empty(),
    );
}

fn arm_a6() {
    // The cross-pairing control. Fused at floor, stepwise at ceil, one signature
    // apart. A zero here would say the instrument cannot see the mode.
    type Down = Signature<Fi<6, 3>, Adapt<Floor, Wrap>>;
    type Up = Signature<Fi<6, 3>, Adapt<Ceil, Wrap>>;
    let den = 1i64 << 3;
    let mut differing = 0u64;
    let mut witness = None;
    for a in -32i64 ..= 31 {
        for b in -32i64 ..= 31 {
            for c in -32i64 ..= 31 {
                let f = p01_the_composition_sweep::fused::<Down>(a, b, c, den);
                let s = p01_the_composition_sweep::stepwise::<Up, Up>(a, b, c, den);
                if f != s {
                    differing += 1;
                    if witness.is_none() {
                        witness = Some((a, b, c, f, s));
                    }
                }
            }
        }
    }
    println!(
        "A6 detail: cross-paired floor against ceil at W=6 F=3 signed wrap, differing={differing}, witness={witness:?}"
    );
    verdict(
        "A6",
        "a cross-paired mode must disagree somewhere",
        differing > 0,
    );
}

fn arm_a7() {
    // The domain control, at the widest cell the sweep runs. Counts what the
    // triples actually reach, so a cell claiming a rounding law is known to have
    // reached positions where rounding fires.
    type S = Signature<Fi<7, 3>, Adapt<Floor, Wrap>>;
    let den = 1i64 << 3;
    let (lo, hi) = p01_the_composition_sweep::bounds::<S>();
    let mut off_grid = 0u64;
    let mut ties = 0u64;
    let mut above = 0u64;
    let mut below = 0u64;
    for a in lo ..= hi {
        for b in lo ..= hi {
            let position = Exact::between(Slot::ZERO, Fraction::of(a * b, den));
            if !position.is_on_grid().get() {
                off_grid += 1;
            }
            if position.is_tie().get() {
                ties += 1;
            }
            // Where the exact product lands relative to the declared range,
            // before any adaptation touches it.
            let rounded = adapt::<S>(position, Dither::UNUSED).index();
            let _ = rounded;
            let whole = (a * b).div_euclid(den);
            if whole > hi {
                above += 1;
            }
            if whole < lo {
                below += 1;
            }
        }
    }
    println!(
        "A7 detail at W=7 F=3 signed: off_grid={off_grid} ties={ties} above_range={above} below_range={below}"
    );
    verdict(
        "A7",
        "the sweep must reach off-grid positions, ties, and both sides of the range",
        off_grid > 0 && ties > 0 && above > 0 && below > 0,
    );
}
