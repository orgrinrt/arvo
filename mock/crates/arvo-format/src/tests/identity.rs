//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The phase, and whether the representable set carries an additive identity.
//!
//! The question is the membership predicate asked at the value zero: some
//! admitted magnitude and some admitted slot cancel the phase, or none does. So
//! the arms below move every coordinate that appears in the cancellation, and the
//! ones that appear are the phase, the radix, the slope and the slot range.
//!
//! **The shipped points cannot carry this sweep.** `Biased` is the only one with
//! a nonzero phase and it fixes the denominator at two, the quantum at the
//! constant family and the slots at the signed range, while the other three set
//! the numerator to zero. A law asserted only through it is a law measured at one
//! point of three axes, and an arm set that never leaves that point cannot see the
//! regions off it: a numerator that is always odd against a denominator fixed at
//! two never once tries a whole phase, and a constant quantum never once gives the
//! magnitude anything to do. The generic format below is what lets the axes move
//! independently.
//!
//! **A half-step phase is not the general fractional case and is not used as one
//! here.** Whether a fractional phase stays fractional depends on the radix and
//! the slope together: a denominator of two is cancelled by one step of a
//! shrinking binary quantum, and a denominator of three is cancelled by no number
//! of them. So the arms wanting "fractional at every magnitude" use a denominator
//! the radix has no power of, and the ones wanting the opposite say so.
//!
//! What an implementor owes each contract is in `obligations`, because that is
//! about four traits rather than about this one question.

use core::marker::PhantomData;

use notko::Maybe;

