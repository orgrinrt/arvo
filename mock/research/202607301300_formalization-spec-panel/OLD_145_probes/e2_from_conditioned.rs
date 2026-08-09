//! Does conditioning the embedding `From` impl rescue it from the reflexive overlap?
//!
//! Follow-up to `e1_from_overlap.rs`. Here the impl carries a where clause that is
//! unsatisfiable at `(I1, F1) == (I2, F2)`: `Picker: Proper<I1, F1, I2, F2>` is
//! implemented only for the pairs where the embedding is proper. The question is
//! whether coherence reads that as disjointness evidence.
//!
//! Two arms, both gate-free, so the answer is about coherence rather than about
//! const machinery.
//!
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]

use core::marker::PhantomData;

pub struct Fixed<const I: u32, const F: u32, G, S>(PhantomData<(G, S)>);
pub struct Picker;

impl<const I: u32, const F: u32, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, G, S> Copy for Fixed<I, F, G, S> {}

/// Witness that `(I1, F1)` embeds properly into `(I2, F2)`.
/// Instantiated below at exactly one pair, so it is knowably not reflexive.
pub trait Proper<const I1: u32, const F1: u32, const I2: u32, const F2: u32> {}
impl Proper<13, 3, 20, 3> for Picker {}

impl<const I1: u32, const F1: u32, const I2: u32, const F2: u32, G, S> From<Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
where
    Picker: Proper<I1, F1, I2, F2>,
{
    fn from(_: Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}
