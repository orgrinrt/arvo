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
//! `Rounded`, the position's floor in four limbs and a step, which names more
//! positions than a slot index does. It is visible to this crate,
//! because addition reads the offset a rounding adds, and to nothing outside it:
//! naming it in the surface would publish a quantity no consumer has any
//! business holding.
//!
//! The position the map takes, `Exact`, and the dither one mode reads live in
//! `position`, and are re-exported here.

mod position;

use core::cmp::Ordering;

#[cfg(test)]
use notko::Maybe;
pub use position::{Dither, Exact, Fraction, Remainder};

use crate::adapt::{Adaptation, DeclaredSignature};
use crate::format::Format;
use crate::overflow::{Overflow, Policy};
use crate::rounding::{Mode, Rounding};
use crate::slots::{Slot, Slots};
use crate::wide::Wide;
use crate::width::Bool;

/// Where the rounding region puts a position: its floor, and whether to step one
/// slot above it.
///
/// The floor in the four limbs rather than the slot above, because the floor of
/// a position past the index is not a slot, and the slot above the index's top
/// is not one either. Forming either in the index would overflow or saturate,
/// and a saturated step is a wrong answer under wrapping and a missed one in the
/// overflow verdict. The pair names every position a formed step reaches, so the
/// completion answers exactly from the two.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Rounded {
    down: Wide,
    up:   Bool,
}

impl Rounded {
    /// A position already on the slot `at`, which rounding leaves where it is.
    #[cfg(test)]
    #[must_use]
    pub(crate) const fn at(at: i128) -> Self {
        Self {
            down: Wide::of(at),
            up:   Bool::FALSE,
        }
    }

    /// The step the rounding adds to the floor, zero or one.
    #[must_use]
    #[inline]
    pub(crate) const fn step(self) -> i128 {
        if self.up.get() { 1 } else { 0 }
    }

    /// Whether the rounded position lies in `[lo, hi]`.
    ///
    /// From below the floor is at least `lo`, or one under it and stepping onto
    /// it; from above it is under `hi`, or at `hi` without stepping past it.
    /// Nothing is added to the floor, and `lo - 1` is formed in the four limbs.
    #[must_use]
    #[inline]
    const fn lands_within(self, lo: i128, hi: i128) -> bool {
        let under = Wide::of_sum(lo, -1);
        let from_below = !matches!(self.down.ordering(Wide::of(lo)), Ordering::Less)
            || (self.up.get() && matches!(self.down.ordering(under), Ordering::Equal));
        let from_above = match self.down.ordering(Wide::of(hi)) {
            Ordering::Less => true,
            Ordering::Equal => !self.up.get(),
            Ordering::Greater => false,
        };
        from_below && from_above
    }

    /// The rounded position reduced onto `[lo, lo + span)`.
    ///
    /// An admitted range spans at most `2^64` slots, so the span fits the index.
    /// The floor and the lowest slot are reduced separately before they are
    /// combined, because their difference can leave the index when the position
    /// is far past it. Each reduction is below the span, so the difference plus
    /// the step lies in `(-span, span]` and the last reduction lands it inside
    /// the range. The step is added after the reductions rather than to the
    /// floor. For a position in range this is the rounded slot itself.
    #[must_use]
    #[inline]
    const fn reduced(self, lo: i128, span: i128) -> Slot {
        let offset = self.down.residue(span) - lo.rem_euclid(span) + self.step();
        Slot::at(lo + offset.rem_euclid(span))
    }

    /// The rounded slot as an index, for a test reading a position the index
    /// holds. A position the index does not hold is not one, and asking for it
    /// panics.
    #[cfg(test)]
    #[must_use]
    pub(crate) const fn index(self) -> i128 {
        match Wide::of_sum(0, self.step()).plus(self.down) {
            Maybe::Is(at) => {
                match at.index() {
                    Maybe::Is(at) => at,
                    Maybe::Isnt => panic!("a position past the index has no index"),
                }
            },
            Maybe::Isnt => panic!("a position past the index has no index"),
        }
    }
}

/// Which slot the rounding region returns for a position.
///
/// Stated over a remainder in `[0, 1)`, so the position is between one slot and
/// the next and every mode is one comparison. Two modes read the floor as well
/// as the remainder, `TowardZero` its sign and `HalfEven` its parity at a tie,
/// and both read the floor itself, in the four limbs.
///
/// Visible to the crate and to nothing outside it, because addition's verdict
/// reads the one offset a rounding adds where that offset is fixed, and reading
/// it through the map rather than restating the six rules is what keeps the two
/// from disagreeing.
#[must_use]
#[inline]
pub(crate) const fn round_slot(mode: Mode, exact: Exact, dither: Dither) -> Rounded {
    let down = Rounded {
        down: exact.floor(),
        up:   Bool::FALSE,
    };
    if exact.part.is_zero() {
        return down;
    }
    // The step up is one slot above the floor, carried as a flag and never added,
    // so the answer is exact past the index as it is everywhere else.
    let up = Rounded {
        up: Bool::TRUE,
        ..down
    };
    // Which side of the midpoint the position is, read off the remainder in its
    // own coordinate so no operand can leave its type.
    let half = exact.part.against_half();
    match mode {
        Mode::Floor => down,
        Mode::Ceil => up,
        // The position is negative exactly when its floor is, because the
        // remainder is non-negative and less than one.
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
            if matches!(half, Ordering::Less) {
                down
            } else {
                up
            }
        },
        Mode::HalfEven => {
            match half {
                Ordering::Greater => up,
                Ordering::Less => down,
                Ordering::Equal => {
                    if exact.is_even() {
                        down
                    } else {
                        up
                    }
                },
            }
        },
        // Up when the dither falls below the position's offset, which makes the
        // probability of rounding up equal to that offset when the dither is
        // uniform. Cross-multiplied so no division happens.
        Mode::Stochastic => {
            if exact.part.is_above(dither.0) {
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
#[inline]
const fn complete_slot(policy: Policy, rounded: Rounded, min: Slot, max: Slot) -> Slot {
    let lo = min.index();
    let hi = max.index();
    // An admitted range spans at most `2^64` slots, so the span fits.
    let span = hi - lo + 1;
    if rounded.lands_within(lo, hi) {
        return rounded.reduced(lo, span);
    }
    match policy {
        Policy::Wrap => rounded.reduced(lo, span),
        // `Clamp` is documented as pinning to a declared bound that need not be
        // the range's own end, and the declared signature carries nowhere to put
        // that bound. With no bound to read it pins to the range, which is
        // saturation. The two agree here because the coordinate that would
        // separate them is missing, which is the admission rule's own diagnosis
        // rather than a shortcut taken in this function.
        // Out of range with the floor under `lo` means the position is under it
        // too, since a step onto `lo` would have landed in range.
        Policy::Saturate | Policy::Clamp => {
            if matches!(rounded.down.ordering(Wide::of(lo)), Ordering::Less) {
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
#[inline]
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
#[inline]
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
#[inline]
pub const fn panic_on_overflow<S: DeclaredSignature>(exact: Exact, dither: Dither) -> Bool {
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    let min = <<S::Format as Format>::Slots as Slots>::MIN;
    let max = <<S::Format as Format>::Slots as Slots>::MAX;
    // The same in-range question the completion asks, of the same floor and
    // step, so the verdict and the map cannot disagree anywhere past the index.
    let rounded = round_slot(mode, exact, dither);
    Bool::of(!rounded.lands_within(min.index(), max.index()))
}

#[cfg(test)]
pub(crate) mod tests;