use crate::ambient::{Ambient, BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
use crate::format::{cancelling_slot, contains, has_additive_identity, Format};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{Constant, Indexed, Quantum};
use crate::slots::{Signed, Slots, Unsigned};

// --- the instruments ---------------------------------------------------------

/// A format with every coordinate free, which is what the sweep needs.
///
/// The shipped points each pin at least two of the four axes the cancellation
/// depends on. This pins none.
struct Grid<A, Q, S, const PN: i64, const PD: i64>(PhantomData<(A, Q, S)>);

impl<A: Ambient, Q: Quantum, S: Slots, const PN: i64, const PD: i64> Format
    for Grid<A, Q, S, PN, PD>
{
    type Ambient = A;
    type Quantum = Q;
    type Slots = S;
    const PHASE_NUM: i64 = PN;
    const PHASE_DEN: i64 = PD;
}

/// A quantum whose step shrinks as the magnitude rises.
///
/// The crate ships slope zero and slope one, so this is the third sign, and it is
/// what an outside implementor may write: the trait is open and unsealed. It is
/// the family in which a phase with a fractional part at magnitude zero can be a
/// whole number of steps higher up.
struct Shrinking<const COUNT: u32>;

impl<const COUNT: u32> Quantum for Shrinking<COUNT> {
    const BASE: i32 = 0;
    const SLOPE: i32 = -1;
    const MAGNITUDES: u32 = COUNT;
}

// --- the identity is decidable at const time ---------------------------------
//
// Asserted as `const` bindings rather than as runtime calls. A runtime assertion
// passes whether or not the function is const, and being const is the property:
// `ruling::never_a_runtime_check_and_one_lowered_path` wants the branching gone
// before the backend, and a predicate that cannot be evaluated at const time
// leaves it there. The magnitude search is inside these, so the search evaluates
// at const time too.

const _IDENTITY_ON_THE_GRID: bool = has_additive_identity::<Integer<8>>();
const _IDENTITY_OFF_THE_GRID: bool = has_additive_identity::<Biased<7, -2, 1>>();
const _CANCELLING_SLOT: Maybe<i64> = cancelling_slot::<Biased<7, -2, 2>>(0);
const _FOUND_BY_SEARCHING_HIGHER_UP: bool =
    has_additive_identity::<Grid<BinaryRationals, Shrinking<2>, Signed<8>, 1, 2>>();
const _SEARCHED_AND_NOT_FOUND: bool =
    has_additive_identity::<Grid<BinaryRationals, Shrinking<40>, Signed<62>, 1, 3>>();

#[test]
fn the_identity_is_decidable_at_const_time_including_the_search() {
    // The five bindings above are the assertion; if any were not const this file
    // would not compile. This body checks they are also correct rather than
    // merely evaluable, which the bindings alone do not say.
    assert!(_IDENTITY_ON_THE_GRID);
    assert!(!_IDENTITY_OFF_THE_GRID);
    assert_eq!(_CANCELLING_SLOT, Maybe::Is(-1));
    assert!(
        _FOUND_BY_SEARCHING_HIGHER_UP,
        "the magnitude search did not run at const time, or ran and found nothing"
    );
    assert!(!_SEARCHED_AND_NOT_FOUND);
}

// --- the identity answer is the membership predicate at zero -----------------

/// Whether some admitted pair cancels the phase, decided through `contains`.
///
/// The design says the identity question is membership asked at the value zero,
/// so the two have to agree. This reaches the answer from the other side: it walks
/// the magnitude range the quantum declares rather than the bounded search, and it
/// asks `contains` rather than `slot_in_range`, so it exercises the membership
/// predicate itself.
///
/// It is not independent of `cancelling_slot` and does not claim to be. What it is
/// independent of is `has_additive_identity`: its search, its bound and its early
/// returns. A stronger check than this one is the predicate against an enumeration
/// written without reference to either, which is a separate instrument rather than
/// anything in this file.
fn zero_is_a_member<F: Format>() -> bool {
    let magnitudes = <F::Quantum as Quantum>::MAGNITUDES;
    (0..magnitudes).any(|magnitude| match cancelling_slot::<F>(magnitude) {
        Maybe::Is(slot) => contains::<F>(slot, magnitude),
        Maybe::Isnt => false,
    })
}

#[test]
fn the_identity_answer_is_membership_asked_at_zero() {
    // The law over a matrix rather than at a point. Every format here differs
    // from its neighbours in at least one of the four coordinates the
    // cancellation depends on, and both sides have to agree on every one.
    macro_rules! agree {
        ($($name:literal => $f:ty),+ $(,)?) => {
            $(
                assert_eq!(
                    has_additive_identity::<$f>(),
                    zero_is_a_member::<$f>(),
                    "{}: the identity answer and membership at zero disagree", $name
                );
            )+
        };
    }

    agree! {
        "integer" => Integer<8>,
        "fixed point" => UFixed<13, -4>,
        "floating" => Floating<11, -14, 30>,
        "half-step biased" => Biased<7, -2, 1>,
        "whole-step biased" => Biased<7, -2, 2>,
        "constant, whole phase" => Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>,
        "constant, whole phase out of range" => Grid<BinaryRationals, Constant<0>, Signed<2>, 4, 1>,
        "constant, third phase" => Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 3>,
        "indexed, whole phase" => Grid<BinaryRationals, Indexed<0, 8>, Signed<8>, 4, 1>,
        "indexed, found higher up" => Grid<BinaryRationals, Indexed<0, 2>, Signed<2>, 4, 1>,
        "indexed, one magnitude" => Grid<BinaryRationals, Indexed<0, 1>, Signed<2>, 4, 1>,
        "shrinking, half phase" => Grid<BinaryRationals, Shrinking<2>, Signed<8>, 1, 2>,
        "shrinking, one magnitude" => Grid<BinaryRationals, Shrinking<1>, Signed<8>, 1, 2>,
        "shrinking, third phase" => Grid<BinaryRationals, Shrinking<8>, Signed<16>, 1, 3>,
        "unsigned, negative phase" =>
            Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<16>, -4, 1>,
        "unsigned, positive phase" =>
            Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<16>, 4, 1>,
        "decimal, whole phase" => Grid<DecimalRationals, Constant<0>, Signed<8>, 5, 1>,
        "decimal, fifth phase" => Grid<DecimalRationals, Constant<0>, Signed<8>, 1, 5>,
        "extreme numerator" => Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, -1>,
        "extreme both" => Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, { i64::MIN }>,
    }
}

#[test]
fn the_control_the_matrix_above_contains_both_verdicts() {
    // A law asserting two functions agree is satisfied by both being stuck at the
    // same value, so the matrix has to contain formats of each kind or the test
    // above establishes nothing.
    assert!(has_additive_identity::<Integer<8>>());
    assert!(!has_additive_identity::<Biased<7, -2, 1>>());
    assert!(zero_is_a_member::<Integer<8>>());
    assert!(!zero_is_a_member::<Biased<7, -2, 1>>());
}

// --- the extremes of the phase coordinates -----------------------------------
//
// The phase numerator and denominator are signed 64-bit and nothing bounds them,
// so the ends of that range are reachable through the open trait. They are not
// reachable through the shipped points: three of the four fix the phase at zero
// over one, and `Biased` fixes the denominator at two.
//
// **Solved in the width the coordinates are declared in, the cancellation has two
// pairs it cannot answer.** The least numerator over minus one overflows the
// remainder, and that numerator over one produces a quotient whose negation
// overflows. Both diverge rather than answering, and diverging on the value path
// is what `ruling::never_a_runtime_check_and_one_lowered_path` forbids. Neither is
// reachable by a guard placed before the arithmetic, because both are the
// arithmetic.
//
// Carrying the division one width up is what makes them defined, because the only
// overflowing pair in a signed division is the least value over minus one and the
// declared width's least value is nowhere near the wider one's. The range check
// afterwards is what turns a quotient no slot index can hold into `Isnt` rather
// than into a wrap.

#[test]
fn the_extreme_phase_coordinates_are_answered_rather_than_overflowing() {
    // The least numerator over minus one. The phase is 2^63 quanta and the
    // cancelling slot is -2^63, which is representable as a slot index. No
    // admitted slot range reaches it, since the widest is 62 bits, so there is no
    // identity and the answer is a decided one.
    type MinOverMinusOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, -1>;
    assert_eq!(cancelling_slot::<MinOverMinusOne>(0), Maybe::Is(i64::MIN));
    assert!(!has_additive_identity::<MinOverMinusOne>());

    // The least numerator over one. The cancelling slot would be +2^63, which is
    // one past what a slot index carries, so it is `Isnt` rather than a wrapped
    // value landing inside somebody's range.
    type MinOverOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, 1>;
    assert_eq!(cancelling_slot::<MinOverOne>(0), Maybe::Isnt);
    assert!(!has_additive_identity::<MinOverOne>());

    // The greatest numerator over minus one, which is the same shape without the
    // asymmetry that makes the pair above overflow.
    type MaxOverMinusOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MAX }, -1>;
    assert_eq!(cancelling_slot::<MaxOverMinusOne>(0), Maybe::Is(i64::MAX));
    assert!(!has_additive_identity::<MaxOverMinusOne>());

    // And the control, which is what keeps the three above from being a function
    // that answers `Isnt` or an unreachable slot for anything extreme: the two
    // ends divide to a phase of one, whose cancelling slot is -1 and is in range,
    // so this one does have an identity.
    type MinOverMin = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, { i64::MIN }>;
    assert_eq!(cancelling_slot::<MinOverMin>(0), Maybe::Is(-1));
    assert!(
        has_additive_identity::<MinOverMin>(),
        "the extreme pair that divides to a phase of one lost its identity"
    );
}

