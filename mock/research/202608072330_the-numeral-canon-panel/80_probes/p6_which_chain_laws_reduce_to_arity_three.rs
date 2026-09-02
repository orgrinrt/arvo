//! p6: attacking my own section 4.1. The frontier's arity axis does not bite on every
//! chain law, and finding out which ones it bites on is the point.
//!
//! p2 measures that the widest width whose exhaustive law check the compiler will
//! evaluate collapses with arity: 5 at arity 3, 3 at arity 5, 1 at arity 8. I drew
//! the consequence that a chain law of length n is an arity-n law and is therefore
//! unvalidatable at any width arvo ships. That is too broad, and this file is the
//! correction.
//!
//! Two kinds of chain statement, and they behave completely differently.
//!
//! GROUPING. "Every parenthesisation of a chain of length n agrees." This is a
//! CONSEQUENCE of the arity-3 statement, by the generalized associative law, which is
//! a theorem of universal algebra and is independent of width, of the operation, and
//! of n. So its arity-n verdict is obtained by lifting the arity-3 verdict through a
//! proof, and the frontier's arity axis does not bite on it at all.
//!
//! SCHEDULE. "Rounding once at the end agrees with rounding after every step." There
//! is no lower-arity statement this follows from. At length 2 it is vacuously true
//! for both schedules, because one rounding happens either way, so the arity-2 and
//! arity-3 verdicts carry no information about the arity-5 verdict. The frontier's
//! arity axis bites on this one in full, and it is exactly the kind op's I7 names.
//!
//! Build: rustc --edition 2021 -O p6_which_chain_laws_reduce_to_arity_three.rs -o p6 && ./p6
//! Toolchain: nightly-2026-05-28. No feature gates.

const LO: i64 = -8;
const HI: i64 = 7;

fn sat(a: i64, b: i64) -> i64 {
    let s = a + b;
    if s > HI {
        HI
    } else if s < LO {
        LO
    } else {
        s
    }
}

fn wrap(a: i64, b: i64) -> i64 {
    let n = HI - LO + 1;
    (a + b - LO).rem_euclid(n) + LO
}

/// Grouping: does every parenthesisation of `xs` under `op` give one answer?
/// Enumerated by generating all binary trees over the sequence.
fn all_groupings(op: fn(i64, i64) -> i64, xs: &[i64], out: &mut Vec<i64>) {
    if xs.len() == 1 {
        out.push(xs[0]);
        return;
    }
    for k in 1..xs.len() {
        let mut l = Vec::new();
        let mut r = Vec::new();
        all_groupings(op, &xs[..k], &mut l);
        all_groupings(op, &xs[k..], &mut r);
        for a in &l {
            for b in &r {
                out.push(op(*a, *b));
            }
        }
    }
}

fn grouping_disagreements(op: fn(i64, i64) -> i64, n: usize) -> (u64, u64) {
    let vals: Vec<i64> = (LO..=HI).collect();
    let mut total = 0u64;
    let mut bad = 0u64;
    let mut idx = vec![0usize; n];
    loop {
        let xs: Vec<i64> = idx.iter().map(|&i| vals[i]).collect();
        let mut out = Vec::new();
        all_groupings(op, &xs, &mut out);
        total += 1;
        if out.iter().any(|v| *v != out[0]) {
            bad += 1;
        }
        // odometer
        let mut i = 0;
        loop {
            if i == n {
                return (total, bad);
            }
            idx[i] += 1;
            if idx[i] < vals.len() {
                break;
            }
            idx[i] = 0;
            i += 1;
        }
    }
}

// ---------- schedule ----------

/// Fixed-point multiply chain at fraction width F, rounding after every step.
fn stepwise_mul(xs: &[i64], f: u32) -> i64 {
    let half: i64 = 1 << (f - 1);
    let mut acc = xs[0];
    for &x in &xs[1..] {
        acc = (acc * x + half) >> f;
    }
    acc
}

/// The same chain with the intermediate carried exactly and rounded once, at the end.
fn wide_mul(xs: &[i64], f: u32) -> i64 {
    let half: i64 = 1 << (f - 1);
    let mut acc: i128 = xs[0] as i128;
    let mut shift: u32 = 0;
    for &x in &xs[1..] {
        acc *= x as i128;
        shift += f;
    }
    if shift == 0 {
        return acc as i64;
    }
    // one rounding, at the accumulated scale
    let h: i128 = 1i128 << (shift - 1);
    let _ = half;
    ((acc + h) >> shift) as i64
}

fn schedule_disagreements(n: usize, f: u32, range: i64, step: i64) -> (u64, u64) {
    let vals: Vec<i64> = (-range..range).step_by(step as usize).collect();
    let mut total = 0u64;
    let mut bad = 0u64;
    let mut idx = vec![0usize; n];
    loop {
        let xs: Vec<i64> = idx.iter().map(|&i| vals[i]).collect();
        total += 1;
        if stepwise_mul(&xs, f) != wide_mul(&xs, f) {
            bad += 1;
        }
        let mut i = 0;
        loop {
            if i == n {
                return (total, bad);
            }
            idx[i] += 1;
            if idx[i] < vals.len() {
                break;
            }
            idx[i] = 0;
            i += 1;
        }
    }
}

fn main() {
    println!("p6: which chain laws reduce to a lower arity, and which do not");
    println!("window Q = [{}, {}]", LO, HI);
    println!();

    println!("GROUPING: disagreeing tuples out of all tuples, over every parenthesisation");
    println!("{:>6} {:>22} {:>22}", "n", "wrap", "saturate (signed)");
    for n in 2..=5usize {
        let w = grouping_disagreements(wrap, n);
        let s = grouping_disagreements(sat, n);
        println!("{:>6} {:>12} / {:<7} {:>12} / {:<7}", n, w.1, w.0, s.1, s.0);
    }
    println!();
    println!("  wrap is zero at every n, which is what the arity-3 verdict predicts by the");
    println!("  generalized associative law. saturate is nonzero from n = 3 and stays so.");
    println!("  In both directions the arity-3 verdict determines every higher arity, so a");
    println!("  compile-time check of THIS kind of chain law only ever needs arity 3.");
    println!();

    println!("SCHEDULE: stepwise rounding against one rounding at the end, F = 4");
    println!("  operands swept over [-64, 64) step 7");
    println!("{:>6} {:>16} {:>12}", "n", "disagreeing", "of");
    for n in 2..=5usize {
        let (t, b) = schedule_disagreements(n, 4, 64, 7);
        println!("{:>6} {:>16} {:>12}", n, b, t);
    }
    println!();
    println!("  n = 2 is zero by construction: one rounding happens under either schedule,");
    println!("  so there is nothing to disagree about. Every higher n is a fresh statement");
    println!("  with no lower-arity statement implying it, so the arity-3 verdict carries no");
    println!("  information about the arity-5 one and no lifting theorem is available.");
    println!();
    println!("  This is the kind of chain statement op's I7 is about, and it is the kind the");
    println!("  const-eval frontier's arity axis bites on in full.");
}
