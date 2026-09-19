//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Applying an adaptation: the second half of the factoring.
//!
//! Arithmetic on a format is an exact operation in the ambient domain composed
//! with a named total adaptation onto the representable set. Everything else in
//! this crate names adaptations; this is where one gets applied.
//!
//! One map with two regions rather than two mechanisms. Rounding decides between
//! grid points, completion decides outside the range, and the order is rounding
//! then completion because a position may round onto a slot that is out of range.
//! A magnitude bound switches off the completion and not the rounding, and a grid
//! bound does the reverse, which is what two regions of one map predict.
//!
//! The exact step is computed in a domain wide enough to hold it, and that
//! intermediate carries no coordinate type on purpose. It is deliberately wider
//! than a slot index, it lives inside two private functions, and naming it in the
//! surface would publish a quantity nothing outside this file has any business
//! holding.

use crate::adapt::{Adaptation, DeclaredSignature};
use crate::format::Format;
use crate::overflow::{Overflow, Policy};
use crate::rounding::{Mode, Rounding};
use crate::slots::{Slot, Slots};
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
    num: i64,
    den: i64,
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
/// The value is `phase + (slot + remainder) * quantum(magnitude)`. The remainder is
/// carried as a `Fraction` rather than as an approximation so that an exactly-half
/// position is representable, which is what makes a tie rule testable rather than
/// a matter of what the host's arithmetic happened to do.
///
/// The remainder is in `[0, 1)`: its numerator is non-negative and less than its
/// denominator. A position is therefore always between `slot` and `slot + 1`,
/// which is what lets the rounding modes be stated once rather than once per sign.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Exact {
    slot: Slot,
    part: Fraction,
}

impl Exact {
    /// A position already on the grid.
    #[must_use]
    pub const fn on_grid(slot: Slot) -> Self {
        Self {
            slot,
            part: Fraction::ZERO,
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
    /// wherever the index holds the sum. Past the index's own end it saturates
    /// there, like every other position this crate computes, so `between` is
    /// total over every slot and every ratio.
    #[must_use]
    pub const fn between(slot: Slot, part: Fraction) -> Self {
        let whole = part.num.div_euclid(part.den);
        let rem = part.num.rem_euclid(part.den);
        Self {
            slot: Slot::at(slot.index().saturating_add(whole as i128)),
            part: Fraction {
                num: rem,
                den: part.den,
            },
        }
    }

    /// The slot the position sits on or just above.
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
}

/// The decision the stochastic mode reads, supplied by the caller.
///
/// Five of the six modes are a function of the position alone. The sixth is not,
/// and how it should be seeded and whether it is keyed on value or on position are
/// both open questions in the register. Taking the decision as an input is what
/// lets the mode be expressed without answering either.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dither(Fraction);

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

/// Where the rounding region puts a position: the slot at or below it, and
/// whether to step one slot above that.
///
/// A pair rather than the slot above, because at `i128::MAX` the slot above is
/// one past the index, and forming it there either overflows or saturates, and a
/// saturated step is a wrong answer under wrapping and a missed one in the
/// overflow verdict. The pair names every position from the index's lowest to
/// one past its highest, which is all the reach a rounding has, so the
/// completion answers exactly from it with no wider integer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Rounded {
    down: i128,
    up:   Bool,
}

impl Rounded {
    /// A position already on the slot `at`, which rounding leaves where it is.
    #[must_use]
    pub(crate) const fn at(at: i128) -> Self {
        Self {
            down: at,
            up:   Bool::FALSE,
        }
    }

    /// The slot one above `down`, named without forming it.
    #[must_use]
    const fn above(down: i128) -> Self {
        Self {
            down,
            up: Bool::TRUE,
        }
    }

    /// The slot at or below the position.
    #[must_use]
    pub(crate) const fn down(self) -> i128 {
        self.down
    }

    /// The step the rounding adds to `down`, zero or one.
    #[must_use]
    pub(crate) const fn step(self) -> i128 {
        if self.up.get() { 1 } else { 0 }
    }

    /// Whether the rounded position lies in `[lo, hi]`.
    ///
    /// `down + 1` is formed only when `down` is below `lo`, so below the index's
    /// top, and the upper test compares `down` against `hi` without adding.
    #[must_use]
    const fn lands_within(self, lo: i128, hi: i128) -> bool {
        let from_below = self.down >= lo || (self.up.get() && self.down + 1 == lo);
        let from_above = self.down < hi || (self.down == hi && !self.up.get());
        from_below && from_above
    }

