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
//! That sentence is a type here rather than a note. `Slot` is the index and
//! `SlotCount` is how many of them a range admits, and the two do not convert into
//! each other, so the off-by-one the inclusive bounds below worry about cannot be
//! written by handing a count where an index was wanted.
//!
//! The declared width is carried rather than recovered. A declaration of
//! thirteen bits knows it is thirteen bits, and reconstructing that from the slot
//! bounds by counting is a computation over a quantity nothing needs.
//!
//! The bound on how wide a declaration may be is the set of impls below and not
//! a check inside a function. A check is a thing a later edit can delete while
//! every test at admitted widths stays green. An absent impl cannot be deleted.
//!
//! That bounds this crate's impls and not the trait, which is open. A numeral
//! wanting a slot range that is neither shipped shape has to be able to supply
//! one, so nothing here is sealed. What an outside implementor owes is stated on
//! the trait as `ADMITTED`, and it is checked at compile time rather than asked
//! for in a comment.

use crate::width::{Bool, Width};

/// An index of a multiple of the quantum.
///
/// An index and never a value. The value a slot denotes is the phase plus the slot
/// times the quantum at its magnitude, and nothing in this crate hands one out as
/// a number in the ambient domain.
///
/// Carried in a signed 128-bit integer, because an unsigned 64-bit range ends at
/// `2^64 - 1` and no signed 64-bit integer holds that. The ranges this crate ships
/// sit far inside it, so the arithmetic over two of their slots stays exact; a
/// range an outside crate places near either end is where the checked and the
/// saturating computations elsewhere in the crate take over.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Slot(i128);

impl Slot {
    /// The slot at zero, which is where the additive identity sits on an unbiased
    /// grid and where it does not sit on a biased one.
    pub const ZERO: Self = Self(0);

    /// A slot at an index.
    #[must_use]
    pub const fn at(index: i128) -> Self {
        Self(index)
    }

    /// The index, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one. `repr(transparent)` and this accessor are
    /// the whole observation surface.
    #[must_use]
    pub const fn index(self) -> i128 {
        self.0
    }

    /// Whether this slot is no higher than another.
    #[must_use]
    pub const fn is_at_most(self, other: Self) -> Bool {
        Bool::of(self.0 <= other.0)
    }

    /// Whether this slot falls between two bounds, inclusive at both ends.
    #[must_use]
    pub const fn is_within(self, low: Self, high: Self) -> Bool {
        Bool::of(self.0 >= low.0 && self.0 <= high.0)
    }
}

/// How many slots a range admits.
///
/// An extent and not an index, on the same reading that separates a magnitude from
/// a count of them. A range from `-4` to `3` admits eight slots and its highest
/// index is three, and the two numbers are never interchangeable.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SlotCount(i128);

impl SlotCount {
    /// A count of slots.
    #[must_use]
    pub const fn of(count: i128) -> Self {
        Self(count)
    }

    /// The count, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn count(self) -> i128 {
        self.0
    }
}

/// Which slot indices a format admits.
///
/// Inclusive at both ends, because the alternative is an off-by-one nobody can
/// see in a const.
///
/// This crate implements it at widths 1 through 64 and at no other width. 64 is
/// the widest pointer width a target has and the widest carrier the placement
/// ladder offers; the slot index's own integer would count further and is not
/// what stops it.
///
/// The trait is open and an outside crate may implement it. What an implementor
/// owes is `ADMITTED`, below, and that is checked rather than requested.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not an admitted slot range",
    label = "no slot range exists for this width",
    note = "this crate implements the trait at widths 1 to 64, the widest pointer width a target \
            has and the widest carrier the placement ladder offers. The trait is open, so another \
            crate may implement it at a range of its own; what such an implementor owes is the \
            `ADMITTED` obligation."
)]
pub trait Slots {
    /// The lowest admitted slot index.
    const MIN: Slot;

