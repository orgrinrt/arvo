//! p3 MUTANT. Negative control for `p3_three_encodings.rs`.
//!
//! One entry of the committed winner table is changed, region 3 from arm 0 to
//! arm 3. That is exactly the defect class `p2` measures as G1 and G2: a
//! generator that read a coordinate in the wrong unit or the wrong order emits a
//! table that is still rationalisable, so `p2` shows the rationalisability check
//! is blind to it, and the compile-time differential in this file is not.
//!
//! It MUST FAIL TO COMPILE with E0080. A check nobody has seen fail is not a
//! check.
//!
//! Original header follows.
//!
//! p3. The three encodings of a strategy's selection, compiled side by side,
//! and the differential between two of them expressed as a compile-time
//! assertion.
//!
//! `93` names the fork in its own phase-two withdrawal (`93:960-968`): `94`'s W1
//! bakes the WINNER per region as an associated const, `93`'s P4 bakes the COST
//! TABLE and computes an argmin at const time, and "neither is obviously right
//! and the register does not carry the fork". `98` section 3.1 cites `93`'s P4
//! as evidence that both sides of that fork compile.
//!
//! `93`'s P4 does not establish the half it is cited for. Its `ARM_COST` is
//! `[[u32; AXES]; ARMS]`, a cost per arm with NO REGION DIMENSION
//! (`93_probes/p4_preference_erases.rs:47`), so its const argmin runs once over
//! three arms and is a constant fold with nothing region-shaped in it. The
//! encoding the fork is actually about indexes cost by region, and the region is
//! a const generic parameter, so the argmin runs per monomorphisation. That is
//! what this file compiles.
//!
//! FOUR ENTRY POINTS, and the differences between them are the result:
//!
//!   e1_named        the winner table, indexed by region. What a generator emits
//!                   and what a person writing a table by hand produces. The
//!                   strategy is a NAME.
//!   e2_weighted     the cost table plus a const argmin over a stated weighting.
//!                   The strategy is a WEIGHTING.
//!   e3_direct       the arm, called directly, with no strategy machinery at all.
//!                   The erasure baseline.
//!   e4_consumer     a weighting NOBODY TABULATED, supplied the way a consumer
//!                   would supply one, resolving to an arm the named strategies
//!                   do not pick here. Only e2's encoding can express it, and
//!                   this is the whole consumer-visible difference between the
//!                   two sides of the fork.
//!
//! WHAT IS ASSERTED AT COMPILE TIME, and this is the part offered to the unit:
//!
//! `AGREEMENT` asserts, for every region, that the committed winner table equals
//! the argmin of the stated weighting over the committed cost table. That is
//! `97`'s decider and `98`'s generator in one artifact: the table is generated so
//! rationalisability is true by construction, AND the generated table is checked
//! against the weighting that generated it, so a generator that computed the
//! wrong argmin cannot ship. It costs one const evaluation and nothing at
//! runtime, and it refuses at build time rather than reporting, which is the
//! shape I15 asks for.
//!
//! `p3_mutant_generator_bug.rs` is the negative control: one entry of the winner
//! table changed, which is exactly the G1/G2 defect class `p2` measures, and it
//! must fail to compile.
//!
//! Constraints held: `#![no_std]`, ZERO feature gates, no `dyn`, no `TypeId`, no
//! `generic_const_exprs`. Sizes const. The inline `const { }` block forces const
//! evaluation rather than relying on the backend to fold a const fn call, so the
//! claim is about const solving and not about LLVM being clever.
//!
//! Build:
//!   rustc --edition 2024 -O -C panic=abort --emit asm -o p3_three_encodings.s \
//!         p3_three_encodings.rs

#![no_std]
#![crate_type = "lib"]

pub const R: usize = 6; // regions
pub const A: usize = 5; // arms
pub const D: usize = 2; // cost coordinates

/// The cost table, region-indexed. Coordinate 0 is time-shaped and coordinate 1
/// is bits per element.
///
/// The numbers are scaffolding and are not measured. Their SHAPE is taken from
/// the committed carrier run so the file is not exercising a degenerate table:
/// five arms at 16, 32, 64, 13 and 13 bits, a packed pair tied on footprint and
/// separated by time, the packed-simd arm winning at the smallest region and the
/// dense 16-bit arm winning from there up, and two arms dominated in every
/// region. `p1` and `97` section 10 both report that last property of the real
/// table; a model without it could not exercise `p2b`'s zero-weight hazard.
pub const COST: [[[u32; D]; A]; R] = [
    [[900, 16], [1000, 32], [1200, 64], [950, 13], [850, 13]],
    [[800, 16], [950, 32], [1150, 64], [980, 13], [870, 13]],
    [[780, 16], [940, 32], [1140, 64], [990, 13], [880, 13]],
    [[760, 16], [930, 32], [1130, 64], [1000, 13], [890, 13]],
    [[740, 16], [920, 32], [1120, 64], [1010, 13], [900, 13]],
    [[720, 16], [910, 32], [1110, 64], [1020, 13], [910, 13]],
];

/// The argmin. One const fn, shared by the generator and by the check, so the
/// two cannot disagree about what a weighting means.
pub const fn resolve(w: [u32; D], r: usize) -> usize {
    let mut best = 0usize;
    let mut best_score = u64::MAX;
    let mut a = 0usize;
    while a < A {
        let mut s = 0u64;
        let mut k = 0usize;
        while k < D {
            s += (w[k] as u64) * (COST[r][a][k] as u64);
            k += 1;
        }
        if s < best_score {
            best_score = s;
            best = a;
        }
        a += 1;
    }
    best
}

