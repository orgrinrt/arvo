//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ambient domain a format's values are drawn from.
//!
//! Half of a format's identity, the other half being the representable set. Two
//! formats with the same representable set under different ambient algebras are
//! two formats, which is why the domain is carried rather than assumed.
//!
//! The radix is a coordinate of the domain and carries a type of its own, so a
//! crate outside this one supplies it in the same type the shipped domains do.

use crate::width::Bool;

/// The base a positional notation counts in.
///
/// A coordinate of the domain rather than of the step law: the same law at radix
/// two and radix ten describes different values. It is a base and not a count of
/// anything, which is why it is this rather than a `Width`.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Radix(u32);

impl Radix {
    /// Base two.
    pub const BINARY: Self = Self(2);

    /// Base ten.
    pub const DECIMAL: Self = Self(10);

    /// A radix from a base.
    #[must_use]
    pub const fn of(base: u32) -> Self {
        Self(base)
    }

    /// The base, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one. `repr(transparent)` and this accessor are
    /// the whole observation surface.
    #[must_use]
    pub const fn base(self) -> u32 {
        self.0
    }

    /// Whether two domains count in the same base.
    #[must_use]
    pub const fn equals(self, other: Self) -> Bool {
        Bool::of(self.0 == other.0)
    }
}

/// The domain a representable set is a subset of.
///
/// The radix sits here because it is a property of the domain rather than of the
/// step law: the same law at radix 2 and radix 10 describes different values.
pub trait Ambient {
    /// The base positional notation counts in.
    const RADIX: Radix;

    /// Whether the domain carries values below zero.
    ///
    /// Read as a coordinate of the domain rather than of the storage. A format
    /// over an unsigned domain has no negative member regardless of what any
    /// carrier could hold.
    const SIGNED: Bool;
}

/// The rationals at radix two, which is where the fixed-point and binary floating
/// families live.
pub struct BinaryRationals;

impl Ambient for BinaryRationals {
    const RADIX: Radix = Radix::BINARY;
    const SIGNED: Bool = Bool::TRUE;
}

/// The non-negative rationals at radix two.
pub struct UnsignedBinaryRationals;

impl Ambient for UnsignedBinaryRationals {
    const RADIX: Radix = Radix::BINARY;
    const SIGNED: Bool = Bool::FALSE;
}

/// The rationals at radix ten, which is where the decimal conventions live.
///
/// Present because the radix is a coordinate and a design that carried only one
/// value of it would have hardcoded a choice the canon leaves open. Nothing in
/// this crate is specialised to radix two.
pub struct DecimalRationals;

impl Ambient for DecimalRationals {
    const RADIX: Radix = Radix::DECIMAL;
    const SIGNED: Bool = Bool::TRUE;
}
