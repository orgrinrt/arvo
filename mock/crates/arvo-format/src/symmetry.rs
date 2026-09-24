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
//! Addition reads these predicates. Its associativity verdict is the completion
//! half of the relocation law, read at the reach addition supplies, and the
//! rounding half decides whether that reading applies at all, so the conjunction
//! is written once, here, rather than again in the operation. What checks the
//! predicates themselves is the test that derives the same answers by walking the
//! map: two predicates that were a `matches!` over an enumeration nothing read
//! were deleted from `overflow` and `rounding`, and the difference here is that an
//! answer is a function of six facts about a domain rather than a restatement of
//! one enumeration, and that the test runs the machinery instead of reading the
//! declaration back.

use crate::adapt::{Adaptation, DeclaredSignature};
use crate::format::Format;
use crate::overflow::{Overflow, Policy};
use crate::rounding::{Mode, Rounding};
use crate::slots::Slots;
use crate::width::Bool;

mod reach;

pub use reach::Reach;

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
    // FIXME: the even-translation region is named here and expressed nowhere.
    // `Reach` carries no parity of the translation band, so
    // `rounding_is_translation_equivariant` has no disjunct for it and answers
    // no on a domain where the law in fact holds. Conservative, so nothing
    // unsound rests on it, and it costs a real region an arm could gate on.
    // Expressing it wants a further fact on `Reach`, that every representable
    // translation the domain reaches is even, which the declared signature does
    // not carry today. `the_classification` walks exactly that domain with a
    // step of two, so the instrument for checking it already exists.
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
/// names the vocabulary carries it equals reading something besides the residue,
/// and that is half a theorem and half a measurement: a rule that meets a tie and
/// reads only the residue cannot reflect, and a rule reading the sign need not.
/// Deriving it would put the measured half into the mechanism, so it is a law
/// with a test of its own instead.
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
///
/// `HalfUp` is `floor(x + q/2)`, which reads the residue and nothing else, so it
/// sits with the directed modes; it fails reflection at a tie and nowhere else.
/// The tie goes toward positive infinity at every sign, so a tie at -2.5 goes to
/// -2, and it is not the `HALF_UP` of Java or Python, which sends that tie the
/// other way below zero. That reflection failure is why the two differ at all:
/// the rule away from zero is the reflected one and it reads the sign, which is
/// a different row of this table.
#[must_use]
pub const fn behaviour_of(mode: Mode) -> Behaviour {
    match mode {
        Mode::Floor | Mode::Ceil | Mode::HalfUp | Mode::Stochastic => {
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
        Mode::HalfEven => {
            Behaviour {
                reads:    Reads::Parity,
                when:     When::AtATie,
                reflects: Bool::of(true),
            }
        },
    }
}

/// Whether the rounding region commutes with translation over what `reach` names.
///
/// Four disjuncts, and they are the four ways the thing a mode reads can fail to
/// be present: it reads nothing, or it reads only at a tie and no tie is reached,
/// or what it reads is the sign and no position it is asked about is negative, or
/// no position is off the grid, so the region is never entered at all.
#[must_use]
pub const fn rounding_is_translation_equivariant(mode: Mode, reach: Reach) -> Bool {
    let behaviour = behaviour_of(mode);
    let reads_nothing = Bool::of(matches!(behaviour.reads(), Reads::Nothing));
    let only_at_a_tie =
        Bool::of(matches!(behaviour.when(), When::AtATie)).and(reach.reaches_a_tie().not());
    let the_sign_does_not_vary = Bool::of(matches!(behaviour.reads(), Reads::Sign))
        .and(reach.reaches_a_negative_position().not());
    let never_entered = reach.reaches_off_the_grid().not();
    reads_nothing
        .or(only_at_a_tie)
        .or(the_sign_does_not_vary)
        .or(never_entered)
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
// FIXME: takes no reach, so it answers no in two regions where the map reflects:
// `HalfUp` over a reach with no tie, and every mode over a reach on the grid.
// Conservative rather than unsound, the same shape as the parity region above.
// Expressing it wants this predicate to read a `Reach`, and the stochastic
// mode's region at a fixed dither depends on the dither, which a reach does not
// carry; the design names both regions and expresses neither.
#[must_use]
pub const fn rounding_is_reflection_equivariant(mode: Mode) -> Bool {
    behaviour_of(mode).reflects()
}

/// Whether the completion region commutes with reflection through zero.
///
/// Wrapping does, because negation is an automorphism of the cyclic group it
/// reduces into. A clamp does where the range is symmetric about zero, and a
/// two's complement range is not: it carries one more slot below zero than above
/// it, so its lowest slot has no positive twin to be pinned to. The negation is
/// checked, because the highest slot of a range an outside crate places at the
/// bottom of the index is one whose negation leaves it, and such a range is not
/// symmetric about zero either way.
#[must_use]
pub const fn completion_is_reflection_equivariant<S: DeclaredSignature>(reach: Reach) -> Bool {
    let () = <<S::Format as Format>::Slots as Slots>::ADMITTED;
    let lowest = <<S::Format as Format>::Slots as Slots>::MIN;
    let highest = <<S::Format as Format>::Slots as Slots>::MAX;
    match <<S::Adaptation as Adaptation>::Overflow as Overflow>::POLICY {
        Policy::Wrap => Bool::of(true),
        Policy::Saturate | Policy::Clamp => {
            let symmetric = match highest.index().checked_neg() {
                Some(negated) => Bool::of(lowest.index() == negated),
                None => Bool::FALSE,
            };
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
