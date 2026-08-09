//! Removing the tag number from the consumer's diagnostic.
//!
//! `e5` produces a good message with one blemish: "the trait `EmbedWitness<1>` is not
//! implemented for `Picker` but trait `EmbedWitness<0>` is". The integers are internal
//! machinery and mean nothing at a call site. This routes the tag through a two-impl
//! decision table so the verdict is a named type, and the same rustc line then reads
//! `EmbedWitness<DoesNotEmbed>` against `EmbedWitness<Embeds>`.
//!
//! Expected to fail on the last two functions; the failure text is the artifact.
//!
//! Build:
//!   rustc --edition 2021 --crate-type lib -Znext-solver=globally e6_named_verdict.rs
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

/// The verdicts, as types rather than as integers.
pub struct Embeds;
pub struct DoesNotEmbed;

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

/// The decision table: total, two rows, the same Pattern C shape the container
/// projection uses.
pub trait Decide<const TAG: usize> {
    type V;
}
impl Decide<0> for Picker {
    type V = Embeds;
}
impl Decide<1> for Picker {
    type V = DoesNotEmbed;
}

#[diagnostic::on_unimplemented(
    message = "this numeral does not embed into that one",
    label = "no exact embedding here",
    note = "an embedding needs the target's integer digits and fraction digits to be \
            both at least the source's. Where either shrinks, the conversion is lossy \
            and is spelled `quantise`, whose resolution the strategy names."
)]
pub trait EmbedWitness<V> {}
impl EmbedWitness<Embeds> for Picker {}

pub trait Embed<T> {
    fn embed(self) -> T;
}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> Embed<Fixed<I2, F2, G, S>>
    for Fixed<I1, F1, G, S>
where
    Picker: Decide<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
    Picker: EmbedWitness<<Picker as Decide<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>>::V>,
{
    fn embed(self) -> Fixed<I2, F2, G, S> {
        Fixed(PhantomData)
    }
}

pub fn accepted(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    a.embed()
}

pub fn refused_a(a: UFixed<13, 3, Warm>) -> UFixed<8, 8, Warm> {
    a.embed()
}

pub fn refused_b(a: UFixed<8, 8, Warm>) -> UFixed<13, 3, Warm> {
    a.embed()
}
