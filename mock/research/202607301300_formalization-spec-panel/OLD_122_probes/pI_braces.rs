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

// 119:315's exact spelling, unbraced.
pub type NatOf<const N: u16> = <Idx<N> as AdmittedWidth>::Nat;
