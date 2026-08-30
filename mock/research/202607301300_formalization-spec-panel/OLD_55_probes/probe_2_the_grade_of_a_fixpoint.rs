//! Probe 2: what a grade means across an unbounded number of iterations.
//!
//! `arvo-spectral`'s `power_iteration` and `fiedler_vector` run until a
//! caller-supplied trip count is exhausted (`power.rs:38-40`, `fiedler.rs:54-58`,
//! both taking `iterations: USize`, a RUNTIME value). Nothing in the design says
//! what a published grade means when the number of composed steps is not known
//! until the program runs.
//!
//! The design has half the answer already and does not know it. Its droplist
//! kills the naive shape (`49:912-913`, "growing an accumulator's own type on
//! every iteration of a runtime-bounded loop: cannot work in principle"). This
//! probe establishes what survives that, which is more than the droplist
//! implies.
//!
//! CLAIM A. The published grade is trip-count independent, and the reason is
//! algebraic rather than fortunate: the grade lattice's join is IDEMPOTENT, so
//! `G join G join ... join G = G` for any number of terms at all, including a
//! number nobody knows. Checked over the whole four-point carrier, not sampled,
//! plus the two- and three-fold compositions that would catch an idempotence
//! that only held at width one.
//!
//! CLAIM B. Interior safety is NOT trip-count independent, and here the
//! droplist entry is exactly right: the arity of an unnormalised accumulating
//! iteration is `trip_count * step_arity`, the trip count is a runtime `USize`,
//! and there is no type to compare against. Compiled below as a refusal.
//!
//! CLAIM C. The gap between A and B is closed by a structural property of the
//! algorithm, not of the numeral: a step that RENORMALISES has a per-step
//! bounded arity, so its interior safety is a per-step obligation the capacity
//! already supplies (probe 1), and the trip count drops out. Both shipped
//! spectral routines renormalise every step (`power.rs:74-81`,
//! `fiedler.rs:130-142`), which is why they work today and why nobody noticed
//! the question.
//!
//! CLAIM D. "This step renormalises" is not derivable from any numeral, policy
//! or lowering. It is a fact the algorithm knows and the number cannot. The
//! design already has the vocabulary for exactly that: D16, safe impl when
//! derived, `unsafe impl` when asserted (`49:220-222`). This is the first
//! consumer-side asserted fact the review has found, as against the operand-side
//! ones D16 was written for.
//!
//! EXPECTED: this file COMPILES CLEAN. Probe 2b is CLAIM B's refusal.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   --extern grade_lib=libgrade_lib.rlib probe_2_the_grade_of_a_fixpoint.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use grade_lib::{
    BothTransferred, EventsTransferred, Faithful, Grade, Join, RefusalsTransferred, Same,
};

// ---------------------------------------------------------------------------
// CLAIM A: join is idempotent, over the whole carrier.
//
// `48_probes/probe_2` checked commutativity, associativity, identity and
// absorption over the whole matrix. It did not check idempotence, which is the
// one law a fixpoint needs and the only one this probe adds.
// ---------------------------------------------------------------------------

const fn same<A: Same<B>, B>() {}

type J<A, B> = <A as Join<B>>::Out;

macro_rules! idempotent {
    ($g:ident) => {
        // One step.
        const _: () = same::<J<$g, $g>, $g>();
        // Two steps: the shape an iteration actually builds.
        const _: () = same::<J<J<$g, $g>, $g>, $g>();
        // Three, in both associations, because a scheduler may group them
        // either way and the report must not depend on that.
        const _: () = same::<J<J<J<$g, $g>, $g>, $g>, $g>();
        const _: () = same::<J<J<$g, $g>, J<$g, $g>>, $g>();
    };
}

idempotent!(Faithful);
idempotent!(RefusalsTransferred);
idempotent!(EventsTransferred);
idempotent!(BothTransferred);

/// The seed's grade joined with the step's, unrolled to depth four, all four
/// starting points, all four step grades. Sixteen cells, whole matrix.
macro_rules! seed_then_steps {
    ($seed:ident, $step:ident) => {
        const _: () = same::<J<J<J<J<$seed, $step>, $step>, $step>, $step>, J<$seed, $step>>();
    };
}

