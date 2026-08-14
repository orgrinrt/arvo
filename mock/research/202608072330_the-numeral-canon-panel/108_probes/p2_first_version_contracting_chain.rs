//! p2. Is "observable" a property of an axis, or of an axis at a region?
//!
//! The pair's first component is "an assignment on the axes a consumer can
//! observe". That phrasing presumes the set of such axes is fixed. Two things
//! in the panel say it is not, and neither was carried into the pair:
//!
//!   - `40` section 5.2: "Headroom is on the unobservable side **only if** the
//!     overflow policy is applied at the logical width rather than at the
//!     container width." It calls that convention "a canon-shaped sentence
//!     nobody has written" and never measures it.
//!   - `102` p2: headroom and intermediate precision are unobservable across a
//!     pure `+ - *` chain and observable past a shift, 0/640 against 500/640.
//!
//! This probe does two things neither did.
//!
//! ONE. It measures `40`'s convention directly, which nobody has. Two arms
//! differ only in the container the fold runs in (the headroom axis). The
//! overflow policy is applied once at the declared width, then once at the
//! container width, everything else held. If the class of the headroom axis
//! flips between those two runs, `40`'s sentence is a measurement rather than
//! an argument, and the pair's first component has a membership that a
//! convention decides.
//!
//! TWO. It tests a criterion that would make the class COMPUTABLE rather than
//! contingent, so the finding lands as a repair rather than as a hole:
//!
//!     Reducing a container-width value to the declared width is a map. An
//!     arm's container is invisible exactly where that map is a congruence for
//!     every operation in the chain.
//!
//! That is `97` section 6.1's criterion for when an algebraic law survives,
//! pointed at the widening map instead of at a law. If it predicts every cell,
//! the observable set is a function of the chain rather than a fixed partition,
//! and it is const-computable from the chain, which is the shape I13 asks for.
//!
//! EXHAUSTIVE, not sampled: every triple over the declared domain at W = 5 is
//! 32768 chains, and every 4-chain at W = 4 is 65536. Both are run in full.
//! `the-test-gate.md` rejects a law asserted over a chosen subset and there is
//! no reason to choose one here.
//!
//! This is a spike. Names, widths and loop shapes are scaffolding to reach the
//! check, not design decisions. It uses `std` to print a table. It contains no
//! `dyn`, no `TypeId`, no feature gate, and no allocation-shaped growth in the
//! measured path.
//!
//! Build: rustc -O --edition 2021 p2_observability_is_a_congruence_question.rs

/// The operations a chain can be built from. Each is applied inside a container
/// of `c` bits, on values that are already inside the declared width `w`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    /// Ring operations. Reduction mod 2^w factors through mod 2^c for c >= w.
    WAdd,
    WSub,
    WMul,
    /// A shift. The cheapest non-ring step there is.
    Shr1,
    /// Integer division by a constant.
    Div3,
    /// Saturating add whose limit is the DECLARED width. `40`'s convention.
    SatAtW,
    /// Saturating add whose limit is the CONTAINER width. `40`'s counterfactual.
    SatAtC,
    /// A lattice operation.
    Min,
    /// A comparison, folded back into the value.
    Cmp,
}

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::WAdd => "wadd",
            Op::WSub => "wsub",
            Op::WMul => "wmul",
            Op::Shr1 => "shr1",
            Op::Div3 => "div3",
            Op::SatAtW => "sat_at_W",
            Op::SatAtC => "sat_at_C",
            Op::Min => "min",
            Op::Cmp => "cmp",
        }
    }

    /// Is this operation a ring operation of Z/2^n, for every n?
    ///
    /// Stated as the prediction the criterion makes, BEFORE any chain is run.
    /// A reduction map `x mod 2^w` is a homomorphism for exactly the operations
    /// that are polynomial in the ring: +, -, *. Everything else reads bits
    /// that reduction throws away, so it cannot commute with reduction.
    ///
    /// `sat_at_W` is the interesting row and it is NOT a ring operation. It is
    /// predicted congruent for a different reason, stated in `predicted_invisible`.
    fn is_ring(self) -> bool {
        matches!(self, Op::WAdd | Op::WSub | Op::WMul)
    }

    /// The criterion's prediction: is the container invisible under this
    /// operation?
    ///
    /// Two independent routes to invisibility, and naming both is the point:
    ///   - the operation commutes with reduction (the ring rows), so the wide
    ///     arm and the narrow arm agree after reduction;
    ///   - the operation's own result never leaves the declared width, so the
    ///     two arms hold bit-identical accumulators at every step and the
    ///     question never arises. `sat_at_W` is this row, and it is exactly the
    ///     convention `40` says the classification rests on.
    fn predicted_invisible(self) -> bool {
        self.is_ring() || matches!(self, Op::SatAtW | Op::Min)
    }

    /// Apply, in a container of `c` bits, with `w` the declared width.
    fn apply(self, acc: u64, x: u64, w: u32, c: u32) -> u64 {
        let cmask: u64 = if c >= 64 { u64::MAX } else { (1u64 << c) - 1 };
        let wmax: u64 = (1u64 << w) - 1;
        match self {
            Op::WAdd => acc.wrapping_add(x) & cmask,
            Op::WSub => acc.wrapping_sub(x) & cmask,
            Op::WMul => acc.wrapping_mul(x) & cmask,
            Op::Shr1 => (acc.wrapping_add(x) >> 1) & cmask,
            Op::Div3 => ((acc.wrapping_add(x) & cmask) / 3) & cmask,
            // limit is the DECLARED width, whatever the container is
            Op::SatAtW => {
                let s = acc.wrapping_add(x);
                if s > wmax {
                    wmax
                } else {
                    s
                }
            }
            // limit is the CONTAINER width, so it moves with the arm
            Op::SatAtC => {
                let s = acc.wrapping_add(x);
                if s > cmask {
                    cmask
                } else {
                    s & cmask
                }
            }
            Op::Min => {
                let m = if acc < x { acc } else { x };
                m & cmask
            }
            Op::Cmp => {
                let v = if acc > x {
                    acc.wrapping_sub(x)
                } else {
                    x.wrapping_sub(acc)
                };
                v & cmask
            }
        }
    }
}

