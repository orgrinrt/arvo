//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The exact rationals the suites check positions against, and the table of an
//! adapted binary operation over every pair of members.
//!
//! Both are oracles: nothing in them comes from the operation under test, so a
//! law checked against them is checked against arithmetic the operation did
//! not do. They live here because more than one operation's suite reads them.

use crate::apply::{Exact, Remainder};
use crate::format::Format;
use crate::slots::Slot;

/// A ratio in the wide carrier, in lowest terms with a positive denominator.
///
/// The oracle's number type. Nothing in it comes from the step, so a law checked
/// against it is checked against arithmetic the step did not do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Ratio {
    pub(crate) num: i128,
    pub(crate) den: i128,
}

/// Euclid's algorithm, the oracle's own copy.
///
/// Kept apart from the step's on purpose: an oracle sharing the implementation's
/// reduction would agree with it about any mistake the reduction makes.
fn common_factor(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 { 1 } else { a }
}

impl Ratio {
    /// `num / den`, normalised.
    pub(crate) fn of(num: i128, den: i128) -> Self {
        assert!(den != 0, "the oracle was handed a ratio over zero");
        let (num, den) = if den < 0 { (-num, -den) } else { (num, den) };
        let g = common_factor(num, den);
        Self {
            num: num / g,
            den: den / g,
        }
    }

    /// A whole number.
    pub(crate) fn whole(n: i128) -> Self {
        Self::of(n, 1)
    }

    /// The phase a format declares, as the ratio it names.
    pub(crate) fn phase_of<F: Format>() -> Self {
        Self::of(F::PHASE.numerator() as i128, F::PHASE.denominator() as i128)
    }

    /// The sum, over the least common denominator so no product runs away.
    pub(crate) fn plus(self, other: Self) -> Self {
        let g = common_factor(self.den, other.den);
        let den = self.den / g * other.den;
        Self::of(
            self.num * (den / self.den) + other.num * (den / other.den),
            den,
        )
    }

    /// The difference.
    pub(crate) fn minus(self, other: Self) -> Self {
        self.plus(Self {
            num: -other.num,
            den: other.den,
        })
    }

    /// The product, with the cross factors taken out first.
    pub(crate) fn times(self, other: Self) -> Self {
        let g1 = common_factor(self.num, other.den);
        let g2 = common_factor(other.num, self.den);
        Self::of(
            (self.num / g1) * (other.num / g2),
            (self.den / g2) * (other.den / g1),
        )
    }

    /// The quotient.
    pub(crate) fn over(self, other: Self) -> Self {
        self.times(Self::of(other.den, other.num))
    }

    /// The greatest whole number not above it.
    pub(crate) fn floor(self) -> i128 {
        self.num.div_euclid(self.den)
    }

    /// Whether it is a whole number.
    pub(crate) fn is_whole(self) -> bool {
        self.den == 1
    }

    /// The position a ratio of slots names: the slot below it and the remainder,
    /// which is in `[0, 1)` and so carries nothing into the slot.
    pub(crate) fn position(self) -> Exact {
        let whole = self.floor();
        let rest = Self::of(self.num.rem_euclid(self.den), self.den);
        Exact::at(
            Slot::at(whole),
            Remainder::of(rest.num as u128, rest.den as u128),
        )
    }
}
