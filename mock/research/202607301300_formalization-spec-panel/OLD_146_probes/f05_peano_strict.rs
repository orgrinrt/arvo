//! Can the reflexive overlap be excluded by UNIFICATION rather than by evaluation?
//!
//! `145_probes/e2` shows coherence does read a where clause, when the obligation is
//! definitely unsatisfiable at the overlap. `e3` shows a computed projection is not
//! definitely anything, because coherence cannot evaluate it at free parameters.
//! So the question is whether a relation decided by STRUCTURAL RECURSION over a
//! type-level width gives coherence the definite failure it needs.
//!
//! Minimal fragment: both coordinates strictly growing. If this is refused, the
//! disjunction (one strict, one loose) cannot be rescued either.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27)
#![no_std]
use core::marker::PhantomData;

pub struct Z;
pub struct S<N>(PhantomData<N>);

/// Irreflexive by construction: `Z: Lt<Z>` has no candidate impl, and
/// `S<A>: Lt<S<A>>` reduces to `A: Lt<A>`, which bottoms out there.
pub trait Lt<Rhs> {}
impl<B> Lt<S<B>> for Z {}
impl<A, B> Lt<S<B>> for S<A> where A: Lt<B> {}

pub struct Fixed<I, F, G, St>(PhantomData<(I, F, G, St)>);
impl<I, F, G, St> Clone for Fixed<I, F, G, St> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<I, F, G, St> Copy for Fixed<I, F, G, St> {}

impl<I1, F1, I2, F2, G, St> From<Fixed<I1, F1, G, St>> for Fixed<I2, F2, G, St>
where
    I1: Lt<I2>,
    F1: Lt<F2>,
{
    fn from(_: Fixed<I1, F1, G, St>) -> Self {
        Fixed(PhantomData)
    }
}
