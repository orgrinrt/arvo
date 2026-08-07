#![no_std]
#![allow(dead_code)]
extern crate base;
use base::*;
pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Nat: Nat;
}
impl AdmittedWidth for Idx<3> {
    type Nat = Pz<I<H>>;
}
impl AdmittedWidth for Idx<13> {
    type Nat = Pz<I<O<I<H>>>>;
}
pub trait NatAdd<Rhs> {
    type Out: Nat;
}
impl NatAdd<Pz<I<H>>> for Pz<I<O<I<H>>>> {
    type Out = Pz<O<O<O<O<H>>>>>;
}

// two const params, named I and F. `I` also names a struct in base (I<P: Pos>).
pub type PrecisionOfA<const INT: u16, const FRAC: u16> =
    <<Idx<INT> as AdmittedWidth>::Nat as NatAdd<<Idx<FRAC> as AdmittedWidth>::Nat>>::Out;
