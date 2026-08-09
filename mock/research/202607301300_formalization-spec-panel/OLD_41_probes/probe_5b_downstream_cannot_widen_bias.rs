//! Probe 5b (the refusing half of probe 5): a genuinely separate crate
//! tries probe 4b's exact attack against `Bias` and is refused, on two
//! independent grounds, both checked.
//!
//! `(a)`: a foreign type implementing `bias::Bias` directly. Refused with
//! E0603 (or the private-trait equivalent): `bias_sealed::BiasSealed` is
//! not `pub`, so a downstream crate cannot even name the supertrait it
//! would need to implement, let alone satisfy it. This is the seal
//! `Adjustment` was missing.
//!
//! `(b)`: inside this crate's own reach, using the exported `BPos`/`BNeg`
//! constructors with an unreduced pair (`Six/Twelve` rather than the
//! reduced `One/Two`). Refused with E0271, the same coprimality failure
//! probe 1b already pins from inside the defining crate; here it is the
//! same check surviving the crate boundary, which is what a bound
//! (independent of any seal) is for.
//!
//! Committed refusing, on purpose. Do not "fix" this file.
//!
//! Build (two steps, both against rustc 1.98.0-nightly (57d06900f
//! 2026-05-27)):
//!   rustc --edition 2021 --crate-type lib \
//!         probe_5_bias_sealed_perimeter_lib.rs --out-dir <dir>
//!   rustc --edition 2021 --crate-type lib \
//!         --extern vu_bias_sealed=<dir>/libvu_bias_sealed.rlib \
//!         probe_5b_downstream_cannot_widen_bias.rs --out-dir <dir>
//! Outcome: FAILS WITH E0603 (part a) and E0271 (part b), verbatim in
//! OUTCOMES.md. Both parts are committed in the SAME file so a partial fix
//! to one cannot be mistaken for closing the perimeter; comment either out
//! to see the other's diagnostic in isolation.

#![allow(dead_code)]
#![no_std]

use vu_bias_sealed::bias::nat::{H, I, O};
use vu_bias_sealed::bias::{BPos, Bias};

// (a): cannot even name the seal to implement it.
pub struct MyBias;
impl vu_bias_sealed::bias::bias_sealed::BiasSealed for MyBias {}
impl Bias for MyBias {
    const NUM: i64 = 7;
    const DEN: u64 = 1;
}

// (b): can name Bias itself (it is pub), but the unreduced pair still
// fails the bound, surviving the crate boundary.
pub type P6 = O<I<H>>;
pub type P12 = O<O<I<H>>>;

pub fn takes_a_bias<B: Bias>() -> i64 {
    B::NUM
}

pub fn a_foreign_unreduced_pair_is_refused() -> i64 {
    takes_a_bias::<BPos<P6, P12>>()
}
