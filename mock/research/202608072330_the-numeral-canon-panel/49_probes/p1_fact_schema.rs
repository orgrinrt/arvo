// Probe 1: does a strategy-parameterised fact schema for a numeral compile
// on the pinned nightly, with no forbidden feature, and does it let facts
// diverge per strategy the way I5/I6/I7 (INTENTS.md) describe?
//
// This is a cold derivation. It does not import or imitate the deleted
// mock/crates tree; the names (Hot/Warm/Cold/Precise) are used only because
// INTENTS.md I2 quotes them as the prior attempt's vocabulary, not because
// this probe treats the four-strategy set as settled (I1: it is not).
//
// No unstable features requested. Compile with:
//   rustc +nightly-2026-05-28 --edition 2024 --crate-type lib p1_fact_schema.rs
//
// The schema and its impls live in _shared_schema.rs (no inner attributes,
// so p2 can `include!` it too without an inner-attribute placement error).

#![no_std]
#![allow(dead_code)]

include!("_shared_schema.rs");
