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
//! The exact step is computed in a form wide enough to hold it, and that
//! intermediate carries no coordinate type on purpose. For rounding it is
//! `Rounded`, the slot below, a step, and the distance past the index, which
//! names more positions than a slot index does. It is visible to this crate,
//! because addition reads the offset a rounding adds, and to nothing outside it:
//! naming it in the surface would publish a quantity no consumer has any
//! business holding.
//!
//! The position the map takes, `Exact`, and the dither one mode reads live in
//! `position`, and are re-exported here.

mod position;

pub use position::{Dither, Exact, Fraction};

use crate::adapt::{Adaptation, DeclaredSignature};
use crate::format::Format;
use crate::overflow::{Overflow, Policy};
use crate::rounding::{Mode, Rounding};
use crate::slots::{Slot, Slots};
use crate::width::Bool;

/// Where the rounding region puts a position: the slot at or below it, whether
/// to step one slot above that, and how far past the index the slot below lies.
///
/// A pair rather than the slot above, because at `i128::MAX` the slot above is
/// one past the index, and forming it there either overflows or saturates, and a
/// saturated step is a wrong answer under wrapping and a missed one in the
/// overflow verdict. The pair names every position from the index's lowest to
/// one past its highest, which is all the reach a rounding has from a position
/// the index holds. A position carried past the index keeps its distance here,
/// with `down` the end it lies past, so the completion answers exactly from the
/// three with no wider integer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Rounded {
    down: i128,
    up:   Bool,
    past: i64,
}

impl Rounded {
    /// A position already on the slot `at`, which rounding leaves where it is.
    #[must_use]
    pub(crate) const fn at(at: i128) -> Self {
        Self {
            down: at,
            up:   Bool::FALSE,
            past: 0,
        }
    }

    /// The slot at or below the position, or the end of the index it lies past.
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
    /// Past the top of the index nothing is in range. Past the bottom the one
    /// position that is, is the slot one under the index stepping up onto
    /// `i128::MIN`, and only for a range starting there. Inside the index,
    /// `down + 1` is formed only when `down` is below `lo`, so below the index's
    /// top, and the upper test compares `down` against `hi` without adding.
    #[must_use]
    const fn lands_within(self, lo: i128, hi: i128) -> bool {
        if self.past > 0 {
            return false;
        }
        if self.past < 0 {
            return self.past == -1 && self.up.get() && lo == i128::MIN;
        }
        let from_below = self.down >= lo || (self.up.get() && self.down + 1 == lo);
        let from_above = self.down < hi || (self.down == hi && !self.up.get());
        from_below && from_above
    }

    /// The rounded slot as an index, for a test reading a position the index
    /// holds. One past the index's top is not one, and asking for it panics, and
    /// so does asking for a position carried past the index.
    #[cfg(test)]
    #[must_use]
    pub(crate) const fn index(self) -> i128 {
        assert!(self.past == 0, "a position past the index has no index");
        self.down + self.step()
    }
}

/// Which slot the rounding region returns for a position.
///
/// Stated over a remainder in `[0, 1)`, so the position is between one slot and
/// the next and every mode is one comparison. Two modes read the slot as well as
/// the remainder, `TowardZero` its sign and `HalfEven` its parity at a tie, and
/// they read the sign and parity of the slot the position names, which past the
/// index is not the slot it pins at.
///
/// Visible to the crate and to nothing outside it, because addition's verdict
/// reads the one offset a rounding adds where that offset is fixed, and reading
/// it through the map rather than restating the six rules is what keeps the two
/// from disagreeing.
#[must_use]
pub(crate) const fn round_slot(mode: Mode, exact: Exact, dither: Dither) -> Rounded {
    let down = Rounded {
        down: exact.slot.index(),
        up:   Bool::FALSE,
        past: exact.past,
    };
    if exact.part.num == 0 {
        return down;
    }
    // The step up is one slot above `down`, carried as a flag and never added,
    // so the answer is exact at the index's top as it is everywhere else.
    let up = Rounded {
        up: Bool::TRUE,
        ..down
    };
    // `2 * num` against `den` decides which side of the midpoint the position is,
    // cross-multiplied in the wide carrier so no operand can leave its type.
    let twice = (exact.part.num as i128) * 2;
    let den = exact.part.den as i128;
    match mode {
        Mode::Floor => down,
        Mode::Ceil => up,
        // The position is negative exactly when the slot it names is, because
        // the remainder is non-negative and less than one.
        Mode::TowardZero => {
            if exact.is_negative() {
                up
            } else {
                down
            }
        },
        // `floor(x + 1/2)`: up exactly when the remainder reaches the midpoint.
        // It reads the remainder and nothing else, so a tie goes toward positive
        // infinity at every sign.
        Mode::HalfUp => {
            if twice >= den {
                up
            } else {
                down
            }
        },
        Mode::HalfEven => {
            if twice > den {
                up
            } else if twice < den {
                down
            } else if exact.is_even() {
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
        // In range, so the position is inside the index. Past the bottom that is
        // the one step onto `i128::MIN`, which is `lo`; otherwise the sum is at
        // most `hi` and cannot leave the index.
        if rounded.past != 0 {
            return min;
        }
        return Slot::at(rounded.down() + rounded.step());
    }
    match policy {
        Policy::Wrap => {
            // An admitted range spans at most `2^64` slots, so the span fits. The
            // slot below the position, its distance past the index and the lowest
            // slot are reduced separately before they are combined, because their
            // difference can leave the index when an outside range sits near one
            // end and the position near the other, or past it. Each reduction is
            // below the span, so the sum plus the step lies in `(-span, 2 * span]`
            // and the last reduction lands it inside the range. The step is added
            // after the reductions rather than to the slot, so a position one past
            // the index's top wraps exactly too.
            let span = hi - lo + 1;
            let offset = rounded.down().rem_euclid(span) + (rounded.past as i128).rem_euclid(span)
                - lo.rem_euclid(span)
                + rounded.step();
            Slot::at(lo + offset.rem_euclid(span))
        },
        // `Clamp` is documented as pinning to a declared bound that need not be
        // the range's own end, and the declared signature carries nowhere to put
        // that bound. With no bound to read it pins to the range, which is
        // saturation. The two agree here because the coordinate that would
        // separate them is missing, which is the admission rule's own diagnosis
        // rather than a shortcut taken in this function.
        // A position past the index lies beyond the end on its distance's side.
        // Otherwise, out of range with the slot below under `lo` means the
        // position is under it too, since a step onto `lo` would have landed in
        // range.
        Policy::Saturate | Policy::Clamp => {
            if rounded.past < 0 || (rounded.past == 0 && rounded.down() < lo) {
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
/// It reads the range without forcing the slot range's obligation. `adapt` is
/// where that obligation is met.
#[must_use]
pub const fn panic_on_overflow<S: DeclaredSignature>(exact: Exact, dither: Dither) -> Bool {
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    let min = <<S::Format as Format>::Slots as Slots>::MIN;
    let max = <<S::Format as Format>::Slots as Slots>::MAX;
    // The same in-range question the completion asks, of the same pair and the
    // same distance, so the verdict and the map cannot disagree at either end of
    // the index.
    let rounded = round_slot(mode, exact, dither);
    Bool::of(!rounded.lands_within(min.index(), max.index()))
}

#[cfg(test)]
mod tests;
