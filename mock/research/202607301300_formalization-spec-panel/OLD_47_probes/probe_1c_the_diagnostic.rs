//! Probe 1c: COMMITTED REFUSING, on purpose. What the consumer reads when a
//! numeral mismatch does surface.
//!
//! Probe 1b's mistyped numeral is silent where nothing constrains it. Where
//! something does (the accumulator-sufficiency check, a fold's declared
//! destination, any signature naming a specific numeral) it surfaces as an
//! ordinary type mismatch. The question this probe answers is what that
//! mismatch reads like, since the design's whole checking story routes
//! through it.
//!
//! EXPECTED: E0308, printing both numerals in full positional form.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_1c_the_diagnostic.rs
//!
//! Verbatim diagnostic in OUTCOMES.md.

#![allow(dead_code)]

use tower::nat::{Nat, Pz, H, I, O};

pub struct Accumulator<P: Nat>(core::marker::PhantomData<P>);

pub type Intended = Pz<I<O<I<O<O<H>>>>>>; // 37
pub type Typo = Pz<I<O<I<O<I<H>>>>>>; // 53

pub fn needs_37(_: Accumulator<Intended>) {}

pub fn consumer(acc: Accumulator<Typo>) {
    needs_37(acc);
}
