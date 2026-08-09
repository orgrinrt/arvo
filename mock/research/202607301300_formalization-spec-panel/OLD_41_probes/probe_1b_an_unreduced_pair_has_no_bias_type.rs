//! Probe 1b (the refusing half of probe 1): the perimeter, checked rather
//! than asserted.
//!
//! `BPos<P6, P12>` is a well-formed type (nothing stops `PhantomData<(N,
//! D)>` from being instantiated with any two `Pos` types) and it is NOT a
//! `Bias`, because `bias_sealed::BiasSealed` (and `Bias` itself) are only
//! implemented under the bound `N: Pos + Gcd<D, Out = H>, D: Pos`, and 6/12
//! is not coprime. So the unreduced spelling that would carry the same
//! defect as file 34's original `Ratio<P6, P12>` E0308 cannot reach any
//! position bounded by `Bias`, exactly the way `Ratio<P6, P12>` cannot
//! reach a position bounded by `Adjustment` (probe 4b, `36_probes/`).
//!
//! Committed refusing, on purpose. Do not "fix" this file.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1b_an_unreduced_pair_has_no_bias_type.rs
//! Outcome: FAILS WITH E0271, verbatim in OUTCOMES.md, against rustc
//! 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#[path = "vu_bias.rs"]
mod bias;

use bias::nat::{H, I, O};
use bias::{BPos, Bias};

pub type P6 = O<I<H>>;
pub type P12 = O<O<I<H>>>;

pub fn takes_a_bias<B: Bias>() -> i64 {
    B::NUM
}

pub fn the_unreduced_spelling_cannot_be_a_bias() {
    let _ = takes_a_bias::<BPos<P6, P12>>();
}
