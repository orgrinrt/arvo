#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
extern crate base;
use base::*;
pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Nat: Nat;
}

// Each row emitted with its own agreement assertion beside it (119:208-211).
impl AdmittedWidth for Idx<3> {
    type Nat = Pz<I<H>>;
}
const _: () = assert!(<<Idx<3> as AdmittedWidth>::Nat as Nat>::VAL == 3);
impl AdmittedWidth for Idx<13> {
    type Nat = Pz<O<I<I<H>>>>;
} // CORRUPTED: 14, not 13
const _: () = assert!(<<Idx<13> as AdmittedWidth>::Nat as Nat>::VAL == 13);
impl AdmittedWidth for Idx<16> {
    type Nat = Pz<O<O<O<O<H>>>>>;
}
const _: () = assert!(<<Idx<16> as AdmittedWidth>::Nat as Nat>::VAL == 16);
