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
//! bounds by counting is a computation over a quantity nothing needs.
//!
//! **The bound on how wide a declaration may be is the set of impls below and not
//! a check inside a function.** A check is a thing a later edit can delete while
//! every test at admitted widths stays green, which is what happened to the first
//! version of this file. An absent impl cannot be deleted.

use crate::width::Width;

/// Which slot indices a format admits.
///
/// Inclusive at both ends, because the alternative is an off-by-one nobody can
/// see in a const.
///
/// **Implemented at widths 1 through 62 and nowhere else.** Slot indices are
/// signed 64-bit and the count of slots is `radix^width`, which is what runs out
/// first: at 62 bits it is 4611686018427387904 and fits, at 63 it is
/// 9223372036854775808 and does not.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an admitted slot range",
    label = "no slot range exists for this width",
    note = "declared widths run from 1 to 62 bits. The count of slots is 2^width, and 2^63 does not \
            fit the signed 64-bit integer a slot index is carried in, so a wider declaration would \
            invert its own range rather than describe one."
)]
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

/// The slot range an unsigned declaration of `BITS` bits produces.
pub struct Unsigned<const BITS: u32>;

/// Writes the impls at the widths the design admits.
///
/// The list is the bound. A width past it does not get an impl, so a use of it is
/// a trait-bound error rather than a value nobody checked.
///
/// **Adding a width past the bound fails here rather than at some use site**: the
/// body computes `1i64 << BITS` for the unsigned bound, which overflows at the
/// definition site at 63 and refuses to compile where somebody added it.
macro_rules! admit_widths {
    ($($w:literal),+ $(,)?) => {
        $(
            impl Slots for Signed<$w> {
                const MIN: i64 = -(1i64 << ($w - 1));
                const MAX: i64 = (1i64 << ($w - 1)) - 1;
                const WIDTH: Width = Width::bits($w);
            }

            impl Slots for Unsigned<$w> {
                const MIN: i64 = 0;
                const MAX: i64 = (1i64 << $w) - 1;
                const WIDTH: Width = Width::bits($w);
            }
        )+

        /// Every width the design admits, for a test that wants the whole set
        /// rather than the widths somebody remembered.
        pub const ADMITTED_WIDTHS: &[u32] = &[$($w),+];
    };
}

admit_widths!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
);

/// Whether a slot index is admitted.
#[must_use]
pub const fn slot_in_range<S: Slots>(slot: i64) -> bool {
    slot >= S::MIN && slot <= S::MAX
}

/// How many slots the range admits.
///
/// Bounded by the impl set: the widest admitted declaration gives `2^62`, which
/// fits. Nothing derives a width from this, and it is kept because a cardinality
/// is a real thing to ask a range for.
#[must_use]
pub const fn slot_count<S: Slots>() -> i64 {
    S::MAX - S::MIN + 1
}

/// The width the declaration stated.
#[must_use]
pub const fn declared_slot_width<S: Slots>() -> Width {
    S::WIDTH
}
