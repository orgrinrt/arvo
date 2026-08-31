//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The slot range: which multiples of the quantum are in the set.
//!
//! The third coordinate of the affine predicate, after the quantum law and the
//! phase. A slot is an index, not a value: the value it denotes is the phase plus
//! the slot times the quantum at its magnitude.

/// Which slot indices a format admits.
///
/// Inclusive at both ends, because the alternative is an off-by-one nobody can
/// see in a const.
pub trait Slots {
    /// The lowest admitted slot index.
    const MIN: i64;

    /// The highest admitted slot index.
    const MAX: i64;
}

/// The slot range a two's complement declaration of `BITS` bits produces on a
/// signed domain.
pub struct Signed<const BITS: u32>;

impl<const BITS: u32> Slots for Signed<BITS> {
    const MIN: i64 = -(1i64 << (BITS - 1));
    const MAX: i64 = (1i64 << (BITS - 1)) - 1;
}

/// The slot range an unsigned declaration of `BITS` bits produces.
pub struct Unsigned<const BITS: u32>;

impl<const BITS: u32> Slots for Unsigned<BITS> {
    const MIN: i64 = 0;
    const MAX: i64 = (1i64 << BITS) - 1;
}

/// Whether a slot index is admitted.
#[must_use]
pub const fn slot_in_range<S: Slots>(slot: i64) -> bool {
    slot >= S::MIN && slot <= S::MAX
}

/// How many slots the range admits.
///
/// Returns an `i64` rather than a count type because the difference of two
/// inclusive bounds is what it is, and the caller that wants a cardinality can
/// say so at its own position.
#[must_use]
pub const fn slot_count<S: Slots>() -> i64 {
    S::MAX - S::MIN + 1
}
