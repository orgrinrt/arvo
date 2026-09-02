//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The cancelling slot on its own, and the axes the shipped points hold fixed.
//!
//! The identity question is a conjunction: an integral cancelling slot exists at
//! some admitted magnitude, and it is in range. A law over the conjunction alone
//! cannot say which half a failure came from, so this file asserts each half
//! separately and then moves the coordinates the conjunction depends on: the
//! phase denominator, the quantum family, the slot family and the radix.
//!
//! The instruments are the parent's, so a construction here is the same `Grid`
//! and the same `Shrinking` the membership law is asserted over rather than a
//! second set that could drift from them.

use notko::Maybe;

use super::{Grid, Shrinking};
use crate::ambient::{BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
use crate::format::{cancelling_slot, has_additive_identity, step_exponent};
use crate::quantum::{Constant, Indexed, Magnitude};
use crate::slots::{Signed, Slot, Unsigned};
// --- the cancelling slot on its own ------------------------------------------
//
// The identity question is a conjunction: an integral cancelling slot exists at
// some admitted magnitude, and it is in range. A law over the conjunction alone
// cannot say which half a failure came from, so each half is asserted separately.

#[test]
fn the_cancelling_slot_is_the_negated_multiple_where_one_exists() {
    type Whole = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>;
    assert_eq!(
        cancelling_slot::<Whole>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-4))
    );

    type Halved = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 2>;
    assert_eq!(
        cancelling_slot::<Halved>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-2))
    );

    type Negative = Grid<BinaryRationals, Constant<0>, Signed<8>, -6, 2>;
    assert_eq!(
        cancelling_slot::<Negative>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(3))
    );

    // A zero phase is cancelled by slot zero, at every magnitude.
    type NoPhase = Grid<BinaryRationals, Indexed<0, 4>, Signed<8>, 0, 1>;
    assert_eq!(
        cancelling_slot::<NoPhase>(Magnitude::SMALLEST),
        Maybe::Is(Slot::ZERO)
    );
    assert_eq!(
        cancelling_slot::<NoPhase>(Magnitude::at(3)),
        Maybe::Is(Slot::ZERO)
    );
}

#[test]
fn the_cancelling_slot_is_isnt_where_the_phase_is_fractional_there() {
    type Half = Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 2>;
    assert_eq!(cancelling_slot::<Half>(Magnitude::SMALLEST), Maybe::Isnt);

    type Third = Grid<DecimalRationals, Constant<0>, Signed<8>, 1, 3>;
    assert_eq!(cancelling_slot::<Third>(Magnitude::SMALLEST), Maybe::Isnt);

    type SevenOverFour = Grid<BinaryRationals, Constant<0>, Signed<8>, 7, 4>;
    assert_eq!(
        cancelling_slot::<SevenOverFour>(Magnitude::SMALLEST),
        Maybe::Isnt
    );
}

#[test]
fn the_cancelling_slot_answers_both_ways_rather_than_one() {
    // The control for the pair above. A function stuck at `Isnt` passes the
    // second test alone, and one stuck at `Is` passes the first alone.
    type Whole = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>;
    type Half = Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 2>;
    assert_ne!(
        cancelling_slot::<Whole>(Magnitude::SMALLEST).is(),
        cancelling_slot::<Half>(Magnitude::SMALLEST).is(),
        "the cancelling slot gives the same verdict to a whole phase and a half one"
    );
}

#[test]
fn the_cancelling_slot_moves_with_the_magnitude_in_both_directions() {
    // Growing quantum: the same absolute phase is fewer steps higher up, so the
    // cancelling slot moves toward zero and stops existing once it would be
    // fractional.
    type Growing = Grid<BinaryRationals, Indexed<0, 6>, Signed<8>, 8, 1>;
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-8))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(1)),
        Maybe::Is(Slot::at(-4))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(2)),
        Maybe::Is(Slot::at(-2))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(3)),
        Maybe::Is(Slot::at(-1))
    );
    assert_eq!(cancelling_slot::<Growing>(Magnitude::at(4)), Maybe::Isnt);

    // Shrinking quantum: the same absolute phase is more steps higher up, so a
    // fractional phase becomes whole. An eighth needs three halvings.
    type Shrink = Grid<BinaryRationals, Shrinking<6>, Signed<62>, 1, 8>;
    assert_eq!(cancelling_slot::<Shrink>(Magnitude::SMALLEST), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Shrink>(Magnitude::at(1)), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Shrink>(Magnitude::at(2)), Maybe::Isnt);
    assert_eq!(
        cancelling_slot::<Shrink>(Magnitude::at(3)),
        Maybe::Is(Slot::at(-1))
    );
    assert_eq!(
        cancelling_slot::<Shrink>(Magnitude::at(4)),
        Maybe::Is(Slot::at(-2))
    );
}

