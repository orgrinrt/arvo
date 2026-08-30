//! Route: specialise core's identity impl. `min_specialization` is on the allowed list.
//! A specialising impl needs the base impl to be `default`, and core's is not.
#![no_std]
#![feature(min_specialization)]
use core::marker::PhantomData;
pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
{
    fn from(_: Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}
