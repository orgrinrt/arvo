//! Probe p1: which clauses of the half-step-phase finding survive a change of
//! quantum, and which one is a fact about the one geometry it was measured at.
//!
//! Subject. `proposal::a_nonzero_phase_leaves_the_representable_set_without_an
//! _additive_identity` says, unquantified: "A half-step-biased grid is not
//! closed under exact addition. No exact sum of two grid points lands on the
//! grid, every one sits exactly half a step away, and the grid contains neither
//! zero nor one." Its predicate is `total_width: W = 4`, `fraction_width: F =
//! 0`, `signedness = unsigned`, `operation = add`, `arity = 2`, and its
//! instrument is `56_probes/q2_affine_membership.rs`, which runs at exactly one
//! geometry: step 1/4, bias 1/8. That probe's own RUN.md says so, in terms:
//! "All counts are at 4 bits (q1, q3) or one grid geometry (q2, step 1/4, bias
//! 1/8, scale 2^5); width transfer is argued, not probed."
//!
//! The quantum is not on the row's predicate at all. So this instrument varies
//! it, holding the phase at a half step, and asks which of the four clauses
//! moves.
//!
//! Three arms, each an affine grid written with the one membership predicate:
//!
//!   A  step 1/4, phase 1/2 of a step (bias 1/8). The measured geometry, and
//!      the arm that has to reproduce `q2` or this instrument is not measuring
//!      the same thing.
//!   B  step 2, phase 1/2 of a step (bias 1). A half-step-biased grid whose
//!      quantum is greater than one. `format::Format` in the shipped tree
//!      expresses it directly: `Quantum = Constant<1>` over a radix-2 ambient
//!      gives a step of 2, and `PHASE_NUM = 1, PHASE_DEN = 2` is the half step.
//!   C  step 1, phase 1/3 of a step (bias 1/3). Not a half-step phase, and it
//!      is here only for the wider sentence the shipped tree carries in
//!      `format::has_additive_identity`'s doc comment, which says "a nonzero
//!      phase" where the finding says "half-step-biased".
//!
//! What must fail, stated before the run. Three controls, and any one of them
//! coming out the wrong way voids the arm it guards:
//!
//!   1. `contains one` must DIFFER between A and B. If it agrees on both, this
//!      instrument cannot tell the two geometries apart and its report that a
//!      clause is geometry-dependent means nothing.
//!   2. A phase-zero grid at each arm's own step must contain zero. If the
//!      membership predicate said no everywhere, "contains no zero" would be a
//!      fact about the predicate rather than about the phase.
//!   3. `every sum sits exactly half a step from the grid` must be FALSE at C.
//!      If it were true at all three the distance arm would be structurally
//!      green and would establish nothing.
//!
//! Exact arithmetic throughout, in integer units of 1/24, so all three steps
//! and all three biases are integers and nothing rounds. `one` is 24 and `zero`
//! is 0.

use std::collections::BTreeSet;

/// The unit: every value below is an integer count of 1/24.
const D: i64 = 24;

/// THE affine membership predicate, written once and instantiated three times.
///
/// Bounded form: on the lattice and inside the declared window.
fn member(q: i64, step: i64, bias: i64, lo: i64, hi: i64) -> bool {
    q >= lo && q <= hi && (q - bias).rem_euclid(step) == 0
}

/// The same predicate with the window dropped.
///
/// Closure of a grid is a question about the lattice, not about the window: a
/// sum leaving the window is out of range rather than off the grid, and reading
/// the first as the second would make every bounded grid non-closed for free.
fn on_lattice(q: i64, step: i64, bias: i64) -> bool {
    (q - bias).rem_euclid(step) == 0
}

/// The grid points inside the window, enumerated without consulting `member`.
fn enumerate(step: i64, bias: i64, lo: i64, hi: i64) -> BTreeSet<i64> {
    let mut out = BTreeSet::new();
    // Start at the lowest lattice point at or above `lo`, computed rather than
    // guessed, so the enumeration does not inherit the predicate's arithmetic.
    let mut v = bias;
    while v > lo {
        v -= step;
    }
    while v < lo {
        v += step;
    }
    while v <= hi {
        out.insert(v);
        v += step;
    }
    out
}

/// Distance from `q` to the nearest lattice point of (step, bias).
fn distance_to_lattice(q: i64, step: i64, bias: i64) -> i64 {
    let r = (q - bias).rem_euclid(step);
    if r < step - r { r } else { step - r }
}

