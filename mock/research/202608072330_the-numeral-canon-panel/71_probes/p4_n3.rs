//! p4. Is the crossing contract expressible as a typestate obligation, and does the type
//! system settle the composite-order question p2 found?
//!
//! HYPOTHESIS, written before the compile.
//!
//! Two halves, and the second is the one worth the probe.
//!
//! 1. A crossing that DECLARES what it preserves can be refused at compile time when the
//!    target's selected reduction lacks the corresponding law. Expected: compiles for the
//!    cells p3 measured as holding, refused with E0277 for the cells p3 measured as failing.
//!
//! 2. PREDICTION: the type system does NOT settle p2's order question. Both routes through a
//!    two-coordinate crossing are well typed, and they compute different functions, so nothing
//!    in the typestate distinguishes them. If that holds, the ambiguity is semantic and only a
//!    canon sentence closes it. If it is refuted and one route fails to type, the typestate
//!    settles it and no canon sentence is needed.
//!
//! The law rows below are p3's MEASURED verdicts (`p3_output.txt`, regime 1), transcribed.
//! They are not assumptions and they are not this probe's findings. Do not cite the impls.
//!
//!   At<SatK,  U4>   monotone yes, coherent yes    (nonneg saturate, add and mul)
//!   At<SatK,  S4>   monotone yes, coherent no     (signed saturate, add and mul)
//!   At<WrapK, U4>   monotone no,  coherent yes
//!   At<WrapK, S4>   monotone no,  coherent yes
//!
//! Scaffolding note, per the probe discipline. The reduction is modelled as a KIND plus the
//! set it is taken at, rather than as one type, so that a set-step can hold the reduction
//! fixed. That is a device for reaching the check, not a proposal about how a design spells
//! reductions.
//!
//! Compile: rustc +nightly-2026-05-28 --edition 2024 --crate-type lib p4_crossing_contract.rs
//! Zero feature gates, no dyn, no TypeId, no alloc.

#![no_std]

use core::marker::PhantomData;

// ----------------------------------------------------------------- telescope coordinates

pub trait Ambient {}
pub struct RingZ;
impl Ambient for RingZ {}
pub struct GF2;
impl Ambient for GF2 {}

/// A representable set, which names the ambient domain it is a reach of. This is the
/// telescope's second component depending on its first.
pub trait Reach {
    type Of: Ambient;
}
pub struct U6;
pub struct U4;
pub struct S6;
pub struct S4;
pub struct B4;
impl Reach for U6 {
    type Of = RingZ;
}
impl Reach for U4 {
    type Of = RingZ;
}
impl Reach for S6 {
    type Of = RingZ;
}
impl Reach for S4 {
    type Of = RingZ;
}
impl Reach for B4 {
    type Of = GF2;
}

pub trait ReductionKind {}
pub struct WrapK;
pub struct SatK;
impl ReductionKind for WrapK {}
impl ReductionKind for SatK {}

pub trait Encoding {}
pub struct Twos;
pub struct ExcessK;
impl Encoding for Twos {}
impl Encoding for ExcessK {}

pub trait Container {}
pub struct Plain;
pub struct Packed;
impl Container for Plain {}
impl Container for Packed {}

/// A completed telescope term.
pub trait System {
    type Dom: Ambient;
    type Set: Reach<Of = Self::Dom>;
    type Red: ReductionKind;
    type Enc: Encoding;
    type Con: Container;
}

pub struct Sys<D, Q, R, E, C>(PhantomData<(D, Q, R, E, C)>);

impl<D, Q, R, E, C> System for Sys<D, Q, R, E, C>
where
    D: Ambient,
    Q: Reach<Of = D>,
    R: ReductionKind,
    E: Encoding,
    C: Container,
{
    type Dom = D;
    type Set = Q;
    type Red = R;
    type Enc = E;
    type Con = C;
}

// ------------------------------------------- the selected reduction, and its law families

/// The reduction kind `R` taken at the set `Q`. `67`'s `Reduce(D, Q)` as an indexed family:
/// the member is not a free choice, it is the kind applied at the identity.
pub struct At<R, Q>(PhantomData<(R, Q)>);

/// Adaptation-law half of `63` C4: what order transport consumes.
pub trait Monotone {}
/// Coherence half of `63` C4: what a reassociating fold, and a crossing, consume.
pub trait Coherent {}

