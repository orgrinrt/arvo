//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Applying an adaptation: the second half of the ratified factoring.
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

use crate::adapt::{Adaptation, DeclaredSignature};
use crate::format::Format;
use crate::overflow::{Overflow, Policy};
use crate::rounding::{Mode, Rounding};
use crate::slots::Slots;

/// An exact result, in the format's own coordinates.
///
/// The value is `phase + (slot + num/den) * quantum(magnitude)`. The remainder is
/// carried as a fraction rather than as an approximation so that an exactly-half
/// position is representable, which is what makes a tie rule testable rather than
/// a matter of what the host's arithmetic happened to do.
///
/// The remainder is in `[0, 1)`: `num` is non-negative and less than `den`, and
/// `den` is positive. A position is therefore always between `slot` and
/// `slot + 1`, which is what lets the rounding modes be stated once rather than
/// once per sign.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Exact {
    slot: i64,
    num: i64,
    den: i64,
}

impl Exact {
    /// A position already on the grid.
    #[must_use]
    pub const fn on_grid(slot: i64) -> Self {
        Self {
            slot,
            num: 0,
            den: 1,
        }
    }

    /// A position between `slot` and `slot + 1`, at `num / den` of the way.
    ///
    /// A remainder outside `[0, 1)` is normalised into the slot rather than
    /// refused, because a caller computing an exact result should not have to
    /// carry the invariant. A non-positive denominator is treated as one.
    #[must_use]
    pub const fn between(slot: i64, num: i64, den: i64) -> Self {
        if den <= 0 {
            return Self::on_grid(slot);
        }
        let whole = num.div_euclid(den);
        let rem = num.rem_euclid(den);
        Self {
            slot: slot + whole,
            num: rem,
            den,
        }
    }

    /// The slot the position sits on or just above.
    #[must_use]
    pub const fn slot(self) -> i64 {
        self.slot
    }

    /// Whether the position is exactly on a grid point.
    #[must_use]
    pub const fn is_on_grid(self) -> bool {
        self.num == 0
    }

    /// Whether the position is exactly half way between two grid points.
    #[must_use]
    pub const fn is_tie(self) -> bool {
        self.num * 2 == self.den
    }
}

/// The decision the stochastic mode reads, supplied by the caller.
///
/// Five of the six modes are a function of the position alone. The sixth is not,
/// and how it should be seeded and whether it is keyed on value or on position are
/// both open questions in the register. Taking the decision as an input is what
/// lets the mode be expressed without answering either.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dither {
    num: i64,
    den: i64,
}

impl Dither {
    /// The value to pass where the adaptation names no stochastic mode.
    ///
    /// It is read by no other mode, so the branch reading it is dead wherever the
    /// mode is not stochastic and goes away with the monomorphisation.
    pub const UNUSED: Self = Self { num: 0, den: 1 };

    /// A dither at `num / den` of the way between two grid points.
    #[must_use]
    pub const fn at(num: i64, den: i64) -> Self {
        if den <= 0 {
            return Self::UNUSED;
        }
        Self { num, den }
    }
}

/// Which slot the rounding region returns for a position.
///
/// Stated over a remainder in `[0, 1)`, so the position is between `slot` and
/// `slot + 1` and every mode is one comparison. The sign cases live in the modes
/// that care about sign rather than in the representation.
#[must_use]
pub const fn round_slot(mode: Mode, exact: Exact, dither: Dither) -> i64 {
    if exact.num == 0 {
        return exact.slot;
    }
    let up = exact.slot + 1;
    let down = exact.slot;
    // `2 * num` against `den` decides which side of the midpoint the position is.
    let twice = exact.num * 2;
    match mode {
        Mode::Floor => down,
        Mode::Ceil => up,
        // The position is negative exactly when the slot is, because the
        // remainder is non-negative and less than one.
        Mode::TowardZero => {
            if exact.slot < 0 {
                up
            } else {
                down
            }
        }
        Mode::HalfUp => {
            if twice > exact.den {
                up
            } else if twice < exact.den {
                down
            } else if exact.slot < 0 {
                // A tie on a negative position goes away from zero, which is down.
                down
            } else {
                up
            }
        }
        Mode::HalfEven => {
            if twice > exact.den {
                up
            } else if twice < exact.den {
                down
            } else if down % 2 == 0 {
                down
            } else {
                up
            }
        }
        // Up when the dither falls below the position's offset, which makes the
        // probability of rounding up equal to that offset when the dither is
        // uniform. Cross-multiplied so no division happens.
        Mode::Stochastic => {
            if dither.num * exact.den < exact.num * dither.den {
                up
            } else {
                down
            }
        }
    }
}

/// Which slot the completion region returns for a slot outside the range.
///
/// The identity on a slot already inside it, which is what makes the two regions
/// separable rather than one pass that always touches the value.
#[must_use]
pub const fn complete_slot(policy: Policy, slot: i64, min: i64, max: i64) -> i64 {
    if slot >= min && slot <= max {
        return slot;
    }
    match policy {
        Policy::Wrap => {
            let span = max - min + 1;
            min + (slot - min).rem_euclid(span)
        }
        // `Clamp` is documented as pinning to a declared bound that need not be
        // the range's own end, and the declared signature carries nowhere to put
        // that bound. With no bound to read it pins to the range, which is
        // saturation. The two agree here because the coordinate that would
        // separate them is missing, which is the admission rule's own diagnosis
        // rather than a shortcut taken in this function.
        Policy::Saturate | Policy::Clamp => {
            if slot < min {
                min
            } else {
                max
            }
        }
    }
}

/// Apply a declared signature's adaptation to an exact position.
///
/// Total: every position returns a slot the format admits, for every mode and
/// every policy. That totality is what makes an adaptation a member of the slot
/// the ratified factoring names, and it is why a panic is not one.
#[must_use]
pub const fn adapt<S: DeclaredSignature>(exact: Exact, dither: Dither) -> i64 {
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    let policy = <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY;
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
/// The bound is deliberately loose. One ratified row bounds the panic by the
/// imitate-a-native-primitive concern and op's own correction rebounds it on the
/// speed-first concern while saying the intent is the inferrable thing rather
/// than the wording, and the correction's own gap records that whether the panic
/// may appear in dev builds of that concern is unsettled. Sharpening it here
/// would go past what was blessed.
#[must_use]
pub const fn panic_on_inexact(exact: Exact) -> bool {
    !exact.is_on_grid()
}

/// Whether a debug build should refuse a slot that leaves the range.
#[must_use]
pub const fn panic_on_overflow<S: DeclaredSignature>(slot: i64) -> bool {
    let min = <<S::Format as Format>::Slots as Slots>::MIN;
    let max = <<S::Format as Format>::Slots as Slots>::MAX;
    slot < min || slot > max
}

#[cfg(test)]
mod tests;
