//! The sum that keeps the order, selected by the overflow policy.

use arvo_format::overflow::{Overflow, Saturate};

use crate::{Order, Sum};

/// A completion under which `a <= b` gives `a + c <= b + c` for every `c`.
///
/// `Saturate` carries it. `Wrap` does not: under wrap no total order of the members is invariant
/// under translation, at any width. A policy of a consumer's own that keeps the order carries it
/// from the consumer's crate.
pub trait MonotoneCompletion: Overflow {}

impl MonotoneCompletion for Saturate {}

/// A sum that keeps the order: `a <= b` gives `a + c <= b + c` for every `c`.
///
/// An instance implements it where its overflow policy carries `MonotoneCompletion`, so a function
/// bounding on it refuses a wrapping value at build.
pub trait MonotoneSum: Sum + Order {}
