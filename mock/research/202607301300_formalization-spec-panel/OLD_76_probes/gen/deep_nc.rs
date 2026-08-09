#![no_std]
use core::marker::PhantomData;
mod seal {
    pub trait Sealed {}
}
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
pub struct Z;
pub struct Pz<P>(PhantomData<P>);
impl seal::Sealed for H {}
impl<P: Pos> seal::Sealed for O<P> {}
impl<P: Pos> seal::Sealed for I<P> {}
impl seal::Sealed for Z {}
impl<P: Pos> seal::Sealed for Pz<P> {}
pub trait Pos: seal::Sealed {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}
pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}
pub trait Cmp<R> {
    const LE: bool;
}
pub struct Slot<N, const K: usize>(PhantomData<N>);
impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> {
    const VAL: usize = N::VAL;
}
pub const fn agrees<N: Nat, const K: usize>() -> bool {
    N::VAL == K
}
pub trait Capacity: Nat {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const CAP: usize;
}
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T> = [T; K];
    const CAP: usize = {
        assert!(agrees::<N, K>());
        K
    };
}
pub fn walk<C: Capacity>(a: &C::Array<u32>) -> u32 {
    let r: &[u32] = a.as_ref();
    let mut acc = 0u32;
    let mut j = 0;
    while j < r.len() {
        acc = acc.wrapping_add(r[j]);
        j += 1;
    }
    acc
}

pub type D0 = Slot<Pz<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967297>;
pub const V0: usize = <D0 as Capacity>::CAP;
const _: () = assert!(V0 == 4294967297);
pub type D1 = Slot<Pz<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967299>;
pub const V1: usize = <D1 as Capacity>::CAP;
const _: () = assert!(V1 == 4294967299);
pub type D2 = Slot<Pz<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967301>;
pub const V2: usize = <D2 as Capacity>::CAP;
const _: () = assert!(V2 == 4294967301);
pub type D3 = Slot<Pz<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967303>;
pub const V3: usize = <D3 as Capacity>::CAP;
const _: () = assert!(V3 == 4294967303);
pub type D4 = Slot<Pz<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967305>;
pub const V4: usize = <D4 as Capacity>::CAP;
const _: () = assert!(V4 == 4294967305);
pub type D5 = Slot<Pz<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967307>;
pub const V5: usize = <D5 as Capacity>::CAP;
const _: () = assert!(1 + V5 == 4294967307);
pub type D6 = Slot<Pz<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967309>;
pub const V6: usize = <D6 as Capacity>::CAP;
const _: () = assert!(V6 == 4294967309);
pub type D7 = Slot<Pz<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967311>;
pub const V7: usize = <D7 as Capacity>::CAP;
const _: () = assert!(V7 == 4294967311);
pub type D8 = Slot<Pz<I<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967313>;
pub const V8: usize = <D8 as Capacity>::CAP;
const _: () = assert!(V8 == 4294967313);
pub type D9 = Slot<Pz<I<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967315>;
pub const V9: usize = <D9 as Capacity>::CAP;
const _: () = assert!(V9 == 4294967315);
pub type D10 = Slot<Pz<I<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967317>;
pub const V10: usize = <D10 as Capacity>::CAP;
const _: () = assert!(V10 == 4294967317);
pub type D11 = Slot<Pz<I<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967319>;
pub const V11: usize = <D11 as Capacity>::CAP;
const _: () = assert!(V11 == 4294967319);
pub type D12 = Slot<Pz<I<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967321>;
pub const V12: usize = <D12 as Capacity>::CAP;
const _: () = assert!(V12 == 4294967321);
pub type D13 = Slot<Pz<I<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967323>;
pub const V13: usize = <D13 as Capacity>::CAP;
const _: () = assert!(V13 == 4294967323);
pub type D14 = Slot<Pz<I<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967325>;
pub const V14: usize = <D14 as Capacity>::CAP;
const _: () = assert!(V14 == 4294967325);
pub type D15 = Slot<Pz<I<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967327>;
pub const V15: usize = <D15 as Capacity>::CAP;
const _: () = assert!(V15 == 4294967327);
pub type D16 = Slot<Pz<I<O<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967329>;
pub const V16: usize = <D16 as Capacity>::CAP;
const _: () = assert!(V16 == 4294967329);
pub type D17 = Slot<Pz<I<I<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967331>;
pub const V17: usize = <D17 as Capacity>::CAP;
const _: () = assert!(V17 == 4294967331);
pub type D18 = Slot<Pz<I<O<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967333>;
pub const V18: usize = <D18 as Capacity>::CAP;
const _: () = assert!(V18 == 4294967333);
pub type D19 = Slot<Pz<I<I<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967335>;
pub const V19: usize = <D19 as Capacity>::CAP;
const _: () = assert!(V19 == 4294967335);
pub type D20 = Slot<Pz<I<O<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967337>;
pub const V20: usize = <D20 as Capacity>::CAP;
const _: () = assert!(V20 == 4294967337);
pub type D21 = Slot<Pz<I<I<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967339>;
pub const V21: usize = <D21 as Capacity>::CAP;
const _: () = assert!(V21 == 4294967339);
pub type D22 = Slot<Pz<I<O<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967341>;
pub const V22: usize = <D22 as Capacity>::CAP;
const _: () = assert!(V22 == 4294967341);
pub type D23 = Slot<Pz<I<I<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967343>;
pub const V23: usize = <D23 as Capacity>::CAP;
const _: () = assert!(V23 == 4294967343);
pub type D24 = Slot<Pz<I<O<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967345>;
pub const V24: usize = <D24 as Capacity>::CAP;
const _: () = assert!(V24 == 4294967345);
pub type D25 = Slot<Pz<I<I<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967347>;
pub const V25: usize = <D25 as Capacity>::CAP;
const _: () = assert!(V25 == 4294967347);
pub type D26 = Slot<Pz<I<O<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967349>;
pub const V26: usize = <D26 as Capacity>::CAP;
const _: () = assert!(V26 == 4294967349);
pub type D27 = Slot<Pz<I<I<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967351>;
pub const V27: usize = <D27 as Capacity>::CAP;
const _: () = assert!(V27 == 4294967351);
pub type D28 = Slot<Pz<I<O<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967353>;
pub const V28: usize = <D28 as Capacity>::CAP;
const _: () = assert!(V28 == 4294967353);
pub type D29 = Slot<Pz<I<I<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967355>;
pub const V29: usize = <D29 as Capacity>::CAP;
const _: () = assert!(V29 == 4294967355);
pub type D30 = Slot<Pz<I<O<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967357>;
pub const V30: usize = <D30 as Capacity>::CAP;
const _: () = assert!(V30 == 4294967357);
pub type D31 = Slot<Pz<I<I<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 4294967359>;
pub const V31: usize = <D31 as Capacity>::CAP;
const _: () = assert!(V31 == 4294967359);
