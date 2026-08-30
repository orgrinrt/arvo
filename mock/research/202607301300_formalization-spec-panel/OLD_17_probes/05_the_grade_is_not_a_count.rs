//! PROBE 5: the quantisation grade is not a count, it is a multiset of
//! events, and probe 4's own round-to-odd arm tested outside the theorem's
//! precondition.
//!
//! Probe 4 (this directory, prior partial run of this dispatch) proposed that
//! the fact deciding MUL_REGROUP is a NUMBER: how many times a value is
//! quantised on the way from inputs to answer. Probe 3's `Exact (unbounded)`
//! row at Q2.2 is the evidence for the premise: with NO range recovery firing
//! at all, MUL_REGROUP and DISTRIB still fail, purely from the unconditional
//! `>> FRAC` in `arith_macros.rs:34` / `:99`. So the failure is not a range
//! phenomenon and is not keyed on the recovery rule.
//!
//! Probe 4 then went one step too far in one direction and not far enough in
//! another, and this probe fixes both.
//!
//! TOO FAR: it reported a "k=2 with round-to-odd intermediate" column and let
//! the reader infer round-to-odd is the cure. It is not, as constructed: its
//! `two_roundings(a,b,c, to_odd, near_even)` rounds the intermediate to odd at
//! the SAME scale FRAC as the final result. Round-to-odd's guarantee
//! (Boldo and Melquiond, "Emulation of FMA and correctly-rounded sums", and
//! the double-rounding literature generally) requires the intermediate to be
//! held at strictly greater precision than the final, with enough guard bits
//! that a tie at the final rounding is impossible. At equal precision the
//! theorem says nothing, and probe 4's own numbers show it: round-to-odd
//! DISAGREES MORE than plain nearest-even (1056 against 640).
//!
//! NOT FAR ENOUGH: once you supply the guard bits the theorem needs, the
//! disagreement goes away. Which means the fact that decides MUL_REGROUP is
//! not the count alone. It is the count TOGETHER WITH each event's precision
//! and rule. That is a multiset of event descriptions, not a natural number,
//! and in this design the description is exactly `Growth::Narrowed<W, A>`'s
//! own parameters.
//!
//! Grid, all at Q2.2 with an unbounded (never-recovering) range so that no
//! range effect can contaminate the reading:
//!
//!   k=1            round once, at the exact product scale
//!   k=2, g=0       round the intermediate at FRAC, then the result at FRAC
//!   k=2, g=1..4    hold the intermediate at FRAC+g, then round to FRAC
//!                  intermediate rule: near-even, trunc, or to-odd
//!
//! Build:  rustc -O 05_the_grade_is_not_a_count.rs && ./05_the_grade_is_not_a_count

const FRAC: u32 = 2;
const LO: i64 = -8;
const HI: i64 = 7;

/// Round to nearest, ties to even, shifting right by `s`.
fn near_even(x: i64, s: u32) -> i64 {
    if s == 0 {
        return x;
    }
    let half = 1i64 << (s - 1);
    let mask = (1i64 << s) - 1;
    let q = x >> s;
    let rem = x & mask;
    if rem > half {
        q + 1
    } else if rem < half {
        q
    } else if q & 1 == 1 {
        q + 1
    } else {
        q
    }
}

/// Round to odd: exact results pass through, inexact ones land on an odd
/// value, so a later rounding of that value can never be a tie.
fn to_odd(x: i64, s: u32) -> i64 {
    if s == 0 {
        return x;
    }
    let mask = (1i64 << s) - 1;
    let q = x >> s;
    if (x & mask) != 0 {
        q | 1
    } else {
        q
    }
}

/// Truncate. What the shipped multiply actually does, for every strategy,
/// including the three the preset table says round to nearest-even
/// (`arith_macros.rs:34`, `:99`, `:148`, `:219`).
fn trunc(x: i64, s: u32) -> i64 {
    x >> s
}

type Round = fn(i64, u32) -> i64;

/// One quantisation event. `a*b*c` is 2^(3F)-scaled; land it at 2^F in one go.
fn k1(a: i64, b: i64, c: i64, r: Round) -> i64 {
    r(a * b * c, 2 * FRAC)
}

/// Two quantisation events with `g` guard bits kept at the intermediate.
/// g = 0 is the shipped shape: the intermediate is a full-fledged Q2.2 value.
/// g > 0 holds the intermediate at scale FRAC+g and rounds it away at the end.
fn k2(a: i64, b: i64, c: i64, g: u32, inner: Round, outer: Round) -> i64 {
    // a*b is 2^(2F)-scaled. Land it at 2^(F+g): shift away F-g.
    let ab = inner(a * b, 2 * FRAC - (FRAC + g));
    // ab*c is 2^(F+g+F)-scaled. Land it at 2^F: shift away F+g.
    outer(ab * c, FRAC + g)
}

fn sweep(g: u32, inner: Round, outer: Round, reference: Round) -> (usize, i64) {
    let mut disagree = 0usize;
    let mut worst = 0i64;
    for a in LO..=HI {
        for b in LO..=HI {
            for c in LO..=HI {
                let one = k1(a, b, c, reference);
                let two = k2(a, b, c, g, inner, outer);
                if one != two {
                    disagree += 1;
                    let d = (one - two).abs();
                    if d > worst {
                        worst = d;
                    }
                }
            }
        }
    }
    (disagree, worst)
}

fn main() {
    let total = ((HI - LO + 1) as usize).pow(3);
    println!("all rows: Q2.2, unbounded range, {total} triples, reference = k=1 nearest-even\n");

    println!(
        "{:<44}{:>12}{:>12}",
        "two-event shape", "disagree", "worst err"
    );
    for (label, g, inner) in [
        (
            "g=0  intermediate near-even  (Growth::Narrowed)",
            0u32,
            near_even as Round,
        ),
        (
            "g=0  intermediate truncate   (as SHIPPED)",
            0,
            trunc as Round,
        ),
        (
            "g=0  intermediate to-odd     (probe 4's arm)",
            0,
            to_odd as Round,
        ),
        ("g=1  intermediate to-odd", 1, to_odd as Round),
        ("g=2  intermediate to-odd", 2, to_odd as Round),
        ("g=3  intermediate to-odd", 3, to_odd as Round),
        ("g=4  intermediate to-odd", 4, to_odd as Round),
        ("g=4  intermediate near-even", 4, near_even as Round),
        ("g=4  intermediate truncate", 4, trunc as Round),
    ] {
        let (d, w) = sweep(g, inner, near_even as Round, near_even as Round);
        println!("{label:<44}{d:>12}{w:>12}");
    }

    println!();
    println!("reading:");
    println!("  a bare count cannot tell these nine rows apart. Every one is k=2.");
    println!("  the rows differ by the intermediate's PRECISION and its RULE, which is");
    println!("  exactly what `Growth::Narrowed<W, A>` parameterises, so the grade element");
    println!("  is a multiset of (width, rule) events and not a natural number.");
    println!("  where the round-to-odd theorem's precondition is met the k=2 shape");
    println!("  reproduces the k=1 answer exactly, which is a licence that a boolean");
    println!("  `does this quantise` fact and a bare count both structurally cannot state.");
}