#[test]
fn an_extreme_phase_still_answers_at_every_magnitude() {
    // The extremes against a moving quantum, because the arms above are all at a
    // constant one and the scaling that could overflow is the one the magnitude
    // drives. A growing quantum divides the phase down toward a slot a range can
    // hold, and from 2^63 that takes 63 magnitudes.
    type Growing = Grid<BinaryRationals, Indexed<0, 64>, Signed<8>, { i64::MIN }, -1>;
    assert_eq!(cancelling_slot::<Growing>(0), Maybe::Is(i64::MIN));
    assert_eq!(cancelling_slot::<Growing>(62), Maybe::Is(-2));
    assert_eq!(cancelling_slot::<Growing>(63), Maybe::Is(-1));
    assert!(
        has_additive_identity::<Growing>(),
        "a phase of 2^63 quanta was never divided down into an admitted slot"
    );

    // The control: the same phase with only one magnitude has nowhere to divide
    // down to, so the identity is off the grid.
    type OneMagnitude = Grid<BinaryRationals, Indexed<0, 1>, Signed<8>, { i64::MIN }, -1>;
    assert!(!has_additive_identity::<OneMagnitude>());
}

// --- the phase through the one shipped point that carries one ----------------

#[test]
fn a_zero_phase_puts_the_additive_identity_on_the_grid() {
    assert!(has_additive_identity::<Integer<8>>());
    assert!(has_additive_identity::<UFixed<13, -4>>());
    assert!(has_additive_identity::<Floating<11, -14, 30>>());
}