#[test]
fn the_base_exponent_does_not_move_the_cancelling_slot() {
    // `BASE` cancels out of the equation, so moving it alone must move no answer.
    // If it does, the arithmetic is not what the doc comment says it is and every
    // arm in this file is suspect.
    type AtZero = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 2>;
    type AtSeven = Grid<BinaryRationals, Constant<7>, Signed<8>, 4, 2>;
    type AtMinusNine = Grid<BinaryRationals, Constant<-9>, Signed<8>, 4, 2>;
    assert_eq!(
        cancelling_slot::<AtZero>(Magnitude::SMALLEST),
        cancelling_slot::<AtSeven>(Magnitude::SMALLEST)
    );
    assert_eq!(
        cancelling_slot::<AtZero>(Magnitude::SMALLEST),
        cancelling_slot::<AtMinusNine>(Magnitude::SMALLEST)
    );
    assert!(has_additive_identity::<AtZero>().get());
    assert!(has_additive_identity::<AtSeven>().get());
    assert!(has_additive_identity::<AtMinusNine>().get());

    // And the step exponent does move with it, which is the control saying the
    // intercept is a live coordinate rather than one nothing reads.
    assert_ne!(
        step_exponent::<AtZero>(Magnitude::SMALLEST),
        step_exponent::<AtSeven>(Magnitude::SMALLEST)
    );
}

// --- the denominator, swept rather than fixed at two -------------------------

#[test]
fn the_identity_law_holds_at_every_denominator_rather_than_at_two() {
    // Every denominator from one to sixteen and a spread past it, with a
    // numerator that is a multiple of it and one that is not. Two is the only
    // denominator the shipped points can express, so without this the law that a
    // whole multiple keeps the identity is asked at one point of this axis.
    //
    // The quantum is constant here, so the magnitude cannot rescue a fractional
    // phase and the two directions are clean.
    macro_rules! sweep_denominators {
        ($($d:literal),+ $(,)?) => {
            $(
                {
                    type Whole = Grid<BinaryRationals, Constant<0>, Signed<16>, { 3 * $d }, $d>;
                    assert!(
                        has_additive_identity::<Whole>().get(),
                        "denominator {} lost the identity at a whole multiple", $d
                    );
                    assert_eq!(
                        cancelling_slot::<Whole>(Magnitude::SMALLEST),
                        Maybe::Is(Slot::at(-3)),
                        "denominator {} put the identity at the wrong slot", $d
                    );

                    type Off = Grid<BinaryRationals, Constant<0>, Signed<16>, { 3 * $d + 1 }, $d>;
                    if $d == 1 {
                        // Every integer is a whole multiple of one, so there is
                        // no fractional case at this denominator to assert.
                        assert!(has_additive_identity::<Off>().get());
                    } else {
                        assert!(
                            !has_additive_identity::<Off>().get(),
                            "denominator {} kept the identity at a fractional phase", $d
                        );
                    }
                }
            )+
        };
    }
    sweep_denominators!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 31, 64, 1000);
}

#[test]
fn a_negative_denominator_is_the_same_phase_with_the_sign_moved() {
    // Nothing says the denominator is positive, so the sign has to land somewhere
    // defined rather than somewhere nobody looked.
    type Positive = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 2>;
    type Negated = Grid<BinaryRationals, Constant<0>, Signed<8>, -4, -2>;
    assert_eq!(
        cancelling_slot::<Positive>(Magnitude::SMALLEST),
        cancelling_slot::<Negated>(Magnitude::SMALLEST)
    );

    // And moving only one of the two flips the slot rather than losing it.
    type Flipped = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, -2>;
    assert_eq!(
        cancelling_slot::<Flipped>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(2))
    );
    assert!(has_additive_identity::<Flipped>().get());
}

// --- both quantum families and both slot families ----------------------------

#[test]
fn a_whole_phase_keeps_the_identity_in_every_quantum_family() {
    // The axis `Biased` pins. A whole phase of four cancels at slot -4 in each
    // family, because at magnitude zero the quantum is the same in all three and
    // the slope only decides what happens above it.
    macro_rules! whole_phase_in {
        ($name:literal, $q:ty) => {{
            type Whole = Grid<BinaryRationals, $q, Signed<16>, 4, 1>;
            assert!(
                has_additive_identity::<Whole>().get(),
                "{} lost the identity at a whole phase",
                $name
            );
            assert_eq!(
                cancelling_slot::<Whole>(Magnitude::SMALLEST),
                Maybe::Is(Slot::at(-4))
            );
        }};
    }
    whole_phase_in!("constant quantum", Constant<0>);
    whole_phase_in!("indexed quantum", Indexed<0, 8>);
    whole_phase_in!("shrinking quantum", Shrinking<8>);
}

