//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The predicate as it is meant to ship, prototyped here first.
//!
//! Written in the sketch so step 05 can check it against the map over the whole
//! cross before a line of it is proposed for the crate. Everything here is
//! `const fn` over values a declared signature and a caller already hold, so
//! nothing in it survives to a lowered path.
//!
//! **The first version of this file was unsound and step 05 caught it.** The
//! rounding region's sign question was asked about the positions a caller
//! declares, and the relocation law evaluates the rounding at the translated
//! position too, so a non-negative position and a negative translation reach a
//! negative position that the reach never mentioned. Six cells said a law held
//! where the map says it does not: toward-zero and half-up under wrapping on the
//! two ranges that admit a negative slot. The repair is `translated_low` below,
//! and it is the reason the reach is quantified over the union rather than over
//! what a caller wrote down.
//!
//! This is a spike. Its arities, its namings and which crate it would sit in are
//! scaffolding for the check, not the design.

use arvo_format::overflow::Policy;
use arvo_format::rounding::Mode;

/// What a rounding mode reads besides the residue.
///
/// This is the whole of what can break translation equivariance, because
/// translating a position moves the slot and leaves the residue alone, so a rule
/// reading only the residue commutes by construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reads {
    /// The residue and the caller's dither, and nothing else.
    Nothing,
    /// The sign of the slot, which a domain with no negative positions defeats.
    Sign,
    /// The parity of the slot, which no restriction on the sign defeats and an
    /// even translation does.
    Parity,
}

/// When a mode reads it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum When {
    /// It does not.
    Never,
    /// At every position off the grid.
    EveryOffGridPosition,
    /// Only where the position is exactly half way between two slots.
    AtATie,
}

/// What a mode reads, when it reads it, and whether it commutes with reflection.
///
/// The third is carried rather than derived. Over the six names this vocabulary
/// carries it happens to equal `reads != Nothing`, and that equality is a
/// measured fact about those six rather than a theorem: a nearest rule whose tie
/// goes toward positive infinity reads nothing beyond the residue and still
/// commutes with reflection away from a tie. Deriving it would bake a
/// coincidence into the mechanism.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Behaviour {
    pub reads:    Reads,
    pub when:     When,
    pub reflects: bool,
}

/// The classification, one row per name.
#[must_use]
pub const fn behaviour_of(mode: Mode) -> Behaviour {
    match mode {
        Mode::Floor | Mode::Ceil | Mode::Stochastic => {
            Behaviour {
                reads:    Reads::Nothing,
                when:     When::Never,
                reflects: false,
            }
        },
        Mode::TowardZero => {
            Behaviour {
                reads:    Reads::Sign,
                when:     When::EveryOffGridPosition,
                reflects: true,
            }
        },
        Mode::HalfUp => {
            Behaviour {
                reads:    Reads::Sign,
                when:     When::AtATie,
                reflects: true,
            }
        },
        Mode::HalfEven => {
            Behaviour {
                reads:    Reads::Parity,
                when:     When::AtATie,
                reflects: true,
            }
        },
    }
}

/// What an operation's exact positions and its translations reach.
///
/// The half of a law's region a declared signature cannot supply. The signature
/// says which slots are admitted; this says which exact positions an operation
/// produces before the adaptation sees them, and which translations it applies.
/// A consumer that knows nothing declares `EVERYTHING` and gets the conservative
/// answer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reach {
    pub position_low:     i64,
    pub position_high:    i64,
    pub translation_low:  i64,
    pub translation_high: i64,
    pub ties:             bool,
}

impl Reach {
    /// Everything the coordinate can carry, with ties reached.
    pub const EVERYTHING: Self = Self {
        position_low:     i64::MIN,
        position_high:    i64::MAX,
        translation_low:  i64::MIN,
        translation_high: i64::MAX,
        ties:             true,
    };

    /// The lowest position the rounding region is asked about.
    ///
    /// The relocation law rounds the position and the translated position both,
    /// so the set the region sees is the union and not the positions a caller
    /// wrote down. Saturating, because the union's bound can leave the
    /// coordinate where the reach is the conservative one, and a bound that
    /// wrapped would answer that the domain has no negatives.
    #[must_use]
    pub const fn translated_low(self) -> i64 {
        let shifted = self.position_low.saturating_add(self.translation_low);
        if shifted < self.position_low { shifted } else { self.position_low }
    }

    /// The highest position the rounding region is asked about.
    #[must_use]
    pub const fn translated_high(self) -> i64 {
        let shifted = self.position_high.saturating_add(self.translation_high);
        if shifted > self.position_high { shifted } else { self.position_high }
    }

