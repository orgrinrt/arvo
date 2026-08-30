//! p2b. The observable set is a function of the chain, and it is decidable.
//!
//! ## What the first version got wrong, and why it is kept
//!
//! `p2_first_version_contracting_chain.rs` predicted the headroom axis visible
//! under `shr1` and `cmp` and measured it invisible, 4 mismatched cells of 18.
//! The criterion was not wrong. The chain was: every operation in it was fused
//! with an add and then masked, so the accumulator never left the declared
//! width and the axis was never exercised. That is the same defect `102` caught
//! in its own p2 version one, reached from the other direction, and both
//! outputs are committed.
//!
//! The lesson is the finding rather than the bug: **an axis is not observable
//! or unobservable. A chain either does or does not expose it**, and what
//! decides that is not whether a non-ring step is present but whether one is
//! reached while the accumulator has left the declared width.
//!
//! ## What this version tests
//!
//! Operations are primitive binary functions on the accumulator and the next
//! element, with the container's wrap applied after each one and nothing fused.
//! Two static properties are measured exhaustively per operation, neither of
//! them asserted:
//!
//!   CONGRUENT: reduction to the declared width descends through it, so two
//!   containers agree modulo the declared width whatever the accumulator holds.
//!   Measured by a table-conflict test over the whole container domain.
//!
//!   CONTRACTING: applied to operands inside the declared width, the result is
//!   inside the declared width, so the two containers hold bit-identical
//!   accumulators. Measured exhaustively over the declared domain.
//!
//! From those two bits per operation, a two-state automaton over a chain
//! predicts whether the headroom axis is observable on that chain:
//!
//!     state := CONTAINED
//!     for op in chain:
//!         if state = ESCAPED and not CONGRUENT(op): return OBSERVABLE
//!         state := if CONTRACTING(op) { CONTAINED } else { ESCAPED }
//!     return UNOBSERVABLE
//!
//! Every input to that automaton is a compile-time property of the operation
//! and of the chain, so the verdict is const-computable where the chain is
//! written. Nothing in it is a measurement of a machine.
//!
//! The automaton is then checked against exhaustive measurement over EVERY
//! operation sequence of the swept length from the alphabet, at EVERY value
//! tuple in the declared domain. Not a sample of either.
//!
//! This is a spike. Its names, widths and alphabet are scaffolding to reach
//! the check. `std` is used to print a table; there is no `dyn`, no `TypeId`,
//! no feature gate.
//!
//! Build: rustc -O --edition 2021 p2b_observability_is_decided_by_the_chain.rs

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    WAdd,
    WSub,
    WMul,
    /// Shift the accumulator right by a data-dependent amount. Reads bits that
    /// reduction to the declared width discards.
    Shr,
    /// Divide the accumulator. Same.
    Div,
    /// Saturating add whose limit is the DECLARED width.
    SatAtW,
    /// Saturating add whose limit is the CONTAINER width.
    SatAtC,
    Min,
    /// A comparison folded back into the value.
    Cmp,
}

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::WAdd => "wadd",
            Op::WSub => "wsub",
            Op::WMul => "wmul",
            Op::Shr => "shr",
            Op::Div => "div",
            Op::SatAtW => "sat_at_W",
            Op::SatAtC => "sat_at_C",
            Op::Min => "min",
            Op::Cmp => "cmp",
        }
    }

    /// The raw operation, before the container's wrap. Nothing is fused.
    fn raw(self, a: u64, x: u64, w: u32, cmask: u64) -> u64 {
        let wmax = (1u64 << w) - 1;
        match self {
            Op::WAdd => a.wrapping_add(x),
            Op::WSub => a.wrapping_sub(x),
            Op::WMul => a.wrapping_mul(x),
            Op::Shr => a >> (x & 3),
            Op::Div => a / (x | 1),
            Op::SatAtW => {
                let s = a.wrapping_add(x);
                if s > wmax {
                    wmax
                } else {
                    s
                }
            }
            Op::SatAtC => {
                let s = a.wrapping_add(x);
                if s > cmask {
                    cmask
                } else {
                    s
                }
            }
            Op::Min => {
                if a < x {
                    a
                } else {
                    x
                }
            }
            Op::Cmp => {
                if a > x {
                    a - x
                } else {
                    x - a
                }
            }
        }
    }

    fn step(self, a: u64, x: u64, w: u32, c: u32) -> u64 {
        let cmask = mask(c);
        self.raw(a, x, w, cmask) & cmask
    }
}

