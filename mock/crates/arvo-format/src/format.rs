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
//! enumeration anybody has to widen.

use crate::ambient::Ambient;
use crate::quantum::{exponent_at, magnitude_in_range, Quantum};
use crate::slots::{slot_in_range, Slots};

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

    /// The numerator of the phase, in units of the quantum at magnitude zero.
    ///
    /// Carried explicitly and never assumed zero. Together with the denominator
    /// it decides whether the set carries an additive identity at all, and
    /// therefore whether the identity adaptation ever occurs, so a design that
    /// hardcoded it would have closed a case the canon leaves open.
    const PHASE_NUM: i64;

    /// The denominator of the phase, in units of the quantum at magnitude zero.
    ///
    /// One for an unbiased grid, two for the half-step bias. Never zero.
    const PHASE_DEN: i64;
}

/// Whether the coordinates name a member of the format's representable set.
///
/// The affine predicate, evaluated. Free rather than a method so it is reachable
/// at stage zero without the trait being const.
#[must_use]
pub const fn contains<F: Format>(slot: i64, magnitude: u32) -> bool {
    magnitude_in_range::<F::Quantum>(magnitude) && slot_in_range::<F::Slots>(slot)
}

/// Whether the format's grid carries an additive identity.
///
/// The phase is `PHASE_NUM` over `PHASE_DEN` in units of the quantum, and zero
/// sits on the grid exactly when some slot cancels it. That needs the phase to
/// be a whole multiple of the quantum, and the slot it lands on is the negated
/// multiple rather than slot zero, so a phase of one whole step keeps the
/// identity and only a fractional part takes it off. A denominator of zero names
/// no position and is answered as no identity rather than divided by.
#[must_use]
pub const fn has_additive_identity<F: Format>() -> bool {
    if F::PHASE_DEN == 0 {
        return false;
    }
    F::PHASE_NUM % F::PHASE_DEN == 0 && slot_in_range::<F::Slots>(-(F::PHASE_NUM / F::PHASE_DEN))
}

/// The exponent of the step at a magnitude, for the format's quantum law.
#[must_use]
pub const fn step_exponent<F: Format>(magnitude: u32) -> i32 {
    exponent_at::<F::Quantum>(magnitude)
}

/// The radix the format's ambient domain counts in.
#[must_use]
pub const fn radix<F: Format>() -> u32 {
    <F::Ambient as Ambient>::RADIX
}

/// Whether the smallest magnitude's slots reach down to the grid's own step.
///
/// This is what makes subnormals fall out rather than be a case: at the smallest
/// magnitude the step is the smallest step the law produces, and the values it
/// admits are exactly those the floating conventions call subnormal. Nothing
/// names them and no branch selects them.
#[must_use]
pub const fn smallest_step_exponent<F: Format>() -> i32 {
    let last = <F::Quantum as Quantum>::MAGNITUDES - 1;
    let at_zero = exponent_at::<F::Quantum>(0);
    let at_last = exponent_at::<F::Quantum>(last);
    if at_zero < at_last {
        at_zero
    } else {
        at_last
    }
}