impl Monotone for At<SatK, U4> {}
impl Coherent for At<SatK, U4> {}
impl Monotone for At<SatK, S4> {}
impl Coherent for At<WrapK, U4> {}
impl Coherent for At<WrapK, S4> {}

// ------------------------------------------------------------- what a crossing may declare

/// A crossing declares what it preserves. The declaration is the whole contract: a crossing
/// that declares nothing is admissible and promises nothing.
pub struct PreservesOrder;
pub struct PreservesOps;
pub struct PreservesNothing;

/// Witness that a single-coordinate step from `A` to `B` exists and preserves `P`.
pub struct Step<A, B, P>(PhantomData<(A, B, P)>);

// The four single-coordinate steps. Each is stated as a where-clause holding every other
// coordinate fixed, which is why nothing is chosen inside a step: the coordinate not moving
// is shared, so where a reduction is needed there is exactly one in scope.

/// Index 2: the representable set moves. This is the only lossy step, and it is the only one
/// whose witness carries a law obligation.
impl<A, B> Step<A, B, PreservesOrder>
where
    A: System,
    B: System<Dom = A::Dom, Red = A::Red, Enc = A::Enc, Con = A::Con>,
    At<B::Red, B::Set>: Monotone,
{
    pub const SET_STEP: Self = Step(PhantomData);
}

impl<A, B> Step<A, B, PreservesOps>
where
    A: System,
    B: System<Dom = A::Dom, Red = A::Red, Enc = A::Enc, Con = A::Con>,
    At<B::Red, B::Set>: Coherent,
{
    pub const SET_STEP: Self = Step(PhantomData);
}

/// Index 3: the selected reduction moves. Identity on values, so it preserves order and
/// patterns unconditionally, and it preserves no operation, which is why no `PreservesOps`
/// constructor exists for it. This is p1's index-3 row expressed as an absence.
pub struct RedStep<A, B>(PhantomData<(A, B)>);

impl<A, B> RedStep<A, B>
where
    A: System,
    B: System<Dom = A::Dom, Set = A::Set, Enc = A::Enc, Con = A::Con>,
{
    pub const NEW: Self = RedStep(PhantomData);
}

/// Index 4 and index 5: realisation moves. Identity on values, so operations are preserved
/// unconditionally, which is p1's measured 256/256 at both indices.
pub struct EncStep<A, B>(PhantomData<(A, B)>);
impl<A, B> EncStep<A, B>
where
    A: System,
    B: System<Dom = A::Dom, Set = A::Set, Red = A::Red, Con = A::Con>,
{
    pub const NEW: Self = EncStep(PhantomData);
}

pub struct ConStep<A, B>(PhantomData<(A, B)>);
impl<A, B> ConStep<A, B>
where
    A: System,
    B: System<Dom = A::Dom, Set = A::Set, Red = A::Red, Enc = A::Enc>,
{
    pub const NEW: Self = ConStep(PhantomData);
}

/// A composite names its intermediate. There is no way to write a composite that does not.
pub struct Via<A, M, B>(PhantomData<(A, M, B)>);

// --------------------------------------------------------------------------- named systems

pub type NonNegWide = Sys<RingZ, U6, SatK, Twos, Plain>;
pub type NonNegNarrowSat = Sys<RingZ, U4, SatK, Twos, Plain>;
pub type NonNegWideWrap = Sys<RingZ, U6, WrapK, Twos, Plain>;
pub type NonNegNarrowWrap = Sys<RingZ, U4, WrapK, Twos, Plain>;
pub type SignedWide = Sys<RingZ, S6, SatK, Twos, Plain>;
pub type SignedNarrowSat = Sys<RingZ, S4, SatK, Twos, Plain>;
pub type SignedNarrowExcess = Sys<RingZ, S4, SatK, ExcessK, Plain>;
pub type SignedNarrowPacked = Sys<RingZ, S4, SatK, Twos, Packed>;

// ------------------------------------------------------------------ positive cases: accept

/// Narrowing into nonneg saturation, declared order-preserving. p3: monotone yes.
pub const ORDER_OK: Step<NonNegWide, NonNegNarrowSat, PreservesOrder> =
    <Step<NonNegWide, NonNegNarrowSat, PreservesOrder>>::SET_STEP;

/// The same crossing declared operation-preserving. p3: nonneg saturation is coherent.
pub const OPS_OK: Step<NonNegWide, NonNegNarrowSat, PreservesOps> =
    <Step<NonNegWide, NonNegNarrowSat, PreservesOps>>::SET_STEP;

