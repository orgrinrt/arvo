//! Probe 1: what a consumer writes when they want one axis different from a
//! preset, and nothing else.
//!
//! The whole review has priced axes and never priced the divergence path. As
//! the spec stands (`202607301200_topic.the-formalization-spec.md:49-59`), a
//! `S` must implement both `Policy` and `Lowering`, five associated types
//! between them. A consumer who wants `Warm` with a different tie direction
//! has to declare a marker and restate all five, four of which they did not
//! want to touch. That is a copy of a preset, and a copy drifts when the
//! preset moves.
//!
//! Question: can a preset be a partially-applied generic whose defaults
//! project out of the preset it derives from, so that divergence is one
//! named override and the untouched members follow the parent automatically?
//! Concretely, does a generic parameter default of the form
//! `<Warm as Policy>::Quantisation` resolve?
//!
//! Nothing here is arvo's real vocabulary. It is a five-member stand-in with
//! the same shape: two contracts, one fused marker, one member a consumer
//! wants to change.

#![no_std]

// ---- the vocabulary, stand-in ----------------------------------------------

pub struct ToEven;
pub struct ToOdd;
pub struct TowardNegative;

pub struct Exact;
pub struct NarrowedToOperand;

pub struct Minimum;
pub struct Doubled;

pub struct Dense;
pub struct Bitpacked;

pub struct PerOperation;
pub struct InContainer;

pub trait Quantisation {
    const TAG: u32;
}
impl Quantisation for ToEven {
    const TAG: u32 = 1;
}
impl Quantisation for ToOdd {
    const TAG: u32 = 2;
}
impl Quantisation for TowardNegative {
    const TAG: u32 = 3;
}

pub trait Growth {
    const TAG: u32;
}
impl Growth for Exact {
    const TAG: u32 = 10;
}
impl Growth for NarrowedToOperand {
    const TAG: u32 = 20;
}

pub trait StoredWidth {
    const TAG: u32;
}
impl StoredWidth for Minimum {
    const TAG: u32 = 100;
}
impl StoredWidth for Doubled {
    const TAG: u32 = 200;
}

pub trait Widening {
    const TAG: u32;
}
impl Widening for PerOperation {
    const TAG: u32 = 1000;
}
impl Widening for InContainer {
    const TAG: u32 = 2000;
}

pub trait Layout {
    const TAG: u32;
}
impl Layout for Dense {
    const TAG: u32 = 10000;
}
impl Layout for Bitpacked {
    const TAG: u32 = 20000;
}

pub trait Policy {
    type Quantisation: Quantisation;
    type Growth: Growth;
}

pub trait Lowering {
    type StoredWidth: StoredWidth;
    type Widening: Widening;
    type Layout: Layout;
}

// ---- the presets, as they are written today --------------------------------

pub struct Warm;
impl Policy for Warm {
    type Quantisation = ToEven;
    type Growth = Exact;
}
impl Lowering for Warm {
    type StoredWidth = Doubled;
    type Widening = InContainer;
    type Layout = Dense;
}

/// The five-member fingerprint of a composition, so a divergence is
/// observable as a number rather than by inspection.
pub const fn fingerprint<S: Policy + Lowering>() -> u32 {
    <S as Policy>::Quantisation::TAG
        + <S as Policy>::Growth::TAG
        + <S as Lowering>::StoredWidth::TAG
        + <S as Lowering>::Widening::TAG
        + <S as Lowering>::Layout::TAG
}

const WARM: u32 = fingerprint::<Warm>();
const _: () = assert!(WARM == 1 + 10 + 200 + 2000 + 10000);

// ---- shape A: what a consumer writes today ---------------------------------
//
// Restate all five. Four of them are a copy of Warm's, maintained by hand.

pub struct WarmToOdd_ByCopy;
impl Policy for WarmToOdd_ByCopy {
    type Quantisation = ToOdd;
    type Growth = Exact;
}
impl Lowering for WarmToOdd_ByCopy {
    type StoredWidth = Doubled;
    type Widening = InContainer;
    type Layout = Dense;
}

const BY_COPY: u32 = fingerprint::<WarmToOdd_ByCopy>();
const _: () = assert!(BY_COPY == WARM - 1 + 2);

// ---- shape B: the preset as a partially-applied generic --------------------
//
// The question this probe exists for. Every member defaults to the parent
// preset's projection, so a consumer names only what they change.

pub struct Like<
    P,
    Q = <P as Policy>::Quantisation,
    G = <P as Policy>::Growth,
    SW = <P as Lowering>::StoredWidth,
    W = <P as Lowering>::Widening,
    L = <P as Lowering>::Layout,
>(core::marker::PhantomData<(P, Q, G, SW, W, L)>)
where
    P: Policy + Lowering;

impl<P, Q, G, SW, W, L> Policy for Like<P, Q, G, SW, W, L>
where
    P: Policy + Lowering,
    Q: Quantisation,
    G: Growth,
    SW: StoredWidth,
    W: Widening,
    L: Layout,
{
    type Quantisation = Q;
    type Growth = G;
}

impl<P, Q, G, SW, W, L> Lowering for Like<P, Q, G, SW, W, L>
where
    P: Policy + Lowering,
    Q: Quantisation,
    G: Growth,
    SW: StoredWidth,
    W: Widening,
    L: Layout,
{
    type StoredWidth = SW;
    type Widening = W;
    type Layout = L;
}

/// The whole point: one override, four inherited, nothing restated.
pub type WarmToOdd = Like<Warm, ToOdd>;

const BY_OVERRIDE: u32 = fingerprint::<WarmToOdd>();
const _: () = assert!(BY_OVERRIDE == BY_COPY);

/// Zero overrides reproduces the parent exactly.
pub type WarmAgain = Like<Warm>;
const _: () = assert!(fingerprint::<WarmAgain>() == WARM);

/// Two overrides, still nothing restated.
pub type WarmToOddBitpacked = Like<Warm, ToOdd, Exact, Doubled, InContainer, Bitpacked>;
const _: () = assert!(fingerprint::<WarmToOddBitpacked>() == WARM - 1 + 2 - 10000 + 20000);

// ---- the limit this shape has, recorded because it decides the ergonomics --
//
// Generic arguments are positional. Overriding `Layout` (position 5) means
// spelling positions 1 through 4 even though they are the parent's, which is
// what `WarmToOddBitpacked` above had to do. So the default-projection shape
// makes the FIRST divergence free and every later one positional. Whether
// that is good enough, or whether the members should be ordered by how often
// they are overridden, or whether a macro is owed, is argued in the
// deliverable. This probe establishes only that the projection defaults
// resolve at all, which was not obvious.
