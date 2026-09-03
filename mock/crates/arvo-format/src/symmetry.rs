//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The grid's two symmetries, and where the applied map commutes with them.
//!
//! A grid is points spaced one quantum apart, so translating it by a whole number
//! of quanta carries it onto itself and so does reflecting it through zero. For
//! each there is a question about whether the adaptation commutes, and each has an
//! answer that holds in one region and fails outside it.
//!
//! ```text
//! relocation:  adapt(position + c) == adapt(adapt(position) + c)
//! reflection:  adapt(-position)    == adapt(-adapt(position))
//! ```
//!
//! Neither quantifies over an operation. Relocation quantifies over every exact
//! position and every representable translation, so a fused multiply-add is the
//! case where the position is a product and the translation is the addend, and
//! the operation decides which region of the law it sits in and nothing else.
//! Both re-adapt on the right, because the translate or the negation of an
//! admitted slot need not itself be admitted.
//!
//! Each decomposes into one property per region of the map and the conjunction is
//! the whole of it, which is what makes the two regions being separable
//! load-bearing rather than descriptive.
//!
//! Nothing in this crate reads these predicates. The consumer that would is an
//! operation and this crate ships none, so what checks them is the test that
//! derives the same answers by walking the map. That is stated rather than dressed
//! up: two predicates that were a `matches!` over an enumeration nothing read were
//! deleted from `overflow` and `rounding`, and the difference here is that an
//! answer is a function of five facts about a domain rather than a restatement of
//! one enumeration, and that the test runs the machinery instead of reading the
//! declaration back.

use crate::adapt::{Adaptation, DeclaredSignature};
use crate::format::Format;
use crate::overflow::{Overflow, Policy};
use crate::rounding::{Mode, Rounding};
use crate::slots::{Slot, Slots};
use crate::width::Bool;

/// What a rounding mode reads besides the residue.
///
/// The whole of what can break translation equivariance. Translating a position
/// moves its slot and leaves its residue alone, so a rule reading only the
/// residue returns the same offset at every slot and commutes by construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reads {
    /// The residue and the caller's dither, and nothing else.
    Nothing,
    /// The sign of the slot. Defeated by a domain with no negative position.
    Sign,
    /// The parity of the slot. Not defeated by any restriction on the sign, and
    /// defeated by translations that are all even.
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
/// The third is carried rather than derived from the first two. Over the six
/// names the vocabulary carries it happens to equal reading something besides the
/// residue, and that equality is a measured fact about those six rather than a
/// theorem: a nearest rule whose tie went toward positive infinity would read
/// nothing beyond the residue and still commute with reflection away from a tie.
/// Deriving it would put a coincidence into the mechanism, so it is a law with a
/// test of its own instead.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Behaviour {
    reads:    Reads,
    when:     When,
    reflects: Bool,
}

impl Behaviour {
    /// What it reads besides the residue.
    #[must_use]
    pub const fn reads(self) -> Reads {
        self.reads
    }

    /// When it reads it.
    #[must_use]
    pub const fn when(self) -> When {
        self.when
    }

    /// Whether the rounding region commutes with reflection through zero.
    #[must_use]
    pub const fn reflects(self) -> Bool {
        self.reflects
    }
}

/// The classification, one row per name in the closed vocabulary.
#[must_use]
pub const fn behaviour_of(mode: Mode) -> Behaviour {
    match mode {
        Mode::Floor | Mode::Ceil | Mode::Stochastic => {
            Behaviour {
                reads:    Reads::Nothing,
                when:     When::Never,
                reflects: Bool::of(false),
            }
        },
        Mode::TowardZero => {
            Behaviour {
                reads:    Reads::Sign,
                when:     When::EveryOffGridPosition,
                reflects: Bool::of(true),
            }
        },
        Mode::HalfUp => {
            Behaviour {
                reads:    Reads::Sign,
                when:     When::AtATie,
                reflects: Bool::of(true),
            }
        },
        Mode::HalfEven => {
            Behaviour {
                reads:    Reads::Parity,
                when:     When::AtATie,
                reflects: Bool::of(true),
            }
        },
    }
}

/// What an operation's exact positions and its translations reach.
///
/// The half of a law's region a declared signature cannot supply. A signature
/// says which slots are admitted; this says which exact positions an operation
/// produces before the adaptation sees them, which translations it applies, and
/// whether an exactly-half position is among them.
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
    ties:             Bool,
}

impl Reach {
    /// Every position and every translation the coordinate can carry, with an
    /// exactly-half position among them.
    ///
    /// What a consumer declares when it knows nothing about its operation. It
    /// licenses only the cells where both regions commute whatever the reach,
    /// because the cost of being wrong is not symmetric: refusing a law that
    /// holds costs a lowering and licensing one that does not costs a result.
    pub const EVERYTHING: Self = Self {
        position_low:     Slot::at(i64::MIN),
        position_high:    Slot::at(i64::MAX),
        translation_low:  Slot::at(i64::MIN),
        translation_high: Slot::at(i64::MAX),
        ties:             Bool::of(true),
    };

