// The crate that declares the sealed carrier AND the bridge table, which is
// where 119:164-167 concludes the table must live. Both `I` the carrier
// constructor (110:3148) and `I` the public integer width (110:3311) are in it.
#![no_std]
#![allow(dead_code)]
use core::marker::PhantomData;
mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;
pub trait Pos: Sealed {}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>); // 110:3148, the carrier constructor
impl Sealed for H {}
impl<P: Pos> Sealed for O<P> {}
impl<P: Pos> Sealed for I<P> {}
impl Pos for H {}
impl<P: Pos> Pos for O<P> {}
impl<P: Pos> Pos for I<P> {}
pub trait Nat: Sealed {}
pub struct Pz<P: Pos>(PhantomData<P>);
impl<P: Pos> Sealed for Pz<P> {}
impl<P: Pos> Nat for Pz<P> {}

pub struct Idx<const N: u16>;
pub trait AdmittedWidth {
    type Nat: Nat;
}
impl AdmittedWidth for Idx<13> {
    type Nat = Pz<I<O<I<H>>>>;
}

// The bridge's one-argument form, with the const parameter named `I` because
// that is what UFixed<I, F, S> calls its integer width (110:3311).
pub type NatOf<const I: u16> = <Idx<{ I }> as AdmittedWidth>::Nat;
