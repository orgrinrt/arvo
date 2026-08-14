//! Is the accuracy coordinate reachable in the harness as it stands?
//!
//! `98`'s F-98-9 and `99` report that no committed bench family carries a
//! column for accuracy or divergence from a reference, and that all thirteen
//! shared crates force their arms to agree. `101_probes/p1` measures why the
//! column is empty: the `score` column is written iff the routine declares a
//! score label (`bench-harness/src/harness.rs:228-231`), and **0 of 94 arvo
//! variant crates implement `score_output` or `score_label`**.
//!
//! That is a reading of two sources. This is the compiled version of the same
//! claim, and it checks three things the reading cannot:
//!
//!   1. A `Routine` carrying a fidelity metric compiles against the pinned
//!      `mockspace-bench-core`, with no feature gate and no fork.
//!   2. The descriptor `routine_bridge!` builds carries a working scorer and a
//!      non-`None` label, which is the exact consent signal `harness.rs:230`
//!      tests before it will emit the column.
//!   3. `outputs_may_differ` is the switch that PERMITS an accuracy comparison
//!      at all. Every arvo family leaves it false, so the harness cross-checks
//!      the arms byte for byte and two arms that round differently cannot both
//!      be run. That is the mechanism behind "the arms are forced to agree",
//!      and it is one method away from not being.
//!
//! The metric instantiated here is the one `101_probes/p6` argues about: the
//! absolute error of a fixed-point accumulation against exact arithmetic,
//! measured in units of the last place, lower being better. Two arms compute
//! it, one truncating and one rounding to nearest, and the probe asserts the
//! scorer separates them in the direction p6 predicts at chain length one.
//!
//! This is a compiled existence check. It is NOT a bench, it times nothing,
//! and no number here prices anything.
//!
//! Run:  cargo run --release

use mockspace_bench_core::{routine_bridge, Routine};

/// Fractional bits of the declared grid. The exact product of two values on
/// this grid needs 2 * F, so every arm must quantise something.
const F: u32 = 8;
const ULP: u64 = 1;
const SCALE: u64 = 1 << F;
const K: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Terms {
    /// Pairs of factors, each a raw fixed-point value with F fractional bits.
    pub a: [u32; K],
    pub b: [u32; K],
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Acc {
    /// The accumulated result, raw, with F fractional bits.
    pub raw: u64,
}

/// The exact accumulation, in units of 2^-2F, which is representable exactly
/// in u64 for these sizes. The reference the score is measured against.
fn exact(t: &Terms) -> u64 {
    let mut s = 0u64;
    for i in 0..K {
        s += (t.a[i] as u64) * (t.b[i] as u64);
    }
    s
}

pub struct FixedPointSum;

impl Routine for FixedPointSum {
    type Input = Terms;
    type Output = Acc;

    fn build_input(seed: u64) -> Terms {
        // A small deterministic generator, so the input is a function of the
        // seed and nothing else.
        let mut s = seed | 1;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            (s % (SCALE - 1) + 1) as u32
        };
        let mut a = [0u32; K];
        let mut b = [0u32; K];
        for i in 0..K {
            a[i] = next();
            b[i] = next();
        }
        Terms { a, b }
    }

    /// Two arms rounding differently produce different outputs, and both are
    /// correct. Without this the harness compares the arms byte for byte and
    /// the pair cannot be run at all.
    fn outputs_may_differ() -> bool {
        true
    }

    /// A structural check that survives either rounding: the answer is within
    /// K ulps of exact. It fails on a wrong sum, which is what makes it a
    /// check rather than a formality.
    fn validate_output(input: &Terms, out: &Acc) -> Result<(), &'static str> {
        let e = exact(input);
        let got = out.raw * SCALE; // raw is in 2^-F, exact is in 2^-2F
        let diff = got.abs_diff(e);
        if diff > (K as u64) * SCALE * ULP {
            return Err("accumulated error exceeds K ulps: this is not the same computation");
        }
        Ok(())
    }

    /// The fidelity coordinate. Absolute error against exact, in units of the
    /// last place of the declared grid, lower better. This is the number the
    /// harness would write into the `score` column.
    fn score_output(input: &Terms, out: &Acc) -> Option<f64> {
        let e = exact(input) as f64 / (SCALE * SCALE) as f64;
        let got = out.raw as f64 / SCALE as f64;
        Some((got - e).abs() * SCALE as f64)
    }

    fn score_label() -> Option<&'static str> {
        Some("ulp error")
    }
}

