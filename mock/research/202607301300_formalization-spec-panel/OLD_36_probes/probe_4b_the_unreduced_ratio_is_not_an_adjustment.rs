//! Probe 4b (the refusing half of probe 4): the perimeter, checked rather
//! than asserted.
//!
//! Two refusals, and they are different in kind, which is the point of
//! committing both.
//!
//! The first is the obligation: `Ratio<P6, P12>` is a well-formed type and
//! is not an `Adjustment`, because the impl carries `N: Gcd<D, Out = H>`.
//! So the unreduced spelling that produced file 34's E0308 cannot reach any
//! position the design bounds. Value-uniqueness is enforced where the type is
//! observed, not maintained by whoever writes it.
//!
//! The second is a precondition on the divider: `ExactDivOdd` is exact
//! division by an ODD divisor, and with an even one it does not resolve at
//! all rather than returning a wrong quotient. That matters because the
//! reduction path reaches it only after `Strip2` has removed the common power
//! of two, and a refactor that dropped that step would otherwise be a silent
//! wrong answer.
//!
//! Committed refusing, on purpose. Do not "fix" this file.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_4b_the_unreduced_ratio_is_not_an_adjustment.rs
//! Outcome: FAILS WITH E0271 and E0277, verbatim in OUTCOMES.md, against
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]

#[path = "vu_nat.rs"]
mod nat;

use nat::{Adjustment, AsPos, ExactDivOdd, Pz};
use nat::{H, I, O};

pub type P2 = O<H>;
pub type P6 = O<I<H>>;
pub type P8 = O<O<O<H>>>;
pub type P12 = O<O<I<H>>>;

pub fn takes_an_adjustment<A: Adjustment>() {}

/// 6/12 denotes one half, and so does 1/2. Only the reduced spelling is an
/// `Adjustment`.
pub fn the_unreduced_spelling_cannot_be_an_adjustment() {
    takes_an_adjustment::<nat::Ratio<P6, P12>>();
}

/// 8 / 2 is exact, and `ExactDivOdd` still refuses it, because 2 is not odd.
pub type EightOverTwo = <<Pz<P8> as ExactDivOdd<P2>>::Out as AsPos>::Out;

pub fn exact_division_refuses_an_even_divisor() -> u64 {
    <EightOverTwo as nat::Pos>::VAL
}