/// Narrowing into wrapping, declared operation-preserving. p3: wrap is coherent.
pub const OPS_OK_WRAP: Step<NonNegWideWrap, NonNegNarrowWrap, PreservesOps> =
    <Step<NonNegWideWrap, NonNegNarrowWrap, PreservesOps>>::SET_STEP;

/// Realisation steps carry no law obligation at all, which is the point of p1's two rows.
pub const ENC_OK: EncStep<SignedNarrowSat, SignedNarrowExcess> =
    <EncStep<SignedNarrowSat, SignedNarrowExcess>>::NEW;
pub const CON_OK: ConStep<SignedNarrowSat, SignedNarrowPacked> =
    <ConStep<SignedNarrowSat, SignedNarrowPacked>>::NEW;

// ------------------------------------------------------------------ negative cases: refuse
//
// Each is compiled separately by the driver with the line enabled. Committed transcripts sit
// beside this file. Enabling any one here is what produces the E0277 in the transcript.

// N1. Narrowing into wrapping, declared ORDER-preserving. p3: wrap is not monotone.
// pub const N1: Step<NonNegWideWrap, NonNegNarrowWrap, PreservesOrder> =
//     <Step<NonNegWideWrap, NonNegNarrowWrap, PreservesOrder>>::SET_STEP;

// N2. Narrowing into SIGNED saturation, declared OPERATION-preserving. p3: not coherent.
// pub const N2: Step<SignedWide, SignedNarrowSat, PreservesOps> =
//     <Step<SignedWide, SignedNarrowSat, PreservesOps>>::SET_STEP;

// N3. A direct crossing between two systems differing at TWO coordinates, the set and the
// reduction. No single step relates them, so the phrase has no type.
pub const N3: Step<NonNegWideWrap, NonNegNarrowSat, PreservesOrder> =
    <Step<NonNegWideWrap, NonNegNarrowSat, PreservesOrder>>::SET_STEP;

// ------------------------------------------- the composite, and what the type system cannot do

/// Route 1: narrow first under the source's reduction, then change the reduction.
/// The intermediate is (U4, WrapK): narrowing happens under wrap.
pub type Route1 = Via<NonNegWideWrap, NonNegNarrowWrap, NonNegNarrowSat>;

/// Route 2: change the reduction first, then narrow under the target's.
/// The intermediate is (U6, SatK): narrowing happens under saturate.
pub type Route2 = Via<NonNegWideWrap, NonNegWide, NonNegNarrowSat>;

/// Both routes are built from steps that exist. p2 measured the two functions and they agree
/// on 30 of 256 source values, so these two well-typed composites are different functions.
pub const ROUTE1_STEP_A: Step<NonNegWideWrap, NonNegNarrowWrap, PreservesOps> =
    <Step<NonNegWideWrap, NonNegNarrowWrap, PreservesOps>>::SET_STEP;
pub const ROUTE1_STEP_B: RedStep<NonNegNarrowWrap, NonNegNarrowSat> =
    <RedStep<NonNegNarrowWrap, NonNegNarrowSat>>::NEW;

pub const ROUTE2_STEP_A: RedStep<NonNegWideWrap, NonNegWide> =
    <RedStep<NonNegWideWrap, NonNegWide>>::NEW;
pub const ROUTE2_STEP_B: Step<NonNegWide, NonNegNarrowSat, PreservesOps> =
    <Step<NonNegWide, NonNegNarrowSat, PreservesOps>>::SET_STEP;

// ------------------------------------------------------------------------------- erasure

const fn is_zst<T>() -> bool {
    core::mem::size_of::<T>() == 0
}

const _: () = assert!(is_zst::<Step<NonNegWide, NonNegNarrowSat, PreservesOrder>>());
const _: () = assert!(is_zst::<RedStep<NonNegNarrowWrap, NonNegNarrowSat>>());
const _: () = assert!(is_zst::<EncStep<SignedNarrowSat, SignedNarrowExcess>>());
const _: () = assert!(is_zst::<ConStep<SignedNarrowSat, SignedNarrowPacked>>());
const _: () = assert!(is_zst::<Route1>());
const _: () = assert!(is_zst::<Route2>());

// The contract erases. The MAP does not necessarily: p1's index-4 row is 0 of 256 at the
// pattern level, so a re-encoding step is real work at runtime even though its witness is a
// zero-sized type. Those are two different claims and the assertions above are only the first.
