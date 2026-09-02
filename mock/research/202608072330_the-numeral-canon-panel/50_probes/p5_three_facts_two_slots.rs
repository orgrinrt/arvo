// p5: 48:227-229 asks for 47 section 4's result restated without 47's modelling choice, and for
// someone who did not write 47 to check whether it holds.
//
// 47_probes/p5's F3 arm sets Precise's flat carrier equal to Warm's, with the comment "Precise
// stores exactly as Warm does; only its COMPUTE type differs", and reports that the pair does not
// separate them. 48 objects that 16:148-150 uses "carrier" for the COMPUTE type throughout, so
// 47 picked one of two available assignments and reported its consequence as forced.
//
// The modelling-independent claim 48 proposes:
//
//   under the widening reading there are three distinct facts and one slot named "carrier", so
//   whichever of the two that slot denotes, the other is unrecoverable from the result.
//
// This file checks both assignments.
//
//   M2, carrier := the at-rest type.  The pair does not separate Warm from Precise. Established by
//                  an assertion that MUST NOT be refused, so its absence from the error list is the
//                  result. That is 47's arm, reproduced.
//   M1, carrier := the compute type.  The pair DOES separate them, which 47's model could not show.
//                  So half of 48's "either way" is wrong on information grounds.
//   M1 again.     But the at-rest type is then unreachable, and the reason is the kind boundary
//                  rather than information: the stride is a const and a type cannot be reached from
//                  it. That half is in p5b, which is expected to be refused.
//
// Net: 48's conclusion holds and its stated reason does not. The asymmetry is real, and what
// closes both sides is the same wall 47 section 2.3 named.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p5_three_facts_two_slots.rs -o bin/p5 && ./bin/p5
//
// No #![feature] gate.

#![no_std]
extern crate std;
use std::println;

pub struct Warm;
pub struct Precise;
pub struct Cold;

pub struct W13;

/// the sealed bridge that makes a type equality a compiled claim rather than a printed one.
pub trait SameType<T> {}
impl<T> SameType<T> for T {}
pub const fn assert_same<A: SameType<B>, B>() {}

// ------------------------------------------------------------------ the three facts, named apart

pub trait Facts<S> {
    /// what one value occupies at rest
    type AtRest: Copy;
    /// what an operation lowers to
    type Compute: Copy;
    /// how a run of them repeats
    const STRIDE: u32;
}

/// the widening reading: Precise stores what Warm stores and computes wider.
impl Facts<Warm> for W13 {
    type AtRest = u16;
    type Compute = u16;
    const STRIDE: u32 = 16;
}
impl Facts<Precise> for W13 {
    type AtRest = u16;
    type Compute = u32;
    const STRIDE: u32 = 16;
}
impl Facts<Cold> for W13 {
    type AtRest = u16;
    type Compute = u16;
    const STRIDE: u32 = 13;
}

// three facts, and the two strategies agree on exactly two of them
const _: () = assert_same::<<W13 as Facts<Warm>>::AtRest, <W13 as Facts<Precise>>::AtRest>();
const _: () = assert!(<W13 as Facts<Warm>>::STRIDE == <W13 as Facts<Precise>>::STRIDE);
// and disagree on the third, which p5b asserts they agree on and is refused for it.

// ------------------------------------------------------- M2: the slot denotes the at-rest type

pub trait PairAtRest<S> {
    type Carrier: Copy;
    const STRIDE: u32;
}
impl<S> PairAtRest<S> for W13
where
    W13: Facts<S>,
{
    type Carrier = <W13 as Facts<S>>::AtRest;
    const STRIDE: u32 = <W13 as Facts<S>>::STRIDE;
}

/// MUST NOT be refused. its absence from any error list is the finding: under M2 the pair is the
/// same pair for two strategies that behave differently. this reproduces 47:274-276.
const _: () =
    assert_same::<<W13 as PairAtRest<Warm>>::Carrier, <W13 as PairAtRest<Precise>>::Carrier>();
const _: () = assert!(<W13 as PairAtRest<Warm>>::STRIDE == <W13 as PairAtRest<Precise>>::STRIDE);

// ------------------------------------------------------- M1: the slot denotes the compute type

pub trait PairCompute<S> {
    type Carrier: Copy;
    const STRIDE: u32;
}
impl<S> PairCompute<S> for W13
where
    W13: Facts<S>,
{
    type Carrier = <W13 as Facts<S>>::Compute;
    const STRIDE: u32 = <W13 as Facts<S>>::STRIDE;
}

/// under M1 the two carriers are u16 and u32, so the pair DOES separate. p5b asserts they are the
/// same and is refused, which is the compiled half of this claim.
const _: () = assert_same::<<W13 as PairCompute<Warm>>::Carrier, u16>();
const _: () = assert_same::<<W13 as PairCompute<Precise>>::Carrier, u32>();

// and the at-rest type is not among M1's slots. the only route to it is from the stride, which is
// a const, and that route is p5b's second refusal.

fn main() {
    println!("three facts at W=13, under the Precise-widens reading:");
    println!("  strategy   at-rest   compute   stride");
    println!(
        "  Warm       u16       u16       {}",
        <W13 as Facts<Warm>>::STRIDE
    );
    println!(
        "  Precise    u16       u32       {}",
        <W13 as Facts<Precise>>::STRIDE
    );
    println!(
        "  Cold       u16       u16       {}",
        <W13 as Facts<Cold>>::STRIDE
    );
    println!();
    println!("M2  slot = at-rest type   (u16, 16) for both  -> DOES NOT SEPARATE");
    println!("    compiled as a must-not-refuse assertion. this is 47's arm, and it holds.");
    println!();
    println!("M1  slot = compute type   (u16, 16) against (u32, 16)  -> SEPARATES");
    println!("    so 48's 'either way you are short one fact' is not symmetric on information.");
    println!("    under M1 the pair separates, and what is missing is the at-rest TYPE, which is");
    println!("    unreachable from a stride const rather than absent from the information.");
    println!();
    println!("both halves close, and the same wall closes them: 47 section 2.3's kind boundary.");
    println!("see p5b_negctl_three_facts.rs for the two refusals.");
}
