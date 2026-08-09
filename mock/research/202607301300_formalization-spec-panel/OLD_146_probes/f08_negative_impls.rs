//! Route: negative reasoning. `negative_impls` is on the allowed list; the coherence
//! half, `with_negative_coherence`, is forbidden. This checks what the allowed half
//! alone buys, which the enumeration needs recorded either way.
#![no_std]
#![feature(negative_impls)]
use core::marker::PhantomData;
pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}

pub trait Distinct<Rhs> {}
impl<T> !Distinct<T> for T {}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> Distinct<Fixed<I2, F2, G, S>>
    for Fixed<I1, F1, G, S>
{
}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
where
    Fixed<I1, F1, G, S>: Distinct<Fixed<I2, F2, G, S>>,
{
    fn from(_: Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}
