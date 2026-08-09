//! Probe 1b (negative control): radix one is unspellable.
//!
//! rustc --edition 2021 --crate-type lib --extern vu54=libvu54.rlib probe_1b_radix_one_refused.rs
//!
//! `Rad<H>` is a well-formed type. It has no `Radix` impl, because `Rad`'s impl is bounded
//! on the sealed `AtLeastTwo`, which has impls only for `O<P>` and `I<P>`. Radix one would
//! collapse every grid of a `Ranged` numeral's union onto the same grid, which falsifies the
//! whole exponent-function statement (`50:40-47`) while looking entirely well typed.
//!
//! Expected: E0277, `the trait bound H: AtLeastTwo is not satisfied`, reported at the
//! `Radix` bound rather than at some later arithmetic that produced a wrong answer.

#![allow(dead_code)]

use vu54::bias::nat::H;
use vu54::numeral::{Rad, Radix};

pub fn radix_one_has_no_impl() -> u64 {
    <Rad<H> as Radix>::R
}