#[test]
fn a_fractional_phase_takes_the_additive_identity_off_the_grid() {
    // `Biased` fixes the denominator at two and the quantum at the constant
    // family, so an odd numerator is the half-step bias and no magnitude can
    // cancel it. The canon carries the phase coordinate precisely because this is
    // not a corner case: no exact sum lands on that grid and it is not a monoid
    // carrier.
    assert!(!has_additive_identity::<Biased<7, -2, 1>>());
    assert!(!has_additive_identity::<Biased<13, 0, 1>>());
    assert!(!has_additive_identity::<Biased<31, -8, 3>>());

    // And a biased format with the phase set back to zero has it again, which is
    // the control saying the phase is what did it rather than the width.
    assert!(has_additive_identity::<Biased<7, -2, 0>>());
}

#[test]
fn a_whole_multiple_phase_keeps_the_identity_at_a_shifted_slot() {
    // An even numerator against the denominator this point fixes at two is a
    // whole number of quanta: the grid shifts onto itself and zero sits at the
    // negated multiple rather than at slot zero.
    assert!(has_additive_identity::<Biased<4, 0, 2>>());
    assert!(has_additive_identity::<Biased<7, -2, 2>>());
    assert!(has_additive_identity::<Biased<13, 0, 4>>());
    assert!(has_additive_identity::<Biased<31, -8, -6>>());

    // The control, and it is what makes the four arms above mean anything: a
    // predicate that ignored the numerator entirely would pass all of them and
    // fail here.
    assert!(!has_additive_identity::<Biased<4, 0, 1>>());
}

// --- the cancelling slot on its own ------------------------------------------
//
// The identity question is a conjunction: an integral cancelling slot exists at
// some admitted magnitude, and it is in range. A law over the conjunction alone
// cannot say which half a failure came from, so each half is asserted separately.

#[test]
fn the_cancelling_slot_is_the_negated_multiple_where_one_exists() {
    type Whole = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>;
    assert_eq!(cancelling_slot::<Whole>(0), Maybe::Is(-4));

    type Halved = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 2>;
    assert_eq!(cancelling_slot::<Halved>(0), Maybe::Is(-2));

    type Negative = Grid<BinaryRationals, Constant<0>, Signed<8>, -6, 2>;
    assert_eq!(cancelling_slot::<Negative>(0), Maybe::Is(3));

    // A zero phase is cancelled by slot zero, at every magnitude.
    type NoPhase = Grid<BinaryRationals, Indexed<0, 4>, Signed<8>, 0, 1>;
    assert_eq!(cancelling_slot::<NoPhase>(0), Maybe::Is(0));
    assert_eq!(cancelling_slot::<NoPhase>(3), Maybe::Is(0));
}

#[test]
fn the_cancelling_slot_is_isnt_where_the_phase_is_fractional_there() {
    type Half = Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 2>;
    assert_eq!(cancelling_slot::<Half>(0), Maybe::Isnt);

    type Third = Grid<DecimalRationals, Constant<0>, Signed<8>, 1, 3>;
    assert_eq!(cancelling_slot::<Third>(0), Maybe::Isnt);

    type SevenOverFour = Grid<BinaryRationals, Constant<0>, Signed<8>, 7, 4>;
    assert_eq!(cancelling_slot::<SevenOverFour>(0), Maybe::Isnt);
}

#[test]
fn the_cancelling_slot_answers_both_ways_rather_than_one() {
    // The control for the pair above. A function stuck at `Isnt` passes the
    // second test alone, and one stuck at `Is` passes the first alone.
    type Whole = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>;
    type Half = Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 2>;
    assert_ne!(
        cancelling_slot::<Whole>(0).is(),
        cancelling_slot::<Half>(0).is(),
        "the cancelling slot gives the same verdict to a whole phase and a half one"
    );
}

