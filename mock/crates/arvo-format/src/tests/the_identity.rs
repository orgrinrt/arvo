//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What decides whether zero is in the representable set.
//!
//! Its own file because it is its own law and because the predicate arms had
//! grown past what one file should carry. What is here is one question: whether
//! the grid holds zero, which the phase decides by being a whole multiple of the
//! quantum rather than by being nonzero.
//!
//! **Every control here is run rather than described.** The superseded predicate
//! is kept as a function and so is the narrow negation it would have used, so
//! "the earlier suite could not have seen this" and "the wide intermediate is
//! load-bearing" are assertions instead of sentences. A control nobody runs is a
//! claim about a mutant nobody built.

use crate::ambient::BinaryRationals;
use crate::format::{contains, has_additive_identity, step_exponent, Format, Phase};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{Constant, Exponent, Magnitude, MagnitudeCount, Quantum};
use crate::slots::{slot_in_range, Signed, Slot, Slots};
use crate::width::{Bool, Width};

/// The predicate this one replaced, kept so the control can be run.
///
/// It asked whether the phase sits at zero and looked at slot zero, which is a
/// different question from whether zero sits on the grid.
const fn the_superseded_predicate<F: Format>() -> Bool {
    F::PHASE
        .is_zero()
        .and(slot_in_range::<F::Slots>(Slot::ZERO))
}

/// The cancelling slot with the negation taken in the index's own width.
///
/// What the predicate would compute without the wide intermediate. `i64::MIN`
/// over one is a writable phase whose cancelling slot is two to the sixty-third,
/// and wrapping it lands on `i64::MIN`, which is a slot a range can admit.
const fn the_narrow_cancelling_slot(phase: Phase) -> Slot {
    Slot::at(
        phase
            .numerator()
            .wrapping_div(phase.denominator())
            .wrapping_neg(),
    )
}

// --- the identity is decidable at const time ---------------------------------

const _BIASED_IDENTITY: Bool = has_additive_identity::<Biased<7, -2, 2>>();

#[test]
fn the_identity_is_decidable_at_const_time() {
    // The binding above is the assertion; if the predicate's wide division
    // escaped const evaluation this file would not compile. The body checks it is
    // also correct rather than merely evaluable.
    assert!(_BIASED_IDENTITY.get());
}

// --- the two directions ------------------------------------------------------

#[test]
fn a_zero_phase_puts_the_additive_identity_on_the_grid() {
    assert!(has_additive_identity::<Integer<8>>().get());
    assert!(has_additive_identity::<UFixed<13, -4>>().get());
    assert!(has_additive_identity::<Floating<11, -14, 30>>().get());
}

#[test]
fn a_fractional_phase_takes_the_additive_identity_off_the_grid() {
    // `Biased` fixes the denominator at two, so an odd numerator is the half-step
    // bias. The canon carries the phase coordinate precisely because this is not
    // a corner case: no exact sum lands on that grid, and when its quantum is one
    // the grid holds neither zero nor one, so it is not a monoid carrier.
    assert!(!has_additive_identity::<Biased<7, -2, 1>>().get());
    assert!(!has_additive_identity::<Biased<13, 0, 1>>().get());
    assert!(!has_additive_identity::<Biased<31, -8, 3>>().get());
    assert!(!has_additive_identity::<Biased<9, -3, -5>>().get());

    // And a biased format with the phase set back to zero has it again, which is
    // the control saying the phase is what did it rather than the width.
    assert!(has_additive_identity::<Biased<7, -2, 0>>().get());
}

#[test]
fn a_whole_multiple_phase_keeps_the_identity_at_a_shifted_slot() {
    // The half the suite could not see. Every arm above writes an odd numerator
    // against a denominator the type fixes at two, so every phase tried was
    // fractional. An even numerator is a whole number of quanta: the grid shifts
    // onto itself and zero sits at the negated multiple rather than at slot zero.
    assert!(has_additive_identity::<Biased<4, 0, 2>>().get());
    assert!(has_additive_identity::<Biased<7, -2, 2>>().get());
    assert!(has_additive_identity::<Biased<13, 0, 4>>().get());
    assert!(has_additive_identity::<Biased<31, -8, -6>>().get());

    // The control, run rather than described. The superseded predicate answers no
    // on all four, so a suite unable to tell the two apart was not testing the
    // phase coordinate at all.
    assert!(!the_superseded_predicate::<Biased<4, 0, 2>>().get());
    assert!(!the_superseded_predicate::<Biased<7, -2, 2>>().get());
    assert!(!the_superseded_predicate::<Biased<13, 0, 4>>().get());
    assert!(!the_superseded_predicate::<Biased<31, -8, -6>>().get());

    // And a whole multiple the slot range cannot reach still has no identity, so
    // the arms are about the phase and the range together rather than about the
    // phase alone. `Signed<4>` runs from -8 to 7, and cancelling a phase of nine
    // quanta wants slot -9.
    assert!(!has_additive_identity::<Biased<4, 0, 18>>().get());
}

