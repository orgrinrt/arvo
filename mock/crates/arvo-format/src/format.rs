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
/// **The denominator is positive by construction.** `of` maps a non-positive one
/// to a whole-numbered phase, which is a total function rather than a check, so
/// there is no way to name the case an earlier doc comment asked a reader not to
/// write.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Phase {
    num: i64,
    den: i64,
}

impl Phase {
    /// No offset, which puts the additive identity on the grid at slot zero.
    pub const ZERO: Self = Self { num: 0, den: 1 };

    /// A phase of `num` over `den`.
    ///
    /// A non-positive denominator is read as one rather than refused, on the same
    /// reading `Exact::between` uses for its own remainder: a caller computing a
    /// coordinate should not have to carry an invariant a constructor can hold.
    #[must_use]
    pub const fn of(num: i64, den: i64) -> Self {
        if den <= 0 {
            Self { num, den: 1 }
        } else {
            Self { num, den }
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
    #[must_use]
    pub const fn is_zero(self) -> Bool {
        Bool::of(self.num == 0)
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
    /// Carried explicitly and never assumed zero. A nonzero phase decides whether
    /// the identity adaptation ever occurs and whether the set carries an
    /// additive identity at all, so a design that hardcoded it would have closed
    /// a case the canon leaves open.
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
/// A zero phase puts zero on the grid at slot zero, provided the slot range
/// admits it. A nonzero phase takes it off, and takes one off with it: every
/// exact sum then lands half a step away from every grid point, which is why the
/// canon carries the coordinate rather than treating the bias as a corner case.
#[must_use]
pub const fn has_additive_identity<F: Format>() -> Bool {
    F::PHASE.is_zero().and(slot_in_range::<F::Slots>(Slot::ZERO))
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
