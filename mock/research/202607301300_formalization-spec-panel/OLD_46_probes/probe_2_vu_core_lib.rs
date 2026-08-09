//! `vu_core`: the one canonical tower this file's whole adversary suite
//! runs against. `Pos`/`Nat` sealed (file 42's fix), `Adjustment` sealed
//! (this file's fix, probe 1b is the attack it closes), `Bias` sealed
//! (file 41's fix, carried unchanged).
//!
//! Compiled as: rustc --edition 2021 --crate-type lib probe_2_vu_core_lib.rs

#![allow(dead_code)]
#![crate_name = "vu_core"]

#[path = "vu_bias_sealed_adj.rs"]
pub mod bias;

pub use bias::nat;
