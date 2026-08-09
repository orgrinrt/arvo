// Probe K. Question two's chosen spelling: the marker trait keeps `Precision`
// (110:3176, the 74b-ratified alias family), and the bridge's result types take
// their own names, `NatOf` at one argument and `PrecisionOf` at two.
#![no_std]
#![feature(const_trait_impl)]
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
    type Nat = Pz<O<I<I<H>>>>;
}
impl AdmittedWidth for Idx<16> {
    type Nat = Pz<O<O<O<O<H>>>>>;
}

pub trait NatAdd<Rhs> {
    type Out: Nat;
}
impl NatAdd<Pz<I<H>>> for Pz<O<I<I<H>>>> {
    type Out = Pz<O<O<O<O<H>>>>>;
}

// The marker trait is untouched. 110:3176, unchanged.
// (base already declares `pub trait Precision: Nat` with its blanket impl.)

// The bridge's result types, under names of their own. Braced because the
// const parameter `I` collides with the carrier constructor `I<P: Pos>`.
pub type NatOf<const I: u16> = <Idx<{ I }> as AdmittedWidth>::Nat;
pub type PrecisionOf<const I: u16, const F: u16> = <NatOf<{ I }> as NatAdd<NatOf<{ F }>>>::Out;

// The document's own assertion at 110:3326, under the chosen spelling.
const _: () = assert!(<PrecisionOf<13, 3> as Nat>::VAL == 16);

// And the marker trait still does its job on the result, which is the whole
// reason both names have to coexist.
fn takes_a_precision<T: Precision>() {}
pub fn both_names_live() {
    takes_a_precision::<PrecisionOf<13, 3>>();
}
