//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The slot range: which multiples of the quantum are in the set.
//!
//! The third coordinate of the affine predicate, after the quantum law and the
//! phase. A slot is an index, not a value: the value it denotes is the phase plus
//! the slot times the quantum at its magnitude.
//!
//! **The declared width is carried rather than recovered.** A declaration of
//! thirteen bits knows it is thirteen bits, and reconstructing that from the slot
//! bounds by counting and taking a logarithm is a computation over a quantity
//! nothing needs, performed at the one place in the chain where the arithmetic
//! can leave the range it is carried in. It is read instead.

use crate::width::Width;

/// The widest declaration a slot index carries.
///
/// Slot indices are signed 64-bit, and the count of slots is `2^width`, which is
/// what runs out first: at 62 bits it is 4611686018427387904 and fits, and at 63
/// it is 9223372036854775808 and does not.
///
/// **A property of this representation rather than a statement about how wide
/// arvo goes.** That question is open and this is not an answer to it.
pub const MAX_DECLARED_WIDTH: u32 = 62;

/// A declared width, refused at compile time if the range cannot carry it.
///
/// `ruling::never_a_runtime_check_and_one_lowered_path` is ratified and says
/// invalids are caught at compile time. A width whose slot range does not fit is
/// an invalid, and this is where it is refused, with a message a reader can act
/// on rather than an arithmetic overflow inside a constant.
#[must_use]
pub const fn checked_width(bits: u32) -> u32 {
    assert!(
        bits >= 1,
        "a declared width of zero bits admits no values and is not a format"
    );
    assert!(
        bits <= MAX_DECLARED_WIDTH,
        "declared width is wider than a slot index carries; the bound is 62 bits, \
         because the count of slots is 2^width and 2^63 does not fit a signed 64-bit integer"
    );
    bits
}

/// Two to the power of a checked width.
#[must_use]
const fn span(bits: u32) -> i64 {
    1i64 << checked_width(bits)
}

/// Which slot indices a format admits.
///
/// Inclusive at both ends, because the alternative is an off-by-one nobody can
/// see in a const.
pub trait Slots {
    /// The lowest admitted slot index.
    const MIN: i64;

    /// The highest admitted slot index.
    const MAX: i64;

    /// The width the declaration stated.
    ///
    /// A coordinate rather than something derived from the bounds. Everything
    /// downstream that wants a width reads this.
    const WIDTH: Width;
}

/// The slot range a two's complement declaration of `BITS` bits produces on a
/// signed domain.
pub struct Signed<const BITS: u32>;

impl<const BITS: u32> Slots for Signed<BITS> {
    const MIN: i64 = -(span(BITS) / 2);
    const MAX: i64 = span(BITS) / 2 - 1;
    const WIDTH: Width = Width::bits(checked_width(BITS));
}

/// The slot range an unsigned declaration of `BITS` bits produces.
pub struct Unsigned<const BITS: u32>;

impl<const BITS: u32> Slots for Unsigned<BITS> {
    const MIN: i64 = 0;
    const MAX: i64 = span(BITS) - 1;
    const WIDTH: Width = Width::bits(checked_width(BITS));
}

/// Whether a slot index is admitted.
#[must_use]
pub const fn slot_in_range<S: Slots>(slot: i64) -> bool {
    slot >= S::MIN && slot <= S::MAX
}

/// How many slots the range admits.
///
/// Bounded by the refusal above: the widest admitted declaration gives `2^62`,
/// which fits. Nothing derives a width from this any more, and it is kept because
/// a cardinality is a real thing to ask a range for.
#[must_use]
pub const fn slot_count<S: Slots>() -> i64 {
    S::MAX - S::MIN + 1
}

/// The width the declaration stated.
#[must_use]
pub const fn declared_slot_width<S: Slots>() -> Width {
    S::WIDTH
}
