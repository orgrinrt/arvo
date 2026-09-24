//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What an operation's exact positions and its translations reach, apart from
//! what a mode or the adaptation does with them.
//!
//! `Reach` is the coordinate the equivariance predicates in the parent module
//! read: which exact positions an operation produces before the adaptation
//! sees them, which translations it applies, and whether those positions leave
//! the grid and reach a tie. It carries no mode and no policy of its own, so it
//! sits apart from `Behaviour` and the predicates that consult it.

use crate::slots::Slot;
use crate::width::Bool;

/// Whether the positions a reach names leave the grid, and whether a tie is among
/// them.
///
/// One coordinate of three values rather than two flags, because an on-grid reach
/// with an exactly-half position in it is a contradiction, and two flags are what
/// would let somebody write it. The values run from the narrowest reach to the
/// widest.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Residues {
    /// Every position is on the grid, so the rounding region is never entered.
    OnGrid,
    /// Some position is off the grid and none is exactly half way.
    OffGrid,
    /// An exactly-half position is among them.
    WithTies,
}

/// What an operation's exact positions and its translations reach.
///
/// The half of a law's region a declared signature cannot supply. A signature
/// says which slots are admitted; this says which exact positions an operation
/// produces before the adaptation sees them, which translations it applies, and
/// whether those positions leave the grid and reach a tie.
///
/// A pair handed in the wrong order is ordered rather than refused, the same act
/// `Fraction::of` performs on a ratio, and the wider reading is the conservative
/// one so nothing is licensed by the normalisation that was not licensed before.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reach {
    position_low:     Slot,
    position_high:    Slot,
    translation_low:  Slot,
    translation_high: Slot,
    grid:             Residues,
}

impl Reach {
    /// Every position and every translation the coordinate can carry, off the
    /// grid and with an exactly-half position among them.
    ///
    /// What a consumer declares when it knows nothing about its operation. It
    /// licenses only the cells where both regions commute whatever the reach,
    /// because the cost of being wrong is not symmetric: refusing a law that
    /// holds costs a lowering and licensing one that does not costs a result.
    pub const EVERYTHING: Self = Self {
        position_low:     Slot::at(i128::MIN),
        position_high:    Slot::at(i128::MAX),
        translation_low:  Slot::at(i128::MIN),
        translation_high: Slot::at(i128::MAX),
        grid:             Residues::WithTies,
    };

    /// The positions an operation reaches, with no translation, off the grid and
    /// with a tie among them.
    #[must_use]
    pub const fn of(low: Slot, high: Slot) -> Self {
        let (low, high) = ordered(low, high);
        Self {
            position_low:     low,
            position_high:    high,
            translation_low:  Slot::ZERO,
            translation_high: Slot::ZERO,
            grid:             Residues::WithTies,
        }
    }

    /// The same reach with the translations an operation applies.
    #[must_use]
    pub const fn translated_by(self, low: Slot, high: Slot) -> Self {
        let (low, high) = ordered(low, high);
        Self {
            translation_low: low,
            translation_high: high,
            ..self
        }
    }

    /// The same reach with no exactly-half position in it.
    ///
    /// A reach already on the grid stays there: taking the ties away from a set
    /// of positions that has none leaves it where it was, rather than moving it
    /// off the grid it never left.
    #[must_use]
    pub const fn without_ties(self) -> Self {
        let grid = match self.grid {
            Residues::OnGrid => Residues::OnGrid,
            Residues::OffGrid | Residues::WithTies => Residues::OffGrid,
        };
        Self {
            grid,
            ..self
        }
    }

    /// The same reach with every position on the grid.
    ///
    /// What an operation declares when its exact positions are whole slots, so
    /// the rounding region is never entered and no mode has anything to read.
    #[must_use]
    pub const fn on_grid(self) -> Self {
        Self {
            grid: Residues::OnGrid,
            ..self
        }
    }

    /// The lowest exact position, before any translation.
    #[must_use]
    pub const fn positions_low(self) -> Slot {
        self.position_low
    }

