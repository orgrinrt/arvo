//! E6: the SAME mismatch, through the standing base's sealed binary Nat tower
//! (110:3317-3324), so the consumer-facing diagnostic can be compared as
//! evidence rather than asserted.
#![no_std]
#![feature(const_trait_impl)]
use core::marker::PhantomData;
mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

pub const trait Pos: Sealed {
    const VAL: u64;
}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);
impl Sealed for H {}
impl<P: Pos> Sealed for O<P> {}
impl<P: Pos> Sealed for I<P> {}
const impl Pos for H {
    const VAL: u64 = 1;
}
const impl<P: [const] Pos> Pos for O<P> {
    const VAL: u64 = 2 * P::VAL;
}
const impl<P: [const] Pos> Pos for I<P> {
    const VAL: u64 = 2 * P::VAL + 1;
}

pub const trait Nat: Sealed {
    const VAL: u64;
}
pub struct Z;
pub struct Pz<P: Pos>(PhantomData<P>);
impl Sealed for Z {}
impl<P: Pos> Sealed for Pz<P> {}
const impl Nat for Z {
    const VAL: u64 = 0;
}
const impl<P: [const] Pos> Nat for Pz<P> {
    const VAL: u64 = P::VAL;
}

// 16 = 10000b ; 17 = 10001b
pub type N16 = Pz<O<O<O<O<H>>>>>;
pub type N17 = Pz<I<O<O<O<H>>>>>;
const _: () = assert!(<N16 as Nat>::VAL == 16);
const _: () = assert!(<N17 as Nat>::VAL == 17);

pub struct Bits<P: Nat>(PhantomData<P>);
pub fn takes16(_: Bits<N16>) {}
pub fn wrong() {
    takes16(Bits::<N17>(PhantomData));
}
