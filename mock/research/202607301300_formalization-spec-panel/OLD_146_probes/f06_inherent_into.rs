//! Route: an inherent `into` on the numeral, which shadows `Into::into`.
//! Question: do the target const parameters infer from the expected type?
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;
pub struct Unsigned;
pub struct Warm;
pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}
pub const fn tag_embeds(i1: u32, f1: u32, i2: u32, f2: u32) -> usize {
    if i1 <= i2 && f1 <= f2 {
        0
    } else {
        1
    }
}
pub struct Picker;
pub struct Pair<const I1: u32, const F1: u32, const I2: u32, const F2: u32>;
pub trait Tagged {
    type const TAG: usize;
}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> Tagged for Pair<I1, F1, I2, F2> {
    type const TAG: usize = const { tag_embeds(I1, F1, I2, F2) };
}
pub trait EmbedWitness<const TAG: usize> {}
impl EmbedWitness<0> for Picker {}

impl<const I1: u32, const F1: u32, G, S> Fixed<I1, F1, G, S> {
    pub fn into<const I2: u32, const F2: u32>(self) -> Fixed<I2, F2, G, S>
    where
        Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
    {
        Fixed(PhantomData)
    }
}

pub fn inherent(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    a.into()
}
pub fn inherent_same(a: UFixed<13, 3, Warm>) -> UFixed<13, 3, Warm> {
    a.into()
}
