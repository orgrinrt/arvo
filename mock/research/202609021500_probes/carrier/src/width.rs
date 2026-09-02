//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The stack's own primitives, defined here because something has to be first.
//!
//! This crate introduces the numeric category, so the bare-primitive lints skip
//! it. That door exists for exactly one situation, which is this one: the crate
//! that defines
//! what a number is in this stack cannot express itself in types it has not
//! defined yet. Every crate above uses what is here and is checked normally.
//!
//! Two types and no more. A count of bits, and a truth value.
//!
//! The coordinates a format is declared with are not a third and a fourth of
//! these. Each is built through this door and each lives beside the contract that
//! reads it: a radix with the ambient domain, an exponent and the magnitudes with
//! the quantum law, a slot and a count of them with the slot range, a phase with
//! the format, an arity with an operation, a fraction with the applied map. That
//! is what keeps the door narrow while nothing above it has to reach past it.

/// A count of bits.
///
/// Used wherever the design says a width: a declared width, a carrier's capacity,
/// an access width, a stride. It is a count and not a value in any format, which
/// is why it is here rather than being a numeral.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Width(u32);

impl Width {
    /// No bits at all, which is what a derivation returns when it has no answer.
    pub const NONE: Self = Self(0);

    /// A width from a count of bits.
    #[must_use]
    pub const fn bits(n: u32) -> Self {
        Self(n)
    }

    /// The count, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one. `repr(transparent)` and this accessor
    /// are the whole observation surface, so the invariant this type carries is
    /// what its constructors establish and nothing widens it.
    #[must_use]
    pub const fn count(self) -> u32 {
        self.0
    }

    /// Whether this width is the absent one.
    #[must_use]
    pub const fn is_none(self) -> Bool {
        Bool::of(self.0 == 0)
    }

    /// Whether this width covers another.
    #[must_use]
    pub const fn covers(self, other: Self) -> Bool {
        Bool::of(self.0 >= other.0)
    }

    /// The sum of two widths.
    #[must_use]
    pub const fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }

    /// This width less one bit, saturating at none.
    #[must_use]
    pub const fn less_one(self) -> Self {
        if self.0 == 0 {
            Self(0)
        } else {
            Self(self.0 - 1)
        }
    }

    /// Whether two widths are the same.
    #[must_use]
    pub const fn equals(self, other: Self) -> Bool {
        Bool::of(self.0 == other.0)
    }
}

/// A truth value.
///
/// The stack's own, so a predicate in a signature is this rather than the host's.
/// It carries no more than the host's does; what it buys is that a position
/// meaning "yes or no" says so in a type the stack owns.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bool(bool);

impl Bool {
    /// Yes.
    pub const TRUE: Self = Self(true);
    /// No.
    pub const FALSE: Self = Self(false);

    /// A truth value from the host's.
    #[must_use]
    pub const fn of(b: bool) -> Self {
        Self(b)
    }

    /// The host's, for the one place a control-flow construct needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn get(self) -> bool {
        self.0
    }

    /// Both.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 && other.0)
    }

    /// Either.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 || other.0)
    }

    /// The opposite.
    #[must_use]
    pub const fn not(self) -> Self {
        Self(!self.0)
    }
}
