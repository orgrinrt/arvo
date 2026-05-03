//! `ConstFrom<T>` / `ConstTryFrom<T>` substrate-owned const traits.
//!
//! Mirror `core::convert::From` and `core::convert::TryFrom` with
//! const-callable bodies. Round 4 (#314) ships the trait declarations;
//! per-substrate-type impls live in the type-declaring crate via
//! forward composition (orphan rule).
//!
//! Per the bridge-home rule, these traits live in `arvo-strategy`:
//! `notko` (which hosts `Outcome`) is reachable from this crate, and
//! the bridges return `Self` and `Outcome<Self, Self::Error>` respectively.
//!
//! Stdlib `From` / `TryFrom` impls remain alongside (boundary
//! coverage). The bridges are additive, not replacements.

use notko::Outcome;

/// Const-callable conversion from `T` into `Self`.
///
/// Mirrors `core::convert::From` for use in const fn bodies. Stdlib
/// `From` does not yet have const counterparts on rustc 1.96.0-nightly;
/// this trait is the substrate's surface for compile-time conversions.
pub const trait ConstFrom<T>: Sized {
    /// Convert `value` into `Self`.
    fn const_from(value: T) -> Self;
}

/// Const-callable fallible conversion from `T` into `Self`.
///
/// Mirrors `core::convert::TryFrom` for use in const fn bodies.
/// Returns `notko::Outcome<Self, Self::Error>` (the substrate's
/// fallible-result shape; not stdlib `Result`).
pub const trait ConstTryFrom<T>: Sized {
    /// Conversion failure carrier.
    type Error;
    /// Try to convert `value` into `Self`. On failure, returns the
    /// `Outcome::Failed(error)` variant.
    fn const_try_from(value: T) -> Outcome<Self, Self::Error>;
}
