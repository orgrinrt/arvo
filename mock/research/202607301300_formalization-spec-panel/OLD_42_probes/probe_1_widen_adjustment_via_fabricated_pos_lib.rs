//! Probe 1 (the library half): exports vu_nat directly, the module
//! `Adjustment` lives in. The same module `Bias`'s own crate re-exports
//! as `bias::nat` in probe 2, so this and probe 2's attack are the same
//! shape one layer apart.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_1_widen_adjustment_via_fabricated_pos_lib.rs --out-dir <dir>
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]
#![crate_name = "vu_nat_unsealed"]

#[path = "vu_nat.rs"]
pub mod nat;
