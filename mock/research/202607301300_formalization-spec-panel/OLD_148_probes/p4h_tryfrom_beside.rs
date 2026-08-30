//! p4h: `TryFrom<&A> for B` placed BESIDE the by-reference `From`.  EXPECTED E0119.
//! Re-checks `146`'s f07 in the trait-bound spelling on the default solver, since a claim
//! inherited from a probe compiled under a different solver is a claim not checked.
//!
//! original header of p4c: the same result over the design's OWN width encoding rather than over Peano.
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

pub struct NotRepresentable;

impl<I1: Nat, F1: Nat, I2: Nat, F2: Nat, G, S> TryFrom<&Fixed<I1, F1, G, S>>
    for Fixed<I2, F2, G, S>
{
    type Error = NotRepresentable;
    fn try_from(_: &Fixed<I1, F1, G, S>) -> Result<Self, Self::Error> {
        Ok(Fixed(PhantomData))
    }
}