    /// The positions an operation reaches, with no translation and a tie among
    /// them.
    #[must_use]
    pub const fn of(low: Slot, high: Slot) -> Self {
        let (low, high) = ordered(low, high);
        Self {
            position_low:     low,
            position_high:    high,
            translation_low:  Slot::ZERO,
            translation_high: Slot::ZERO,
            ties:             Bool::of(true),
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
    #[must_use]
    pub const fn without_ties(self) -> Self {
        Self {
            ties: Bool::of(false),
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

    /// Whether an exactly-half position is among them.
    #[must_use]
    pub const fn reaches_a_tie(self) -> Bool {
        self.ties
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
    #[must_use]
    pub const fn reaches_above(self, highest: Slot) -> Bool {
        Bool::of(self.position_high.index() >= highest.index())
    }
}

/// The pair in order, so a reach handed the ends the wrong way round widens
/// rather than inverting.
#[must_use]
const fn ordered(low: Slot, high: Slot) -> (Slot, Slot) {
    if low.index() <= high.index() { (low, high) } else { (high, low) }
}

/// Whether the rounding region commutes with translation over what `reach` names.
///
/// Three disjuncts, and they are the three ways the thing a mode reads can fail
/// to be present: it reads nothing, or it reads only at a tie and no tie is
/// reached, or what it reads is the sign and no position it is asked about is
/// negative.
#[must_use]
pub const fn rounding_is_translation_equivariant(mode: Mode, reach: Reach) -> Bool {
    let behaviour = behaviour_of(mode);
    let reads_nothing = Bool::of(matches!(behaviour.reads(), Reads::Nothing));
    let only_at_a_tie =
        Bool::of(matches!(behaviour.when(), When::AtATie)).and(reach.reaches_a_tie().not());
    let the_sign_does_not_vary = Bool::of(matches!(behaviour.reads(), Reads::Sign))
        .and(reach.reaches_a_negative_position().not());
    reads_nothing.or(only_at_a_tie).or(the_sign_does_not_vary)
}

/// Whether the completion region commutes with translation over what `reach`
/// names, at the range the signature declares.
///
/// Wrapping is reduction modulo the span, which is a homomorphism of the additive
/// group, so it commutes whatever the reach. A clamp throws away how far past a
/// bound a value went, and the law fails exactly where a translation could have
/// pointed back at the range from the side the excursion left on.
#[must_use]
pub const fn completion_is_translation_homomorphic<S: DeclaredSignature>(reach: Reach) -> Bool {
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    let lowest = <<S::Format as Format>::Slots as Slots>::MIN;
    let highest = <<S::Format as Format>::Slots as Slots>::MAX;
    match <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY {
        Policy::Wrap => Bool::of(true),
        Policy::Saturate | Policy::Clamp => {
            let high_side = reach
                .reaches_above(highest)
                .not()
                .or(reach.reaches_a_negative_translation().not());
            let low_side = reach
                .reaches_below(lowest)
                .not()
                .or(reach.reaches_a_positive_translation().not());
            high_side.and(low_side)
        },
    }
}

/// Whether the adaptation relocates: adapting a translated position gives the
/// same slot as adapting the position and then translating.
///
/// The conjunction, and it is the whole of it. A fusion licence for a composite
/// operation is this predicate read at the positions and translations that
/// operation reaches.
#[must_use]
pub const fn adaptation_relocates<S: DeclaredSignature>(reach: Reach) -> Bool {
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    rounding_is_translation_equivariant(mode, reach)
        .and(completion_is_translation_homomorphic::<S>(reach))
}

/// Whether the rounding region commutes with reflection through zero.
#[must_use]
pub const fn rounding_is_reflection_equivariant(mode: Mode) -> Bool {
    behaviour_of(mode).reflects()
}

/// Whether the completion region commutes with reflection through zero.
///
/// Wrapping does, because negation is an automorphism of the cyclic group it
/// reduces into. A clamp does where the range is symmetric about zero, and a
/// two's complement range is not: it carries one more slot below zero than above
/// it, so its lowest slot has no positive twin to be pinned to. The comparison is
/// in the wide carrier, because negating the lowest admitted slot is exactly the
/// arithmetic that would leave the coordinate.
#[must_use]
pub const fn completion_is_reflection_equivariant<S: DeclaredSignature>(reach: Reach) -> Bool {
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    let lowest = <<S::Format as Format>::Slots as Slots>::MIN;
    let highest = <<S::Format as Format>::Slots as Slots>::MAX;
    match <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY {
        Policy::Wrap => Bool::of(true),
        Policy::Saturate | Policy::Clamp => {
            let symmetric = Bool::of((lowest.index() as i128) == -(highest.index() as i128));
            let never_leaves = reach
                .reaches_below(lowest)
                .not()
                .and(reach.reaches_above(highest).not());
            symmetric.or(never_leaves)
        },
    }
}

/// Whether the adaptation reflects: adapting a negated position gives the same
/// slot as adapting the position and negating.
///
/// The same conjunction one symmetry over, which is what says the two laws are
/// one shape read at the grid's two symmetries rather than two mechanisms.
#[must_use]
pub const fn adaptation_reflects<S: DeclaredSignature>(reach: Reach) -> Bool {
    let mode = <<S::Adaptation as Adaptation>::Rounding as Rounding>::MODE;
    rounding_is_reflection_equivariant(mode).and(completion_is_reflection_equivariant::<S>(reach))
}

#[cfg(test)]
mod tests;