    /// The rounded slot as an index, for a test reading a position the index
    /// holds. One past the index's top is not one, and asking for it panics.
    #[cfg(test)]
    #[must_use]
    pub(crate) const fn index(self) -> i128 {
        self.down + self.step()
    }
}

/// Which slot the rounding region returns for a position.
///
/// Stated over a remainder in `[0, 1)`, so the position is between `slot` and
/// `slot + 1` and every mode is one comparison. The sign cases live in the modes
/// that care about sign rather than in the representation.
///
/// Visible to the crate and to nothing outside it, because addition's verdict
/// reads the one offset a rounding adds where that offset is fixed, and reading
/// it through the map rather than restating the six rules is what keeps the two
/// from disagreeing.
#[must_use]
pub(crate) const fn round_slot(mode: Mode, exact: Exact, dither: Dither) -> Rounded {
    if exact.part.num == 0 {
        return Rounded::at(exact.slot.index());
    }
    // The step up is one slot above `down`, carried as a flag and never added,
    // so the answer is exact at the index's top as it is everywhere else.
    let down = Rounded::at(exact.slot.index());
    let up = Rounded::above(exact.slot.index());
    // `2 * num` against `den` decides which side of the midpoint the position is,
    // cross-multiplied in the wide carrier so no operand can leave its type.
    let twice = (exact.part.num as i128) * 2;
    let den = exact.part.den as i128;
    match mode {
        Mode::Floor => down,
        Mode::Ceil => up,
        // The position is negative exactly when the slot is, because the
        // remainder is non-negative and less than one.
        Mode::TowardZero => {
            if exact.slot.index() < 0 {
                up
            } else {
                down
            }
        },
        Mode::HalfUp => {
            if twice > den {
                up
            } else if twice < den {
                down
            } else if exact.slot.index() < 0 {
                // A tie on a negative position goes away from zero, which is down.
                down
            } else {
                up
            }
        },
        Mode::HalfEven => {
            if twice > den {
                up
            } else if twice < den {
                down
            } else if exact.slot.index() % 2 == 0 {
                down
            } else {
                up
            }
        },
        // Up when the dither falls below the position's offset, which makes the
        // probability of rounding up equal to that offset when the dither is
        // uniform. Cross-multiplied so no division happens.
        Mode::Stochastic => {
            if (dither.0.num as i128) * den < (exact.part.num as i128) * (dither.0.den as i128) {
                up
            } else {
                down
            }
        },
    }
}

/// Which slot the completion region returns for a slot outside the range.
///
/// The identity on a slot already inside it, which is what makes the two regions
/// separable rather than one pass that always touches the value.
#[must_use]
const fn complete_slot(policy: Policy, rounded: Rounded, min: Slot, max: Slot) -> Slot {
    let lo = min.index();
    let hi = max.index();
    if rounded.lands_within(lo, hi) {
        // In range, so the sum is at most `hi` and cannot leave the index.
        return Slot::at(rounded.down() + rounded.step());
    }
    match policy {
        Policy::Wrap => {
            // An admitted range spans at most `2^64` slots, so the span fits. The
            // slot below the position and the lowest slot are reduced separately
            // before they are subtracted, because their difference can leave the
            // index when an outside range sits near one end and the position near
            // the other. Each reduction is below the span, so the difference plus
            // the step lies in `(-span, span]` and the sum stays inside the range.
            // The step is added after the reductions rather than to the slot, so
            // a position one past the index's top wraps exactly too.
            let span = hi - lo + 1;
            let offset = rounded.down().rem_euclid(span) - lo.rem_euclid(span) + rounded.step();
            Slot::at(lo + offset.rem_euclid(span))
        },
        // `Clamp` is documented as pinning to a declared bound that need not be
        // the range's own end, and the declared signature carries nowhere to put
        // that bound. With no bound to read it pins to the range, which is
        // saturation. The two agree here because the coordinate that would
        // separate them is missing, which is the admission rule's own diagnosis
        // rather than a shortcut taken in this function.
        // Out of range with the slot below under `lo` means the position is under
        // it too, since a step onto `lo` would have landed in range.
        Policy::Saturate | Policy::Clamp => {
            if rounded.down() < lo {
                min
            } else {
                max
            }
        },
    }
}

/// Apply a declared signature's adaptation to an exact position.
///
/// Total: every position returns a slot the format admits, for every mode and
/// every policy. That totality is what makes an adaptation a member of the slot
/// the factoring names, and it is why a panic is not one.
#[must_use]
pub const fn adapt<S: DeclaredSignature>(exact: Exact, dither: Dither) -> Slot {
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    let policy = <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY;
    // Forces the contract's obligation. Reading `MIN` and `MAX` does not force it
    // on its own, so without this line the completion would work over a range
    // that merely arrived rather than one that was admitted.
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    let min = <<S::Format as Format>::Slots as Slots>::MIN;
    let max = <<S::Format as Format>::Slots as Slots>::MAX;
    let rounded = round_slot(mode, exact, dither);
    complete_slot(policy, rounded, min, max)
}

/// Whether a debug build should refuse this position rather than adapt it.
///
/// Returns a verdict rather than diverging, so this crate stays total and the
/// decision belongs to the caller's build profile.
///
/// The bound is deliberately loose, and it stays loose because whether the panic
/// may appear in a dev build of a speed-first strategy is not settled. Sharpening
/// it here would answer that question in the wrong place.
#[must_use]
pub const fn panic_on_inexact(exact: Exact) -> Bool {
    exact.is_on_grid().not()
}

/// Whether a debug build should refuse a position whose rounded slot leaves the
/// range.
///
/// Takes the position and the dither rather than a slot. What leaves the range
/// is the slot the position rounds to, the rounding is internal to this file, and
/// a verdict over a slot would be asked about something no caller can produce
/// before adapting. The dither is the one `adapt` is handed, because the
/// stochastic mode's rounded slot is a function of it.
///
/// It reads the range without forcing the slot range's obligation, the same as
/// the slot verdict it replaces did. `adapt` is where that obligation is met.
#[must_use]
pub const fn panic_on_overflow<S: DeclaredSignature>(exact: Exact, dither: Dither) -> Bool {
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    let min = <<S::Format as Format>::Slots as Slots>::MIN;
    let max = <<S::Format as Format>::Slots as Slots>::MAX;
    // The same in-range question the completion asks, of the same pair, so the
    // verdict and the map cannot disagree at the index's top.
    let rounded = round_slot(mode, exact, dither);
    Bool::of(!rounded.lands_within(min.index(), max.index()))
}

#[cfg(test)]
mod tests;