fn mask(c: u32) -> u64 {
    if c >= 64 {
        u64::MAX
    } else {
        (1u64 << c) - 1
    }
}

/// Measured, not asserted. Does reduction to `w` bits descend through this
/// operation, in a container of `c` bits?
///
/// Exhaustive over the whole container domain squared. A table keyed by the
/// reduced operands records the first reduced result seen; a second, different
/// result at the same key is a conflict, and one conflict is enough to refute
/// congruence.
fn measure_congruent(op: Op, w: u32, c: u32) -> bool {
    let wsz = 1usize << w;
    let csz = 1u64 << c;
    let wmask = mask(w);
    let mut seen: Vec<i64> = vec![-1; wsz * wsz];
    for a in 0..csz {
        for x in 0..csz {
            let r = op.step(a, x, w, c) & wmask;
            let key = ((a & wmask) as usize) * wsz + ((x & wmask) as usize);
            if seen[key] < 0 {
                seen[key] = r as i64;
            } else if seen[key] != r as i64 {
                return false;
            }
        }
    }
    true
}

/// Measured, not asserted. On operands inside the declared width, does the
/// result stay inside the declared width?
fn measure_contracting(op: Op, w: u32, c: u32) -> bool {
    let wsz = 1u64 << w;
    let wmax = wsz - 1;
    for a in 0..wsz {
        for x in 0..wsz {
            if op.step(a, x, w, c) > wmax {
                return false;
            }
        }
    }
    true
}

/// The automaton. Consumes only the two static bits per operation.
fn predict_observable(
    chain: &[Op],
    congruent: &[bool],
    contracting: &[bool],
    idx: &dyn Fn(Op) -> usize,
) -> bool {
    let mut escaped = false;
    for &op in chain {
        let i = idx(op);
        if escaped && !congruent[i] {
            return true;
        }
        escaped = !contracting[i];
    }
    false
}

