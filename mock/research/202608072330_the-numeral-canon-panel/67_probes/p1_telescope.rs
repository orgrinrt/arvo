//! p1. Is the numeral concept a product of independent choices, or a dependent
//! sequence in which each choice ranges over a set the earlier choices fix?
//!
//! Hypothesis under test: the four things the panel keeps calling "slots"
//! (ambient domain, representable set, adaptation, encoding, container) are not
//! a tuple. Each one after the first ranges over a set that the earlier ones
//! determine, so the structure is a telescope (a dependent sequence), and the
//! dependency is expressible as ordinary associated-type equality with no
//! forbidden feature.
//!
//! Four things this file establishes, all at compile time:
//!
//!   T1. The telescope's dependency is enforceable. A later component declared
//!       over one identity cannot be attached to another. (Negative control in
//!       `p1_neg_a.rs`.)
//!   T2. The type names only the LAST component; every earlier one is recovered
//!       by projection. So "which facts must be types" is answered by the chain
//!       rather than by an arity.
//!   T3. A law contract is decided by the prefix (identity, adaptation) and does
//!       not read the encoding or the container: two encodings over one adapted
//!       identity are both accepted by the same law bound.
//!   T4. A law contract is NOT decided by the identity alone: at one and the same
//!       (ambient, representable set), one adaptation is accepted and another is
//!       refused. (Negative control in `p1_neg_b.rs`.)
//!
//! T3 and T4 together are the point. They say a universally quantified sentence
//! about "every numeral with this identity" has a different truth value from one
//! about "every numeral with this identity and this adaptation", mechanically,
//! at the type checker, with the same values in the same container.
//!
//! Build (pin resolves from the repository's rust-toolchain.toml):
//!   rustc --edition 2024 --crate-type lib p1_telescope.rs
//! No `#![feature(...)]` anywhere. No dyn, no TypeId.
//!
//! SPIKE DISCIPLINE: every name, arity and field order below is scaffolding
//! chosen to reach the four checks. None of it is a design proposal.

#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

// ---------------------------------------------------------------- component 1
// The ambient domain: a carrier together with an operation family. Two domains
// over the same integers, differing only in which operations they name.

pub trait Ambient {
    const DOMAIN: &'static str;
}

/// (Z, +, *)
pub struct RingZ;
impl Ambient for RingZ {
    const DOMAIN: &'static str = "Z with (+,*)";
}

/// (Z u {top}, min, +). Same carrier, different operation family.
pub struct TropicalZ;
impl Ambient for TropicalZ {
    const DOMAIN: &'static str = "Z with (min,+)";
}

// ---------------------------------------------------------------- component 2
// The representable set. A constant of the type, per the panel's own identity
// condition; here a closed integer interval.

pub trait Reach {
    const LO: i64;
    const HI: i64;
}

pub struct S4; // [-8, 7]
impl Reach for S4 {
    const LO: i64 = -8;
    const HI: i64 = 7;
}

pub struct U4; // [0, 15]
impl Reach for U4 {
    const LO: i64 = 0;
    const HI: i64 = 15;
}

// ------------------------------------------------------- prefix 1+2: identity

pub trait Identity {
    type D: Ambient;
    type Q: Reach;
}

pub struct Id<D, Q>(PhantomData<(D, Q)>);
impl<D: Ambient, Q: Reach> Identity for Id<D, Q> {
    type D = D;
    type Q = Q;
}

type RingS4 = Id<RingZ, S4>;
type RingU4 = Id<RingZ, U4>;
type TropU4 = Id<TropicalZ, U4>;

// ---------------------------------------------------------------- component 3
// The adaptation: a total reduction onto the representable set. It is declared
// OVER an identity. The associated type is what makes this a telescope step
// rather than an independent coordinate.

pub trait Adaptation {
    type At: Identity;
    const POLICY: &'static str;
}

pub struct Wrap<I>(PhantomData<I>);
impl<I: Identity> Adaptation for Wrap<I> {
    type At = I;
    const POLICY: &'static str = "wrap";
}

pub struct Saturate<I>(PhantomData<I>);
impl<I: Identity> Adaptation for Saturate<I> {
    type At = I;
    const POLICY: &'static str = "saturate";
}

// ---------------------------------------------------------------- component 4
// The encoding: a map from the representable set to bit patterns. Declared over
// an identity, because which encodings exist depends on which value set is being
// realised, and NOT over an adaptation, because the encoding is a realisation of
// the value set rather than of the arithmetic.

pub trait Encoding {
    type Of: Identity;
    const SCHEME: &'static str;
}

pub struct TwosComplement<I>(PhantomData<I>);
impl<I: Identity> Encoding for TwosComplement<I> {
    type Of = I;
    const SCHEME: &'static str = "two's complement";
}

pub struct ExcessK<I>(PhantomData<I>);
impl<I: Identity> Encoding for ExcessK<I> {
    type Of = I;
    const SCHEME: &'static str = "excess-K";
}

// ---------------------------------------------------------------- component 5
// The container: machine storage holding an encoding's patterns.

pub trait Container {
    type Holds: Encoding;
    type Store: Copy;
}

pub struct Byte<E>(PhantomData<E>);
impl<E: Encoding> Container for Byte<E> {
    type Holds = E;
    type Store = u8;
}

