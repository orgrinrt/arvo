//! Route: put a projection in the target's strategy slot, so the reflexive pair
//! cannot be produced by unification. `Bump` has no fixed point, so semantically the
//! reflexive case is excluded. The question is whether coherence can see it.
#![no_std]
use core::marker::PhantomData;
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;
pub trait Bump {
    type Out;
}
impl Bump for Hot {
    type Out = Warm;
}
impl Bump for Warm {
    type Out = Cold;
}
impl Bump for Cold {
    type Out = Precise;
}
impl Bump for Precise {
    type Out = Precise;
}

pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S: Bump>
    From<Fixed<I1, F1, G, S>> for Fixed<I2, F2, G, <S as Bump>::Out>
{
    fn from(_: Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}
