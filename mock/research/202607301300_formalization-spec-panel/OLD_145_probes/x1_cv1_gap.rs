//! Does `131`'s own conversion probe check that `widen` is lossless?
//!
//! `131:598` says widen is "total and value-preserving", and `131_probes/cv1_conversion.rs`
//! declares `impl<I, F, J> Widen<Fixed<J, F, G, S>> for Fixed<I, F, G, S>` with no relation
//! between I and J. This reproduces that impl shape in isolation and asks for the
//! direction the claim forbids.
//!
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
use core::marker::PhantomData;
pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
pub struct Unsigned;
pub struct Warm;
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;

pub trait Widen<T> {}
impl<const I: u32, const F: u32, const J: u32, G, S> Widen<Fixed<J, F, G, S>>
    for Fixed<I, F, G, S>
{
}

fn wants_widen<A: Widen<B>, B>() {}

pub fn narrowing_is_admitted_as_widening() {
    // 13 integer digits into 8. The claim says widen is lossless; the impl accepts it.
    wants_widen::<UFixed<13, 3, Warm>, UFixed<8, 3, Warm>>();
}
