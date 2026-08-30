//! p4c: the same result over the design's OWN width encoding rather than over Peano.
//!
//! `124:1.2` states the encoding: "`Exponent` itself is `EZero | EPos<P> | ENeg<P>` over the
//! sealed `Pos`", and "`Pos` has exactly three constructors and `Pos` is sealed", named in
//! the section 1.23 constructor list as `O<P>`, `I<P>` and `H`.  That is a BINARY positive
//! integer: `H` is one, `O<P>` is two P, `I<P>` is two P plus one.  So a width of thirteen is
//! four constructors deep rather than thirteen, and a width of a hundred and twenty-eight is
//! eight.
//!
//! Two things this file establishes that p4a cannot.
//!
//! One: the by-reference `From` with a trait-carried order works over the design's own
//! encoding and not merely over the toy one, on the default solver with no feature gate.
//!
//! Two: the refusal diagnostic becomes readable.  p4a's refusal unrolls the whole Peano
//! tower, hides seven redundant requirements and writes a long-type file to disk.  Here the
//! same refusal names types a person can read.
//!
//! Build: rustup run nightly-2026-05-28 rustc --edition 2021 --crate-type lib p4c...rs

#![no_std]

use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}

// ---- the design's sealed positive integer, binary, outermost constructor least significant

pub trait Pos: sealed::Sealed {}
pub struct H;
pub struct O<P: Pos>(PhantomData<P>);
pub struct I<P: Pos>(PhantomData<P>);
impl sealed::Sealed for H {}
impl Pos for H {}
impl<P: Pos> sealed::Sealed for O<P> {}
impl<P: Pos> Pos for O<P> {}
impl<P: Pos> sealed::Sealed for I<P> {}
impl<P: Pos> Pos for I<P> {}

/// at most, over positives
pub trait PLe<R: Pos>: Pos {}
/// strictly less, over positives.  The pair is mutually recursive because a binary
/// comparison at an odd digit needs the strict answer one digit up.
pub trait PLt<R: Pos>: Pos {}

impl PLe<H> for H {}
impl<B: Pos> PLe<O<B>> for H {}
impl<B: Pos> PLe<I<B>> for H {}
impl<A: Pos, B: Pos> PLe<O<B>> for O<A> where A: PLe<B> {}
impl<A: Pos, B: Pos> PLe<I<B>> for O<A> where A: PLe<B> {}
impl<A: Pos, B: Pos> PLe<O<B>> for I<A> where A: PLt<B> {}
impl<A: Pos, B: Pos> PLe<I<B>> for I<A> where A: PLe<B> {}

impl<B: Pos> PLt<O<B>> for H {}
impl<B: Pos> PLt<I<B>> for H {}
impl<A: Pos, B: Pos> PLt<O<B>> for O<A> where A: PLt<B> {}
impl<A: Pos, B: Pos> PLt<I<B>> for O<A> where A: PLe<B> {}
impl<A: Pos, B: Pos> PLt<O<B>> for I<A> where A: PLt<B> {}
impl<A: Pos, B: Pos> PLt<I<B>> for I<A> where A: PLt<B> {}

// ---- widths admit zero, which positives do not

pub trait Nat: sealed::Sealed {}
pub struct Zero;
pub struct Pv<P: Pos>(PhantomData<P>);
impl sealed::Sealed for Zero {}
impl Nat for Zero {}
impl<P: Pos> sealed::Sealed for Pv<P> {}
impl<P: Pos> Nat for Pv<P> {}

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
impl<A: Pos, B: Pos> Le<Pv<B>> for Pv<A> where A: PLe<B> {}

// ---- the numeral, and the one impl

pub struct Unsigned;
pub struct Warm;

pub struct Fixed<I: Nat, F: Nat, G, S>(PhantomData<(I, F, G, S)>);
impl<I: Nat, F: Nat, G, S> Clone for Fixed<I, F, G, S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<I: Nat, F: Nat, G, S> Copy for Fixed<I, F, G, S> {}

impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> From<&Fixed<I1, F1, G, S>> for Fixed<I2, F2, G, S>
where
    I1: Le<I2>,
    F1: Le<F2>,
{
    fn from(_: &Fixed<I1, F1, G, S>) -> Self {
        Fixed(PhantomData)
    }
}

// ---- widths.  Depth is log of the width, not the width.

pub type W0 = Zero;
pub type W1 = Pv<H>; // 1
pub type W3 = Pv<I<H>>; // 3
pub type W8 = Pv<O<O<O<H>>>>; // 8
pub type W13 = Pv<I<O<I<H>>>>; // 13
pub type W20 = Pv<O<O<I<O<H>>>>>; // 20
pub type W64 = Pv<O<O<O<O<O<O<H>>>>>>>; // 64
pub type W128 = Pv<O<O<O<O<O<O<O<H>>>>>>>>; // 128

pub type U<I, F> = Fixed<I, F, Unsigned, Warm>;

pub fn widen_both(a: U<W13, W3>) -> U<W20, W8> {
    (&a).into()
}
pub fn widen_f_only(a: U<W13, W3>) -> U<W13, W8> {
    (&a).into()
}
pub fn widen_wide(a: U<W64, W64>) -> U<W128, W128> {
    (&a).into()
}
pub fn from_zero(a: U<W0, W0>) -> U<W1, W1> {
    (&a).into()
}
pub fn reflexive(a: U<W13, W3>) -> U<W13, W3> {
    a.into()
}
pub fn generic<A, B>(a: A) -> B
where
    A: Into<B>,
{
    a.into()
}
pub fn use_generic(a: U<W13, W3>) -> U<W20, W8> {
    generic(&a)
}
pub fn hrtb<A: Copy, B>(a: A) -> B
where
    for<'x> &'x A: Into<B>,
{
    (&a).into()
}
pub fn use_hrtb(a: U<W13, W3>) -> U<W20, W8> {
    hrtb(a)
}
