//! P4. Does a PREFERENCE, resolved by a const cost model, erase.
//!
//! The proposal this probe tests: the type parameter names what the consumer
//! WANTS (a weighting over cost axes) rather than what the compiler PICKED (an
//! arm). The arm is then computed at compile time from the weighting, the
//! format, and whatever else is const-available. Op's addendum to I13 licenses
//! exactly that category: "the above collapses to whatever is available at
//! const time".
//!
//! That proposal is worthless unless it satisfies I15: "Never any runtime
//! checks, ever. We catch invalids on compile time, and unused paths we clear
//! out when lowered." So the question is whether a cost model evaluated over a
//! candidate table, at const time, leaves anything behind in the emitted code.
//!
//! Three functions with identical observable behaviour are emitted:
//!   direct_wrap      the arm, hand written, no strategy machinery at all
//!   pref_speed       the same arm reached through a weighting that prefers time
//!   pref_accuracy    a different arm reached through a weighting that prefers
//!                    accuracy, to show the model is actually load bearing and
//!                    the first two are not identical by accident
//!
//! If the first two lower to the same instructions and the third differs, the
//! preference machinery costs nothing and does something. If the first two
//! differ, the proposal has a runtime residue and is refused on I15.
//!
//! Run:
//!   rustc --edition 2024 -O --emit asm p4_preference_erases.rs -o p4.s
//!   rustc --edition 2024 -O p4_preference_erases.rs -o /tmp/p4 && /tmp/p4

#![no_std]
#![feature(const_trait_impl)]

extern crate std;
use std::{println, string::String, vec::Vec};

// --------------------------------------------------------------------------
// The cost model. Each candidate arm carries a cost vector; a preference is a
// weighting over the axes; the chosen arm is the argmin. All const.
// --------------------------------------------------------------------------

const AXES: usize = 3; // (time, space, error)
const ARMS: usize = 3;

/// Candidate arms for "accumulate a run of values at a declared width".
/// Costs are illustrative: the point is the resolution mechanism, not these
/// numbers, and nothing downstream reads them as measured.
const ARM_COST: [[u32; AXES]; ARMS] = [
    // time, space, error
    [1, 1, 9], // 0: wrap at the declared width
    [3, 1, 4], // 1: saturate at the declared width
    [7, 3, 0], // 2: accumulate in a wider carrier, never lose a bit
];

const ARM_NAME: [&str; ARMS] = ["wrap", "saturate", "widen"];

/// A weighting is the whole content of a strategy at this layer.
pub trait Preference {
    const W: [u32; AXES];
    const NAME: &'static str;
}

pub struct PrefSpeed;
impl Preference for PrefSpeed {
    const W: [u32; AXES] = [8, 1, 1];
    const NAME: &'static str = "prefer time";
}

pub struct PrefAccuracy;
impl Preference for PrefAccuracy {
    const W: [u32; AXES] = [1, 1, 8];
    const NAME: &'static str = "prefer accuracy";
}

pub struct PrefSpace;
impl Preference for PrefSpace {
    const W: [u32; AXES] = [1, 8, 1];
    const NAME: &'static str = "prefer space";
}

/// The resolution. A const fn, so the whole thing is a compile-time argmin over
/// a table and there is nothing here for the backend to branch on.
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
// The arms. Written once; the strategy chooses which one is instantiated.
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

/// The generic entry point. The `match` is over a const, so it is not a branch:
/// it is a compile-time selection, and the two arms not taken must not survive
/// into the emitted body.
#[inline(always)]
fn fold<P: Preference>(xs: &[u64]) -> u64 {
    match resolve(P::W) {
        0 => arm_wrap(xs),
        1 => arm_sat(xs),
        _ => arm_widen(xs),
    }
}

// --------------------------------------------------------------------------
// The three emitted symbols the asm comparison reads.
// --------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub fn direct_wrap(xs: &[u64]) -> u64 {
    arm_wrap(xs)
}

#[unsafe(no_mangle)]
pub fn pref_speed(xs: &[u64]) -> u64 {
    fold::<PrefSpeed>(xs)
}

#[unsafe(no_mangle)]
pub fn pref_accuracy(xs: &[u64]) -> u64 {
    fold::<PrefAccuracy>(xs)
}

#[unsafe(no_mangle)]
pub fn direct_widen(xs: &[u64]) -> u64 {
    arm_widen(xs)
}

fn main() {
    println!("P4. Preference resolution at const time");
    println!("=======================================");
    println!();
    println!("The cost model, resolved now at runtime for reporting only. The");
    println!("emitted code resolved it at compile time; see the asm comparison.");
    println!();
    println!(
        "  {:<16} {:>6} {:>6} {:>6}   ",
        "arm", "time", "space", "error"
    );
    for i in 0..ARMS {
        println!(
            "  {:<16} {:>6} {:>6} {:>6}",
            ARM_NAME[i], ARM_COST[i][0], ARM_COST[i][1], ARM_COST[i][2]
        );
    }
    println!();
    let prefs: Vec<(&str, [u32; AXES])> = std::vec![
        (PrefSpeed::NAME, PrefSpeed::W),
        (PrefSpace::NAME, PrefSpace::W),
        (PrefAccuracy::NAME, PrefAccuracy::W),
    ];
    for (n, w) in prefs {
        let picked = resolve(w);
        let scores: String = (0..ARMS)
            .map(|i| {
                let s: u32 = (0..AXES).map(|a| w[a] * ARM_COST[i][a]).sum();
                std::format!("{}={}", ARM_NAME[i], s)
            })
            .collect::<Vec<_>>()
            .join("  ");
        println!(
            "  {:<16} weights {:?} -> {:<9} [{}]",
            n, w, ARM_NAME[picked], scores
        );
    }
    println!();
    println!("Three preferences, three different arms, from one table. The");
    println!("preference is the whole content of the strategy at this layer, and");
    println!("the arm is derived rather than named.");
    println!();
    // Behavioural equality of the two that should be identical, on real data,
    // so the asm comparison is not comparing two functions that merely look
    // alike.
    let xs: Vec<u64> = (0..1000u64).map(|i| (i * 7919) % 4096).collect();
    let a = direct_wrap(&xs);
    let b = pref_speed(&xs);
    let c = direct_widen(&xs);
    let d = pref_accuracy(&xs);
    println!("  direct_wrap    = {a}");
    println!("  pref_speed     = {b}   same as direct_wrap: {}", a == b);
    println!("  direct_widen   = {c}");
    println!("  pref_accuracy  = {d}   same as direct_widen: {}", c == d);
}
