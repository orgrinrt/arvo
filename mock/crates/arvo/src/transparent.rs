//! Facade re-export of the transparent-wrapper surface.
//!
//! Per round 202604271346, the `Transparent` trait, `NumericPrimitive`
//! marker, and free `raw` fn moved to the `arvo-transparent` crate.
//! This module re-exports them so `arvo::Transparent`, `arvo::raw`,
//! and `arvo::NumericPrimitive` import paths remain valid.

pub use arvo_transparent::{NumericPrimitive, Transparent, raw};
