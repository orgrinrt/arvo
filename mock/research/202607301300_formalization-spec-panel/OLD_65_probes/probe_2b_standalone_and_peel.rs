//! Probe 2b. The two remaining const-to-type routes.
//!
//! Route A: `ToNat<I, F>` with the const params passed as standalone
//! arguments (legal grammar, no const operation), the addition done inside
//! the impl. Needs the impl body to produce a TYPE from a CONST.
//! Route B: recursive peel, `ToNat<N>::Out = Pz<ToNat<{N-1}>::Out>`.
#![no_std]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct IBits(pub u16);

pub struct Z;
pub struct Pz<P>(core::marker::PhantomData<P>);
pub trait Nat {}
impl Nat for Z {}
impl<P: Nat> Nat for Pz<P> {}

// Route B: recursive peel over a plain u16 const.
pub trait Peel<const N: u16> {
    type Out: Nat;
}
pub struct Lift;
impl Peel<0> for Lift {
    type Out = Z;
}
impl<const N: u16> Peel<N> for Lift {
    type Out = Pz<<Lift as Peel<{ N - 1 }>>::Out>;
}
