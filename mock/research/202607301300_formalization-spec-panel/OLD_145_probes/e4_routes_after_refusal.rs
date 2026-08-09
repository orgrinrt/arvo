//! Three routes after `e3` refuted the generic embedding `From`.
//!
//! A. An owned trait `Embed`, written as `.embed()`. Local trait, no core overlap.
//! B. A `From` impl that moves the numeral AND names two distinct concrete strategies.
//!    This one compiles, which is the finding: implicit numeral embedding is reachable
//!    only when the strategy also changes, and that asymmetry is the argument against it.
//! C. The narrowing, as a named operation whose result type the strategy decides.
//!
//! Build:
//!   rustc --edition 2021 --crate-type lib -Znext-solver=globally e4_routes_after_refusal.rs
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct Unsigned;
pub struct Signed;
#[derive(Clone, Copy)]
pub struct Hot;
#[derive(Clone, Copy)]
pub struct Warm;
#[derive(Clone, Copy)]
pub struct Cold;
#[derive(Clone, Copy)]
pub struct Precise;

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

// ---------------------------------------------------------------- route A
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

pub fn route_a(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    a.embed()
}

pub fn route_a_reflexive(a: UFixed<13, 3, Warm>) -> UFixed<13, 3, Warm> {
    a.embed() // the reflexive case is admitted here, unlike under `From`
}

// ---------------------------------------------------------------- route B
// Moving the numeral AND naming two distinct concrete strategies. No reflexive
// overlap is possible, because `Hot` and `Warm` are distinct types.
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G> From<Fixed<I1, F1, G, Hot>>
    for Fixed<I2, F2, G, Warm>
where
    Picker: EmbedWitness<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
{
    fn from(_: Fixed<I1, F1, G, Hot>) -> Self {
        Fixed(PhantomData)
    }
}

pub fn route_b(a: UFixed<13, 3, Hot>) -> UFixed<20, 8, Warm> {
    a.into() // implicit, and only because the strategy moved too
}

// ---------------------------------------------------------------- route C
/// The quantiser, keyed on the target strategy. Total for every pair of numerals;
/// what it does with a value the target cannot hold is the strategy's own row.
pub trait Quantise<T> {
    fn quantise(self) -> T;
}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> Quantise<Fixed<I2, F2, G, S>>
    for Fixed<I1, F1, G, S>
{
    fn quantise(self) -> Fixed<I2, F2, G, S> {
        Fixed(PhantomData)
    }
}

pub fn route_c_down_in_range(a: UFixed<13, 3, Warm>) -> UFixed<8, 8, Warm> {
    a.quantise() // range event only: the grid refines, the range shrinks
}

pub fn route_c_down_in_grid(a: UFixed<8, 8, Warm>) -> UFixed<13, 3, Warm> {
    a.quantise() // grid event only: the range grows, the grid coarsens
}

pub fn route_c_up(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    a.quantise() // total and exact: quantise agrees with embed here
}
