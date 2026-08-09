//! Probe 2b: the arity of a loop nobody counted, in three shapes.
//!
//! Probe 2 CLAIM B says an unnormalised accumulating iteration's arity is
//! `trip_count * step_arity` and the trip count is runtime. This file compiles
//! the three shapes that follow from that and reports which of them exist.
//!
//! SHAPE 1, multiply the counts as consts. FAILS, and the compiler names the
//! forbidden feature itself. Same wall as probe 1b, one level up.
//!
//! SHAPE 2, lift the trip count to a `Pos` type. COMPILES, which is the
//! finding: the door is not shut, it costs the trip count becoming compile-time
//! knowledge. For a convergence loop that is exactly the wrong price, because
//! "iterate until converged" means the count is a function of the DATA. The
//! signature is written out below so the price is visible rather than asserted.
//!
//! SHAPE 3, the honest one. A loop whose trip count is not knowable at all has
//! an UNBOUNDED arity, and the design's interior-safety comparison has no `Pos`
//! to compare against because `Pos` has no top. Two lines fix that: an
//! `Unbounded` arity marker, and one blanket saying every finite headroom is
//! `Unsafe` against it. The comparison becomes total, the fixpoint case gets a
//! grade rather than an error, and by probe 2's idempotence that grade is
//! stable.
//!
//! CLAIM. The resulting grade is at the TOP of the lattice and it is still
//! actionable, which is the thing worth checking, because a grade no consumer
//! can act on would be machinery serving nobody. Under refusing resolutions the
//! published grade is `RefusalsTransferred` and the remedy is "read the
//! refusal"; under wrapping resolutions it is `EventsTransferred` and the
//! remedy is "do not trust the magnitude". Both are compiled as consumer code
//! at the bottom of this file, and a `Definite`-style bound refuses the second
//! at the call site.
//!
//! EXPECTED: shape 1 is a committed refusal in its own file (probe 2c); shapes
//! 2 and 3 compile here.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   --extern grade_lib=libgrade_lib.rlib probe_2b_the_arity_of_an_unbounded_loop.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use grade_lib::{
    BothTransferred, EventsTransferred, Faithful, Grade, Join, RefusalsTransferred, WeakerThan,
};
use tower::nat::{Cmp, Eq3, Gt, Lt, Nat, Pos, Pz, H, I, O};

pub struct Safe;
pub struct Unsafe;
pub trait Safety {}
impl Safety for Safe {}
impl Safety for Unsafe {}

pub trait SafetyOf {
    type Out: Safety;
}
impl SafetyOf for Gt {
    type Out = Safe;
}
impl SafetyOf for Eq3 {
    type Out = Safe;
}
impl SafetyOf for Lt {
    type Out = Unsafe;
}

pub trait InteriorSafety<ArityMinusOne> {
    type Out: Safety;
}
impl<Hd: Pos + Cmp<A>, A: Pos> InteriorSafety<A> for Hd
where
    <Hd as Cmp<A>>::Out: SafetyOf,
{
    type Out = <<Hd as Cmp<A>>::Out as SafetyOf>::Out;
}

// ---------------------------------------------------------------------------
// SHAPE 3's two lines. `Unbounded` is not a `Pos`, so it cannot collide with
// the blanket above, and the two impls are disjoint by construction.
// ---------------------------------------------------------------------------

/// The arity of a loop whose trip count is a function of the data.
pub struct Unbounded;

impl<Hd: Pos> InteriorSafety<Unbounded> for Hd {
    type Out = Unsafe;
}

// ---------------------------------------------------------------------------
// SHAPE 2: the trip count as a type, and what it costs.
// ---------------------------------------------------------------------------

/// Multiplication on the tower, for the one product this needs. `Mul` is not in
/// the shipped tower, so this is the addition shape 2 would owe, written for
/// the two instances the call sites below use rather than in general, because
/// the point of this section is the SIGNATURE, not the arithmetic.
pub trait MulArity<Rhs> {
    type Out: Pos;
}

type P3 = I<H>; // 3
type P7 = I<I<H>>; // 7
type P21 = I<O<I<O<H>>>>; // 21
type P63 = I<I<I<I<I<H>>>>>; // 63

impl MulArity<P7> for P3 {
    type Out = P21;
}
impl MulArity<P21> for P3 {
    type Out = P63;
}

const _: () = assert!(<Pz<P3> as Nat>::VAL == 3);
const _: () = assert!(<Pz<P7> as Nat>::VAL == 7);
const _: () = assert!(<Pz<P21> as Nat>::VAL == 21);
const _: () = assert!(<Pz<P63> as Nat>::VAL == 63);

pub struct Iterated<G: Grade>(PhantomData<G>);

/// The price, in one signature. `Trips` is a type parameter, so a caller that
/// wants to run "until converged" cannot call this at all: it has to decide the
/// count before it has seen the data. That is not a limitation of the encoding,
/// it is what compile-time knowledge of a data-dependent quantity means.
pub fn iterate_counted<Hd, StepArity, Trips, Top, Bot, Dom>() -> Iterated<
    <(
        <Hd as InteriorSafety<<Trips as MulArity<StepArity>>::Out>>::Out,
        Top,
        Bot,
        Dom,
    ) as FoldGrade>::Out,