#[test]
fn the_cancelling_slot_moves_with_the_magnitude_in_both_directions() {
    // Growing quantum: the same absolute phase is fewer steps higher up, so the
    // cancelling slot moves toward zero and stops existing once it would be
    // fractional.
    type Growing = Grid<BinaryRationals, Indexed<0, 6>, Signed<8>, 8, 1>;
    assert_eq!(cancelling_slot::<Growing>(0), Maybe::Is(-8));
    assert_eq!(cancelling_slot::<Growing>(1), Maybe::Is(-4));
    assert_eq!(cancelling_slot::<Growing>(2), Maybe::Is(-2));
    assert_eq!(cancelling_slot::<Growing>(3), Maybe::Is(-1));
    assert_eq!(cancelling_slot::<Growing>(4), Maybe::Isnt);

    // Shrinking quantum: the same absolute phase is more steps higher up, so a
    // fractional phase becomes whole. An eighth needs three halvings.
    type Shrink = Grid<BinaryRationals, Shrinking<6>, Signed<62>, 1, 8>;
    assert_eq!(cancelling_slot::<Shrink>(0), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Shrink>(1), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Shrink>(2), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Shrink>(3), Maybe::Is(-1));
    assert_eq!(cancelling_slot::<Shrink>(4), Maybe::Is(-2));
}

#[test]
fn the_base_exponent_does_not_move_the_cancelling_slot() {
    // `BASE` cancels out of the equation, so moving it alone must move no answer.
    // If it does, the arithmetic is not what the doc comment says it is and every
    // arm in this file is suspect.
    type AtZero = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 2>;
    type AtSeven = Grid<BinaryRationals, Constant<7>, Signed<8>, 4, 2>;
    type AtMinusNine = Grid<BinaryRationals, Constant<-9>, Signed<8>, 4, 2>;
    assert_eq!(cancelling_slot::<AtZero>(0), cancelling_slot::<AtSeven>(0));
    assert_eq!(
        cancelling_slot::<AtZero>(0),
        cancelling_slot::<AtMinusNine>(0)
    );
    assert!(has_additive_identity::<AtZero>());
    assert!(has_additive_identity::<AtSeven>());
    assert!(has_additive_identity::<AtMinusNine>());
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
                        has_additive_identity::<Whole>(),
                        "denominator {} lost the identity at a whole multiple", $d
                    );
                    assert_eq!(
                        cancelling_slot::<Whole>(0),
                        Maybe::Is(-3),
                        "denominator {} put the identity at the wrong slot", $d
                    );

                    type Off = Grid<BinaryRationals, Constant<0>, Signed<16>, { 3 * $d + 1 }, $d>;
                    if $d == 1 {
                        // Every integer is a whole multiple of one, so there is
                        // no fractional case at this denominator to assert.
                        assert!(has_additive_identity::<Off>());
                    } else {
                        assert!(
                            !has_additive_identity::<Off>(),
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
    assert_eq!(cancelling_slot::<Positive>(0), cancelling_slot::<Negated>(0));

    // And moving only one of the two flips the slot rather than losing it.
    type Flipped = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, -2>;
    assert_eq!(cancelling_slot::<Flipped>(0), Maybe::Is(2));
    assert!(has_additive_identity::<Flipped>());
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
                has_additive_identity::<Whole>(),
                "{} lost the identity at a whole phase",
                $name
            );
            assert_eq!(cancelling_slot::<Whole>(0), Maybe::Is(-4));
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
                !has_additive_identity::<Third>(),
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
        has_additive_identity::<Half>(),
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
    assert_eq!(cancelling_slot::<UnsignedNegativePhase>(0), Maybe::Is(4));
    assert!(has_additive_identity::<UnsignedNegativePhase>());

    type UnsignedPositivePhase = Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<16>, 4, 1>;
    assert_eq!(cancelling_slot::<UnsignedPositivePhase>(0), Maybe::Is(-4));
    assert!(
        !has_additive_identity::<UnsignedPositivePhase>(),
        "an unsigned range cancelled a positive phase, which needs a negative slot"
    );

    // The control: the same positive phase over a signed range of the same width
    // keeps the identity, so what the assertion above measured is the range.
    type SignedPositivePhase = Grid<BinaryRationals, Constant<0>, Signed<16>, 4, 1>;
    assert!(has_additive_identity::<SignedPositivePhase>());

    // And a zero phase sits at slot zero, which both families admit.
    type UnsignedNoPhase = Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<16>, 0, 1>;
    assert!(has_additive_identity::<UnsignedNoPhase>());
}

