//! Baseline. The naive blanket `From` between two numerals of one family.
//! Expected: E0119 against core's `impl<T> From<T> for T`.
//! This reproduces `145_probes/e1_from_overlap.rs` independently so that every
//! later probe in this series is measured against a failure I observed myself.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
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
