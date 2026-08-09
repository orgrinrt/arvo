//! The ratified tower as a library crate, so every consumer probe in this
//! directory compiles against it the way a real consumer crate would:
//! across a crate boundary, with only the public surface reachable.
//!
//! Contents are file 46's `vu_nat_sealed_adj.rs` + `vu_bias_sealed_adj.rs`
//! unmodified (the `Adjustment` seal included), retargeted by `#[path]`
//! only. Diff against `46_probes/` to audit.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --crate-name tower tower.rs

#![allow(dead_code)]

#[path = "vu_bias_annotated.rs"]
pub mod bias;

pub use bias::nat;
