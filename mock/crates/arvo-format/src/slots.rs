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
//!
//! **That bounds this crate's impls and not the trait, which is open.** A numeral
//! wanting a slot range that is neither shipped shape has to be able to supply
//! one, so nothing here is sealed. What an outside implementor owes is stated on
//! the trait as `ADMITTED`, and it is checked at compile time rather than asked
//! for in a comment.

use crate::width::Width;

/// Which slot indices a format admits.
///
/// Inclusive at both ends, because the alternative is an off-by-one nobody can
/// see in a const.
///
/// **This crate implements it at widths 1 through 62 and at no other width.**
/// Slot indices are signed 64-bit and the count of slots is `radix^width`, which
/// is what runs out first: at 62 bits it is 4611686018427387904 and fits, at 63
/// it is 9223372036854775808 and does not.
///
/// **The trait is open and an outside crate may implement it.** An earlier
/// version of this sentence said the impls existed at those widths and nowhere
/// else, which was false and which nothing enforced. What an implementor owes is
/// `ADMITTED`, below, and that is checked rather than requested.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an admitted slot range",
    label = "no slot range exists for this width",
    note = "this crate implements the trait at widths 1 to 62. The count of slots is 2^width, and \
            2^63 does not fit the signed 64-bit integer a slot index is carried in, so a wider \
            declaration would invert its own range rather than describe one. The trait is open, so \
            another crate may implement it at a range of its own; what such an implementor owes is \
            the `ADMITTED` obligation."
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

    /// What an implementor owes, checked rather than asked for.
    ///
    /// The three coordinates above can disagree with each other, and an
    /// implementor supplying an inverted range has not supplied a slot range.
    /// `proposal::the_concept_is_closed_and_the_inventory_is_open` says a new
    /// instance earns admission by supplying the concept's obligations, and that
    /// closing the concept while opening the inventory is what makes admission a
    /// check rather than a negotiation. This is that check.
    ///
    /// Every function in this crate that reads the range forces this const, so an
    /// implementor that does not meet it stops the build at the use site with a
    /// named message. Nothing reaches a lowered path, so
    /// `ruling::never_a_runtime_check_and_one_lowered_path` is satisfied rather
    /// than bent.
    ///
    /// **It fires at codegen, not at `cargo check`.** A const is evaluated when
    /// the instantiation is codegened and `check` skips that, so `cargo build`
    /// refuses and `cargo check` does not. The guarantee is that an inadmissible
    /// range cannot reach a produced binary; it can reach a passing check. Said
    /// exactly because an unqualified "refused at compile time" would be the same
    /// shape as the totality claims this replaced.
    ///
    /// **And that is why the refusal is a doctest rather than a `trybuild` case.**
    /// `trybuild` runs `cargo check`, so it never reaches the evaluation and
    /// would report a refused program as compiling. Nothing tested this refusal
    /// at all until the round that gave the other three contracts the same
    /// obligation; the sentence above was a claim about a mechanism with no arm
    /// behind it.
    ///
    /// ```compile_fail
    /// use arvo_format::slots::{slot_in_range, Slots};
    /// use arvo_format::width::Width;
    ///
    /// struct Inverted;
    ///
    /// impl Slots for Inverted {
    ///     const MIN: i64 = 8;
    ///     const MAX: i64 = -8;
    ///     const WIDTH: Width = Width::bits(8);
    /// }
    ///
    /// fn main() {
    ///     let _ = slot_in_range::<Inverted>(0);
    /// }
    /// ```
    ///
    /// The control, which is what says the refusal above is this obligation and
    /// not the outside impl being rejected for some other reason: the same shape
    /// with the ends the right way round builds.
    ///
    /// ```
    /// use arvo_format::slots::{slot_in_range, Slots};
    /// use arvo_format::width::Width;
    ///
    /// struct Ordered;
    ///
    /// impl Slots for Ordered {
    ///     const MIN: i64 = -8;
    ///     const MAX: i64 = 7;
    ///     const WIDTH: Width = Width::bits(8);
    /// }
    ///
    /// fn main() {
    ///     assert!(slot_in_range::<Ordered>(0));
    /// }
    /// ```
    ///
    /// `is_admissible` below is the same question asked without forcing the
    /// const, which is what a test can use at check time and on a construction
    /// that must keep compiling.
    ///
    /// It refuses what the assertions below name and nothing further. The
    /// obligations are the design decision; this is only how they are enforced.
    const ADMITTED: () = {
        assert!(
            Self::MIN <= Self::MAX,
            "slot range is inverted: its lowest index exceeds its highest, so it admits nothing"
        );
        assert!(
            Self::WIDTH.count() >= 1,
            "a declared width of zero bits admits no values and is not a slot range"
        );
        assert!(
            Self::WIDTH.count() <= 62,
            "declared width is wider than a slot index carries; the count of slots is 2^width and \
             2^63 does not fit a signed 64-bit integer"
        );
        // In `i128`, so the obligation cannot overflow while checking that the
        // thing it is about does not. An earlier version of this checked only
        // `MIN <= MAX` and the width's range, and admitted a span of 2^63, which
        // made `slot_count` panic under `overflow-checks` and wrap without it.
        assert!(
            (Self::MAX as i128) - (Self::MIN as i128) < i64::MAX as i128,
            "slot range spans more indices than a count can carry, so counting it would overflow"
        );
        assert!(
            (Self::MAX as i128) - (Self::MIN as i128) < (1i128 << Self::WIDTH.count()),
            "the declared width does not cover the range: the range holds more indices than the \
             width can address"
        );
    };
}

/// Whether a slot range meets what the contract asks of it.
///
/// The law, returning a verdict rather than asserting one, so a construction that
/// compiles and is wrong can be reported on without forcing the const that would
/// refuse it. That is what lets the wrong construction live permanently in a test
/// rather than in a scratch file somebody deletes.
#[must_use]
pub const fn is_admissible<S: Slots>() -> bool {
    S::MIN <= S::MAX
        && S::WIDTH.count() >= 1
        && S::WIDTH.count() <= 62
        && (S::MAX as i128) - (S::MIN as i128) < i64::MAX as i128
        && (S::MAX as i128) - (S::MIN as i128) < (1i128 << S::WIDTH.count())
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
    let () = S::ADMITTED;
    slot >= S::MIN && slot <= S::MAX
}

/// How many slots the range admits.
///
/// Bounded by the impl set: the widest admitted declaration gives `2^62`, which
/// fits. Nothing derives a width from this, and it is kept because a cardinality
/// is a real thing to ask a range for.
#[must_use]
pub const fn slot_count<S: Slots>() -> i64 {
    let () = S::ADMITTED;
    S::MAX - S::MIN + 1
}

/// The width the declaration stated.
#[must_use]
pub const fn declared_slot_width<S: Slots>() -> Width {
    let () = S::ADMITTED;
    S::WIDTH
}
