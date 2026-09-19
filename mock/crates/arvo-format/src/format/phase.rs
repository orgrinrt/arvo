//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The phase coordinate, the grid's offset from zero.

use crate::width::Bool;

/// The grid's offset from zero, in units of the quantum at magnitude zero.
///
/// A ratio, so the half-step bias is exact rather than approximated, and one
/// coordinate rather than two consts an implementor can put out of step with each
/// other. The parameterisation carries a phase, singular, and this is it.
///
/// It holds the pair it was declared with. Nothing here normalises the sign
/// or reduces the fraction. Both questions asked of it, whether the ratio is a
/// whole number and what that whole number is, are independent of the sign of the
/// denominator, and no other reader consults it, so an invariant that a positive
/// denominator would buy has no buyer. What it would cost is real: two writable
/// pairs have no normalisation inside the width the coordinates are declared in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Phase {
    num: i64,
    den: i64,
}

impl Phase {
    /// No offset, which leaves the additive identity at slot zero.
    pub const ZERO: Self = Self {
        num: 0,
        den: 1,
    };

    /// A phase of `num` over `den`.
    ///
    /// Total, and lossless on every pair. The numerator and denominator come back
    /// as they went in, so `of(3, i64::MIN)` is exactly three over the least
    /// value and not a tiny positive wearing its sign, and `of(i64::MIN, -7)` is
    /// exactly that ratio and not its negation.
    ///
    /// A denominator of zero is not refused here and is not reinterpreted
    /// either. It names no position on the grid, so the phase does not denote,
    /// and that is a condition on the format rather than on the pair:
    /// `Format::ADMITTED` refuses it where the coordinates are declared together,
    /// which is the same shape `Slots`, `Quantum` and `Ambient` already carry.
    /// Reading it as a denominator of one is the one thing that cannot be
    /// right, because one over zero and one over one are different positions
    /// and only one of them exists.
    #[must_use]
    pub const fn of(num: i64, den: i64) -> Self {
        Self {
            num,
            den,
        }
    }

    /// A phase counted in half steps, which is the biased grid's shape.
    #[must_use]
    pub const fn halves(num: i64) -> Self {
        Self {
            num,
            den: 2,
        }
    }

    /// The numerator, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn numerator(self) -> i64 {
        self.num
    }

    /// The denominator, for the one place a host contract needs it back.
    ///
    /// The second unwrap door. Two, rather than one, because the pair is what the
    /// coordinate is and handing back a single number would mean dividing.
    #[must_use]
    pub const fn denominator(self) -> i64 {
        self.den
    }

    /// Whether the pair names a position at all.
    ///
    /// The condition `Format::ADMITTED` refuses, written over the coordinate so
    /// the obligation and the verdict read the same predicate rather than
    /// restating it.
    #[must_use]
    pub const fn denotes(self) -> Bool {
        Bool::of(self.den != 0)
    }

    /// Whether the grid sits at zero.
    ///
    /// Not the question that decides the additive identity, which is the one
    /// below. A grid at zero has zero on it at slot zero; a grid one whole step
    /// along has zero on it one slot down, and this answers no about the second.
    #[must_use]
    pub const fn is_zero(self) -> Bool {
        Bool::of(self.num == 0)
    }

    /// Whether the offset is a whole number of quanta at magnitude zero.
    ///
    /// A phase of one whole step shifts the grid onto itself, so zero stays on it
    /// at a shifted slot, and only a fractional part takes it off there.
    ///
    /// This is the constant family's answer and not the general one. The
    /// phase is stated in units of the quantum at magnitude zero, and a law whose
    /// step moves with the magnitude names a different quantum at each one, so a
    /// ratio with a fractional part here can be a whole number of steps higher up.
    /// `has_additive_identity` is the question over the whole magnitude range;
    /// this is one coordinate of it, kept because the reduction is what the
    /// constant family's arms are about.
    ///
    /// Computed one domain wider than the coordinates. The least value over
    /// minus one overflows a remainder taken in the declared width, and it is a
    /// pair an implementor can write.
    #[must_use]
    pub const fn is_whole_multiple(self) -> Bool {
        if self.den == 0 {
            return Bool::FALSE;
        }
        Bool::of((self.num as i128) % (self.den as i128) == 0)
    }
}
