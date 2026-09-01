//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A format: an ambient domain and a representable set, and membership in it.
//!
//! The set is a constant of the type. A value set that depends on other data is
//! not a format but storage, which is the line the canon draws and the reason
//! every coordinate here is an associated item rather than a field.
//!
//! The concept is closed and the inventory is open: a new numeral joins by
//! implementing `Format` and supplying the coordinates, and nothing here is an
//! enumeration anybody has to widen. Every coordinate it supplies carries a type
//! this crate owns, so a numeral declared elsewhere is written in the same types
//! as the ones shipped here and is not refused by the lints that read every crate
//! but this one.
//!
//! Membership is one predicate, so a question about one particular value is that
//! predicate at the coordinates denoting it. `has_additive_identity` below is
//! `contains` asked at the slot where zero would sit, and it is written that way
//! because the alternative already happened: its own conjunction over the same
//! coordinates, short one of them, disagreeing with `contains` about the same
//! question.

use crate::ambient::{Ambient, Radix};
use crate::quantum::{exponent_at, magnitude_in_range, Exponent, Magnitude, Quantum};
use crate::slots::{slot_in_range, Slot, Slots};
use crate::width::Bool;

/// The grid's offset from zero, in units of the quantum at magnitude zero.
///
/// A ratio, so the half-step bias is exact rather than approximated, and one
/// coordinate rather than two consts an implementor can put out of step with each
/// other. The ratified parameterisation carries a phase, singular, and this is it.
///
/// **The denominator is positive by construction.** `of` is total rather than a
/// check: a negative one moves its sign to the numerator and a zero one is read
/// as one, so there is no way to name the case an earlier doc comment asked a
/// reader not to write.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Phase {
    num: i64,
    den: i64,
}

impl Phase {
    /// No offset, which leaves the additive identity at slot zero.
    pub const ZERO: Self = Self { num: 0, den: 1 };

    /// A phase of `num` over `den`.
    ///
    /// A non-positive denominator is read rather than refused, on the same
    /// reading `Exact::between` uses for its own remainder: a caller computing a
    /// coordinate should not have to carry an invariant a constructor can hold.
    ///
    /// **A negative denominator carries a value and a zero one does not**, which
    /// is why they take different paths. `of(3, -7)` is exactly `-3/7` and
    /// normalises by moving the sign to the numerator, losing nothing; a zero
    /// denominator names no value at all and is read as one. Folding the two
    /// together, which an earlier version did, changed both the sign and the
    /// magnitude of every negative case, silently, in a type whose whole point
    /// is exactness.
    ///
    /// **Two pairs cannot be normalised and are read as whole-numbered.**
    /// `i64::MIN` has no negation in `i64`, so a denominator of `i64::MIN`, or a
    /// numerator of `i64::MIN` under a negative denominator, would need a
    /// magnitude one past what the type carries. Those are the only inputs on
    /// which this is lossy, and they are named rather than folded in.
    ///
    /// **Neither the sign nor the magnitude survives on those two.**
    /// `of(3, i64::MIN)` names a tiny negative and answers `3/1`, and
    /// `of(i64::MIN, -7)` names a large positive and answers negative. Said
    /// here because an earlier version of this paragraph left a reader to
    /// assume the sign was kept, which is the assumption the guarding test was
    /// also making.
    #[must_use]
    pub const fn of(num: i64, den: i64) -> Self {
        if den > 0 {
            Self { num, den }
        } else if den == 0 || den == i64::MIN || num == i64::MIN {
            Self { num, den: 1 }
        } else {
            Self {
                num: -num,
                den: -den,
            }
        }
    }

