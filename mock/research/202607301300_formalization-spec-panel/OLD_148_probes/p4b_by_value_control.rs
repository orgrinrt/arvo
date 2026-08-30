//! p4b: THE SAME FILE BY VALUE, expected E0119.  This is the control that shows the
//! `&` is what defeats coherence and the trait bound is not.  Everything else is identical
//! to p4a, including the reflexive-admitting `Le`.
//!
//! original header of p4a: the by-reference `From` with the embedding condition carried as a TRAIT BOUND
//! rather than as an associated-const projection in a const-argument position.
//!
//! Why this file exists.  `146_probes/f03_ref_source_full.rs` establishes the by-reference
//! spelling, and it does so under `#![feature(min_generic_const_args, generic_const_args)]`
//! compiled with `-Znext-solver=globally`.  Both of those are outside the arrangement:
//! `generic_const_args` is not on `unstable-features.md`'s allowed list, and the standing
//! record quoted at `124:1.2` says `generic_const_args` "needs `-Znext-solver=globally`,
//! mutually exclusive with the rest of the arrangement per the workspace's own record".
//! Verified: f03 exits 0 with the flag and fails with E0308 without it.
//!
//! `a-refused-bound-wants-a-trait-not-a-feature.md` says what to do about that, and it is
//! the workspace's own standing answer: break the constraint into pieces that each hold on
//! their own, carry them on a trait, and bound on the trait rather than on an expression.
//!
//! So: widths are types, the order is a recursive trait over them, and the coherence
//! evasion is `146`'s `&`, which is orthogonal to both.
//!
//! Build: rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib p4a...rs
//! No -Z flags.  No generic_const_args.

#![no_std]

use core::marker::PhantomData;

// ---- widths as types.  Peano here; p4c does the design's own binary encoding. ----

mod sealed {
    pub trait Sealed {}
}
pub trait Nat: sealed::Sealed {}

pub struct Z;
pub struct Su<N: Nat>(PhantomData<N>);
impl sealed::Sealed for Z {}
impl Nat for Z {}
impl<N: Nat> sealed::Sealed for Su<N> {}
impl<N: Nat> Nat for Su<N> {}

/// `Self` is at most `Rhs`.  Reflexive, which is what an embedding wants: every numeral
/// embeds into itself, and the reflexive conversion is core's, not arvo's.
#[diagnostic::on_unimplemented(
    message = "this numeral does not embed into that one",
    label = "no exact embedding here",
    note = "an embedding needs the target integer digits and fraction digits to be both \
            at least the source. Where either shrinks the conversion is lossy and is \
            written, and the strategy names what it does with what does not fit."
)]
pub trait Le<Rhs: Nat>: Nat {}
impl<N: Nat> Le<N> for Z {}
impl<A: Nat, B: Nat> Le<Su<B>> for Su<A> where A: Le<B> {}

// ---- the numeral ----

pub struct Unsigned;
pub struct Warm;

pub struct Fixed<I: Nat, F: Nat, G, S>(PhantomData<(I, F, G, S)>);
impl<I: Nat, F: Nat, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<I: Nat, F: Nat, G, S> Copy for Fixed<I, F, G, S> {}

// ---- the one impl.  No enumeration.  No const arguments.  No feature gate. ----

impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> From<Fixed<I1, F1, G, S>> for Fixed<I2, F2, G, S>
where
    I1: Le<I2>,
    F1: Le<F2>,
{
    fn from(_: Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}