#[test]
fn a_phase_no_power_of_the_radix_cancels_has_no_identity_in_any_family() {
    // The other direction, and the denominator is three rather than two on
    // purpose. No power of two is divisible by three, so a third stays fractional
    // at every magnitude in every family, which a half does not: the shrinking
    // family cancels a half at magnitude one, so a half here would be asserting a
    // rule that holds only where the quantum does not move.
    macro_rules! never_cancelled_in {
        ($name:literal, $q:ty) => {{
            type Third = Grid<BinaryRationals, $q, Signed<16>, 1, 3>;
            assert!(
                !has_additive_identity::<Third>().get(),
                "{} cancelled a phase no power of the radix divides",
                $name
            );
        }};
    }
    never_cancelled_in!("constant quantum", Constant<0>);
    never_cancelled_in!("indexed quantum", Indexed<0, 8>);
    never_cancelled_in!("shrinking quantum", Shrinking<8>);

    // The control that separates this from the half-step case, and it is the
    // whole distinction in one line: the same shrinking family does cancel a half.
    type Half = Grid<BinaryRationals, Shrinking<8>, Signed<16>, 1, 2>;
    assert!(
        has_additive_identity::<Half>().get(),
        "the shrinking family stopped cancelling a half-step phase, so this test no \
         longer distinguishes a denominator the radix reaches from one it does not"
    );
}

#[test]
fn the_identity_law_holds_over_both_slot_families() {
    // The cancelling slot is the negated multiple, so its sign decides which
    // ranges can hold it. An unsigned range holds it for a negative phase and
    // never for a positive one, and a signed range holds both.
    type UnsignedNegativePhase = Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<16>, -4, 1>;
    assert_eq!(
        cancelling_slot::<UnsignedNegativePhase>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(4))
    );
    assert!(has_additive_identity::<UnsignedNegativePhase>().get());

    type UnsignedPositivePhase = Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<16>, 4, 1>;
    assert_eq!(
        cancelling_slot::<UnsignedPositivePhase>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-4))
    );
    assert!(
        !has_additive_identity::<UnsignedPositivePhase>().get(),
        "an unsigned range cancelled a positive phase, which needs a negative slot"
    );

    // The control: the same positive phase over a signed range of the same width
    // keeps the identity, so what the assertion above measured is the range.
    type SignedPositivePhase = Grid<BinaryRationals, Constant<0>, Signed<16>, 4, 1>;
    assert!(has_additive_identity::<SignedPositivePhase>().get());

    // And a zero phase sits at slot zero, which both families admit.
    type UnsignedNoPhase = Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<16>, 0, 1>;
    assert!(has_additive_identity::<UnsignedNoPhase>().get());
}

#[test]
fn the_identity_law_holds_at_a_radix_other_than_two() {
    // The radix is a coordinate of the cancellation and every other arm here runs
    // at two, so a claim about it is a claim about one value unless this runs.
    // At radix ten a fifth is cancelled by one step of a shrinking quantum, and a
    // third is cancelled by none, which is the same shape as halves and thirds at
    // radix two landing on different denominators.
    type Fifth = Grid<DecimalRationals, Shrinking<3>, Signed<16>, 1, 5>;
    assert_eq!(cancelling_slot::<Fifth>(Magnitude::SMALLEST), Maybe::Isnt);
    assert_eq!(
        cancelling_slot::<Fifth>(Magnitude::at(1)),
        Maybe::Is(Slot::at(-2))
    );
    assert!(has_additive_identity::<Fifth>().get());

    type Third = Grid<DecimalRationals, Shrinking<8>, Signed<16>, 1, 3>;
    assert!(
        !has_additive_identity::<Third>().get(),
        "a third was cancelled at radix ten, and no power of ten is divisible by three"
    );

    // The control: the same denominator at radix two gives the opposite verdict,
    // so which denominators are reachable is a fact about the radix.
    type FifthBinary = Grid<BinaryRationals, Shrinking<8>, Signed<16>, 1, 5>;
    assert!(
        !has_additive_identity::<FifthBinary>().get(),
        "a fifth was cancelled at radix two, and no power of two is divisible by five"
    );
}

// --- a whole phase whose cancelling slot is out of range ---------------------

#[test]
fn a_whole_phase_whose_cancelling_slot_is_out_of_range_has_no_identity() {
    // The case an arm set that only ever asserts a whole phase keeps the identity
    // cannot reach. A whole phase is necessary and not sufficient: the slot it
    // lands on has to be one the range admits.
    //
    // `Signed<2>` runs from -2 to 1, and a phase of four quanta cancels at slot
    // -4, which is outside it.
    type OutOfReach = Grid<BinaryRationals, Constant<0>, Signed<2>, 4, 1>;
    assert_eq!(
        cancelling_slot::<OutOfReach>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-4))
    );
    assert!(
        !has_additive_identity::<OutOfReach>().get(),
        "a whole phase kept the identity at a slot the range does not admit"
    );

    // The control: the same whole phase over a range wide enough to hold the
    // cancelling slot does keep it, so what the assertion above measured is the
    // range and not the phase.
    type InReach = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>;
    assert_eq!(
        cancelling_slot::<InReach>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-4))
    );
    assert!(has_additive_identity::<InReach>().get());
}