    /// The highest exact position, before any translation.
    #[must_use]
    pub const fn positions_high(self) -> Slot {
        self.position_high
    }

    /// The lowest translation.
    #[must_use]
    pub const fn translations_low(self) -> Slot {
        self.translation_low
    }

    /// The highest translation.
    #[must_use]
    pub const fn translations_high(self) -> Slot {
        self.translation_high
    }

    /// Whether any position is off the grid.
    #[must_use]
    pub const fn reaches_off_the_grid(self) -> Bool {
        Bool::of(!matches!(self.grid, Residues::OnGrid))
    }

    /// Whether an exactly-half position is among them.
    #[must_use]
    pub const fn reaches_a_tie(self) -> Bool {
        Bool::of(matches!(self.grid, Residues::WithTies))
    }

    /// The lowest position the rounding region is asked about.
    ///
    /// The relocation law rounds the position and the translated position both,
    /// so the set the region sees is the union rather than the positions a caller
    /// wrote down. A reach of non-negative positions with a translation that may
    /// be negative reaches a negative position, and a region stated over the
    /// declared positions alone would license a mode that reads the sign where it
    /// does not hold.
    ///
    /// Saturating, because the union's bound leaves the coordinate on the
    /// conservative reach and a bound that wrapped would answer that the domain
    /// has no negatives.
    #[must_use]
    pub const fn lowest_rounded_position(self) -> Slot {
        let shifted = self
            .position_low
            .index()
            .saturating_add(self.translation_low.index());
        if shifted < self.position_low.index() {
            Slot::at(shifted)
        } else {
            self.position_low
        }
    }

    /// The highest position the rounding region is asked about.
    #[must_use]
    pub const fn highest_rounded_position(self) -> Slot {
        let shifted = self
            .position_high
            .index()
            .saturating_add(self.translation_high.index());
        if shifted > self.position_high.index() {
            Slot::at(shifted)
        } else {
            self.position_high
        }
    }

    /// Whether the rounding region is ever asked about a position below zero.
    #[must_use]
    pub const fn reaches_a_negative_position(self) -> Bool {
        Bool::of(self.lowest_rounded_position().index() < 0)
    }

    /// Whether a translation can point back down the range.
    #[must_use]
    pub const fn reaches_a_negative_translation(self) -> Bool {
        Bool::of(self.translation_low.index() < 0)
    }

    /// Whether a translation can point back up it.
    #[must_use]
    pub const fn reaches_a_positive_translation(self) -> Bool {
        Bool::of(self.translation_high.index() > 0)
    }

    /// Whether a rounded position can sit below the lowest admitted slot.
    ///
    /// The rounding region returns the position's own slot or the one above it
    /// and never anything else, so the rounded value is in
    /// `[positions_low, positions_high + 1]`. That bound is what lets the
    /// excursion sides be derived from the position bounds rather than declared
    /// beside them, and it is a law with a test rather than an assumption.
    #[must_use]
    pub const fn reaches_below(self, lowest: Slot) -> Bool {
        Bool::of(self.position_low.index() < lowest.index())
    }

    /// Whether a rounded position can sit above the highest admitted slot.
    ///
    /// Off the grid a position at the highest slot can round to the one above
    /// it, so a reach whose highest position is the highest slot already
    /// reaches past it. On the grid the rounding region returns the position
    /// itself, so it reaches past only where a position does, and the test is
    /// strict. Reading it inclusively there refuses a relocation the map honours.
    #[must_use]
    pub const fn reaches_above(self, highest: Slot) -> Bool {
        if self.reaches_off_the_grid().get() {
            Bool::of(self.position_high.index() >= highest.index())
        } else {
            Bool::of(self.position_high.index() > highest.index())
        }
    }
}

/// The pair in order, so a reach handed the ends the wrong way round widens
/// rather than inverting.
#[must_use]
const fn ordered(low: Slot, high: Slot) -> (Slot, Slot) {
    if low.index() <= high.index() { (low, high) } else { (high, low) }
}
