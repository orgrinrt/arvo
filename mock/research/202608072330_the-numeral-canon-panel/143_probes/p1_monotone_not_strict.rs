//! p1 (143): reproduce `141`'s refutation of my F2 on my own instrument.
//!
//! `140`'s F2 said the class count is "a strictly increasing function of the
//! witness set". `141` says that is false with 714 counterexamples, and that
//! what survives is monotone non-decreasing, which is a theorem rather than a
//! measurement.
//!
//! `RULES.md` says reproduce before conceding. So this reruns the question on
//! MY axis set (5 rounding x 3 overflow x 2 intermediate, unsigned), which is
//! not `141`'s (2 rounding x 2 overflow x 2 intermediate, both signednesses).
//! I therefore expect a DIFFERENT count of counterexamples, and matching 714
//! would be suspicious rather than reassuring. What must reproduce is the
//! qualitative pair: zero monotonicity violations, and a non-zero number of
//! operations that add nothing.
//!
//! Predictions, written before running:
//!   P1a. Zero monotonicity violations over every subset pair at every shape.
//!        A violation would mean my instrument is broken, not that the theorem
//!        is false, and I say so rather than presenting a theorem as a finding.
//!   P1b. A non-zero number of (shape, subset, operation) triples add exactly
//!        zero classes, refuting "strictly increasing" on my own axis set too.
//!   P1c. The zero-add triples concentrate at F = 0, because that is where my
//!        own p1 already shows add, sub and mul all collapsing to 2 classes.
//!
//! THE CASE THAT MUST FAIL. Monotonicity is a theorem, so a run reporting zero
//! violations proves nothing on its own: an instrument that always reports
//! "subset" would also report zero. So the sweep carries an ANTI-MONOTONE
//! control: a deliberately broken comparator that partitions on a truncated
//! prefix of the answer vector, which can lose distinctions as the vector grows.
//! It MUST produce violations. If it does not, the checker cannot detect a
//! violation and the zero above is worthless, and the probe exits non-zero.

