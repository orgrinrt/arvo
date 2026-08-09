//! p5: two things at once.
//!
//! One, erasure.  `146` establishes that its by-reference `From` lowers to the same symbol as
//! the hand-written shift, under `-Znext-solver=globally`.  The trait-bound spelling is a
//! different impl on a different solver, so the claim is re-established here rather than
//! inherited.
//!
//! Two, the by-value arm.  The `&` exists only to defeat the overlap with core's
//! `impl<T> From<T> for T`.  An arvo-OWNED conversion trait has no such overlap, because
//! core does not ship a reflexive impl of it, so the same condition can be carried by value
//! with one blanket impl and no coherence question at all.  This file compiles both beside
//! each other to establish they do not conflict, and emits assembly for all three routes.
//!
//! Build and emit:
//!   rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib -O \
//!     --emit asm p5_erasure_and_by_value_arm.rs -o p5.s

#![no_std]

use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}
pub trait Pos: sealed::Sealed {
    /// the positive's own value.  Carried on the sealed trait itself rather than on a
    /// second trait, so no bound has to be repeated at every use site.
    const V: u32;
}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);
impl sealed::Sealed for H {}
impl Pos for H {
    const V: u32 = 1;
}
impl<P: Pos> sealed::Sealed for O<P> {}
impl<P: Pos> Pos for O<P> {
    const V: u32 = 2 * P::V;
}
impl<P: Pos> sealed::Sealed for I<P> {}
impl<P: Pos> Pos for I<P> {
    const V: u32 = 2 * P::V + 1;
}

pub trait PLe<R: Pos>: Pos {}
pub trait PLt<R: Pos>: Pos {}
impl PLe<H> for H {}
impl<B: Pos> PLe<O<B>> for H {}
impl<B: Pos> PLe<I<B>> for H {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLe<O<B>> for O<A> where A: PLe<B> {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLe<I<B>> for O<A> where A: PLe<B> {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLe<O<B>> for I<A> where A: PLt<B> {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLe<I<B>> for I<A> where A: PLe<B> {}
impl<B: Pos> PLt<O<B>> for H {}
impl<B: Pos> PLt<I<B>> for H {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLt<O<B>> for O<A> where A: PLt<B> {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLt<I<B>> for O<A> where A: PLe<B> {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLt<O<B>> for I<A> where A: PLt<B> {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> PLt<I<B>> for I<A> where A: PLt<B> {}

pub trait Nat: sealed::Sealed {
    /// the width as a number, so the shift amount is available to the body.  This is a
    /// spike shortcut: the real design derives the container and the shift through the
    /// lowering projection, and this stands in for it so the codegen question can be asked.
    const N: u32;
}
pub struct Zero;
pub struct Pv<P: Pos>(PhantomData<P>);
impl sealed::Sealed for Zero {}
impl Nat for Zero {
    const N: u32 = 0;
}
impl<P: Pos> sealed::Sealed for Pv<P> {}
impl<P: Pos> Nat for Pv<P> {
    const N: u32 = P::V;
}

#[diagnostic::on_unimplemented(
    message = "this numeral does not embed into that one",
    label = "no exact embedding here",
    note = "an embedding needs the target integer digits and fraction digits to be both \
            at least the source. Where either shrinks the conversion is lossy and is \
            written, and the strategy names what it does with what does not fit."
)]
pub trait Le<R: Nat>: Nat {}
impl Le<Zero> for Zero {}
impl<B: Pos> Le<Pv<B>> for Zero {}
#[diagnostic::do_not_recommend]
impl<A: Pos, B: Pos> Le<Pv<B>> for Pv<A> where A: PLe<B> {}

pub struct Unsigned;
pub struct Warm;

/// a real datum, so there is something to lower
#[repr(transparent)]
pub struct Fixed<I: Nat, F: Nat, G, S>(pub u32, PhantomData<(I, F, G, S)>);
impl<I: Nat, F: Nat, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        Fixed(self.0, PhantomData)
    }
}
impl<I: Nat, F: Nat, G, S> Copy for Fixed<I, F, G, S> {}

// ---- route one: core's `From`, by reference, condition on the trait ----

impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> From<&Fixed<I1, F1, G, S>> for Fixed<I2, F2, G, S>
where
    I1: Le<I2>,
    F1: Le<F2>,
{
    fn from(a: &Fixed<I1, F1, G, S>) -> Self {
        Fixed(a.0 << (F2::N - F1::N), PhantomData)
    }
}

// ---- route two: an arvo-owned trait, BY VALUE, no coherence question ----

/// core ships no reflexive impl of this, so the diagonal is ours to include, and it should
/// be included: a numeral does embed into itself.
#[diagnostic::on_unimplemented(
    message = "this numeral does not embed into that one",
    label = "no exact embedding here"
)]
pub trait Embed<T> {
    fn embed(self) -> T;
}

impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> Embed<Fixed<I2, F2, G, S>> for Fixed<I1, F1, G, S>
where
    I1: Le<I2>,
    F1: Le<F2>,
{
    fn embed(self) -> Fixed<I2, F2, G, S> {
        Fixed(self.0 << (F2::N - F1::N), PhantomData)
    }
}

pub type W3 = Pv<I<H>>;
pub type W8 = Pv<O<O<O<H>>>>;
pub type W13 = Pv<I<O<I<H>>>>;
pub type W20 = Pv<O<O<I<O<H>>>>>;
pub type U<I, F> = Fixed<I, F, Unsigned, Warm>;

// ---- the three routes, same computation ----

#[inline(never)]
pub fn scalar_by_hand(a: U<W13, W3>) -> U<W20, W8> {
    Fixed(a.0 << 5, PhantomData)
}

#[inline(never)]
pub fn scalar_via_from(a: U<W13, W3>) -> U<W20, W8> {
    (&a).into()
}

#[inline(never)]
pub fn scalar_via_embed(a: U<W13, W3>) -> U<W20, W8> {
    a.embed()
}

#[inline(never)]
pub fn loop_by_hand(xs: &[U<W13, W3>], out: &mut [U<W20, W8>]) {
    for (i, x) in xs.iter().enumerate() {
        out[i] = Fixed(x.0 << 5, PhantomData);
    }
}

#[inline(never)]
pub fn loop_via_from(xs: &[U<W13, W3>], out: &mut [U<W20, W8>]) {
    for (i, x) in xs.iter().enumerate() {
        out[i] = x.into();
    }
}

#[inline(never)]
pub fn loop_via_embed(xs: &[U<W13, W3>], out: &mut [U<W20, W8>]) {
    for (i, x) in xs.iter().enumerate() {
        out[i] = (*x).embed();
    }
}

// the reflexive case still belongs to the language for `From`, and to arvo for `Embed`
pub fn reflexive_from(a: U<W13, W3>) -> U<W13, W3> {
    a.into()
}
pub fn reflexive_embed(a: U<W13, W3>) -> U<W13, W3> {
    a.embed()
}

// generic reachability, both registers
pub fn generic_into<A, B>(a: A) -> B
where
    A: Into<B>,
{
    a.into()
}
pub fn use_generic_into(a: U<W13, W3>) -> U<W20, W8> {
    generic_into(&a)
}
pub fn generic_embed<A, B>(a: A) -> B
where
    A: Embed<B>,
{
    a.embed()
}
pub fn use_generic_embed(a: U<W13, W3>) -> U<W20, W8> {
    generic_embed(a)
}
