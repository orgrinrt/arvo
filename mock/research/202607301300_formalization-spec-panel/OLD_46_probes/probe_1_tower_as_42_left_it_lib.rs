//! The composing tower exactly as file 42 left it: `Pos`/`Nat` sealed,
//! `Bias` sealed, `Adjustment` a bare `pub trait` with no supertrait
//! (`vu_nat_sealed.rs:448-455`, copied unmodified from `42_probes/`).
//!
//! Hypothesis: file 41's ORIGINAL attack (probe_4b there: a genuinely
//! separate downstream crate implements `Adjustment` directly on a local
//! type, fabricating `NUM`/`DEN` with no `Ratio`, no `Pos`, no coprimality
//! anywhere) still lands against this tower, because file 42's fix closed
//! the deeper fabricated-`Pos` route and never carried file 41's own
//! recommended `Adjustment` seal into the composed copy.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   probe_1_tower_as_42_left_it_lib.rs
//! then probe_1b against the rlib.

#![allow(dead_code)]
#![crate_name = "tower_as_42_left_it"]

#[path = "vu_bias_sealed.rs"]
pub mod bias;

pub use bias::nat;