#[test]
fn the_identity_law_holds_at_a_radix_other_than_two() {
    // The radix is a coordinate of the cancellation and every other arm here runs
    // at two, so a claim about it is a claim about one value unless this runs.
    // At radix ten a fifth is cancelled by one step of a shrinking quantum, and a
    // third is cancelled by none, which is the same shape as halves and thirds at
    // radix two landing on different denominators.
    type Fifth = Grid<DecimalRationals, Shrinking<3>, Signed<16>, 1, 5>;
    assert_eq!(cancelling_slot::<Fifth>(0), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Fifth>(1), Maybe::Is(-2));
    assert!(has_additive_identity::<Fifth>());

    type Third = Grid<DecimalRationals, Shrinking<8>, Signed<16>, 1, 3>;
    assert!(
        !has_additive_identity::<Third>(),
        "a third was cancelled at radix ten, and no power of ten is divisible by three"
    );

    // The control: the same denominator at radix two gives the opposite verdict,
    // so which denominators are reachable is a fact about the radix.
    type FifthBinary = Grid<BinaryRationals, Shrinking<8>, Signed<16>, 1, 5>;
    assert!(
        !has_additive_identity::<FifthBinary>(),
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
    assert_eq!(cancelling_slot::<OutOfReach>(0), Maybe::Is(-4));
    assert!(
        !has_additive_identity::<OutOfReach>(),
        "a whole phase kept the identity at a slot the range does not admit"
    );

    // The control: the same whole phase over a range wide enough to hold the
    // cancelling slot does keep it, so what the assertion above measured is the
    // range and not the phase.
    type InReach = Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>;
    assert_eq!(cancelling_slot::<InReach>(0), Maybe::Is(-4));
    assert!(has_additive_identity::<InReach>());
}

// --- the magnitude range, which is the coordinate a constant quantum hides ----

#[test]
fn a_whole_phase_out_of_reach_low_down_is_found_at_a_higher_magnitude() {
    // Every coordinate here is one this crate ships, so an outside `Format`
    // reaches this with no outside `Quantum` at all. The quantum doubles per
    // magnitude, so the same absolute phase is half as many steps at each one:
    // slot -4 at magnitude zero, outside `Signed<2>`, and slot -2 at magnitude
    // one, which is the range's own lowest index.
    type Growing = Grid<BinaryRationals, Indexed<0, 2>, Signed<2>, 4, 1>;
    assert_eq!(cancelling_slot::<Growing>(0), Maybe::Is(-4));
    assert_eq!(cancelling_slot::<Growing>(1), Maybe::Is(-2));
    assert!(
        has_additive_identity::<Growing>(),
        "the cancelling slot in range at magnitude one was not found"
    );

    // The control, and it is what says the search found it rather than the first
    // magnitude: cutting the magnitude range to one takes the identity away while
    // every other coordinate stays where it was.
    type OneMagnitude = Grid<BinaryRationals, Indexed<0, 1>, Signed<2>, 4, 1>;
    assert_eq!(cancelling_slot::<OneMagnitude>(0), Maybe::Is(-4));
    assert!(
        !has_additive_identity::<OneMagnitude>(),
        "cutting the magnitude range to one did not take the identity back"
    );
}

#[test]
fn a_fractional_phase_that_becomes_whole_higher_up_keeps_the_identity() {
    // The half that refutes the whole-multiple reading outright. The quantum
    // halves per magnitude, so a phase of one half is a whole step at magnitude
    // one, and the identity is on the grid despite the phase having a fractional
    // part at magnitude zero.
    type Shrink = Grid<BinaryRationals, Shrinking<2>, Signed<8>, 1, 2>;
    assert_eq!(cancelling_slot::<Shrink>(0), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Shrink>(1), Maybe::Is(-1));
    assert!(
        has_additive_identity::<Shrink>(),
        "a fractional phase that becomes whole at magnitude one lost the identity"
    );

    // The control: one magnitude and the same fractional phase has no identity,
    // so the magnitude range is what made the difference.
    type OneMagnitude = Grid<BinaryRationals, Shrinking<1>, Signed<8>, 1, 2>;
    assert!(
        !has_additive_identity::<OneMagnitude>(),
        "cutting the magnitude range to one did not take the identity back"
    );

    // And the phase still has to become whole eventually. A denominator of three
    // never divides a power of two, so no magnitude cancels it.
    type NeverWhole = Grid<BinaryRationals, Shrinking<40>, Signed<62>, 1, 3>;
    assert!(
        !has_additive_identity::<NeverWhole>(),
        "a phase whose denominator no power of the radix divides gained an identity"
    );
}

#[test]
fn the_magnitude_the_identity_is_found_at_is_not_always_the_first() {
    // Stated on its own because it is the whole content of the law: the
    // existential runs over the magnitude, so the answer can come from anywhere
    // in the range rather than from its bottom.
    type Growing = Grid<BinaryRationals, Indexed<0, 4>, Signed<2>, 16, 1>;
    assert_eq!(cancelling_slot::<Growing>(0), Maybe::Is(-16));
    assert_eq!(cancelling_slot::<Growing>(1), Maybe::Is(-8));
    assert_eq!(cancelling_slot::<Growing>(2), Maybe::Is(-4));
    assert_eq!(cancelling_slot::<Growing>(3), Maybe::Is(-2));
    assert!(has_additive_identity::<Growing>());

    // Only magnitude three answers, so a search reading any single magnitude
    // gives the wrong answer whichever one it reads. If a later change made a
    // second magnitude answer, this arm would stop showing that the search is
    // what did it, which is why the count is asserted rather than assumed.
    let mut answering = 0;
    for magnitude in 0..<Indexed<0, 4> as Quantum>::MAGNITUDES {
        if let Maybe::Is(slot) = cancelling_slot::<Growing>(magnitude) {
            if (-2..=1).contains(&slot) {
                answering += 1;
            }
        }
    }
    assert_eq!(
        answering, 1,
        "the witness is not unique, so this arm does not show the search mattered"
    );
}

// --- the search bound, which is the derivation pinned ------------------------

#[test]
fn the_search_bound_is_past_where_a_radix_of_two_can_still_answer() {
    // The first half of the derivation on `MAGNITUDE_SEARCH_BOUND`: at a radix of
    // at least two the running product leaves the wider width within 127 steps,
    // so every magnitude past the bound answers `Isnt` and stopping there loses
    // nothing.
    type Growing = Grid<BinaryRationals, Indexed<0, 300>, Signed<62>, 4, 1>;
    assert!(cancelling_slot::<Growing>(2).is());
    assert_eq!(cancelling_slot::<Growing>(127), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Growing>(200), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Growing>(u32::MAX), Maybe::Isnt);

    type Shrink = Grid<BinaryRationals, Shrinking<300>, Signed<62>, 1, 2>;
    assert!(cancelling_slot::<Shrink>(1).is());
    assert_eq!(cancelling_slot::<Shrink>(127), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Shrink>(u32::MAX), Maybe::Isnt);
}

#[test]
fn the_search_bound_loses_nothing_where_the_quantum_does_not_move() {
    // The second half: at a zero slope every magnitude gives the same equation,
    // so one answers them all and a bound cannot cut anything off.
    type Flat = Grid<BinaryRationals, Constant<3>, Signed<8>, 4, 1>;
    assert_eq!(cancelling_slot::<Flat>(0), Maybe::Is(-4));
    assert_eq!(cancelling_slot::<Flat>(1), Maybe::Is(-4));
    assert_eq!(cancelling_slot::<Flat>(127), Maybe::Is(-4));
    assert_eq!(cancelling_slot::<Flat>(u32::MAX), Maybe::Is(-4));
}

#[test]
fn a_magnitude_range_past_the_bound_still_finds_what_is_below_it() {
    // The bound cuts the search and not the answer. A format declaring far more
    // magnitudes than the bound still gets the identity that sits at a low one.
    type Wide = Grid<BinaryRationals, Indexed<0, 100_000>, Signed<8>, 4, 1>;
    assert!(has_additive_identity::<Wide>());

    // And one whose phase no magnitude can cancel does not, which is the honest
    // statement of what the bound costs: nothing, because no magnitude past it
    // can answer at a radix of at least two.
    type NoAnswer = Grid<BinaryRationals, Indexed<0, 100_000>, Signed<2>, 1, 3>;
    assert!(!has_additive_identity::<NoAnswer>());
}
