//! The embedding order as a bound, computed rather than enumerated.
//!
//! `e1` showed an unconditioned `From` between two numerals conflicts with core's
//! `impl<T> From<T> for T`. `e2` showed a where clause rescues it when the witness is
//! knowably unsatisfiable at the reflexive pair. This builds the real thing: the
//! witness tag is COMPUTED from the four coordinates by the same Pattern C shape the
//! container projection uses (`131` section 3.3), so nothing is enumerated over widths.
//!
//! The predicate is the PROPER embedding: I1 <= I2 and F1 <= F2 and not both equal.
//! The reflexive case must fall outside it or coherence returns.
//!
//! Build:
//!   rustc --edition 2021 --crate-type lib -Znext-solver=globally e3_embed_gca.rs
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct Unsigned;
pub struct Signed;
pub struct Warm;
pub struct Hot;

pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;

impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}

// ------------------------------------------------------------------ the order
/// 0 when the proper embedding holds, 1 otherwise. One tag, one impl, no width table.
pub const fn tag_embeds(i1: u32, f1: u32, i2: u32, f2: u32) -> usize {
    if i1 <= i2 && f1 <= f2 && !(i1 == i2 && f1 == f2) {
        0
    } else {
        1
    }
}

pub struct Picker;

#[diagnostic::on_unimplemented(
    message = "no exact embedding into this numeral",
    label = "the source does not fit",
    note = "an implicit conversion exists only when the target's integer digits and \
            fraction digits are both at least the source's, and the two are not the \
            same numeral. Where either coordinate shrinks the conversion is lossy: \
            write it, and the strategy names what it does with what does not fit."
)]
pub trait Embeds<const TAG: usize> {}
impl Embeds<0> for Picker {}
// deliberately no `impl Embeds<1> for Picker`

pub struct Pair<const I1: u32, const F1: u32, const I2: u32, const F2: u32>;
pub trait Tagged {
    type const TAG: usize;
}
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32> Tagged for Pair<I1, F1, I2, F2> {
    type const TAG: usize = const { tag_embeds(I1, F1, I2, F2) };
}

// ------------------------------------------------- the embedding, as a From impl
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
where
    Picker: Embeds<{ <Pair<I1, F1, I2, F2> as Tagged>::TAG }>,
{
    fn from(_: Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}

// ------------------------------------------------------------------ consumers
pub fn up_in_both(a: UFixed<13, 3, Warm>) -> UFixed<20, 8, Warm> {
    a.into()
}

pub fn up_in_i_only(a: UFixed<13, 3, Warm>) -> UFixed<20, 3, Warm> {
    a.into()
}

pub fn up_in_f_only(a: UFixed<13, 3, Warm>) -> UFixed<13, 8, Warm> {
    a.into()
}

pub fn signed_too(a: IFixed<12, 3, Warm>) -> IFixed<24, 6, Warm> {
    a.into()
}

/// The reflexive case still goes through core's own identity impl.
pub fn reflexive(a: UFixed<13, 3, Warm>) -> UFixed<13, 3, Warm> {
    a.into()
}