    /// Whether the rounding region is ever asked about a position below zero.
    #[must_use]
    pub const fn reaches_negative_positions(self) -> bool {
        self.translated_low() < 0
    }

    #[must_use]
    pub const fn reaches_negative_translations(self) -> bool {
        self.translation_low < 0
    }

    #[must_use]
    pub const fn reaches_positive_translations(self) -> bool {
        self.translation_high > 0
    }

    /// Whether a rounded position can sit below the lowest admitted slot.
    ///
    /// The rounding region returns the slot below the position or the one above
    /// it and never anything else, so the rounded value is in
    /// `[position_low, position_high + 1]`. That bound is what lets the
    /// excursion sides be derived from the position bounds rather than declared
    /// separately, and step 05's `P4` measures it rather than assuming it.
    #[must_use]
    pub const fn reaches_below(self, lo: i64) -> bool {
        self.position_low < lo
    }

    /// Whether a rounded position can sit above the highest admitted slot.
    #[must_use]
    pub const fn reaches_above(self, hi: i64) -> bool {
        self.position_high >= hi
    }
}

/// Whether the rounding region commutes with translation over what `reach`
/// names.
///
/// Three disjuncts and they are the three ways the thing a mode reads can fail
/// to be there: it reads nothing, or it reads only at a tie and no tie is
/// reached, or what it reads is the sign and no position it is asked about is
/// negative.
#[must_use]
pub const fn rounding_is_translation_equivariant(mode: Mode, reach: Reach) -> bool {
    let b = behaviour_of(mode);
    matches!(b.reads, Reads::Nothing)
        || (matches!(b.when, When::AtATie) && !reach.ties)
        || (matches!(b.reads, Reads::Sign) && !reach.reaches_negative_positions())
}

/// Whether the completion region commutes with translation over what `reach`
/// names, at a range from `lo` to `hi`.
///
/// Wrapping is reduction modulo the span, which is a homomorphism of the
/// additive group, so it commutes whatever the reach. A clamp throws away how
/// far past a bound a value went, and the law fails exactly when a translation
/// could have pointed back at the range from that side.
#[must_use]
pub const fn completion_is_translation_homomorphic(
    policy: Policy,
    lo: i64,
    hi: i64,
    reach: Reach,
) -> bool {
    match policy {
        Policy::Wrap => true,
        Policy::Saturate | Policy::Clamp => {
            (!reach.reaches_above(hi) || !reach.reaches_negative_translations())
                && (!reach.reaches_below(lo) || !reach.reaches_positive_translations())
        },
    }
}

/// Whether the adaptation relocates: adapting a translated position gives the
/// same slot as adapting the position and then translating.
///
/// The conjunction, and it is the whole of it. A fusion licence for any
/// composite operation is this predicate read at the positions and translations
/// that operation reaches.
#[must_use]
pub const fn adaptation_relocates(
    mode: Mode,
    policy: Policy,
    lo: i64,
    hi: i64,
    reach: Reach,
) -> bool {
    rounding_is_translation_equivariant(mode, reach)
        && completion_is_translation_homomorphic(policy, lo, hi, reach)
}

/// Whether the rounding region commutes with reflection through zero.
#[must_use]
pub const fn rounding_is_reflection_equivariant(mode: Mode) -> bool {
    behaviour_of(mode).reflects
}

/// Whether the completion region commutes with reflection through zero.
///
/// Wrapping does, because negation is an automorphism of the cyclic group it
/// reduces into. A clamp does where the range is symmetric about zero, and a
/// two's complement range is not: it carries one more slot below zero than
/// above it, so the low bound has no positive twin to be pinned to.
#[must_use]
pub const fn completion_is_reflection_equivariant(
    policy: Policy,
    lo: i64,
    hi: i64,
    reach: Reach,
) -> bool {
    match policy {
        Policy::Wrap => true,
        Policy::Saturate | Policy::Clamp => {
            lo == -hi || (!reach.reaches_below(lo) && !reach.reaches_above(hi))
        },
    }
}

/// Whether the adaptation reflects: adapting a negated position gives the same
/// slot as adapting the position and negating.
///
/// The same conjunction one symmetry over, which is what says the two laws are
/// one shape read at the grid's two symmetries rather than two mechanisms.
#[must_use]
pub const fn adaptation_reflects(
    mode: Mode,
    policy: Policy,
    lo: i64,
    hi: i64,
    reach: Reach,
) -> bool {
    rounding_is_reflection_equivariant(mode)
        && completion_is_reflection_equivariant(policy, lo, hi, reach)
}
