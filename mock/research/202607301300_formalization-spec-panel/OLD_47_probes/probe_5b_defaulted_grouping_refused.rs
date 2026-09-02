//! Probe 5b: COMMITTED REFUSING, on purpose. Shape C does not exist.
//!
//! The shape that would keep both doors AND keep the common path free of
//! ceremony is one combinator with the grouping as a defaulted type parameter:
//! `fold(xs)` for the regrouped case, `fold::<Sequential>(xs)` for the other.
//! Rust does not have it, for a free function or an inherent method, so the
//! ergonomic argument for one-name cannot be made this way and the choice
//! really is between probe 5's shape A and shape B.
//!
//! EXPECTED: `error: defaults for generic parameters are not allowed here`,
//! twice, with the future-incompatibility note (issue #36887).
//!
//! Compiled as: rustc --edition 2021 --crate-type lib probe_5b_defaulted_grouping_refused.rs
//!
//! Verbatim diagnostic in OUTCOMES.md.

#![allow(dead_code)]

pub struct Regrouped;
pub struct Sequential;
pub struct Column;

pub fn fold<G = Regrouped>(xs: &[i32]) -> i32 {
    xs.len() as i32
}

impl Column {
    pub fn fold<G = Regrouped>(&self) -> i32 {
        0
    }
}
