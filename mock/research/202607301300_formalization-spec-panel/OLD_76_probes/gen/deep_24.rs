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

pub type D0 = Slot<Pz<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777217>;
pub const V0: usize = <D0 as Capacity>::CAP;
const _: () = assert!(V0 == 16777217);
pub type D1 = Slot<Pz<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777219>;
pub const V1: usize = <D1 as Capacity>::CAP;
const _: () = assert!(V1 == 16777219);
pub type D2 = Slot<Pz<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777221>;
pub const V2: usize = <D2 as Capacity>::CAP;
const _: () = assert!(V2 == 16777221);
pub type D3 = Slot<Pz<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777223>;
pub const V3: usize = <D3 as Capacity>::CAP;
const _: () = assert!(V3 == 16777223);
pub type D4 = Slot<Pz<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777225>;
pub const V4: usize = <D4 as Capacity>::CAP;
const _: () = assert!(V4 == 16777225);
pub type D5 = Slot<Pz<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777227>;
pub const V5: usize = <D5 as Capacity>::CAP;
const _: () = assert!(V5 == 16777227);
pub type D6 = Slot<Pz<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777229>;
pub const V6: usize = <D6 as Capacity>::CAP;
const _: () = assert!(V6 == 16777229);
pub type D7 = Slot<Pz<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777231>;
pub const V7: usize = <D7 as Capacity>::CAP;
const _: () = assert!(V7 == 16777231);
pub type D8 = Slot<Pz<I<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777233>;
pub const V8: usize = <D8 as Capacity>::CAP;
const _: () = assert!(V8 == 16777233);
pub type D9 = Slot<Pz<I<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777235>;
pub const V9: usize = <D9 as Capacity>::CAP;
const _: () = assert!(V9 == 16777235);
pub type D10 = Slot<Pz<I<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777237>;
pub const V10: usize = <D10 as Capacity>::CAP;
const _: () = assert!(V10 == 16777237);
pub type D11 = Slot<Pz<I<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777239>;
pub const V11: usize = <D11 as Capacity>::CAP;
const _: () = assert!(V11 == 16777239);
pub type D12 = Slot<Pz<I<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777241>;
pub const V12: usize = <D12 as Capacity>::CAP;
const _: () = assert!(V12 == 16777241);
pub type D13 = Slot<Pz<I<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777243>;
pub const V13: usize = <D13 as Capacity>::CAP;
const _: () = assert!(V13 == 16777243);
pub type D14 = Slot<Pz<I<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777245>;
pub const V14: usize = <D14 as Capacity>::CAP;
const _: () = assert!(V14 == 16777245);
pub type D15 = Slot<Pz<I<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777247>;
pub const V15: usize = <D15 as Capacity>::CAP;
const _: () = assert!(V15 == 16777247);
pub type D16 = Slot<Pz<I<O<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777249>;
pub const V16: usize = <D16 as Capacity>::CAP;
const _: () = assert!(V16 == 16777249);
pub type D17 = Slot<Pz<I<I<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777251>;
pub const V17: usize = <D17 as Capacity>::CAP;
const _: () = assert!(V17 == 16777251);
pub type D18 = Slot<Pz<I<O<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777253>;
pub const V18: usize = <D18 as Capacity>::CAP;
const _: () = assert!(V18 == 16777253);
pub type D19 = Slot<Pz<I<I<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777255>;
pub const V19: usize = <D19 as Capacity>::CAP;
const _: () = assert!(V19 == 16777255);
pub type D20 = Slot<Pz<I<O<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777257>;
pub const V20: usize = <D20 as Capacity>::CAP;
const _: () = assert!(V20 == 16777257);
pub type D21 = Slot<Pz<I<I<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777259>;
pub const V21: usize = <D21 as Capacity>::CAP;
const _: () = assert!(V21 == 16777259);
pub type D22 = Slot<Pz<I<O<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777261>;
pub const V22: usize = <D22 as Capacity>::CAP;
const _: () = assert!(V22 == 16777261);
pub type D23 = Slot<Pz<I<I<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777263>;
pub const V23: usize = <D23 as Capacity>::CAP;
const _: () = assert!(V23 == 16777263);
pub type D24 = Slot<Pz<I<O<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777265>;
pub const V24: usize = <D24 as Capacity>::CAP;
const _: () = assert!(V24 == 16777265);
pub type D25 = Slot<Pz<I<I<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777267>;
pub const V25: usize = <D25 as Capacity>::CAP;
const _: () = assert!(V25 == 16777267);
pub type D26 = Slot<Pz<I<O<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777269>;
pub const V26: usize = <D26 as Capacity>::CAP;
const _: () = assert!(V26 == 16777269);
pub type D27 = Slot<Pz<I<I<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777271>;
pub const V27: usize = <D27 as Capacity>::CAP;
const _: () = assert!(V27 == 16777271);
pub type D28 = Slot<Pz<I<O<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777273>;
pub const V28: usize = <D28 as Capacity>::CAP;
const _: () = assert!(V28 == 16777273);
pub type D29 = Slot<Pz<I<I<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777275>;
pub const V29: usize = <D29 as Capacity>::CAP;
const _: () = assert!(V29 == 16777275);
pub type D30 = Slot<Pz<I<O<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777277>;
pub const V30: usize = <D30 as Capacity>::CAP;
const _: () = assert!(V30 == 16777277);
pub type D31 = Slot<Pz<I<I<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>, 16777279>;
pub const V31: usize = <D31 as Capacity>::CAP;
const _: () = assert!(V31 == 16777279);
