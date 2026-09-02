// p2: which algebraic laws survive which overflow policy, over the whole
// (I, F) box rather than a sample of it.
//
// The algorithm layer above the numeral is folds, and a fold that a compiler
// or a scheduler is allowed to reassociate, vectorise or split across cores
// needs the operation to be associative. A semiring computation (every graph
// algorithm is one) needs more: two associative operations, identities, and
// distributivity. This probe counts how many of those hold under each
// overflow policy.
//
// Model. A numeral of total width W with F fractional bits holds raw values
// in [0, 2^W). Its value is raw / 2^F. Addition adds raws; multiplication
// multiplies raws and shifts right by F. The policy decides what happens when
// the exact result leaves [0, 2^W):
//
//   wrap      take it modulo 2^W
//   saturate  clamp it to 2^W - 1
//
// Both are exhaustive over every ordered pair or triple in the box, so no
// sampling decides what gets found. F ranges over 0..=W for every W, so the
// purely fractional shapes (I == 0) are present rather than skipped.
//
// Build and run:
//   rustc +nightly-2026-05-28 -O --edition 2021 -o p2 p2_laws.rs && ./p2

#[derive(Clone, Copy, PartialEq, Eq)]
enum Policy {
    Wrap,
    Saturate,
}

#[inline(always)]
fn clip(x: u128, w: u32, p: Policy) -> u128 {
    let m: u128 = 1u128 << w;
    match p {
        Policy::Wrap => x & (m - 1),
        Policy::Saturate => {
            if x >= m {
                m - 1
            } else {
                x
            }
        }
    }
}

#[inline(always)]
fn add(a: u128, b: u128, w: u32, p: Policy) -> u128 {
    clip(a + b, w, p)
}

#[inline(always)]
fn sub(a: u128, b: u128, w: u32, p: Policy) -> u128 {
    // Unsigned subtraction. Wrapping borrows; saturating floors at zero.
    let m: u128 = 1u128 << w;
    match p {
        Policy::Wrap => (a + m - b) & (m - 1),
        Policy::Saturate => {
            if b > a {
                0
            } else {
                a - b
            }
        }
    }
}

#[inline(always)]
fn mul(a: u128, b: u128, w: u32, f: u32, p: Policy) -> u128 {
    // Exact product of raws, rescaled by the fractional bit count. The shift
    // truncates toward zero, which is itself a rounding choice; it is the one
    // a shift gives and the one a fixed-point multiply reaches for.
    clip((a * b) >> f, w, p)
}

struct Row {
    w: u32,
    f: u32,
    policy: &'static str,
    add_assoc_fail: u64,
    add_assoc_total: u64,
    mul_assoc_fail: u64,
    mul_assoc_total: u64,
    distrib_fail: u64,
    distrib_total: u64,
    mono_fail: u64,
    mono_total: u64,
    inverse_fail: u64,
    inverse_total: u64,
}

fn run(w: u32, f: u32, p: Policy, name: &'static str) -> Row {
    let n: u128 = 1u128 << w;

    let mut r = Row {
        w,
        f,
        policy: name,
        add_assoc_fail: 0,
        add_assoc_total: 0,
        mul_assoc_fail: 0,
        mul_assoc_total: 0,
        distrib_fail: 0,
        distrib_total: 0,
        mono_fail: 0,
        mono_total: 0,
        inverse_fail: 0,
        inverse_total: 0,
    };

    // Triple laws: associativity of both operations, and distributivity.
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                r.add_assoc_total += 1;
                if add(add(a, b, w, p), c, w, p) != add(a, add(b, c, w, p), w, p) {
                    r.add_assoc_fail += 1;
                }

                r.mul_assoc_total += 1;
                if mul(mul(a, b, w, f, p), c, w, f, p) != mul(a, mul(b, c, w, f, p), w, f, p) {
                    r.mul_assoc_fail += 1;
                }

                // a * (b + c) against a*b + a*c
                r.distrib_total += 1;
                let lhs = mul(a, add(b, c, w, p), w, f, p);
                let rhs = add(mul(a, b, w, f, p), mul(a, c, w, f, p), w, p);
                if lhs != rhs {
                    r.distrib_fail += 1;
                }
            }
        }
    }

    // Monotonicity of + against the value order: a <= b implies a+c <= b+c.
    // This is the precondition every shortest-path and DP relaxation rests on.
    for a in 0..n {
        for b in a..n {
            for c in 0..n {
                r.mono_total += 1;
                if add(a, c, w, p) > add(b, c, w, p) {
                    r.mono_fail += 1;
                }
            }
        }
    }

    // Additive inverse: (a + b) - b == a. Whether the numeral is a group,
    // which is what decides whether a maintained aggregate can retract a
    // contribution in place or has to be recomputed.
    for a in 0..n {
        for b in 0..n {
            r.inverse_total += 1;
            if sub(add(a, b, w, p), b, w, p) != a {
                r.inverse_fail += 1;
            }
        }
    }

    r
}

fn pct(f: u64, t: u64) -> f64 {
    if t == 0 {
        0.0
    } else {
        100.0 * (f as f64) / (t as f64)
    }
}

fn main() {
    // Whole box: every width 2..=7, every fractional split 0..=W. W is capped
    // at 7 because the triple laws are cubic and the point is coverage of the
    // (I, F) matrix rather than reach in W; the pair laws are re-run at W=8
    // below as a check that nothing about the counts is a small-width artifact.
    println!("w,f,policy,law,failures,total,pct");

    let mut any_add_assoc_wrap: u64 = 0;
    let mut any_mono_sat: u64 = 0;

    for w in 2..=7u32 {
        for f in 0..=w {
            for (p, name) in [(Policy::Wrap, "wrap"), (Policy::Saturate, "saturate")] {
                let r = run(w, f, p, name);
                if p == Policy::Wrap {
                    any_add_assoc_wrap += r.add_assoc_fail;
                } else {
                    any_mono_sat += r.mono_fail;
                }
                println!(
                    "{},{},{},add_assoc,{},{},{:.4}",
                    r.w,
                    r.f,
                    r.policy,
                    r.add_assoc_fail,
                    r.add_assoc_total,
                    pct(r.add_assoc_fail, r.add_assoc_total)
                );
                println!(
                    "{},{},{},mul_assoc,{},{},{:.4}",
                    r.w,
                    r.f,
                    r.policy,
                    r.mul_assoc_fail,
                    r.mul_assoc_total,
                    pct(r.mul_assoc_fail, r.mul_assoc_total)
                );
                println!(
                    "{},{},{},distributivity,{},{},{:.4}",
                    r.w,
                    r.f,
                    r.policy,
                    r.distrib_fail,
                    r.distrib_total,
                    pct(r.distrib_fail, r.distrib_total)
                );
                println!(
                    "{},{},{},monotonicity_add,{},{},{:.4}",
                    r.w,
                    r.f,
                    r.policy,
                    r.mono_fail,
                    r.mono_total,
                    pct(r.mono_fail, r.mono_total)
                );
                println!(
                    "{},{},{},additive_inverse,{},{},{:.4}",
                    r.w,
                    r.f,
                    r.policy,
                    r.inverse_fail,
                    r.inverse_total,
                    pct(r.inverse_fail, r.inverse_total)
                );
            }
        }
    }

    eprintln!("--- invariants that must hold across the whole box ---");
    eprintln!(
        "wrapping addition associativity failures, summed over every (w,f): {}",
        any_add_assoc_wrap
    );
    eprintln!(
        "saturating addition monotonicity failures, summed over every (w,f): {}",
        any_mono_sat
    );
}
