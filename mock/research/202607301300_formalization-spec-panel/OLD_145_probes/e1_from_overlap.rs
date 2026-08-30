//! Can the exact embedding be a `From` impl at all?
//!
//! `131:598` recommends shipping the lossless conversion "as both: `widen` for the
//! explicit reading and a `From` impl for the `.into()` reading". This checks whether
//! a generic `From` impl between two numerals of the same family is admissible, given
//! that `core` ships `impl<T> From<T> for T`.
//!
//! No feature gate: the question is coherence, which is decided before any const
//! machinery is involved.
//!
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

// The exact embedding, written as a `From` impl with no condition at all.
// If this is refused, no conditioned version can be admitted either, since a
// condition only shrinks the impl and coherence does not read where clauses
// as disjointness evidence.
impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
{
    fn from(_: Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}
