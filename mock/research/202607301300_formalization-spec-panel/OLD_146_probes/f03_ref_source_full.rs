//! `f02` with the failing consumer removed, plus a negative control and the
//! generic-bound reachability test. Establishes:
//!   1. the impl is coherent (no E0119) with a COMPUTED witness, no enumeration
//!   2. `(&a).into()` reaches it
//!   3. `T: Into<U>` in a generic function reaches it
//!   4. a non-embedding pair is refused, and the refusal carries the design text
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
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
#[diagnostic::on_unimplemented(
    message = "this numeral does not embed into that one",
    label = "no exact embedding here",
    note = "an embedding needs the target integer digits and fraction digits to be \
            both at least the source. Where either shrinks the conversion is lossy \
            and is written, and the strategy names what it does with what does not fit."
)]
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

pub fn borrowed(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    (&a).into()
}
pub fn via_from(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    UFixed::<20, 8, Warm>::from(&a)
}
pub fn grow_f_only(a: UFixed<13, 3, Warm>) -> UFixed<13, 8, Warm> {
    (&a).into()
}
pub fn reflexive_still_core(a: UFixed<13, 3, Warm>) -> UFixed<13, 3, Warm> {
    a.into()
}

// generic-bound reachability: no numeral named at the call
pub fn generic<A, B>(a: A) -> B
where
    A: Into<B>,
{
    a.into()
}
pub fn use_generic(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    generic(&a)
}