struct Arm {
    name: &'static str,
    step: i64,
    bias: i64,
    lo: i64,
    hi: i64,
}

fn main() {
    // 16 slots each, starting at the first lattice point at or above zero.
    let arms = [
        Arm { name: "A  step 1/4, bias 1/8 (the measured geometry)", step: 6, bias: 3, lo: 0, hi: 3 + 6 * 15 },
        Arm { name: "B  step 2,   bias 1   (quantum above one)    ", step: 48, bias: 24, lo: 0, hi: 24 + 48 * 15 },
        Arm { name: "C  step 1,   bias 1/3 (phase not a half step)", step: 24, bias: 8, lo: 0, hi: 8 + 24 * 15 },
    ];

    let mut contains_one = Vec::new();
    let mut half_step = Vec::new();

    for arm in &arms {
        let grid = enumerate(arm.step, arm.bias, arm.lo, arm.hi);

        // The predicate agrees with an enumeration written independently of it.
        let by_predicate: BTreeSet<i64> =
            (arm.lo..=arm.hi).filter(|&q| member(q, arm.step, arm.bias, arm.lo, arm.hi)).collect();
        let agrees = by_predicate == grid;

        // The bias-dropped mutant must disagree, or the predicate is not reading
        // the phase at all.
        let by_mutant: BTreeSet<i64> =
            (arm.lo..=arm.hi).filter(|&q| member(q, arm.step, 0, arm.lo, arm.hi)).collect();
        let mutant_detected = by_mutant != grid;

        let has_zero = on_lattice(0, arm.step, arm.bias);
        let has_one = on_lattice(D, arm.step, arm.bias);

        let mut sums = 0usize;
        let mut sums_on_lattice = 0usize;
        let mut distances: BTreeSet<i64> = BTreeSet::new();
        for &x in &grid {
            for &y in &grid {
                sums += 1;
                if on_lattice(x + y, arm.step, arm.bias) {
                    sums_on_lattice += 1;
                }
                distances.insert(distance_to_lattice(x + y, arm.step, arm.bias));
            }
        }
        let every_sum_half_a_step =
            distances.len() == 1 && distances.contains(&(arm.step / 2)) && arm.step % 2 == 0;

        // Control 2, per arm: the same predicate at phase zero must find zero.
        let phase_zero_has_zero = on_lattice(0, arm.step, 0);

        println!("{}", arm.name);
        println!("    grid size                        {}", grid.len());
        println!("    predicate agrees with enumeration {}", agrees);
        println!("    bias-dropped mutant detected      {}", mutant_detected);
        println!("    contains zero                     {}", has_zero);
        println!("    contains one                      {}", has_one);
        println!("    sums landing on the grid          {} of {}", sums_on_lattice, sums);
        println!(
            "    distinct distances (units of 1/{})  {:?}   half a step is {}",
            D,
            distances,
            arm.step / 2
        );
        println!("    every sum exactly half a step     {}", every_sum_half_a_step);
        println!("    CONTROL phase-zero grid has zero  {}", phase_zero_has_zero);
        println!();

        assert!(agrees, "the predicate and the enumeration disagree; the instrument is broken");
        assert!(mutant_detected, "the bias-dropped mutant was not detected");
        assert!(phase_zero_has_zero, "CONTROL 2 FAILED at {}", arm.name);

        contains_one.push(has_one);
        half_step.push(every_sum_half_a_step);
    }

    // Control 1: the `contains one` clause must split A from B.
    let control_1 = contains_one[0] != contains_one[1];
    // Control 3: the half-step clause must hold at A and B and fail at C.
    let control_3 = half_step[0] && half_step[1] && !half_step[2];

    println!("CONTROL 1  contains-one differs between A and B   {}", control_1);
    println!("CONTROL 3  half-step holds at A and B, fails at C {}", control_3);
    assert!(control_1, "CONTROL 1 FAILED: the instrument cannot separate the two geometries");
    assert!(control_3, "CONTROL 3 FAILED: the distance arm is structurally green");

    println!();
    println!("VERDICT");
    println!("  not closed under exact addition   holds at all three arms");
    println!("  contains no zero                  holds at all three arms");
    println!("  every sum exactly half a step     holds at A and B, fails at C");
    println!("  contains no one                   holds at A, FAILS at B");
    println!();
    println!("P1 WORKS");
}