>
where
    Hd: Pos + InteriorSafety<<Trips as MulArity<StepArity>>::Out>,
    Trips: Pos + MulArity<StepArity>,
    StepArity: Pos,
    (
        <Hd as InteriorSafety<<Trips as MulArity<StepArity>>::Out>>::Out,
        Top,
        Bot,
        Dom,
    ): FoldGrade,
{
    Iterated(PhantomData)
}

// ---------------------------------------------------------------------------
// The grade table, the two arms this file reaches, from 47_probes/probe_3.
// ---------------------------------------------------------------------------

pub struct Refuse;
pub struct ReduceModulo;
pub struct Signed;

pub trait FoldGrade {
    type Out: Grade;
}
impl<Top, Bot, Dom> FoldGrade for (Safe, Top, Bot, Dom) {
    type Out = Faithful;
}
impl<Dom> FoldGrade for (Unsafe, Refuse, Refuse, Dom) {
    type Out = RefusalsTransferred;
}
impl<Dom> FoldGrade for (Unsafe, ReduceModulo, ReduceModulo, Dom) {
    type Out = EventsTransferred;
}

/// Three trips of a seven-wide step against sixty-three of headroom: exactly at
/// the boundary, so interior-safe, so `Faithful`. The caller knew the count.
pub fn counted_safe() -> Iterated<Faithful> {
    iterate_counted::<P63, P7, P3, Refuse, Refuse, Signed>()
}

// ---------------------------------------------------------------------------
// SHAPE 3: the fixpoint that iterates until it converges, which is every real
// eigenvector solver, and the grade it publishes.
// ---------------------------------------------------------------------------

pub fn iterate_until_converged<Hd, Top, Bot, Dom>(
    _trips_taken: usize,
) -> Iterated<<(<Hd as InteriorSafety<Unbounded>>::Out, Top, Bot, Dom) as FoldGrade>::Out>
where
    Hd: Pos + InteriorSafety<Unbounded>,
    (<Hd as InteriorSafety<Unbounded>>::Out, Top, Bot, Dom): FoldGrade,
{
    Iterated(PhantomData)
}

/// A `Precise` solver. The grade says definedness may differ from the
/// sequential answer, and the remedy is to read the refusal.
pub fn fiedler_precise(t: usize) -> Iterated<RefusalsTransferred> {
    iterate_until_converged::<P63, Refuse, Refuse, Signed>(t)
}

/// A `Hot` solver. The grade says the event multiset may differ, and the remedy
/// is not to trust the magnitude.
pub fn fiedler_hot(t: usize) -> Iterated<EventsTransferred> {
    iterate_until_converged::<P63, ReduceModulo, ReduceModulo, Signed>(t)
}

// ---------------------------------------------------------------------------
// Actionability: the two remedies, as consumer code that compiles, and the
// refusal that stops a caller who wanted neither.
// ---------------------------------------------------------------------------

/// `spectral_bisection` reads only the SIGN pattern of the Fiedler vector
/// (`fiedler.rs:24-26`, "Only the sign pattern of the result is meaningful").
/// A transferred event multiset does not move a sign, so this consumer is
/// correct at `EventsTransferred` and says so in its bound.
pub fn bisection_reads_signs<G: Grade + WeakerThan<EventsTransferred>>(_v: Iterated<G>) -> u8 {
    0
}

pub fn bisection_on_hot(t: usize) -> u8 {
    bisection_reads_signs(fiedler_hot(t))
}

/// A consumer that reads the MAGNITUDES (an algebraic-connectivity estimate,
/// say) is not correct at `EventsTransferred`, and its bound is what refuses.
pub fn needs_faithful(_v: Iterated<Faithful>) -> u8 {
    0
}

// UNCOMMENT to reproduce the refusal; it is probe 2c.
// pub fn magnitude_on_hot(t: usize) -> u8 {
//     needs_faithful(fiedler_hot(t))
// }

/// The grade is stable across the two solvers a consumer might swap between,
/// which is the property that makes it worth publishing at all.
pub fn stable(
    a: usize,
    b: usize,
) -> (Iterated<RefusalsTransferred>, Iterated<RefusalsTransferred>) {
    (fiedler_precise(a), fiedler_precise(b))
}

/// And the join with a seed still collapses, so a solver seeded from a previous
/// solver's output publishes the same grade as one seeded from scratch.
pub fn joined_seed() -> PhantomData<<RefusalsTransferred as Join<RefusalsTransferred>>::Out> {
    PhantomData
}

const _: () = {
    fn assert_top_is_reachable<A: Grade + WeakerThan<BothTransferred>>() {}
    let _ = assert_top_is_reachable::<EventsTransferred>;
    let _ = assert_top_is_reachable::<RefusalsTransferred>;
};
