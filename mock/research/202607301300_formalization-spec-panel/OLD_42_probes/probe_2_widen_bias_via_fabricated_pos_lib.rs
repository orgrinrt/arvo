//! Probe 2 (the library half): exports vu_bias, exactly probe 5's own
//! library half in file 41, unsealed.
//!
//! Build: rustc --edition 2021 --crate-type lib \
//!        probe_2_widen_bias_via_fabricated_pos_lib.rs --out-dir <dir>
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#![no_std]
#![crate_name = "vu_bias_unsealed"]

#[path = "vu_bias.rs"]
pub mod bias;
