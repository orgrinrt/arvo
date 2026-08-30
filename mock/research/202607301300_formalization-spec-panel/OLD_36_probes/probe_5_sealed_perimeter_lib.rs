//! Probe 5 (the library half): the perimeter is not closed until the trait
//! is sealed, and this is the crate that seals it.
//!
//! Probe 2 establishes that every `Pos` inhabitant denotes exactly one
//! positive integer, by induction over the three constructors. That induction
//! has a hypothesis nothing has checked: that those three are the only
//! impls. A downstream crate adding `impl Pos for MyThing { const VAL = 6; }`
//! reinstates exactly the defect probe 1 measures, one crate away and
//! invisible to arvo, with `MyThing` and `O<I<H>>` two `Pos` types denoting
//! six.
//!
//! So `Pos` and `Nat` carry a private supertrait, which is the ordinary
//! sealing pattern and costs nothing at either compile or run time. Probe 5b
//! is the downstream crate that tries anyway and is refused.
//!
//! This is the part of the obligation that no formula can supply. A
//! self-normalising formula guarantees that the numbers arvo computes are
//! canonical; only a closed perimeter guarantees that the numbers arvo is
//! handed are.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_5_sealed_perimeter_lib.rs --out-dir <dir>
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]
#![crate_name = "vu_sealed"]

use core::marker::PhantomData;

mod sealed {
    pub trait PosSealed {}
    pub trait NatSealed {}
}

/// A positive integer, value-unique by construction and sealed so that
/// stays true from outside this crate.
pub trait Pos: sealed::PosSealed {
    const VAL: u64;
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

impl sealed::PosSealed for H {}
impl<P: Pos> sealed::PosSealed for O<P> {}
impl<P: Pos> sealed::PosSealed for I<P> {}

impl Pos for H {
    const VAL: u64 = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub trait Nat: sealed::NatSealed {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl sealed::NatSealed for Z {}
impl<P: Pos> sealed::NatSealed for Pz<P> {}

impl Nat for Z {
    const VAL: u64 = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

/// A width position, as every generic one in the design is spelled.
pub fn takes_a_width<N: Nat>() -> u64 {
    N::VAL
}

pub type P6 = O<I<H>>;
const _: () = assert!(<P6 as Pos>::VAL == 6);
const _: () = assert!(<Pz<P6> as Nat>::VAL == 6);