/// A second routine identical but for declaring no label, as the negative
/// control on point 2: the harness's consent signal is the label, so this one
/// must produce `None` and would emit no column.
pub struct Unscored;

impl Routine for Unscored {
    type Input = Terms;
    type Output = Acc;
    fn build_input(seed: u64) -> Terms {
        <FixedPointSum as Routine>::build_input(seed)
    }
}

/// Arm one: truncate each partial sum onto the declared grid.
fn arm_truncating(t: &Terms) -> Acc {
    let mut acc = 0u64; // raw at 2^-F
    for i in 0..K {
        let prod = (t.a[i] as u64) * (t.b[i] as u64); // at 2^-2F
        acc += prod / SCALE; // truncation toward zero
    }
    Acc { raw: acc }
}

/// Arm two: round each partial sum to nearest on the declared grid.
fn arm_rounding(t: &Terms) -> Acc {
    let mut acc = 0u64;
    for i in 0..K {
        let prod = (t.a[i] as u64) * (t.b[i] as u64);
        acc += (prod + SCALE / 2) / SCALE;
    }
    Acc { raw: acc }
}

fn main() {
    let bridge = routine_bridge!(FixedPointSum);
    let unscored = routine_bridge!(Unscored);

    println!("1. THE DESCRIPTOR THE HARNESS READS");
    println!(
        "   score_label, scored routine   : {:?}",
        bridge.score_label
    );
    println!(
        "   score_label, control routine  : {:?}",
        unscored.score_label
    );
    println!(
        "   outputs_may_differ, scored    : {}",
        bridge.outputs_may_differ
    );
    println!(
        "   outputs_may_differ, control   : {}",
        unscored.outputs_may_differ
    );
    assert!(
        bridge.score_label.is_some(),
        "the scored routine must carry a label"
    );
    assert!(unscored.score_label.is_none(), "the control must not");
    assert!(
        bridge.outputs_may_differ,
        "two roundings cannot be compared byte for byte"
    );

    println!();
    println!("2. THE SCORER, THROUGH THE BYTE BRIDGE THE HARNESS ACTUALLY CALLS");
    println!(
        "   {:>12} {:>16} {:>16}   closer to exact",
        "seed", "truncating", "rounding"
    );
    let mut wins_rounding = 0;
    let mut seeds = 0;
    for seed in 1u64..=8 {
        let t = <FixedPointSum as Routine>::build_input(seed);
        let ib = <FixedPointSum as Routine>::build_input_bytes(seed);
        let ta = arm_truncating(&t);
        let ra = arm_rounding(&t);
        let tb = unsafe {
            core::slice::from_raw_parts(
                (&ta as *const Acc) as *const u8,
                core::mem::size_of::<Acc>(),
            )
        };
        let rb = unsafe {
            core::slice::from_raw_parts(
                (&ra as *const Acc) as *const u8,
                core::mem::size_of::<Acc>(),
            )
        };
        let st = (bridge.scorer)(&ib, tb).expect("the scorer returns a value");
        let sr = (bridge.scorer)(&ib, rb).expect("the scorer returns a value");
        let who = if sr < st { "rounding" } else { "truncating" };
        wins_rounding += usize::from(sr < st);
        seeds += 1;
        println!("   {seed:12} {st:16.4} {sr:16.4}   {who}");

        (bridge.validator)(&ib, tb).expect("truncating arm validates");
        (bridge.validator)(&ib, rb).expect("rounding arm validates");
    }
    println!();
    println!(
        "   rounding closer on {wins_rounding} of {seeds} seeds, which is p6's k = 1 prediction:"
    );
    println!("   an unbiased rounding beats truncation at the same grid at every chain length.");
    assert_eq!(
        wins_rounding, seeds,
        "the fidelity coordinate must separate the arms"
    );

    println!();
    println!("3. WHAT THIS ESTABLISHES");
    println!("   A Routine carrying a fidelity metric compiles against the pinned");
    println!("   mockspace-bench-core with no feature gate and no fork; the descriptor");
    println!("   carries the label the harness tests at harness.rs:230 before writing the");
    println!("   score column, and a working scorer reachable through the byte bridge; and");
    println!("   the same routine sets outputs_may_differ, without which two arms that");
    println!("   round differently cannot be compared at all.");
    println!();
    println!("   So the missing accuracy coordinate is a routine nobody wrote, not a");
    println!("   harness that cannot carry one.");
}
