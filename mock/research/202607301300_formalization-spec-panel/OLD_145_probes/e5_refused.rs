//! What a consumer sees when the embedding does not exist.
//!
//! Negative control for `e4` route A. Expected to fail; the failure text is the
//! artifact. Q13.3 and Q8.8 are the antichain pair op was reasoning about at
//! `130b:19-23`: equal precision, incomparable in the order, no embedding either way.
//!
//! Build (expected to fail):
//!   rustc --edition 2021 --crate-type lib -Znext-solver=globally e5_refused.rs
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct Unsigned;
#[derive(Clone, Copy)]
pub struct Warm;

pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;

impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}

pub struct Picker;
pub struct Pair<const I1: u32, const F1: u32, const I2: u32, const F2: u32>;

pub const fn tag_embeds(i1: u32, f1: u32, i2: u32, f2: u32) -> usize {
    if i1 <= i2 && f1 <= f2 {
        0
    } else {
        1
    }
}

pub trait Tagged {
    type const TAG: usize;
}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> Tagged for Pair<I1, F1, I2, F2> {
    type const TAG: usize = const { tag_embeds(I1, F1, I2, F2) };
}

#[diagnostic::on_unimplemented(
    message = "this numeral does not embed into that one",
    label = "no exact embedding here",
    note = "an embedding needs the target's integer digits and fraction digits to be \
            both at least the source's. Where either shrinks, the conversion is lossy \
            and is spelled `quantise`, whose resolution the strategy names."
)]
pub trait EmbedWitness<const TAG: usize> {}
impl EmbedWitness<0> for Picker {}

pub trait Embed<T> {
    fn embed(self) -> T;
}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> Embed<Fixed<I2, F2, G, S>>
    for Fixed<I1, F1, G, S>
where
    Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
{
    fn embed(self) -> Fixed<I2, F2, G, S> {
        Fixed(PhantomData)
    }
}

// The antichain pair, both directions. Neither embeds.
pub fn refused_a(a: UFixed<13, 3, Warm>) -> UFixed<8, 8, Warm> {
    a.embed()
}

pub fn refused_b(a: UFixed<8, 8, Warm>) -> UFixed<13, 3, Warm> {
    a.embed()
}

// Plain assignment, for comparison with `131:617-623`'s E0308.
pub fn refused_c(a: UFixed<13, 3, Warm>) -> UFixed<8, 8, Warm> {
    a
}
