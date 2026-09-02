//! p5. A strategy parameter selecting a reduction member over one fixed identity,
//! and an algorithm bound refusing when the selected member loses the law.
//!
//! `68` section 7 names this as a claim the unit carries with **zero probe
//! instances**: "the strategy is a parameter of the correctness relation" (I9,
//! `65` candidate 3, `66`'s identity result, unit two's C4). Its diagnosis is
//! exact: `65`'s probe keys its two impls by ROLE, `66`'s strategy selects the
//! ENCODING, and neither instantiates a strategy selecting a reduction member.
//! It calls the missing probe "the cheapest constructive item the unit's second
//! half could build". This is that probe.
//!
//! The shape it asks for, verbatim: "one probe where a strategy parameter
//! selects among reduction members over one fixed (D, Q), refusing an algorithm
//! bound when the selected member loses the law."
//!
//! What this establishes:
//!
//!   S1. A strategy is expressible as a selector over the derived reduction
//!       space, parameterised by the identity rather than tied to one, so the
//!       same strategy names a member at every identity. Gate-free.
//!   S2. Over ONE fixed identity, one strategy is accepted by a law-bounded
//!       algorithm and another is refused. The refusal is the E0277 in
//!       `p5_neg.stderr`.
//!   S3. The refusal moves with the identity and not only with the strategy:
//!       the same strategy that is refused at the signed window is accepted at
//!       the unsigned one, so the law is a fact about the pair rather than
//!       about either coordinate.
//!
//! S3 is the part `68`'s request did not ask for and is what makes the
//! instantiation an argument rather than a demonstration: if the law tracked the
//! strategy alone, a canon could attach it to the strategy; if it tracked the
//! identity alone, to the format. It tracks neither.
//!
//! The law rows implemented here are `p4`'s measured verdicts, not assumptions:
//! saturating addition over a nonnegative window is associative (0 violations
//! over Q^3), and over a sign-crossing window it is not (952 violations at this
//! width). Wrapping is associative in both.
//!
//! Build:
//!   rustc --edition 2024 --crate-type lib p5_strategy_selects_the_member.rs
//!   rustc --edition 2024 --crate-type lib p5_neg.rs        # refused, E0277
//!
//! SPIKE DISCIPLINE: names, arities and trait shapes are scaffolding for these
//! three checks. None of it is a design proposal, and in particular nothing here
//! says a strategy should be spelled as a trait with an associated type.

#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;

// The identity, unchanged in substance from p1: an ambient domain and a
// representable set, both constants of the type.

pub trait Ambient {}
pub struct RingZ;
impl Ambient for RingZ {}

pub trait Reach {
    const LO: i64;
    const HI: i64;
}
pub struct S4;
impl Reach for S4 {
    const LO: i64 = -8;
    const HI: i64 = 7;
}
pub struct U4;
impl Reach for U4 {
    const LO: i64 = 0;
    const HI: i64 = 15;
}

pub trait Identity {
    type D: Ambient;
    type Q: Reach;
}
pub struct Id<D, Q>(PhantomData<(D, Q)>);
impl<D: Ambient, Q: Reach> Identity for Id<D, Q> {
    type D = D;
    type Q = Q;
}

pub type SignedWindow = Id<RingZ, S4>;
pub type UnsignedWindow = Id<RingZ, U4>;

// The derived reduction space: members total on the representable set. Declared
// over an identity, so the space a strategy chooses from is a function of the
// identity, which is the "derived, not chosen" content of the panel's C4.

pub trait Reduction {
    type At: Identity;
    const MEMBER: &'static str;
}
pub struct Wrap<I>(PhantomData<I>);
impl<I: Identity> Reduction for Wrap<I> {
    type At = I;
    const MEMBER: &'static str = "wrap";
}
pub struct Saturate<I>(PhantomData<I>);
impl<I: Identity> Reduction for Saturate<I> {
    type At = I;
    const MEMBER: &'static str = "saturate";
}

// S1. The strategy: a selector over that space, parameterised by the identity so
// that one strategy names a member at every identity rather than at one.

pub trait Strategy<I: Identity> {
    type Selected: Reduction<At = I>;
    const STRATEGY: &'static str;
}

/// Performance-weighted: takes the wrapping member.
pub struct Fast;
impl<I: Identity> Strategy<I> for Fast {
    type Selected = Wrap<I>;
    const STRATEGY: &'static str = "fast";
}

/// Intuition-weighted: takes the saturating member.
pub struct Guarded;
impl<I: Identity> Strategy<I> for Guarded {
    type Selected = Saturate<I>;
    const STRATEGY: &'static str = "guarded";
}

// The law layer. Rows are p4's measured verdicts over Q^3 at this width:
//   wrap, either window          -> 0 associativity violations
//   saturate, nonnegative window -> 0 violations
//   saturate, sign-crossing      -> 952 violations
// so there is an impl for the first two shapes and none for the third.

pub trait AddAssociates {}
impl<I: Identity> AddAssociates for Wrap<I> {}
impl<D: Ambient> AddAssociates for Saturate<Id<D, U4>> {}

/// An algorithm that reassociates, so it demands the law of whatever member the
/// strategy selected. The bound is on the SELECTION, which is the whole point.
pub fn reassociating_fold<I, S>() -> &'static str
where
    I: Identity,
    S: Strategy<I>,
    <S as Strategy<I>>::Selected: AddAssociates,
{
    <<S as Strategy<I>>::Selected as Reduction>::MEMBER
}

/// A left fold, demanding nothing, so a refusal above is about the law rather
/// than about the term being ill-formed.
pub fn sequential_fold<I, S>() -> &'static str
where
    I: Identity,
    S: Strategy<I>,
{
    <S as Strategy<I>>::STRATEGY
}

// S2 and S3, as accepted instances. The refused one is in p5_neg.rs.
pub fn accepted_cases() {
    // Fast at the signed window: wrap associates, accepted.
    let _ = reassociating_fold::<SignedWindow, Fast>();
    // Fast at the unsigned window: accepted.
    let _ = reassociating_fold::<UnsignedWindow, Fast>();
    // Guarded at the UNSIGNED window: saturation associates there, accepted.
    let _ = reassociating_fold::<UnsignedWindow, Guarded>();

    // Guarded at the SIGNED window is refused (p5_neg.rs), and the same
    // instantiation is fine for a fold that demands no law:
    let _ = sequential_fold::<SignedWindow, Guarded>();
}

// S3 stated as the pair of facts that makes the point:
//   Guarded is accepted at UnsignedWindow and refused at SignedWindow.
//   At SignedWindow, Fast is accepted and Guarded is refused.
// So the law is a fact about (identity, strategy), and attaching it to either
// coordinate alone would be wrong in one of these four cells.
