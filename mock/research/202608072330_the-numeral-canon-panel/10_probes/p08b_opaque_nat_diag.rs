//! P08b. TREATMENT. Identical construction, one change: the nat is an opaque
//! `struct N13` carrying its binary tower as an associated type, instead of a
//! type alias for the tower. rustc prints struct names and does not expand
//! them, so every surface-adjacent diagnostic names the written width.
#![no_std]
#![crate_type = "lib"]
use core::marker::PhantomData;

pub struct Term;
pub struct D0<T>(PhantomData<T>);
pub struct D1<T>(PhantomData<T>);

// opaque, one per bridge row that already exists
pub struct N13;
pub struct N16;
pub trait Digits {
    type B;
}
impl Digits for N13 {
    type B = D1<D0<D1<D1<Term>>>>;
}
impl Digits for N16 {
    type B = D0<D0<D0<D0<D1<Term>>>>>;
}

pub struct Idx<const N: u32>;
pub struct Arvo;
pub trait ToNat<M> {
    type N;
}
impl ToNat<Arvo> for Idx<13> {
    type N = N13;
}
impl ToNat<Arvo> for Idx<16> {
    type N = N16;
}

pub struct Fixed<const I: u32, S>(PhantomData<S>);
pub struct Hot;

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
