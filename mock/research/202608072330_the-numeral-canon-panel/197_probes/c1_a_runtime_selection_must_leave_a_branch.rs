//! C1. The case that had to fail, supplied late, for `93_probes/p4` and its two siblings.
//!
//! Those three probes establish that a strategy resolved at compile time leaves
//! no residue in the emitted body. Each does it the same way: compile two forms,
//! normalise the labels, compare the instructions. **None of them supplies a
//! form that must come out different**, so the comparison has no natural failing
//! case, and the registry marks the standing `uncontrolled` for exactly that
//! reason. Its own note names the missing arm in one sentence: an arm whose
//! selection genuinely cannot be resolved at compile time, and which must
//! therefore leave a branch.
//!
//! That arm is here. The cost model, the arm table, the three arms and the
//! argmin are `p4`'s, unchanged, so the only difference between the two entry
//! points below is where the weighting comes from.
//!
//!   `fold_const`     the weighting is an associated const, resolved at compile
//!                    time, and must lower to the chosen arm alone
//!   `fold_runtime`   the identical argmin over a weighting that arrives as an
//!                    argument, so the selection cannot be resolved and the
//!                    emitted body must carry the branch and all three arms
//!
//! **What the control establishes.** If the two lower alike, the comparison
//! instrument cannot distinguish a resolved selection from an unresolved one,
//! and the three probes it backs measured nothing: their agreement would be the
//! instrument's blindness rather than the design's property. If they differ,
//! with the branch present in one and absent in the other, the instrument can
//! come out both ways and the original finding stands as a measurement.
//!
//! Run:
//!   rustc --edition 2024 -O --emit asm c1_a_runtime_selection_must_leave_a_branch.rs \
//!     -o c1.s
//!   rustc --edition 2024 -O c1_a_runtime_selection_must_leave_a_branch.rs -o /tmp/c1 \
//!     && /tmp/c1
//!
//! The verdict is read off the assembly rather than off the program's output:
//! the program only shows that the two entry points agree on values, which is
//! the premise rather than the finding.

#![no_std]

extern crate std;
use std::println;

// --------------------------------------------------------------------------
// `p4`'s cost model, copied rather than paraphrased so the comparison is
// against the same thing it claims to control.
// --------------------------------------------------------------------------

const AXES: usize = 3; // (time, space, error)
const ARMS: usize = 3;

const ARM_COST: [[u32; AXES]; ARMS] = [
    // time, space, error
    [1, 1, 9], // 0: wrap at the declared width
    [3, 1, 4], // 1: saturate at the declared width
    [7, 3, 0], // 2: accumulate in a wider carrier, never lose a bit
];

/// A weighting is the whole content of a strategy at this layer.
pub trait Preference {
    const W: [u32; AXES];
}

pub struct PrefSpeed;
impl Preference for PrefSpeed {
    const W: [u32; AXES] = [8, 1, 1];
}

/// The argmin. `const fn`, so it can be called in either position, which is the
/// point: the function is identical and only the argument's binding time moves.
const fn resolve(w: [u32; AXES]) -> usize {
    let mut best = 0usize;
    let mut best_score = u32::MAX;
    let mut i = 0usize;
    while i < ARMS {
        let mut s = 0u32;
        let mut a = 0usize;
        while a < AXES {
            s += w[a] * ARM_COST[i][a];
            a += 1;
        }
        if s < best_score {
            best_score = s;
            best = i;
        }
        i += 1;
    }
    best
}

// --------------------------------------------------------------------------
// The arms, `p4`'s again.
// --------------------------------------------------------------------------

const W_BITS: u32 = 12;
const MODULUS: u64 = 1u64 << W_BITS;
const MAXV: u64 = MODULUS - 1;

#[inline(always)]
fn arm_wrap(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i < xs.len() {
        acc = (acc + xs[i]) % MODULUS;
        i += 1;
    }
    acc
}

#[inline(always)]
fn arm_sat(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i < xs.len() {
        let s = acc + xs[i];
        acc = if s > MAXV { MAXV } else { s };
        i += 1;
    }
    acc
}

#[inline(always)]
fn arm_widen(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i < xs.len() {
        acc += xs[i];
        i += 1;
    }
    acc
}

#[inline(always)]
fn dispatch(arm: usize, xs: &[u64]) -> u64 {
    match arm {
        0 => arm_wrap(xs),
        1 => arm_sat(xs),
        _ => arm_widen(xs),
    }
}

// --------------------------------------------------------------------------
// The two entry points. Same argmin, same arms, same dispatch. The weighting's
// binding time is the only thing that differs.
// --------------------------------------------------------------------------

/// The resolved form. `P::W` is const, so `resolve` folds and one arm survives.
#[unsafe(no_mangle)]
pub fn fold_const(xs: &[u64]) -> u64 {
    dispatch(resolve(PrefSpeed::W), xs)
}

/// The unresolved form, and the case that must fail.
///
/// `w` arrives as an argument, so nothing can fold `resolve`, the `match` is a
/// real branch, and all three arms have to survive into the body.
#[unsafe(no_mangle)]
pub fn fold_runtime(xs: &[u64], w: [u32; AXES]) -> u64 {
    dispatch(resolve(w), xs)
}

/// The hand-written arm, which `p4` compares against.
#[unsafe(no_mangle)]
pub fn fold_direct(xs: &[u64]) -> u64 {
    arm_wrap(xs)
}

fn main() {
    let xs: [u64; 6] = [1000, 2000, 3000, 4000, 5000, 6000];

    // The premise, not the finding: at this weighting the two entry points
    // agree on values, so any difference in the emitted body is the binding
    // time rather than a difference in what is computed.
    let a = fold_const(&xs);
    let b = fold_runtime(&xs, PrefSpeed::W);
    let c = fold_direct(&xs);
    println!("fold_const   = {a}");
    println!("fold_runtime = {b}  (same weighting, arriving at runtime)");
    println!("fold_direct  = {c}  (the arm, hand written)");
    println!(
        "values agree: {}",
        if a == b && b == c {
            "yes"
        } else {
            "NO, the premise fails"
        }
    );

    // And the weighting is load-bearing, which is what says `resolve` is not a
    // constant function wearing a table.
    let accuracy = fold_runtime(&xs, [1, 1, 8]);
    println!("fold_runtime at a weighting preferring accuracy = {accuracy}");
    println!(
        "the weighting selects: {}",
        if accuracy != b {
            "yes"
        } else {
            "NO, the model is inert"
        }
    );

    println!();
    println!("The verdict is in the assembly, not here. Count conditional jumps");
    println!("in `fold_const` and in `fold_runtime`: the control passes when the");
    println!("first has none and the second has them, and fails when they match,");
    println!("because then the comparison cannot tell the two apart at all.");
}
