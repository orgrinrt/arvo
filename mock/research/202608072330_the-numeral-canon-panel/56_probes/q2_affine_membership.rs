//! Probe q2: the phase (bias) parameter lives inside Q, leaves R's laws
//! intact, and has an arithmetic consequence the law layer must know.
//!
//! Context. `55` conceded (phase two, 2a) that its slot-function Q bakes in
//! phase zero and cannot say a grid is offset by half a step, and asked where
//! the phase parameter belongs. `08` carries the record's affine value map
//! (value = Adjustment * radix^exponent * k + Bias) with Bias a separate
//! parameter. This probe answers the placement question by construction:
//!
//!   1. an AFFINE membership predicate, written once, parameterised by
//!      (step S, bias B, bounds): x in Q iff x == B (mod S) and in bounds.
//!      Instantiated at plain fixed point (B = 0) and at a half-unit-biased
//!      grid (B = S/2), each compared exhaustively against a direct
//!      enumeration that never mentions the predicate.
//!   2. round-to-nearest ONTO the biased grid keeps every adaptation law
//!      (total, retraction, monotone, distance minimising), exhaustively.
//!      So the phase parameter changes Q and changes NOTHING in R's laws:
//!      it belongs in Q, and R is untouched.
//!   3. the arithmetic consequence: a grid at phase B = S/2 is NOT closed
//!      under exact addition. The exact sum of two grid points lies on the
//!      PHASE-ZERO grid, never on the biased grid (checked exhaustively),
//!      and its distance to the nearest biased-grid point is exactly S/2,
//!      a tie, EVERY time. So on a half-unit-biased format every addition
//!      adapts, the identity-adaptation case never occurs, and the
//!      round-to-nearest tie rule stops being a corner case and becomes the
//!      dominant policy. Also: neither 0 nor 1 is representable, which is
//!      the predecessor's no-representable-one finding reappearing as a
//!      phase consequence.
//!
//! Instrument validation: a mutant predicate with the bias dropped must
//! disagree with the biased enumeration, and a mutant rounder targeting the
//! phase-zero grid must fail the retraction law on the biased Q.
//!
//! Exact arithmetic in scaled integers: x = q / 32 (scale 2^5), step S = 1/4
//! (8 scaled), bias B = 1/8 (4 scaled). Bounds [0, 4) in value terms.

const SCALE: i64 = 32;
const STEP: i64 = 8; // 1/4 scaled
const BIAS: i64 = 4; // 1/8 scaled
const LO: i64 = 0;
const HI: i64 = 127; // just below 4.0 scaled

// THE affine membership predicate, written once
fn member(q: i64, step: i64, bias: i64, lo: i64, hi: i64) -> bool {
    q >= lo && q <= hi && (q - bias).rem_euclid(step) == 0
}

// mutant: bias dropped
fn member_mutant(q: i64, step: i64, lo: i64, hi: i64) -> bool {
    q >= lo && q <= hi && q.rem_euclid(step) == 0
}

use std::collections::BTreeSet;

fn enum_grid(step: i64, bias: i64, lo: i64, hi: i64) -> BTreeSet<i64> {
    let mut s = BTreeSet::new();
    let mut v = bias;
    while v <= hi {
        if v >= lo {
            s.insert(v);
        }
        v += step;
    }
    s
}

fn predicate_set(step: i64, bias: i64, lo: i64, hi: i64) -> BTreeSet<i64> {
    (lo..=hi)
        .filter(|&q| member(q, step, bias, lo, hi))
        .collect()
}

// round to nearest onto a grid, ties toward positive infinity (a stated rule;
// the tie RULE is exactly what part 3 shows becomes load bearing)
fn round_nearest(x: i64, grid: &BTreeSet<i64>) -> i64 {
    *grid
        .iter()
        .min_by_key(|&&g| ((g - x).abs(), -g))
        .expect("nonempty grid")
}

fn main() {
    let mut ok = true;

    // part 1: predicate against enumeration, both instances
    let plain = enum_grid(STEP, 0, LO, HI);
    let hub = enum_grid(STEP, BIAS, LO, HI);
    let p_plain = predicate_set(STEP, 0, LO, HI);
    let p_hub = predicate_set(STEP, BIAS, LO, HI);
    println!(
        "plain: predicate {} values, enumeration {} values, equal: {}",
        p_plain.len(),
        plain.len(),
        p_plain == plain
    );
    println!(
        "hub:   predicate {} values, enumeration {} values, equal: {}",
        p_hub.len(),
        hub.len(),
        p_hub == hub
    );
    ok &= p_plain == plain && p_hub == hub && p_hub.len() == 16;

    // mutant A: bias dropped must disagree with the biased enumeration
    let pm: BTreeSet<i64> = (LO..=HI)
        .filter(|&q| member_mutant(q, STEP, LO, HI))
        .collect();
    let detect_a = pm != hub;
    println!("mutant predicate (bias dropped) detected: {}", detect_a);
    ok &= detect_a;

    // part 2: round-to-nearest onto the biased grid keeps the adaptation laws
    let window = -64i64..=191; // well past the bounds both ways
    let total = window
        .clone()
        .all(|x| hub.contains(&round_nearest(x, &hub)));
    let retraction = hub.iter().all(|&x| round_nearest(x, &hub) == x);
    let mut monotone = true;
    for x in window.clone() {
        for y in window.clone() {
            if x <= y && round_nearest(x, &hub) > round_nearest(y, &hub) {
                monotone = false;
            }
        }
    }
    let nearest = window.clone().all(|x| {
        hub.iter()
            .all(|&c| (round_nearest(x, &hub) - x).abs() <= (c - x).abs())
    });
    println!(
        "nearest onto biased grid: total {} retraction {} monotone {} distance-minimising {}",
        total, retraction, monotone, nearest
    );
    ok &= total && retraction && monotone && nearest;

    // mutant B: a rounder targeting the phase-zero grid fails retraction on hub Q
    let wrong_target_fails = hub.iter().any(|&x| round_nearest(x, &plain) != x);
    println!(
        "mutant rounder (phase-zero target) fails retraction on hub: {}",
        wrong_target_fails
    );
    ok &= wrong_target_fails;

    // part 3: the biased grid is not closed under exact addition, and every
    // exact sum is a TIE for round-to-nearest onto the biased grid
    let mut sums_on_hub = 0u64;
    let mut sums_total = 0u64;
    let mut always_tie = true;
    for &a in &hub {
        for &b in &hub {
            let s = a + b;
            sums_total += 1;
            if member(s, STEP, BIAS, i64::MIN / 4, i64::MAX / 4) {
                sums_on_hub += 1;
            }
            // distance to the nearest biased-grid point, on the unbounded grid:
            // (s - BIAS) mod STEP gives the residue; tie means residue == STEP/2
            let r = (s - BIAS).rem_euclid(STEP);
            if r != STEP / 2 {
                always_tie = false;
            }
        }
    }
    println!(
        "exact sums of hub points landing on the hub grid: {} of {}",
        sums_on_hub, sums_total
    );
    println!(
        "every exact sum sits exactly half a step from the hub grid (tie): {}",
        always_tie
    );
    ok &= sums_on_hub == 0 && always_tie;

    // and the identities: neither 0 nor 1 is on the biased grid
    let zero_in = hub.contains(&0);
    let one_in = hub.contains(&SCALE);
    println!(
        "hub grid contains zero: {}   contains one: {}",
        zero_in, one_in
    );
    ok &= !zero_in && !one_in;

    println!("{}", if ok { "Q2 WORKS" } else { "Q2 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
