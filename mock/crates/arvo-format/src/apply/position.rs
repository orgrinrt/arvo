//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the applied map takes: an exact position, and the dither one mode reads.
//!
//! A position is a slot and a remainder within one step, both in the format's
//! own coordinates. Where a carry takes the slot past an end of the index, the
//! slot pins at that end and the distance past it is kept beside it, so the
//! position is still the one named.

use crate::slots::Slot;
use crate::width::Bool;

/// A ratio with a positive denominator.
///
/// What sits between two grid points, in both of the places this file needs one:
/// the remainder of an exact position, and the decision the stochastic mode reads.
/// Its own contract is only that the denominator is positive, and the tighter
/// `[0, 1)` reading belongs to `Exact`, which normalises into its slot rather than
/// refusing what a caller computed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fraction {
    pub(super) num: i64,
    pub(super) den: i64,
}

impl Fraction {
    /// Exactly half way, which is the tie every rounding rule has an answer for.
    pub const HALF: Self = Self {
        num: 1,
        den: 2,
    };
    /// Nothing between the grid points, which is a position already on one.
    pub const ZERO: Self = Self {
        num: 0,
        den: 1,
    };

    /// A ratio of `num` over `den`.
    ///
    /// Total, and no branch here is a check. The exact operation is the ratio in
    /// the ambient rationals, the representable set is the pairs this type
    /// carries with a positive denominator, and the constructor is the adaptation
    /// between them. So it has two regions, the way the applied map does.
    ///
    /// **Exact normalisation is the first, and it covers all but one condition.**
    /// A negative denominator names a value exactly, so the sign moves to the
    /// numerator and `of(3, -7)` is `-3/7`. Where one operand is `i64::MIN` the
    /// pair still reduces whenever the other is even, because the two then share
    /// a factor of two and cancelling it lands both inside the type:
    /// `of(i64::MIN, -2)` is `2^62` over one, exactly.
    ///
    /// **Magnitude saturation is the second, and it applies where the other
    /// operand is odd.** Nothing cancels then, so writing the exact form would
    /// want `2^63` in one of the two positions, which the type does not carry.
    /// That operand takes the largest the type does carry instead, and it is
    /// the numerator or the denominator depending on which side the `i64::MIN`
    /// sat. The answer keeps the sign of the ratio that was named and sits
    /// within a relative `1 / i64::MAX` of it, on whichever side the operand
    /// moved. That is the same act `complete_slot` performs on a slot leaving
    /// its range, one coordinate up.
    ///
    /// **A zero denominator is neither region.** It names no ratio at all, so
    /// there is nothing for an answer to be near, and it reads as `ZERO`.
    #[must_use]
    pub const fn of(num: i64, den: i64) -> Self {
        if den > 0 {
            return Self {
                num,
                den,
            };
        }
        if den == 0 {
            return Self::ZERO;
        }
        if num != i64::MIN && den != i64::MIN {
            return Self {
                num: -num,
                den: -den,
            };
        }
        if num == 0 {
            // The denominator is `i64::MIN` and the ratio is zero, which `ZERO`
            // names exactly.
            return Self::ZERO;
        }
        if num == i64::MIN && den == i64::MIN {
            return Self {
                num: 1,
                den: 1,
            };
        }
        // Exactly one operand is `i64::MIN` and the other decides how far the
        // pair cancels. That other one is neither zero nor `i64::MIN`, both of
        // which the branches above answered, so it carries at most 62 factors of
        // two: each shift below is an exact division and each negation has room.
        let other = if den == i64::MIN { num } else { den };
        let k = other.trailing_zeros();
        if k > 0 {
            return Self {
                num: -(num >> k),
                den: -(den >> k),
            };
        }
        if den == i64::MIN {
            Self {
                num: -num,
                den: i64::MAX,
            }
        } else {
            Self {
                num: i64::MAX,
                den: -den,
            }
        }
    }

    /// The numerator, for the one place a host contract needs it back.
    ///
    /// The unwrap door, declared as one.
    #[must_use]
    pub const fn numerator(self) -> i64 {
        self.num
    }

    /// The denominator, positive by construction.
    ///
    /// The second unwrap door, because the pair is what the ratio is.
    #[must_use]
    pub const fn denominator(self) -> i64 {
        self.den
    }

    /// Whether the ratio is nothing at all.
    #[must_use]
    pub const fn is_zero(self) -> Bool {
        Bool::of(self.num == 0)
    }
}