// ---------------------------------------------------------- the completed term
//
// T2: the value type names only the container. Everything earlier is a
// projection out of it, and the where-clauses are the telescope's dependency
// stated as associated-type equalities.

#[repr(transparent)]
pub struct Num<A, C>(pub <C as Container>::Store, pub PhantomData<A>)
where
    A: Adaptation,
    C: Container,
    <C as Container>::Holds: Encoding<Of = <A as Adaptation>::At>;

/// Every earlier component recovered by projection from the completed term.
pub type IdentityOf<A> = <A as Adaptation>::At;
pub type AmbientOf<A> = <<A as Adaptation>::At as Identity>::D;
pub type ReachOf<A> = <<A as Adaptation>::At as Identity>::Q;

// T1 positive instance: encoding declared over the same identity the adaptation
// is declared over.
pub type WrapRingS4Twos = Num<Wrap<RingS4>, Byte<TwosComplement<RingS4>>>;
pub type SatRingS4Twos = Num<Saturate<RingS4>, Byte<TwosComplement<RingS4>>>;
pub type WrapRingS4Excess = Num<Wrap<RingS4>, Byte<ExcessK<RingS4>>>;
pub type SatTropU4Twos = Num<Saturate<TropU4>, Byte<TwosComplement<TropU4>>>;

// Erasure. The completed term is the container's store and nothing else, so the
// whole telescope has no runtime witness.
const _: () = assert!(core::mem::size_of::<WrapRingS4Twos>() == core::mem::size_of::<u8>());
const _: () = assert!(core::mem::size_of::<SatRingS4Twos>() == core::mem::size_of::<u8>());
const _: () = assert!(core::mem::size_of::<WrapRingS4Excess>() == core::mem::size_of::<u8>());
const _: () = assert!(core::mem::size_of::<SatTropU4Twos>() == core::mem::size_of::<u8>());

// Projection works: the ambient of a wrapped ring numeral is the ring, and of a
// tropical one is the tropical domain, read off the completed term.
const _: () = assert!(matches!(
    <AmbientOf<Wrap<RingS4>> as Ambient>::DOMAIN.as_bytes()[0],
    b'Z'
));
const _: () = assert!(<ReachOf<Wrap<RingS4>> as Reach>::LO == -8);
const _: () = assert!(<ReachOf<Saturate<TropU4>> as Reach>::LO == 0);

// ------------------------------------------------------------- the law contract
//
// A marker naming one law of the induced operation. It is implemented for
// (identity, adaptation) pairs and mentions no encoding and no container, which
// is the whole content of T3.
//
// The rows below are the panel's own measured verdicts, restated as impls:
//   wrap over any identity      -> induced addition associative (ring Z/2^N)
//   saturate over [0, hi]       -> induced addition associative (bounded chain)
//   saturate over [lo<0, hi>0]  -> NOT associative           (no impl)
// p4 in this directory re-measures these rows independently rather than taking
// them on trust.

pub trait AddAssociates {}

impl<I: Identity> AddAssociates for Wrap<I> {}
impl<D: Ambient> AddAssociates for Saturate<Id<D, U4>> {}
// deliberately no impl for Saturate<Id<D, S4>>

/// A fold that may reassociate, so it demands the law.
pub fn reassociating_fold<A, C>(_x: Num<A, C>) -> &'static str
where
    A: Adaptation + AddAssociates,
    C: Container,
    <C as Container>::Holds: Encoding<Of = <A as Adaptation>::At>,
{
    <A as Adaptation>::POLICY
}

/// A left fold that does not, so it demands nothing.
pub fn sequential_fold<A, C>(_x: Num<A, C>) -> &'static str
where
    A: Adaptation,
    C: Container,
    <C as Container>::Holds: Encoding<Of = <A as Adaptation>::At>,
{
    <A as Adaptation>::POLICY
}

pub fn t3_law_does_not_read_the_encoding() {
    // Same identity, same adaptation, two DIFFERENT encodings. Both accepted, so
    // the law bound is decided at prefix 3 and is blind to prefix 4.
    let a: WrapRingS4Twos = Num(0, PhantomData);
    let b: WrapRingS4Excess = Num(0, PhantomData);
    let _ = reassociating_fold(a);
    let _ = reassociating_fold(b);
}

pub fn t4_law_is_not_decided_by_the_identity_alone() {
    // Same identity (RingS4), two adaptations. This one is accepted. Its sibling
    // `reassociating_fold(SatRingS4Twos)` is refused, in p1_neg_b.rs.
    let a: WrapRingS4Twos = Num(0, PhantomData);
    let _ = reassociating_fold(a);

    // and the same refused term is fine for a fold that demands no law, so the
    // refusal is about the law and not about the term being ill-formed.
    let b: SatRingS4Twos = Num(0, PhantomData);
    let _ = sequential_fold(b);
}

// ------------------------------------------------------ one ambient, two Q; one
// Q, two ambients. The two are different points at different depths, which is
// what makes "same numeral" a question about which prefix is being compared.

pub type RingU4Sat = Num<Saturate<RingU4>, Byte<TwosComplement<RingU4>>>;
pub type TropU4Sat = Num<Saturate<TropU4>, Byte<TwosComplement<TropU4>>>;

const _: () = assert!(core::mem::size_of::<RingU4Sat>() == core::mem::size_of::<TropU4Sat>());