/// Run a chain in a container of `c` bits and reduce the result to `w` bits.
/// The reduction at the end is what a consumer of the value sees.
fn run(op: Op, vals: &[u64], w: u32, c: u32) -> u64 {
    let mut acc = vals[0];
    for &x in &vals[1..] {
        acc = op.apply(acc, x, w, c);
    }
    acc & ((1u64 << w) - 1)
}

/// Exhaustive over every chain of length `len` whose elements are inside the
/// declared width. Returns (differing, total) for the headroom axis: the
/// minimum container against a wider one.
fn sweep(op: Op, w: u32, len: usize, c_min: u32, c_wide: u32) -> (u64, u64) {
    let domain: u64 = 1u64 << w;
    let mut idx = vec![0u64; len];
    let mut differing = 0u64;
    let mut total = 0u64;
    loop {
        let a = run(op, &idx, w, c_min);
        let b = run(op, &idx, w, c_wide);
        if a != b {
            differing += 1;
        }
        total += 1;

        // odometer over the whole declared domain, every position
        let mut i = 0;
        loop {
            if i == len {
                return (differing, total);
            }
            idx[i] += 1;
            if idx[i] < domain {
                break;
            }
            idx[i] = 0;
            i += 1;
        }
    }
}

const OPS: [Op; 9] = [
    Op::WAdd,
    Op::WSub,
    Op::WMul,
    Op::Shr1,
    Op::Div3,
    Op::SatAtW,
    Op::SatAtC,
    Op::Min,
    Op::Cmp,
];

fn main() {
    println!("p2. Is 'observable' a property of an axis, or of an axis at a region?");
    println!();
    println!("The axis under test is HEADROOM: the same declared width, two containers.");
    println!("A difference in the reduced result means a consumer can tell the arms");
    println!("apart, which is exactly the pair's test for component-one membership.");
    println!();

    // Two independent settings so no conclusion rests on one geometry.
    let settings: [(u32, usize, u32, u32); 2] = [
        // (declared width, chain length, minimum container, wide container)
        (5, 3, 5, 16),
        (4, 4, 4, 16),
    ];

    let mut mismatches = 0u32;
    let mut cells = 0u32;

    for &(w, len, c_min, c_wide) in settings.iter() {
        let total_chains = (1u64 << w).pow(len as u32);
        println!(
            "--- declared width W = {w}, chain length {len}, containers {c_min} against {c_wide}"
        );
        println!("    exhaustive over {total_chains} chains, every element in [0, 2^{w})");
        println!();
        println!(
            "{:<10} {:>12} {:>10}   {:<12} {:<12} {}",
            "op", "differing", "pct", "predicted", "measured", "agree"
        );
        for &op in OPS.iter() {
            let (d, t) = sweep(op, w, len, c_min, c_wide);
            let pct = 100.0 * (d as f64) / (t as f64);
            let predicted_inv = op.predicted_invisible();
            let measured_inv = d == 0;
            let agree = predicted_inv == measured_inv;
            cells += 1;
            if !agree {
                mismatches += 1;
            }
            println!(
                "{:<10} {:>12} {:>9.3}%   {:<12} {:<12} {}",
                op.name(),
                d,
                pct,
                if predicted_inv {
                    "invisible"
                } else {
                    "visible"
                },
                if measured_inv { "invisible" } else { "visible" },
                if agree { "yes" } else { "NO  <<<<" }
            );
        }
        println!();
    }

    println!("criterion cells: {cells}, mismatches: {mismatches}");
    println!();
    println!("=== what the two saturating rows are for ===");
    println!();
    println!("sat_at_W and sat_at_C are the SAME axis assignment on overflow policy");
    println!("(saturate), differing only in the width the limit is read at. `40`");
    println!("section 5.2 says the headroom axis sits on the unobservable side ONLY");
    println!("IF the limit is the declared width, and calls that 'a canon-shaped");
    println!("sentence nobody has written'. If those two rows land on opposite sides");
    println!("of the cut, the sentence is load-bearing as measured fact and the");
    println!("membership of the pair's first component depends on it.");
}
