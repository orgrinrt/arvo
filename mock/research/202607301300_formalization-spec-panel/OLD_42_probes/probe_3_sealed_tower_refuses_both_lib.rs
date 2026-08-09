//! Probe 3 (the library half): the fix. vu_nat_sealed.rs is vu_nat.rs
//! with the private supertrait seal `36_probes/probe_5_sealed_perimeter_
//! lib.rs` already demonstrated in isolation, applied to the module
//! everything actually composes with, and vu_bias_sealed.rs is vu_bias.rs
//! retargeted at it (`bias::nat` is the sealed module; `Adjustment` is
//! reached at `bias::nat::Adjustment`, the same type the standalone
//! probe 1/1b attack targets, so one crate here suffices for both
//! attacks rather than duplicating the nat module under two separate
//! module trees). No change to Adjustment, Bias, Gcd, ExactDivOdd,
//! Strip2 or Reduce.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_3_sealed_tower_refuses_both_lib.rs --out-dir <dir>
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]
#![crate_name = "vu_sealed_tower"]

#[path = "vu_bias_sealed.rs"]
pub mod bias;
