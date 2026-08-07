#![no_std]
extern crate tower;
use core::marker::PhantomData;
use tower::*;

pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Out: Nat;
}

// Can a const argument be an anonymous const block computed from a type?
impl AdmittedWidth for Idx<{ <Pz<I<O<I<H>>>> as Nat>::VAL as u16 }> {
    type Out = Pz<I<O<I<H>>>>;
}
const _: () = assert!(<<Idx<13> as AdmittedWidth>::Out as Nat>::VAL == 13);
