//! Probe 1b: the committed refusal behind probe 1's CLAIM B.
//!
//! `upward_rank`'s interior-safety obligation is `Headroom >= Capacity - 1`.
//! `Capacity` ships the size as `const CAP: Cap`
//! (`mock/crates/arvo-tensor/src/capacity.rs:24`). `InteriorSafety<A>` takes a
//! `Pos` TYPE. This file is the two spellings a maintainer would reach for and
//! what each one does.
//!
//! EXPECTED: FAILS. Two errors, both recorded verbatim in OUTCOMES.md.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_1b_the_const_capacity_cannot_state_it.rs

#![allow(dead_code)]

use tower::nat::{Cmp, Eq3, Gt, Lt, Pos};

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

pub trait CapacityAsShipped {
    type Array<T>;
    const CAP: usize;
}

/// Spelling 1: compute the arity in the bound. This is the shape the design
/// already refused once, for the grade, before the spine rule moved it to a
/// type (`49:66-68`).
pub fn rank_shipped_shape_1<C: CapacityAsShipped, Hd: Pos>()
where
    Hd: InteriorSafety<{ C::CAP - 1 }>,
{
}

/// Spelling 2: name the associated const where a type is wanted.
pub fn rank_shipped_shape_2<C: CapacityAsShipped, Hd: Pos>()
where
    Hd: InteriorSafety<<C as CapacityAsShipped>::CAP>,
{
}