use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Round {
    TowardZero,
    TiesEven,
    TiesAway,
    TowardNegInf,
    TowardPosInf,
}
const ROUNDS: [Round; 5] = [
    Round::TowardZero,
    Round::TiesEven,
    Round::TiesAway,
    Round::TowardNegInf,
    Round::TowardPosInf,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overflow {
    Wrap,
    SaturateBoth,
    SaturateHighOnly,
}
const OVERFLOWS: [Overflow; 3] = [
    Overflow::Wrap,
    Overflow::SaturateBoth,
    Overflow::SaturateHighOnly,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Intermediate {
    RoundEachStep,
    ExactThenRoundOnce,
}

#[derive(Clone, Copy, Debug)]
struct Assignment {
    round: Round,
    overflow: Overflow,
    intermediate: Intermediate,
}

fn shift_round(v: i128, s: u32, mode: Round) -> i128 {
    if s == 0 {
        return v;
    }
    let d: i128 = 1i128 << s;
    let q = v.div_euclid(d);
    let r = v.rem_euclid(d);
    match mode {
        Round::TowardNegInf => q,
        Round::TowardPosInf => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        }
        Round::TowardZero => {
            if v >= 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        Round::TiesAway => {
            let t = 2 * r;
            if t > d {
                q + 1
            } else if t < d {
                q
            } else if v >= 0 {
                q + 1
            } else {
                q
            }
        }
        Round::TiesEven => {
            let t = 2 * r;
            if t > d {
                q + 1
            } else if t < d {
                q
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

fn apply_overflow(r: i128, w: u32, mode: Overflow) -> i128 {
    let modulus: i128 = 1i128 << w;
    let max = modulus - 1;
    match mode {
        Overflow::Wrap => r.rem_euclid(modulus),
        Overflow::SaturateBoth => {
            if r < 0 {
                0
            } else if r > max {
                max
            } else {
                r
            }
        }
        Overflow::SaturateHighOnly => {
            if r > max {
                max
            } else {
                r.rem_euclid(modulus)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    ChainAdd,
    ChainSub,
}
const OPS: [Op; 5] = [Op::Add, Op::Sub, Op::Mul, Op::ChainAdd, Op::ChainSub];

impl Op {
    fn name(&self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::ChainAdd => "a*b+c",
            Op::ChainSub => "a*b-c",
        }
    }
}

fn eval(op: Op, a: i128, b: i128, c: i128, w: u32, f: u32, asg: Assignment) -> i128 {
    let r = match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => shift_round(a * b, f, asg.round),
        Op::ChainAdd | Op::ChainSub => {
            let sign: i128 = if matches!(op, Op::ChainAdd) { 1 } else { -1 };
            match asg.intermediate {
                Intermediate::RoundEachStep => {
                    let p = shift_round(a * b, f, asg.round);
                    let p = apply_overflow(p, w, asg.overflow);
                    p + sign * c
                }
                Intermediate::ExactThenRoundOnce => {
                    shift_round(a * b + sign * (c << f), f, asg.round)
                }
            }
        }
    };
    apply_overflow(r, w, asg.overflow)
}

fn answer_vector(op: Op, w: u32, f: u32, asg: Assignment) -> Vec<i128> {
    let n: i128 = 1i128 << w;
    let mut out = Vec::new();
    if matches!(op, Op::ChainAdd | Op::ChainSub) {
        for a in 0..n {
            for b in 0..n {
                for c in 0..n {
                    out.push(eval(op, a, b, c, w, f, asg));
                }
            }
        }
    } else {
        for a in 0..n {
            for b in 0..n {
                out.push(eval(op, a, b, 0, w, f, asg));
            }
        }
    }
    out
}

fn assignments() -> Vec<Assignment> {
    let mut v = Vec::new();
    for &round in ROUNDS.iter() {
        for &overflow in OVERFLOWS.iter() {
            for &intermediate in [
                Intermediate::RoundEachStep,
                Intermediate::ExactThenRoundOnce,
            ]
            .iter()
            {
                v.push(Assignment {
                    round,
                    overflow,
                    intermediate,
                });
            }
        }
    }
    v
}

/// Count classes of assignments under observational equality over `ops`.
///
/// `truncate_to` is the anti-monotone control: when `Some(k)`, the comparator
/// only looks at the first k entries of the concatenated answer vector, which
/// can lose a distinction as more operations are appended. A correct comparator
/// passes `None`.
fn classes(asgs: &[Assignment], ops: &[Op], w: u32, f: u32, truncate_to: Option<usize>) -> usize {
    let mut vecs: Vec<Vec<i128>> = Vec::with_capacity(asgs.len());
    for &a in asgs {
        let mut v = Vec::new();
        for &op in ops {
            v.extend(answer_vector(op, w, f, a));
        }
        if let Some(k) = truncate_to {
            v.truncate(k);
        }
        vecs.push(v);
    }
    let mut reps: Vec<usize> = Vec::new();
    for i in 0..vecs.len() {
        if !reps.iter().any(|&r| vecs[r] == vecs[i]) {
            reps.push(i);
        }
    }
    reps.len()
}

fn subsets(ops: &[Op]) -> Vec<Vec<Op>> {
    let mut out = Vec::new();
    for mask in 1u32..(1 << ops.len()) {
        let mut s = Vec::new();
        for (i, &op) in ops.iter().enumerate() {
            if mask & (1 << i) != 0 {
                s.push(op);
            }
        }
        out.push(s);
    }
    out
}

fn is_subset(a: &[Op], b: &[Op]) -> bool {
    let sb: BTreeSet<Op> = b.iter().copied().collect();
    a.iter().all(|x| sb.contains(x))
}

fn main() {
    let asgs = assignments();
    let subs = subsets(&OPS);
    let shapes: Vec<(u32, u32)> = vec![(4, 0), (4, 1), (4, 2)];

    println!("p1 (143): monotonicity and strictness on 140's own axis set");
    println!(
        "{} assignments (rounding {} x overflow {} x intermediate 2), unsigned",
        asgs.len(),
        ROUNDS.len(),
        OVERFLOWS.len()
    );
    println!(
        "{} non-empty operation subsets, shapes {:?}\n",
        subs.len(),
        shapes
    );

    // precompute counts per (shape, subset)
    let mut pairs_checked = 0usize;
    let mut violations = 0usize;
    let mut zero_add = 0usize;
    let mut zero_add_by_f: Vec<(u32, usize)> = Vec::new();
    let mut first_witness: Option<String> = None;

    for &(w, f) in shapes.iter() {
        let counts: Vec<usize> = subs.iter().map(|s| classes(&asgs, s, w, f, None)).collect();

        // monotonicity over every ordered subset pair
        for i in 0..subs.len() {
            for j in 0..subs.len() {
                if i != j && is_subset(&subs[i], &subs[j]) {
                    pairs_checked += 1;
                    if counts[j] < counts[i] {
                        violations += 1;
                    }
                }
            }
        }

        // strictness: adding one operation to a subset that lacks it
        let mut zf = 0usize;
        for (i, s) in subs.iter().enumerate() {
            for &op in OPS.iter() {
                if s.contains(&op) {
                    continue;
                }
                let mut bigger = s.clone();
                bigger.push(op);
                bigger.sort();
                let jdx = subs
                    .iter()
                    .position(|t| {
                        let mut t2 = t.clone();
                        t2.sort();
                        t2 == bigger
                    })
                    .unwrap();
                if counts[jdx] == counts[i] {
                    zero_add += 1;
                    zf += 1;
                    if first_witness.is_none() {
                        let names: Vec<&str> = s.iter().map(|o| o.name()).collect();
                        first_witness = Some(format!(
                            "W={w} F={f}: {{{}}} plus {} stays at {} classes",
                            names.join(","),
                            op.name(),
                            counts[i]
                        ));
                    }
                }
            }
        }
        zero_add_by_f.push((f, zf));
        println!(
            "W={w} F={f}: full-set classes {}, zero-add triples {}",
            counts[subs.len() - 1],
            zf
        );
    }

    println!("\nP1a monotonicity: {pairs_checked} ordered subset pairs, {violations} violations");
    println!(
        "  -> {}",
        if violations == 0 {
            "zero violations, as the theorem requires"
        } else {
            "VIOLATIONS, which means this instrument is broken, not the theorem"
        }
    );

    println!(
        "\nP1b strictness: {zero_add} (shape, subset, operation) triples add exactly zero classes"
    );
    println!(
        "  -> 140's F2 wording \"strictly increasing\" is {}",
        if zero_add > 0 {
            "REFUTED on my own axis set too"
        } else {
            "not refuted here"
        }
    );
    if let Some(w) = &first_witness {
        println!("  first witness: {w}");
    }

    println!("\nP1c distribution of zero-add triples by fraction width:");
    for (f, n) in zero_add_by_f.iter() {
        println!("  F={f}: {n}");
    }
    let f0 = zero_add_by_f
        .iter()
        .find(|(f, _)| *f == 0)
        .map(|(_, n)| *n)
        .unwrap_or(0);
    println!(
        "  -> P1c (they concentrate at F=0): {}",
        if f0 > zero_add - f0 {
            "CONFIRMED"
        } else {
            "REFUTED"
        }
    );

    // ---- the anti-monotone control ----
    //
    // A comparator that reads only a prefix of the answer vector can lose a
    // distinction when the vector grows, so it MUST produce violations. If it
    // does not, this sweep cannot detect a violation at all.
    println!("\n=== ANTI-MONOTONE CONTROL ===");
    let mut ctrl_violations = 0usize;
    let mut ctrl_pairs = 0usize;
    let (w, f) = (4u32, 2u32);
    let counts_bad: Vec<usize> = subs
        .iter()
        .map(|s| classes(&asgs, s, w, f, Some(64)))
        .collect();
    for i in 0..subs.len() {
        for j in 0..subs.len() {
            if i != j && is_subset(&subs[i], &subs[j]) {
                ctrl_pairs += 1;
                if counts_bad[j] < counts_bad[i] {
                    ctrl_violations += 1;
                }
            }
        }
    }
    println!(
        "prefix-truncated comparator at W=4 F=2: {ctrl_pairs} pairs, {ctrl_violations} violations"
    );
    if ctrl_violations == 0 {
        println!("  !! CONTROL FAIL: a comparator that provably loses information produced no");
        println!("  violations, so this sweep cannot detect one. the zero above is worthless.");
        std::process::exit(1);
    }
    println!("  control fires, so the real comparator's zero is a real zero.");
}
