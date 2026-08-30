//! P08a. CONTROL. Reproduces `137`'s diagnostic shape at the law-relation site.
//! The nat is a type ALIAS, so rustc expands it and prints the tower.
#![no_std]
#![crate_type = "lib"]
use core::marker::PhantomData;

pub struct Term;
pub struct D0<T>(PhantomData<T>);
pub struct D1<T>(PhantomData<T>);

pub type T13 = D1<D0<D1<D1<Term>>>>;
pub type T16 = D0<D0<D0<D0<D1<Term>>>>>;

pub struct Idx<const N: u32>;
pub struct Arvo;
pub trait ToNat<M> {
    type N;
}
impl ToNat<Arvo> for Idx<13> {
    type N = T13;
}
impl ToNat<Arvo> for Idx<16> {
    type N = T16;
}

pub struct Fixed<const I: u32, S>(PhantomData<S>);
pub struct Hot;

// the law-relation site: output coordinate pinned by an associated-type equality
pub fn relate<const I: u32, const O: u32, S>(_a: Fixed<I, S>) -> Fixed<O, S>
where
    Idx<I>: ToNat<Arvo, N = <Idx<O> as ToNat<Arvo>>::N>,
    Idx<O>: ToNat<Arvo>,
{
    Fixed(PhantomData)
}

pub fn bad(a: Fixed<13, Hot>) -> Fixed<16, Hot> {
    relate(a)
}