/// An exact result, in the format's own coordinates.
///
/// The value is `phase + (slot + past + remainder) * quantum(magnitude)`. The
/// remainder is carried as a `Fraction` rather than as an approximation so that
/// an exactly-half position is representable, which is what makes a tie rule
/// testable rather than a matter of what the host's arithmetic happened to do.
///
/// The remainder is in `[0, 1)`: its numerator is non-negative and less than its
/// denominator. A position is therefore always between one slot and the next,
/// which is what lets the rounding modes be stated once rather than once per sign.
///
/// `past` is how far beyond the end of the index the named slot lies, and it is
/// zero for every position the index holds. Where it is not, `slot` is the end it
/// lies past, above it for a positive distance and below it for a negative one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Exact {
    pub(super) slot: Slot,
    pub(super) part: Fraction,
    pub(super) past: i64,
}

impl Exact {
    /// A position already on the grid.
    #[must_use]
    pub const fn on_grid(slot: Slot) -> Self {
        Self {
            slot,
            part: Fraction::ZERO,
            past: 0,
        }
    }

    /// A position between `slot` and `slot + 1`, at `part` of the way.
    ///
    /// A remainder outside `[0, 1)` is normalised into the slot rather than
    /// refused, because a caller computing an exact result should not have to
    /// carry the invariant. That normalisation is why `Fraction` does not hold the
    /// `[0, 1)` bound itself: holding it there would drop the carry.
    ///
    /// The carry is at most `2^63` either way and lands as an ordinary slot
    /// wherever the index holds the sum. Where it does not, which is only from a
    /// slot within `2^63` of either end, the slot pins at that end and the
    /// distance past it is kept, which is at most the carry and so fits the
    /// fraction's own integer. So `between` is total over every slot and every
    /// ratio and names the position it was asked for:
    /// `between(Slot::at(i128::MAX), 9/4)` is the slot `i128::MAX`, two past it,
    /// and a quarter.
    #[must_use]
    pub const fn between(slot: Slot, part: Fraction) -> Self {
        let whole = part.num.div_euclid(part.den);
        let rem = part.num.rem_euclid(part.den);
        let part = Fraction {
            num: rem,
            den: part.den,
        };
        match slot.index().checked_add(whole as i128) {
            Some(at) => {
                Self {
                    slot: Slot::at(at),
                    part,
                    past: 0,
                }
            },
            // The sum left the index, so it left through the end on the carry's
            // side, and the distance past that end is at most the carry.
            None => {
                let end = if whole > 0 { i128::MAX } else { i128::MIN };
                Self {
                    slot: Slot::at(end),
                    part,
                    past: ((slot.index() - end) + whole as i128) as i64,
                }
            },
        }
    }

    /// The slot the position sits on or just above, or for a position past an
    /// end of the index, that end.
    #[must_use]
    pub const fn slot(self) -> Slot {
        self.slot
    }

    /// Whether the position is exactly on a grid point.
    #[must_use]
    pub const fn is_on_grid(self) -> Bool {
        self.part.is_zero()
    }

    /// Whether the position is exactly half way between two grid points.
    ///
    /// Cross-multiplied in the wide carrier, which is what `round_slot` does with
    /// the same comparison. The stored remainder reaches one below the
    /// denominator, so doubling it in the coordinate's own carrier leaves the
    /// type on any remainder above half the carrier's maximum, and a verdict
    /// function that diverges is not one.
    #[must_use]
    pub const fn is_tie(self) -> Bool {
        Bool::of((self.part.num as i128) * 2 == self.part.den as i128)
    }

    /// Whether the slot the position names is below zero.
    ///
    /// Past an end of the index it is the end's sign, and otherwise the slot's.
    #[must_use]
    pub(crate) const fn is_negative(self) -> bool {
        if self.past == 0 { self.slot.index() < 0 } else { self.past < 0 }
    }

    /// Whether the slot the position names is even.
    ///
    /// The pinned slot's parity and the distance's together, reduced one at a
    /// time so nothing is formed past the index.
    #[must_use]
    pub(crate) const fn is_even(self) -> bool {
        (self.slot.index().rem_euclid(2) + (self.past as i128).rem_euclid(2)) % 2 == 0
    }
}

/// The decision the stochastic mode reads, supplied by the caller.
///
/// Five of the six modes are a function of the position alone. The sixth is not,
/// and how it should be seeded and whether it is keyed on value or on position are
/// both open questions in the register. Taking the decision as an input is what
/// lets the mode be expressed without answering either.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dither(pub(super) Fraction);

impl Dither {
    /// The value to pass where the adaptation names no stochastic mode.
    ///
    /// It is read by no other mode, so the branch reading it is dead wherever the
    /// mode is not stochastic and goes away with the monomorphisation.
    pub const UNUSED: Self = Self(Fraction::ZERO);

    /// A dither at `part` of the way between two grid points.
    #[must_use]
    pub const fn at(part: Fraction) -> Self {
        Self(part)
    }
}
