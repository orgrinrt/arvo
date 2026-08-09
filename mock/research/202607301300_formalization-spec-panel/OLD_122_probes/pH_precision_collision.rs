// Probe H. The Precision collision, both spellings, in the crate the canon
// would declare them in.
// Spelling (b): the marker trait renamed, `Precision` becomes the type
// constructor. Modelled here by declaring BOTH under the one token, which is
// what the document currently says (110:3176 trait, 110:3326 type).
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
extern crate base;
use base::*;

// The bridge table, three rows, enough to carry 13 + 3 = 16.
pub struct Width<const N: u16>;
pub trait AdmittedWidth {
    type Nat: Nat;
}
impl AdmittedWidth for Width<3> {
    type Nat = Pz<I<H>>;
} // 3
impl AdmittedWidth for Width<13> {
    type Nat = Pz<I<O<I<H>>>>;
} // 13
impl AdmittedWidth for Width<16> {
    type Nat = Pz<O<O<O<O<H>>>>>;
} // 16

// The tower's own addition, stubbed at the one instance the assertion needs.
pub trait NatAdd<Rhs> {
    type Out: Nat;
}
impl NatAdd<Pz<I<H>>> for Pz<I<O<I<H>>>> {
    type Out = Pz<O<O<O<O<H>>>>>;
}

// The document declares this at 110:3176.
pub trait Precision: Nat {}
impl<T: Nat> Precision for T {}

// And the bridge paragraph at 110:3326 spells this. Same token, same namespace.
pub type Precision<const I: u16, const F: u16> =
    <<Width<I> as AdmittedWidth>::Nat as NatAdd<<Width<F> as AdmittedWidth>::Nat>>::Out;