// ---------------------------------------------------------------------------
// E1. The strategy is a NAME, and the winner table is what reaches the compiler.
// ---------------------------------------------------------------------------

/// What a generator emits, or what a person reads off a findings file. Written
/// out as literals rather than computed from `resolve`, because a table computed
/// from the thing it is checked against is a tautology and the assertion below
/// would be incapable of failing.
pub const WINNER_SPEED: [usize; R] = [4, 0, 0, 3, 0, 0]; // MUTANT: region 3 changed from 0 to 3, the shape of a generator that read the wrong column

pub trait NamedStrategy {
    const PICK: [usize; R];
}

pub struct SpeedNamed;
impl NamedStrategy for SpeedNamed {
    const PICK: [usize; R] = WINNER_SPEED;
}

// ---------------------------------------------------------------------------
// E2. The strategy is a WEIGHTING, and the cost table is what reaches the
// compiler. A consumer can write an impl of this trait; they cannot write a row
// of somebody else's winner table.
// ---------------------------------------------------------------------------

pub trait Weighting {
    const W: [u32; D];
}

pub struct SpeedWeighted;
impl Weighting for SpeedWeighted {
    const W: [u32; D] = [32, 1];
}

/// A weighting nobody tabulated, standing in for one a consumer brings. It
/// weighs footprint heavily enough to take the packed arm at a region where the
/// named speed strategy takes the dense one.
pub struct ConsumerFootprintFirst;
impl Weighting for ConsumerFootprintFirst {
    const W: [u32; D] = [1, 64];
}

// ---------------------------------------------------------------------------
// THE DIFFERENTIAL, at compile time. The generated table must be the argmin of
// the weighting that generated it, at every region.
// ---------------------------------------------------------------------------

const AGREEMENT: () = {
    let mut r = 0usize;
    while r < R {
        assert!(
            WINNER_SPEED[r] == resolve(SpeedWeighted::W, r),
            "the committed winner table disagrees with the argmin of the \
             weighting that is supposed to have generated it"
        );
        r += 1;
    }
};
const _: () = AGREEMENT;

// ---------------------------------------------------------------------------
// The shared arm set. Written once; every encoding selects from it.
// ---------------------------------------------------------------------------

#[inline(never)]
pub fn arm0(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i < xs.len() {
        acc = acc.wrapping_add(xs[i]);
        i += 1;
    }
    acc
}

#[inline(never)]
pub fn arm1(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i < xs.len() {
        acc = acc.wrapping_add(xs[i] & 0xffff);
        i += 1;
    }
    acc
}

#[inline(never)]
pub fn arm2(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i < xs.len() {
        acc ^= xs[i];
        i += 1;
    }
    acc
}

#[inline(never)]
pub fn arm3(xs: &[u64]) -> u64 {
    let mut acc = 1u64;
    let mut i = 0;
    while i < xs.len() {
        acc = acc.wrapping_mul(xs[i] | 1);
        i += 1;
    }
    acc
}

#[inline(never)]
pub fn arm4(xs: &[u64]) -> u64 {
    let mut acc = 0u64;
    let mut i = 0;
    while i < xs.len() {
        acc = acc.wrapping_add(xs[i].rotate_left(7));
        i += 1;
    }
    acc
}

#[inline(always)]
fn dispatch(arm: usize, xs: &[u64]) -> u64 {
    match arm {
        0 => arm0(xs),
        1 => arm1(xs),
        2 => arm2(xs),
        3 => arm3(xs),
        _ => arm4(xs),
    }
}

// ---------------------------------------------------------------------------
// The four entry points. `const { }` forces const evaluation of the selection,
// so nothing here depends on the backend folding a const fn call.
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn fold_named<S: NamedStrategy, const I: usize>(xs: &[u64]) -> u64 {
    dispatch(const { S::PICK[I] }, xs)
}

#[inline(always)]
pub fn fold_weighted<S: Weighting, const I: usize>(xs: &[u64]) -> u64 {
    dispatch(const { resolve(S::W, I) }, xs)
}

#[unsafe(no_mangle)]
pub fn e1_named(xs: &[u64]) -> u64 {
    fold_named::<SpeedNamed, 3>(xs)
}

#[unsafe(no_mangle)]
pub fn e2_weighted(xs: &[u64]) -> u64 {
    fold_weighted::<SpeedWeighted, 3>(xs)
}

#[unsafe(no_mangle)]
pub fn e3_direct(xs: &[u64]) -> u64 {
    arm4(xs)
}

#[unsafe(no_mangle)]
pub fn e4_consumer(xs: &[u64]) -> u64 {
    fold_weighted::<ConsumerFootprintFirst, 3>(xs)
}

/// The consumer's weighting must reach an arm the named strategy does not pick
/// at this region, or `e4_consumer` proves nothing about expressiveness and the
/// comparison against `e1_named` is vacuous.
const _: () = assert!(
    resolve(ConsumerFootprintFirst::W, 3) != WINNER_SPEED[3],
    "the consumer weighting resolves to the same arm as the named strategy, so \
     this file cannot distinguish the two encodings"
);
