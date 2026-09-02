//! Does `TryFrom` coexist with the by-reference `From`?
//!
//! core ships `impl<T, U> TryFrom<U> for T where U: Into<T>`. Once `&Fixed<A>` has an
//! `Into<Fixed<B>>` conditioned on a computed witness, an arvo `TryFrom<&Fixed<A>>`
//! must be disjoint from that blanket, and coherence cannot evaluate the witness.
//! EXPECTED TO FAIL. The question is which error, since the answer decides whether
//! the narrowing can wear `TryFrom` at all.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
use core::marker::PhantomData;
pub struct Unsigned;
pub struct Warm;
pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
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
pub const fn tag_narrows(i1: u32, f1: u32, i2: u32, f2: u32) -> usize {
    if i1 <= i2 && f1 <= f2 {
        1
    } else {
        0
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
pub trait NarrowTagged {
    type const TAG: usize;
}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> NarrowTagged
    for Pair<I1, F1, I2, F2>
{
    type const TAG: usize = const { tag_narrows(I1, F1, I2, F2) };
}
pub trait EmbedWitness<const TAG: usize> {}
impl EmbedWitness<0> for Picker {}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<&Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
where
    Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
{
    fn from(_: &Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}

// the narrowing, wearing TryFrom, on the strictly complementary region
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> TryFrom<&Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
where
    Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as NarrowTagged>::TAG }>,
{
    type Error = ();
    fn try_from(_: &Fixed<I1, F1, G, S>) -> Result<Self, ()> {
        Ok(Fixed(PhantomData))
    }
}