#[test]
fn the_superseded_predicate_differs_exactly_at_a_nonzero_whole_multiple() {
    // One sentence about the whole disagreement rather than a list of the arms
    // that happen to show it. The old question was whether the numerator is zero;
    // the right one is whether the ratio is an integer the range can cancel, so
    // the two part company on a nonzero whole multiple and agree everywhere else.
    let mut differed = 0;
    let mut agreed = 0;

    macro_rules! compare {
        ($($h:literal),+ $(,)?) => {
            $(
                {
                    type F = Biased<9, -3, $h>;
                    let now = has_additive_identity::<F>().get();
                    let before = the_superseded_predicate::<F>().get();
                    let phase = <F as Format>::PHASE;
                    // `Signed<9>` runs from -256 to 255, so every whole multiple
                    // below is one the range can cancel and the two really do
                    // differ there rather than agreeing by falling out of range.
                    let nonzero_whole = phase.numerator() != 0
                        && phase.numerator() % phase.denominator() == 0;
                    assert_eq!(
                        now != before,
                        nonzero_whole,
                        "at {} half steps the two predicates part company where they should not, \
                         or agree where they should not",
                        $h
                    );
                    if now != before { differed += 1; } else { agreed += 1; }
                }
            )+
        };
    }

    compare!(-8, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 8);

    // The control: both outcomes occur, so the equality above is a claim about
    // where they differ rather than one about a set that is empty on one side.
    assert!(
        differed > 0,
        "the two predicates never differ, so nothing was fixed"
    );
    assert!(
        agreed > 0,
        "the two predicates never agree, so the replacement is not a refinement of it"
    );
}

#[test]
fn the_two_questions_about_a_phase_are_two_questions() {
    // `is_zero` was asked where `is_whole_multiple` was meant, and the coordinate
    // being a ratio is what lets the two come apart. Written down because the
    // names read alike and only one of them decides the additive identity.
    let whole = Phase::halves(2);
    assert!(!whole.is_zero().get());
    assert!(whole.is_whole_multiple().get());

    let fractional = Phase::halves(1);
    assert!(!fractional.is_zero().get());
    assert!(!fractional.is_whole_multiple().get());

    assert!(Phase::ZERO.is_zero().get());
    assert!(Phase::ZERO.is_whole_multiple().get());

    // Over pairs the constructor normalises, so the second question is about the
    // ratio rather than about the numerator it was handed. `of(6, -3)` is `-2/1`
    // and whole; `of(1, -3)` is `-1/3` and not.
    assert!(Phase::of(6, -3).is_whole_multiple().get());
    assert!(!Phase::of(1, -3).is_whole_multiple().get());
    assert!(Phase::of(0, 7).is_whole_multiple().get());

    // The control: the two questions give different answers somewhere, so neither
    // is the other under a different name.
    let phases = [
        Phase::ZERO,
        Phase::halves(1),
        Phase::halves(2),
        Phase::of(1, 3),
    ];
    let disagreements = phases
        .iter()
        .filter(|p| p.is_zero().get() != p.is_whole_multiple().get())
        .count();
    assert_eq!(
        disagreements, 1,
        "the two questions agree everywhere here, so this arm separates nothing"
    );
}

// --- the law: the identity is membership at the slot it names ----------------

#[test]
fn the_identity_agrees_with_membership_at_the_slot_it_names() {
    // The two disagreed, which is the whole finding, so the agreement is a law
    // rather than a property of how the predicate happens to be written. The
    // cancelling slot is worked out here from the phase rather than taken from
    // the crate, so a predicate naming the wrong slot fails instead of agreeing
    // with itself.
    let mut present = 0;
    let mut absent = 0;

    macro_rules! agrees {
        ($($h:literal),+ $(,)?) => {
            $(
                {
                    type F = Biased<9, -3, $h>;
                    let phase = <F as Format>::PHASE;
                    let expected = if phase.numerator() % phase.denominator() == 0 {
                        let slot = Slot::at(-(phase.numerator() / phase.denominator()));
                        contains::<F>(slot, Magnitude::SMALLEST).get()
                    } else {
                        false
                    };
                    assert_eq!(
                        has_additive_identity::<F>().get(),
                        expected,
                        "at {} half steps the predicate and membership disagree", $h
                    );
                    if expected { present += 1; } else { absent += 1; }
                }
            )+
        };
    }

    // `Signed<9>` runs from -256 to 255, so a phase of 512 half steps cancels at
    // -256 and one of 514 wants -257, which the range does not hold. Both sides
    // of that edge, both signs, and the odd numerators in between.
    agrees!(-514, -512, -9, -8, -4, -3, -2, -1, 0, 1, 2, 3, 4, 8, 9, 510, 512, 514);

    // The control. A law that answered one way everywhere would pass every
    // assertion above, so both outcomes have to occur.
    assert!(
        present > 0,
        "no arm has an identity, so the law answers one way"
    );
    assert!(
        absent > 0,
        "every arm has an identity, so the law answers one way"
    );
}

