//! Probe 2. Can the width become a type while `I` and `F` stay const params?
//!
//! This is the question that decides the migration's blast radius. If a
//! `const I: IBits` can be mapped to a type-level `Nat` by any mechanism the
//! forbidden-feature list permits, the change is contained inside `Bits` /
//! `UFixed` / `IFixed` and every call site keeps its current spelling. If it
//! cannot, `I` and `F` themselves become types and every call site that names
//! a width changes.
//!
//! Route tried: a trait keyed on the const param projecting to a type
//! (`ToNat<{ ufixed_bits(I, F) }>::Out`), which is the only shape that could
//! carry a computed const into type position without an unbounded impl table.
#![no_std]
#![feature(adt_const_params)]
#![allow(incomplete_features)]
use core::marker::ConstParamTy;

#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct IBits(pub u16);
#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct FBits(pub u16);

pub const fn ufixed_bits(i: IBits, f: FBits) -> u16 {
    i.0 + f.0
}

// A type-level Nat, peano-shaped (what the numeral tower's `Nat` is).
pub struct Z;
pub struct Pz<P>(core::marker::PhantomData<P>);
pub trait Nat {}
impl Nat for Z {}
impl<P: Nat> Nat for Pz<P> {}

// The escape: a const-keyed projection to a type.
pub trait ToNat<const N: u16> {
    type Out: Nat;
}
pub struct Lift;

pub trait Container<W: Nat> {
    type T: Copy;
}
pub struct Hot;
impl<W: Nat> Container<W> for Hot {
    type T = u64;
}

// The width is now a TYPE, but reaching it still needs the computed const
// in argument position.
#[repr(transparent)]
pub struct UFixed<const I: IBits, const F: FBits, S>(
    <S as Container<<Lift as ToNat<{ ufixed_bits(I, F) }>>::Out>>::T,
)
where
    Lift: ToNat<{ ufixed_bits(I, F) }>,
    S: Container<<Lift as ToNat<{ ufixed_bits(I, F) }>>::Out>;
