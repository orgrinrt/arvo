//! probe 1, crate `carrier`: the sealed value-unique type-level number
//! vocabulary alone in one bottom crate. no_std, no gates, const-readable.
#![no_std]
use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}

/// sealed positive naturals, value-unique binary encoding: H = 1, O<P> = 2P, I<P> = 2P + 1
pub trait Pos: sealed::Sealed {
    const VALUE: u128;
}
/// sealed naturals: Z = 0, Pz<P> = P
pub trait Nat: sealed::Sealed {
    const VALUE: u128;
}

pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);
pub struct Z;
pub struct Pz<P: Pos>(PhantomData<P>);

impl sealed::Sealed for H {}
impl Pos for H {
    const VALUE: u128 = 1;
}
impl<P: Pos> sealed::Sealed for O<P> {}
impl<P: Pos> Pos for O<P> {
    const VALUE: u128 = 2 * P::VALUE;
}
impl<P: Pos> sealed::Sealed for I<P> {}
impl<P: Pos> Pos for I<P> {
    const VALUE: u128 = 2 * P::VALUE + 1;
}
impl sealed::Sealed for Z {}
impl Nat for Z {
    const VALUE: u128 = 0;
}
impl<P: Pos> sealed::Sealed for Pz<P> {}
impl<P: Pos> Nat for Pz<P> {
    const VALUE: u128 = P::VALUE;
}