// --- the wide intermediate, and the mutant that says why it is there ----------

/// A slot range sitting at the bottom of what an index carries.
///
/// Admissible: it is not inverted, its span is three, and three bits address it.
/// It exists so a negation that wrapped to `i64::MIN` would land inside a real
/// range rather than harmlessly outside every shipped one.
struct AtTheBottomSlots;

impl Slots for AtTheBottomSlots {
    const MIN: Slot = Slot::at(i64::MIN);
    const MAX: Slot = Slot::at(i64::MIN + 3);
    const WIDTH: Width = Width::bits(3);
}

/// A whole-multiple phase whose cancelling slot is one past what an index carries.
struct AtTheBottom;

impl Format for AtTheBottom {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = AtTheBottomSlots;
    const PHASE: Phase = Phase::of(i64::MIN, 1);
}

/// The same range with a phase whose cancelling slot it does hold.
struct NearTheBottom;

impl Format for NearTheBottom {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = AtTheBottomSlots;
    const PHASE: Phase = Phase::of(-(i64::MIN + 3), 1);
}

#[test]
fn a_phase_whose_cancelling_slot_leaves_the_index_answers_no() {
    // The phase is a whole multiple, so the divisibility half says yes and the
    // answer turns entirely on where the cancelling slot lands.
    assert!(<AtTheBottom as Format>::PHASE.is_whole_multiple().get());
    assert!(!has_additive_identity::<AtTheBottom>().get());

    // The mutant, run rather than described. Taking the negation in the index's
    // own width wraps two to the sixty-third down to `i64::MIN`, which this range
    // admits, so the narrow form answers yes to a position no slot can name. That
    // is what the wide intermediate is for, and it is why the range above is
    // declared where it is rather than at some convenient width.
    assert!(
        slot_in_range::<AtTheBottomSlots>(the_narrow_cancelling_slot(
            <AtTheBottom as Format>::PHASE
        ))
        .get()
    );

    // The control: the same range with a reachable cancelling slot answers yes,
    // so the arm above is about the overshoot rather than about a range that
    // refuses everything.
    assert!(has_additive_identity::<NearTheBottom>().get());

    // And the mutant agrees with the predicate wherever nothing overflows, so it
    // is the one value that separates them rather than a function that differs
    // everywhere.
    assert!(
        slot_in_range::<AtTheBottomSlots>(the_narrow_cancelling_slot(
            <NearTheBottom as Format>::PHASE
        ))
        .get()
    );
}

// --- the bound this predicate has, catalogued rather than claimed away --------

/// A step law whose quantum shrinks as the magnitude rises.
///
/// Nothing bounds the sign of a rate and the trait is open, so this is writable
/// and admitted. It is the shape `has_additive_identity` does not reach.
struct Shrinking;

impl Quantum for Shrinking {
    const BASE: Exponent = Exponent::ZERO;
    const SLOPE: Exponent = Exponent::of(-1);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(3);
}

/// A half-step phase over that law.
struct HalfOnShrinking;

impl Format for HalfOnShrinking {
    type Ambient = BinaryRationals;
    type Quantum = Shrinking;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::halves(1);
}

#[test]
#[ignore = "catalogue: the predicate answers at magnitude zero, which is the magnitude the phase is \
            stated in units of, so a law whose step shrinks as the magnitude rises cancels a \
            fractional phase higher up and nothing looks there. The canon does not reach this \
            shape: the finding that decides the predicate names its fraction width as the constant \
            family's exponent, so under the omission rule it says nothing about a magnitude-indexed \
            law at all. Red until that is settled and a decision procedure over the magnitudes is \
            designed rather than invented here."]
fn the_identity_survives_a_shrinking_quantum() {
    // The geometry, worked out rather than taken from the predicate. The step is
    // radix^0 at magnitude zero and radix^-1 at magnitude one, so it halves, and a
    // phase of one half is exactly one step up there.
    assert_eq!(
        step_exponent::<HalfOnShrinking>(Magnitude::SMALLEST),
        Exponent::ZERO
    );
    assert_eq!(
        step_exponent::<HalfOnShrinking>(Magnitude::at(1)),
        Exponent::of(-1)
    );

    // So 1/2 + (-1) * 1/2 is zero, at coordinates the format admits.
    assert!(contains::<HalfOnShrinking>(Slot::at(-1), Magnitude::at(1)).get());
    assert!(has_additive_identity::<HalfOnShrinking>().get());

    // The control that says this arm is about the magnitude rather than about the
    // phase: the same phase over a law that does not shrink genuinely has no
    // identity, and that is the region the predicate does cover.
    assert!(!has_additive_identity::<Biased<8, 0, 1>>().get());
}