    /// The highest admitted slot index.
    const MAX: Slot;

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
    /// It fires at codegen, not at `cargo check`, where a runtime call forces it.
    /// A const is evaluated when the instantiation is codegened and `check` skips
    /// that, so `cargo build` refuses and `cargo check` does not. The guarantee is
    /// that an inadmissible range cannot reach a produced binary; it can reach a
    /// passing check.
    ///
    /// Where the obligation is forced decides which tool can see the refusal.
    /// A const is evaluated where it is used, so a runtime call reaches it only at
    /// codegen and a `const` item reaches it at check time. A doctest builds a
    /// binary and catches both; a `trybuild` case runs `cargo check` and catches
    /// the second, which is why the ones in `tests/ui/` bind the predicate to a
    /// `const` rather than calling it from `main`.
    ///
    /// ```compile_fail
    /// use arvo_format::slots::{slot_in_range, Slot, Slots};
    /// use arvo_format::width::Width;
    ///
    /// struct Inverted;
    ///
    /// impl Slots for Inverted {
    ///     const MIN: Slot = Slot::at(8);
    ///     const MAX: Slot = Slot::at(-8);
    ///     const WIDTH: Width = Width::bits(8);
    /// }
    ///
    /// fn main() {
    ///     let _ = slot_in_range::<Inverted>(Slot::ZERO);
    /// }
    /// ```
    ///
    /// The control, which is what says the refusal above is this obligation and
    /// not the outside impl being rejected for some other reason: the same shape
    /// with the lower index below the higher one builds.
    ///
    /// ```
    /// use arvo_format::slots::{Slot, Slots, slot_in_range};
    /// use arvo_format::width::Width;
    ///
    /// struct Ordered;
    ///
    /// impl Slots for Ordered {
    ///     const MAX: Slot = Slot::at(7);
    ///     const MIN: Slot = Slot::at(-8);
    ///     const WIDTH: Width = Width::bits(8);
    /// }
    ///
    /// fn main() {
    ///     assert!(slot_in_range::<Ordered>(Slot::ZERO).get());
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
            Self::MIN.index() <= Self::MAX.index(),
            "slot range is inverted: its lowest index exceeds its highest, so it admits nothing"
        );
        assert!(
            Self::WIDTH.count() >= 1,
            "a declared width of zero bits admits no values and is not a slot range"
        );
        assert!(
            Self::WIDTH.count() <= 64,
            "declared width is wider than 64 bits, the widest pointer width a target has and the \
             widest carrier the placement ladder offers"
        );
        // The span is computed checked, because an outside range may place its
        // ends anywhere the index reaches and the difference of two such ends
        // can leave it. A span the index cannot state is wider than any admitted
        // width addresses, so the overflow is this refusal and not another one.
        // No separate condition bounds the count: a width of at most 64 that
        // addresses the span caps it at `2^64`, which a count holds with room.
        assert!(
            width_addresses_span(Self::MIN, Self::MAX, Self::WIDTH),
            "the declared width does not cover the range: the range holds more indices than the \
             width can address"
        );
    };
}

/// Whether a width addresses every slot between two ends, inclusive.
///
/// Asked only of a width already known to be at most 64, so the shift below is
/// defined; both callers establish that first.
const fn width_addresses_span(min: Slot, max: Slot, width: Width) -> bool {
    match max.0.checked_sub(min.0) {
        Some(span) => span < (1i128 << width.count()),
        None => false,
    }
}

/// Whether a slot range meets what the contract asks of it.
///
/// The law, returning a verdict rather than asserting one, so a construction that
/// compiles and is wrong can be reported on without forcing the const that would
/// refuse it. That is what lets the wrong construction live permanently in a test
/// rather than in a scratch file somebody deletes.
#[must_use]
pub const fn is_admissible<S: Slots>() -> Bool {
    Bool::of(
        S::MIN.index() <= S::MAX.index()
            && S::WIDTH.count() >= 1
            && S::WIDTH.count() <= 64
            && width_addresses_span(S::MIN, S::MAX, S::WIDTH),
    )
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
/// Adding a width past the bound fails here rather than at some use site: the
/// ends are shifts of the 64-bit host integers, the signed ones from `i64::MIN`
/// and `i64::MAX` and the unsigned top from `u64::MAX`, each shifted right by
/// `64 - width`. At 65 that shift is by a negative amount and refuses to compile
/// where somebody added it. Written as shifts of the 128-bit index instead, the
/// same body would build to 126 and the bound would move into a sentence.
macro_rules! admit_widths {
    ($($w:literal),+ $(,)?) => {
        $(
            impl Slots for Signed<$w> {
                const MIN: Slot = Slot::at((i64::MIN >> (64 - $w)) as i128);
                const MAX: Slot = Slot::at((i64::MAX >> (64 - $w)) as i128);
                const WIDTH: Width = Width::bits($w);
            }

            impl Slots for Unsigned<$w> {
                const MIN: Slot = Slot::ZERO;
                const MAX: Slot = Slot::at((u64::MAX >> (64 - $w)) as i128);
                const WIDTH: Width = Width::bits($w);
            }
        )+

        /// Every width the design admits, for a test that wants the whole set
        /// rather than the widths somebody remembered.
        pub const ADMITTED_WIDTHS: &[Width] = &[$(Width::bits($w)),+];
    };
}

admit_widths!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

/// Whether a slot index is admitted.
#[must_use]
pub const fn slot_in_range<S: Slots>(slot: Slot) -> Bool {
    let () = S::ADMITTED;
    slot.is_within(S::MIN, S::MAX)
}

/// How many slots the range admits.
///
/// Bounded by the obligation: an admitted width addresses the span, so the count
/// is at most `2^64`, which the count's integer holds. Nothing derives a width from this, and it is kept because a cardinality
/// is a real thing to ask a range for.
#[must_use]
pub const fn slot_count<S: Slots>() -> SlotCount {
    let () = S::ADMITTED;
    SlotCount::of(S::MAX.index() - S::MIN.index() + 1)
}

/// The width the declaration stated.
#[must_use]
pub const fn declared_slot_width<S: Slots>() -> Width {
    let () = S::ADMITTED;
    S::WIDTH
}
