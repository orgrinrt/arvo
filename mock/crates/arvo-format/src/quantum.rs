//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The quantum law: how the step between neighbouring values changes with magnitude.
//!
//! The canon says membership is one affine predicate, so the law is affine in the
//! magnitude and carries two coordinates rather than a function. A slope of zero
//! is the constant-quantum family, where integers, fixed point and scaled integers
//! live. A nonzero slope is the magnitude-indexed family, where floating point
//! lives and where subnormals fall out of the smallest magnitude rather than out
//! of a special case.
//!
//! Carrying it as coordinates instead of a method is what lets membership be a
//! free `const fn` over a format's associated items. A trait method would have to
//! be const to be reachable at stage zero, and that needs a feature this crate is
//! not permitted.
//!
//! The three coordinates below carry types of their own, and an index into the
//! magnitudes is not the same type as a count of them. That distinction is the one
//! this crate already refuses to leave to a reader's care in the slot range, and
//! carrying both as one host integer is the same hazard with nothing done about it.

use crate::width::Bool;

/// A signed power of the ambient domain's radix.
///
/// Both coordinates of the step law are one of these. The intercept is the
/// exponent at magnitude zero and the rate is how much that exponent moves per
/// magnitude, and they sit on the same additive axis, which is why the law adds
/// them rather than combining two kinds.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Exponent(i32);

impl Exponent {
    /// The zeroth power, which is the integers' step and a flat law's rate.
    pub const ZERO: Self = Self(0);

    /// One power, which is the rate the magnitude-indexed family runs at.
    pub const ONE: Self = Self(1);

    /// An exponent from a power.
    #[must_use]
    pub const fn of(power: i32) -> Self {
        Self(power)
    }

    /// The power, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn power(self) -> i32 {
        self.0
    }

    /// This exponent advanced by a rate over a magnitude.
    ///
    /// The step law's whole arithmetic, in one place, so a caller never
    /// reconstructs `base + slope * magnitude` out of three unwrapped host
    /// integers and gets the multiplication the wrong way round.
    #[must_use]
    pub const fn advanced(self, rate: Self, magnitude: Magnitude) -> Self {
        Self(self.0 + rate.0 * (magnitude.0 as i32))
    }

    /// The smaller of two exponents.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }
}

/// An index into a quantum law's magnitudes.
///
/// An index and not a count. The value it names is the position on the magnitude
/// axis the step law is being asked about, and a law over `n` magnitudes has
/// indices `0` through `n - 1`.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Magnitude(u32);

impl Magnitude {
    /// The smallest magnitude, which is where a constant law's only step sits and
    /// where the floating conventions' subnormals fall out.
    pub const SMALLEST: Self = Self(0);

    /// A magnitude at an index.
    #[must_use]
    pub const fn at(index: u32) -> Self {
        Self(index)
    }

    /// The index, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }

    /// Whether this index is one a law of that many magnitudes ranges over.
    ///
    /// The index-against-extent comparison, written once here rather than at every
    /// site that would otherwise have to remember which side the equality falls on.
    #[must_use]
    pub const fn is_within(self, count: MagnitudeCount) -> Bool {
        Bool::of(self.0 < count.0)
    }
}

/// How many magnitudes a quantum law ranges over.
///
/// An extent and not an index, which is the whole reason it is not a `Magnitude`.
/// One for the constant family, and never zero: a law over no magnitudes describes
/// no values.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MagnitudeCount(u32);

impl MagnitudeCount {
    /// One magnitude, which is the constant family's whole axis.
    pub const ONE: Self = Self(1);

    /// A count of magnitudes.
    #[must_use]
    pub const fn of(count: u32) -> Self {
        Self(count)
    }

    /// The count, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn count(self) -> u32 {
        self.0
    }

    /// The largest index this extent admits.
    ///
    /// Saturating at the smallest magnitude, so an extent of zero names an index
    /// rather than wrapping to the largest one there is. An extent of zero admits
    /// nothing and `is_within` says so, which is where that case is answered.
    #[must_use]
    pub const fn largest(self) -> Magnitude {
        if self.0 == 0 {
            Magnitude::SMALLEST
        } else {
            Magnitude(self.0 - 1)
        }
    }
}

/// How the quantum varies with magnitude, as an affine law.
///
/// The quantum at magnitude `m` is `radix^(BASE + SLOPE * m)`. The radix lives on
/// the ambient domain because it is a property of the domain rather than of the
/// step law.
pub trait Quantum {
    /// The exponent at magnitude zero.
    const BASE: Exponent;

    /// The change in exponent per step of magnitude. Zero for the constant family.
    const SLOPE: Exponent;

    /// How many distinct magnitudes the law ranges over. One for the constant
    /// family, and never zero: a law over no magnitudes describes no values.
    const MAGNITUDES: MagnitudeCount;
}

/// A step that does not change with magnitude.
///
/// `EXP` is the exponent, so `EXP = 0` is the integers and `EXP = -F` is fixed
/// point at fraction width `F`.
pub struct Constant<const EXP: i32>;

impl<const EXP: i32> Quantum for Constant<EXP> {
    const BASE: Exponent = Exponent::of(EXP);
    const SLOPE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::ONE;
}

/// A step that grows by one exponent per magnitude, which is the floating shape.
///
/// `MIN_EXP` is the exponent at the smallest magnitude and `COUNT` is how many
/// magnitudes there are.
pub struct Indexed<const MIN_EXP: i32, const COUNT: u32>;

impl<const MIN_EXP: i32, const COUNT: u32> Quantum for Indexed<MIN_EXP, COUNT> {
    const BASE: Exponent = Exponent::of(MIN_EXP);
    const SLOPE: Exponent = Exponent::ONE;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(COUNT);
}

/// The exponent of the quantum at a magnitude.
///
/// Free rather than a trait method so it is callable at stage zero without the
/// trait having to be const.
#[must_use]
pub const fn exponent_at<Q: Quantum>(magnitude: Magnitude) -> Exponent {
    Q::BASE.advanced(Q::SLOPE, magnitude)
}

/// Whether a magnitude is one the law ranges over.
#[must_use]
pub const fn magnitude_in_range<Q: Quantum>(magnitude: Magnitude) -> Bool {
    magnitude.is_within(Q::MAGNITUDES)
}

/// Whether the law is the constant-quantum family.
///
/// This is the axis the two families differ on and the only one, so a claim about
/// one family is a claim predicated on this.
#[must_use]
pub const fn is_constant_family<Q: Quantum>() -> Bool {
    Bool::of(Q::SLOPE.power() == Exponent::ZERO.power())
}
