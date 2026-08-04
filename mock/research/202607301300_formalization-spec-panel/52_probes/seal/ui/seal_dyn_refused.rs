//! The one remaining dispatch route the constraints already forbid,
//! checked anyway so the perimeter enumeration is complete rather than
//! complete-except-the-obvious: a trait object over a sealed trait, which
//! would erase which inhabitant a value is. EXPECTED: E0038, `Pos` is not
//! dyn-compatible (associated const `VAL`), so the route does not exist
//! even before the workspace's own no-dyn rule applies.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_8_dyn_refused.rs

#![allow(dead_code)]

use vu_core::nat::Pos;

pub fn observe(_x: &dyn Pos) {}
