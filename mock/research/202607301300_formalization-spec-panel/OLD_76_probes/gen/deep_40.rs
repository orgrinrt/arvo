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

pub type D0 = Slot<Pz<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627777>;
pub const V0: usize = <D0 as Capacity>::CAP;
const _: () = assert!(V0 == 1099511627777);
pub type D1 = Slot<Pz<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627779>;
pub const V1: usize = <D1 as Capacity>::CAP;
const _: () = assert!(V1 == 1099511627779);
pub type D2 = Slot<Pz<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627781>;
pub const V2: usize = <D2 as Capacity>::CAP;
const _: () = assert!(V2 == 1099511627781);
pub type D3 = Slot<Pz<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627783>;
pub const V3: usize = <D3 as Capacity>::CAP;
const _: () = assert!(V3 == 1099511627783);
pub type D4 = Slot<Pz<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627785>;
pub const V4: usize = <D4 as Capacity>::CAP;
const _: () = assert!(V4 == 1099511627785);
pub type D5 = Slot<Pz<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627787>;
pub const V5: usize = <D5 as Capacity>::CAP;
const _: () = assert!(V5 == 1099511627787);
pub type D6 = Slot<Pz<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627789>;
pub const V6: usize = <D6 as Capacity>::CAP;
const _: () = assert!(V6 == 1099511627789);
pub type D7 = Slot<Pz<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627791>;
pub const V7: usize = <D7 as Capacity>::CAP;
const _: () = assert!(V7 == 1099511627791);
pub type D8 = Slot<Pz<I<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627793>;
pub const V8: usize = <D8 as Capacity>::CAP;
const _: () = assert!(V8 == 1099511627793);
pub type D9 = Slot<Pz<I<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627795>;
pub const V9: usize = <D9 as Capacity>::CAP;
const _: () = assert!(V9 == 1099511627795);
pub type D10 = Slot<Pz<I<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627797>;
pub const V10: usize = <D10 as Capacity>::CAP;
const _: () = assert!(V10 == 1099511627797);
pub type D11 = Slot<Pz<I<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627799>;
pub const V11: usize = <D11 as Capacity>::CAP;
const _: () = assert!(V11 == 1099511627799);
pub type D12 = Slot<Pz<I<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627801>;
pub const V12: usize = <D12 as Capacity>::CAP;
const _: () = assert!(V12 == 1099511627801);
pub type D13 = Slot<Pz<I<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627803>;
pub const V13: usize = <D13 as Capacity>::CAP;
const _: () = assert!(V13 == 1099511627803);
pub type D14 = Slot<Pz<I<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627805>;
pub const V14: usize = <D14 as Capacity>::CAP;
const _: () = assert!(V14 == 1099511627805);
pub type D15 = Slot<Pz<I<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627807>;
pub const V15: usize = <D15 as Capacity>::CAP;
const _: () = assert!(V15 == 1099511627807);
pub type D16 = Slot<Pz<I<O<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627809>;
pub const V16: usize = <D16 as Capacity>::CAP;
const _: () = assert!(V16 == 1099511627809);
pub type D17 = Slot<Pz<I<I<O<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627811>;
pub const V17: usize = <D17 as Capacity>::CAP;
const _: () = assert!(V17 == 1099511627811);
pub type D18 = Slot<Pz<I<O<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627813>;
pub const V18: usize = <D18 as Capacity>::CAP;
const _: () = assert!(V18 == 1099511627813);
pub type D19 = Slot<Pz<I<I<I<O<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627815>;
pub const V19: usize = <D19 as Capacity>::CAP;
const _: () = assert!(V19 == 1099511627815);
pub type D20 = Slot<Pz<I<O<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627817>;
pub const V20: usize = <D20 as Capacity>::CAP;
const _: () = assert!(V20 == 1099511627817);
pub type D21 = Slot<Pz<I<I<O<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627819>;
pub const V21: usize = <D21 as Capacity>::CAP;
const _: () = assert!(V21 == 1099511627819);
pub type D22 = Slot<Pz<I<O<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627821>;
pub const V22: usize = <D22 as Capacity>::CAP;
const _: () = assert!(V22 == 1099511627821);
pub type D23 = Slot<Pz<I<I<I<I<O<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627823>;
pub const V23: usize = <D23 as Capacity>::CAP;
const _: () = assert!(V23 == 1099511627823);
pub type D24 = Slot<Pz<I<O<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627825>;
pub const V24: usize = <D24 as Capacity>::CAP;
const _: () = assert!(V24 == 1099511627825);
pub type D25 = Slot<Pz<I<I<O<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627827>;
pub const V25: usize = <D25 as Capacity>::CAP;
const _: () = assert!(V25 == 1099511627827);
pub type D26 = Slot<Pz<I<O<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627829>;
pub const V26: usize = <D26 as Capacity>::CAP;
const _: () = assert!(V26 == 1099511627829);
pub type D27 = Slot<Pz<I<I<I<O<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627831>;
pub const V27: usize = <D27 as Capacity>::CAP;
const _: () = assert!(V27 == 1099511627831);
pub type D28 = Slot<Pz<I<O<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627833>;
pub const V28: usize = <D28 as Capacity>::CAP;
const _: () = assert!(V28 == 1099511627833);
pub type D29 = Slot<Pz<I<I<O<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627835>;
pub const V29: usize = <D29 as Capacity>::CAP;
const _: () = assert!(V29 == 1099511627835);
pub type D30 = Slot<Pz<I<O<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627837>;
pub const V30: usize = <D30 as Capacity>::CAP;
const _: () = assert!(V30 == 1099511627837);
pub type D31 = Slot<Pz<I<I<I<I<I<I<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>, 1099511627839>;
pub const V31: usize = <D31 as Capacity>::CAP;
const _: () = assert!(V31 == 1099511627839);