    /// A phase counted in half steps, which is the biased grid's shape.
    #[must_use]
    pub const fn halves(num: i64) -> Self {
        Self { num, den: 2 }
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
    /// The second unwrap door. Two, rather than one, because the pair is what the
    /// coordinate is and handing back a single number would mean dividing.
    #[must_use]
    pub const fn denominator(self) -> i64 {
        self.den
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

    /// Whether the offset is a whole number of quanta.
    ///
    /// The question the additive identity turns on. A phase of one whole step
    /// shifts the grid onto itself, so zero stays on it at a shifted slot, and
    /// only a fractional part takes it off. The denominator is positive by
    /// construction, so this divides rather than asking whether it may.
    #[must_use]
    pub const fn is_whole_multiple(self) -> Bool {
        Bool::of(self.num % self.den == 0)
    }
}

/// A representable set, together with the domain it sits in.
///
/// Membership is one affine predicate over these coordinates: a value is in the
/// set exactly when it is `phase + slot * quantum(magnitude)` for some admitted
/// slot and magnitude.
pub trait Format {
    /// The domain the set is drawn from.
    type Ambient: Ambient;

    /// How the step changes with magnitude.
    type Quantum: Quantum;

    /// Which multiples of the step are admitted.
    type Slots: Slots;

    /// The grid's offset, in units of the quantum at magnitude zero.
    ///
    /// Carried explicitly and never assumed zero. Whether it is a whole multiple
    /// of the quantum decides whether the set carries an additive identity at all,
    /// and therefore whether the identity adaptation ever occurs, so a design that
    /// hardcoded it would have closed a case the canon leaves open.
    const PHASE: Phase;
}

/// Whether the coordinates name a member of the format's representable set.
///
/// The affine predicate, evaluated. Free rather than a method so it is reachable
/// at stage zero without the trait being const.
#[must_use]
pub const fn contains<F: Format>(slot: Slot, magnitude: Magnitude) -> Bool {
    magnitude_in_range::<F::Quantum>(magnitude).and(slot_in_range::<F::Slots>(slot))
}

/// Whether the format's grid carries an additive identity.
///
/// Zero is a member exactly when some admitted slot cancels the phase, so this is
/// the membership predicate asked at the position zero would occupy rather than a
/// second reading of the coordinates. The phase is a rational in units of the
/// quantum at magnitude zero, so cancelling it takes a whole number of quanta, and
/// the slot it lands on is the negated multiple rather than slot zero: one whole
/// step keeps the identity and only a fractional part takes it off.
///
/// **It answers at magnitude zero**, which is the magnitude the phase is stated in
/// units of. A law whose step shrinks as the magnitude rises can cancel a
/// fractional phase higher up, and nothing here looks there.
/// `the_identity_survives_a_shrinking_quantum` is the catalogued red arm holding
/// that case, and the canon does not reach it: the finding that decides this one
/// states its fraction width as the constant family's exponent, so it says nothing
/// about a magnitude-indexed law.
///
/// **The quotient is taken wide enough to hold its own negation.**
/// `Phase::of(i64::MIN, 1)` is writable and the slot cancelling it is one past
/// what an index carries, so negating in the index's own width would wrap it into
/// a slot some range admits.
#[must_use]
pub const fn has_additive_identity<F: Format>() -> Bool {
    if !F::PHASE.is_whole_multiple().get() {
        return Bool::FALSE;
    }
    let cancelling = -((F::PHASE.numerator() as i128) / (F::PHASE.denominator() as i128));
    // Only the top can be out of reach. The negated quotient runs from `-i64::MAX`
    // to `2^63`, so it overshoots an index at exactly one writable pair and cannot
    // undershoot at all, which is why there is one bound here rather than two.
    if cancelling > i64::MAX as i128 {
        return Bool::FALSE;
    }
    contains::<F>(Slot::at(cancelling as i64), Magnitude::SMALLEST)
}

/// The exponent of the step at a magnitude, for the format's quantum law.
#[must_use]
pub const fn step_exponent<F: Format>(magnitude: Magnitude) -> Exponent {
    exponent_at::<F::Quantum>(magnitude)
}

/// The radix the format's ambient domain counts in.
#[must_use]
pub const fn radix<F: Format>() -> Radix {
    <F::Ambient as Ambient>::RADIX
}

/// Whether the smallest magnitude's slots reach down to the grid's own step.
///
/// This is what makes subnormals fall out rather than be a case: at the smallest
/// magnitude the step is the smallest step the law produces, and the values it
/// admits are exactly those the floating conventions call subnormal. Nothing
/// names them and no branch selects them.
#[must_use]
pub const fn smallest_step_exponent<F: Format>() -> Exponent {
    let last = <F::Quantum as Quantum>::MAGNITUDES.largest();
    let at_zero = exponent_at::<F::Quantum>(Magnitude::SMALLEST);
    let at_last = exponent_at::<F::Quantum>(last);
    at_zero.min(at_last)
}