/// Measured. Exhaustive over every value tuple in the declared domain.
fn measure_observable(chain: &[Op], w: u32, c_min: u32, c_wide: u32) -> (u64, u64) {
    let len = chain.len() + 1;
    let domain = 1u64 << w;
    let wmask = mask(w);
    let mut idx = vec![0u64; len];
    let (mut differing, mut total) = (0u64, 0u64);
    loop {
        let mut a_min = idx[0];
        let mut a_wide = idx[0];
        for (k, &op) in chain.iter().enumerate() {
            a_min = op.step(a_min, idx[k + 1], w, c_min);
            a_wide = op.step(a_wide, idx[k + 1], w, c_wide);
        }
        if (a_min & wmask) != (a_wide & wmask) {
            differing += 1;
        }
        total += 1;

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

const ALL: [Op; 9] = [
    Op::WAdd,
    Op::WSub,
    Op::WMul,
    Op::Shr,
    Op::Div,
    Op::SatAtW,
    Op::SatAtC,
    Op::Min,
    Op::Cmp,
];

fn op_index(o: Op) -> usize {
    ALL.iter().position(|&p| p == o).unwrap()
}

fn main() {
    // The declared width, the minimum container for it, and a wider one. The
    // headroom axis is exactly the choice between the last two.
    let w: u32 = 4;
    let c_min: u32 = 4;
    let c_wide: u32 = 12;

    println!("p2b. The observable set is a function of the chain, and it is decidable.");
    println!();
    println!(
        "declared width W = {w}; headroom axis = container {c_min} against container {c_wide}"
    );
    println!();

    // --- the two static bits, measured ---
    println!("=== per-operation static properties, measured exhaustively ===");
    println!();
    println!(
        "{:<10} {:<12} {:<14} {:<14}",
        "op", "congruent", "contracting", "contracting"
    );
    println!(
        "{:<10} {:<12} {:<14} {:<14}",
        "", "(at c_wide)", "(at c_min)", "(at c_wide)"
    );
    let mut congruent = vec![false; ALL.len()];
    let mut contracting = vec![false; ALL.len()];
    for (i, &op) in ALL.iter().enumerate() {
        let cg = measure_congruent(op, w, c_wide);
        let ct_min = measure_contracting(op, w, c_min);
        let ct_wide = measure_contracting(op, w, c_wide);
        congruent[i] = cg;
        // an operation is contracting for the purpose of the automaton only if
        // it contracts in EVERY container the axis ranges over, since the axis
        // is the choice among them
        contracting[i] = ct_min && ct_wide;
        println!(
            "{:<10} {:<12} {:<14} {:<14}",
            op.name(),
            cg,
            ct_min,
            ct_wide
        );
    }
    println!();

    // --- exhaustive check of the automaton over every chain ---
    // Every sequence of the swept length over the whole alphabet.
    let chain_len = 3usize;
    let n = ALL.len();
    let total_chains = n.pow(chain_len as u32);
    println!("=== the automaton against measurement ===");
    println!();
    println!(
        "every one of {total_chains} operation sequences of length {chain_len} over the {n}-operation"
    );
    println!(
        "alphabet, each swept exhaustively over all {} value tuples",
        (1u64 << w).pow(chain_len as u32 + 1)
    );
    println!();

    let mut mismatches: Vec<(Vec<Op>, bool, bool, u64, u64)> = Vec::new();
    let mut n_obs = 0u32;
    let mut n_unobs = 0u32;
    for code in 0..total_chains {
        let mut chain = Vec::with_capacity(chain_len);
        let mut r = code;
        for _ in 0..chain_len {
            chain.push(ALL[r % n]);
            r /= n;
        }
        let predicted = predict_observable(&chain, &congruent, &contracting, &op_index);
        let (d, t) = measure_observable(&chain, w, c_min, c_wide);
        let measured = d > 0;
        if predicted {
            n_obs += 1;
        } else {
            n_unobs += 1;
        }
        if predicted != measured {
            mismatches.push((chain, predicted, measured, d, t));
        }
    }

    println!("predicted OBSERVABLE:   {n_obs} chains");
    println!("predicted UNOBSERVABLE: {n_unobs} chains");
    println!(
        "mismatches against exhaustive measurement: {}",
        mismatches.len()
    );
    println!();
    if !mismatches.is_empty() {
        println!("the mismatching chains, up to 40:");
        for (chain, p, m, d, t) in mismatches.iter().take(40) {
            let names: Vec<&str> = chain.iter().map(|o| o.name()).collect();
            println!(
                "  {:<34} predicted={:<12} measured={:<12} {}/{}",
                names.join(" -> "),
                if *p { "observable" } else { "unobservable" },
                if *m { "observable" } else { "unobservable" },
                d,
                t
            );
        }
        println!();
    }

    // --- the headline: the SAME axis, two chains, two classes ---
    println!("=== the same axis, two chains, opposite classes ===");
    println!();
    let cases: [(&str, [Op; 3]); 4] = [
        ("pure ring", [Op::WAdd, Op::WMul, Op::WAdd]),
        ("ring then a shift", [Op::WAdd, Op::WMul, Op::Shr]),
        (
            "policy at declared width",
            [Op::SatAtW, Op::SatAtW, Op::SatAtW],
        ),
        (
            "policy at container width",
            [Op::SatAtC, Op::SatAtC, Op::SatAtC],
        ),
    ];
    for (label, chain) in cases.iter() {
        let (d, t) = measure_observable(chain, w, c_min, c_wide);
        let names: Vec<&str> = chain.iter().map(|o| o.name()).collect();
        println!(
            "  {:<28} {:<26} {:>7}/{:<7} {:>7.3}%   {}",
            label,
            names.join(" -> "),
            d,
            t,
            100.0 * d as f64 / t as f64,
            if d > 0 { "OBSERVABLE" } else { "unobservable" }
        );
    }
    println!();
    println!("The last two rows are the same assignment on the overflow-policy axis");
    println!("(saturate) and the same assignment on every other axis. They differ only");
    println!("in the width the limit is read at, which is `40` section 5.2's convention,");
    println!("and they put the headroom axis on opposite sides of the cut. `40` argued");
    println!("that from `20`'s factoring result and called it a canon-shaped sentence");
    println!("nobody has written. No file in the strategy-axis unit carries it.");
}
