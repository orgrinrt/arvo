//! Probe 4b (the widening half of probe 4): a genuinely separate crate
//! implements `Adjustment` for a foreign type with a fabricated, unreduced
//! pair, and it is accepted.
//!
//! `Six` is not `Ratio<N, D>` at all, so `Six: Gcd<...>` never enters into
//! it: the impl below satisfies `Adjustment`'s signature directly with no
//! coprimality check anywhere, because none is asked. `Six::NUM = 6,
//! Six::DEN = 12` denotes the same rational as `Reduced<P6, P12>` (one
//! half), under a second, independent type. This is exactly the class of
//! defect probe 1.2 of `36_probes/probe_6` measures for `Pos`'s own width
//! chain, one layer up: two types, one value, and nothing in `Adjustment`'s
//! own definition rules it out.
//!
//! Committed as a WORKS that should have been a refusal. Do not "fix" this
//! file by making it compile differently; the finding is that it compiles
//! at all.
//!
//! Build (two steps, both against rustc 1.98.0-nightly (57d06900f
//! 2026-05-27)):
//!   rustc --edition 2021 --crate-type lib \
//!         probe_4_adjustment_is_not_sealed_lib.rs --out-dir <dir>
//!   rustc --edition 2021 --crate-type lib \
//!         --extern vu_adjustment_unsealed=<dir>/libvu_adjustment_unsealed.rlib \
//!         probe_4b_downstream_widens_adjustment.rs --out-dir <dir>
//! Outcome: WORKS (the defect: this should have been refused, and was
//! not), verbatim in OUTCOMES.md.

#![allow(dead_code)]
#![no_std]

use vu_adjustment_unsealed::nat::Adjustment;

/// A foreign type, nothing to do with `Ratio<N, D>`, claiming the same
/// value `Reduced<P6, P12>` already denotes: one half, spelled unreduced.
pub struct Six;

impl Adjustment for Six {
    const NUM: u64 = 6;
    const DEN: u64 = 12;
}

pub fn takes_an_adjustment<A: Adjustment>() -> (u64, u64) {
    (A::NUM, A::DEN)
}

pub fn a_foreign_unreduced_pair_is_accepted() -> (u64, u64) {
    takes_an_adjustment::<Six>()
}