macro_rules! all_steps {
    ($seed:ident) => {
        seed_then_steps!($seed, Faithful);
        seed_then_steps!($seed, RefusalsTransferred);
        seed_then_steps!($seed, EventsTransferred);
        seed_then_steps!($seed, BothTransferred);
    };
}

all_steps!(Faithful);
all_steps!(RefusalsTransferred);
all_steps!(EventsTransferred);
all_steps!(BothTransferred);

// ---------------------------------------------------------------------------
// The statement CLAIM A earns, as a signature.
//
// An iteration takes a seed of grade `S` and a step of grade `T` and runs it an
// unknown number of times. The result's grade is `S join T`, independent of the
// trip count, and the trip count stays an ordinary runtime argument.
// ---------------------------------------------------------------------------

pub struct Iterated<G: Grade>(PhantomData<G>);
pub struct Seeded<G: Grade>(PhantomData<G>);

/// The step, as a fact rather than a closure: `StepGrade` is what one
/// application of the body publishes, and it is a per-step property.
pub trait Step {
    type Publishes: Grade;
}

/// The design's answer for a fixpoint. Note what is NOT in the signature: the
/// trip count. It is a runtime value and stays one.
pub fn iterate<S, B>(_seed: Seeded<S>, _body: &B, _trips: usize) -> Iterated<J<S, B::Publishes>>
where
    S: Grade + Join<B::Publishes>,
    B: Step,
{
    Iterated(PhantomData)
}

// ---------------------------------------------------------------------------
// CLAIM C: the two kinds of step, spelled apart.
//
// The difference between them is not visible in any numeral. It is whether the
// step's output is bounded by its input, which is a property of the algorithm's
// own recurrence.
// ---------------------------------------------------------------------------

/// A step whose output range is bounded by its input range, so composing it
/// with itself does not grow the accumulator. Power iteration's normalise is
/// the shipped instance.
///
/// # Safety
///
/// D16 (`49:220-222`): derived facts are safe impls, asserted facts are unsafe.
/// No numeral can derive this, so every impl is an assertion by the algorithm's
/// author that the step's output lies in the numeral's own range whenever its
/// input does. Getting it wrong makes the interior-safety obligation below
/// vacuous, which is the same failure mode as a wrong `unsafe impl` anywhere in
/// the design.
pub unsafe trait Contractive: Step {}

/// Power iteration's body. Renormalises to the L2 unit at `power.rs:74-81`, so
/// the vector's range after a step is exactly the range before it.
pub struct NormalisedMatvec;
impl Step for NormalisedMatvec {
    // The per-step arity is the inner-product width, which is the capacity, so
    // whatever the step publishes at that arity it publishes at every trip.
    type Publishes = EventsTransferred;
}
// SAFETY: `power.rs:74-81` divides by the L2 norm on every step, so the
// output vector is a unit vector whatever the input was.
unsafe impl Contractive for NormalisedMatvec {}

/// The unnormalised sibling, which is what `power_iteration` would be with the
/// three lines at `power.rs:67-81` deleted. It has no `Contractive` impl, and
/// probe 2b is what happens when a caller wants a grade from it anyway.
pub struct RawMatvec;
impl Step for RawMatvec {
    type Publishes = EventsTransferred;
}

/// The design's fixpoint combinator, gated on the assertion. A caller gets a
/// trip-count-independent grade exactly when the step is contractive.
pub fn iterate_to_fixpoint<S, B>(
    seed: Seeded<S>,
    body: &B,
    trips: usize,
) -> Iterated<J<S, B::Publishes>>
where
    S: Grade + Join<B::Publishes>,
    B: Step + Contractive,
{
    iterate(seed, body, trips)
}

pub fn spectral_call(trips: usize) -> Iterated<EventsTransferred> {
    iterate_to_fixpoint(Seeded::<Faithful>(PhantomData), &NormalisedMatvec, trips)
}

/// And the whole point of the exercise: the grade a consumer reads off a
/// Fiedler vector does not depend on how long the solver ran, so it is a fact
/// about the program rather than about the input data.
pub fn same_grade_whatever_the_trips(
    a: usize,
    b: usize,
) -> (Iterated<EventsTransferred>, Iterated<EventsTransferred>) {
    (spectral_call(a), spectral_call(b))
}
