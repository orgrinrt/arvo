//! p4a: the by-reference `From` with the embedding condition carried as a TRAIT BOUND
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

impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> From<&Fixed<I1, F1, G, S>> for Fixed<I2, F2, G, S>
where
    I1: Le<I2>,
    F1: Le<F2>,
{
    fn from(_: &Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}

// ---- consumers ----

type N0 = Z;
type N1 = Su<Z>;
type N3 = Su<Su<Su<Z>>>;
type N8 = Su<Su<Su<Su<Su<Su<Su<Su<Z>>>>>>>>;
type N13 = Su<Su<Su<Su<Su<N8>>>>>;
type N20 = Su<Su<Su<Su<Su<Su<Su<N13>>>>>>>;

pub type U<I: Nat, F: Nat> = Fixed<I, F, Unsigned, Warm>;

/// EXPECTED TO FAIL: the antichain pair from `130b`, in both directions.  Q13.3 and Q8.8
/// are equal-precision and maximally unrelated, so neither embeds in the other.
pub fn antichain_a(a: U<N13, N3>) -> U<N8, N8> {
    (&a).into()
}
pub fn antichain_b(a: U<N8, N8>) -> U<N13, N3> {
    (&a).into()
}
/// EXPECTED TO FAIL: a plain by-value `.into()` on a non-reflexive pair, to see what
/// rustc suggests to a consumer who forgets the token.
pub fn forgot_the_token(a: U<N13, N3>) -> U<N20, N8> {
    a.into()
}
