//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What decides whether zero is in the representable set.
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
//! magnitude anything to do. `Grid` below is what lets the axes move
//! independently.
//!
//! **A half-step phase is not the general fractional case and is not used as one
//! here.** Whether a fractional phase stays fractional depends on the radix and
//! the slope together: a denominator of two is cancelled by one step of a
//! shrinking binary quantum, and a denominator of three is cancelled by no number
//! of them. So the arms wanting "fractional at every magnitude" use a denominator
//! the radix has no power of, and the ones wanting the opposite say so.
//!
//! **Every control here is run rather than described.** The superseded predicate
//! is kept as a function and so is the narrow negation it would have used, so
//! "the earlier suite could not have seen this" and "the wide intermediate is
//! load-bearing" are assertions instead of sentences. A control nobody runs is a
//! claim about a mutant nobody built.
//!
//! What an implementor owes each contract is in `obligations`, because that is
//! about four traits rather than about this one question.

use core::marker::PhantomData;

use notko::Maybe;

use crate::ambient::{Ambient, BinaryRationals, DecimalRationals, UnsignedBinaryRationals};
use crate::format::{cancelling_slot, contains, has_additive_identity, Format, Phase};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{Constant, Exponent, Indexed, Magnitude, MagnitudeCount, Quantum};
use crate::slots::{slot_in_range, Signed, Slot, Slots, Unsigned};
use crate::width::Bool;

mod the_cancelling_slot;
mod the_magnitude_range;

// --- the instruments ---------------------------------------------------------

/// A format with every coordinate free, which is what the sweep needs.
///
/// The shipped points each pin at least two of the four axes the cancellation
/// depends on. This pins none.
///
/// The phase arrives as two const generic parameters and becomes a `Phase` in the
/// impl body. An associated const takes an arbitrary expression, so the coordinate
/// carrying a type of its own costs the sweep nothing; only a const generic
/// parameter is restricted to the host's types, and that is the position
/// `tests/ui/an_arvo_type_as_a_const_parameter.rs` pins.
struct Grid<A, Q, S, const PN: i64, const PD: i64>(PhantomData<(A, Q, S)>);

impl<A: Ambient, Q: Quantum, S: Slots, const PN: i64, const PD: i64> Format
    for Grid<A, Q, S, PN, PD>
{
    type Ambient = A;
    type Quantum = Q;
    type Slots = S;
    const PHASE: Phase = Phase::of(PN, PD);
}

/// A quantum whose step shrinks as the magnitude rises.
///
/// The crate ships slope zero and slope one, so this is the third sign, and it is
/// what an outside implementor may write: the trait is open and unsealed. It is
/// the family in which a phase with a fractional part at magnitude zero can be a
/// whole number of steps higher up.
struct Shrinking<const COUNT: u32>;

impl<const COUNT: u32> Quantum for Shrinking<COUNT> {
    const BASE: Exponent = Exponent::ZERO;
    const SLOPE: Exponent = Exponent::of(-1);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(COUNT);
}

/// The predicate this one replaced, kept so the control can be run.
///
/// It asked whether the phase sits at zero and looked at slot zero, which is a
/// different question from whether zero sits on the grid.
const fn the_superseded_predicate<F: Format>() -> Bool {
    F::PHASE
        .is_zero()
        .and(slot_in_range::<F::Slots>(Slot::ZERO))
}

/// The predicate that answers at one magnitude, kept so that control runs too.
///
/// The reduction the constant family licenses, applied to both families. It is
/// what a predicate reading `Phase::is_whole_multiple` and stopping would compute,
/// and the arms below are what say the two are not the same function.
fn the_one_magnitude_predicate<F: Format>() -> Bool {
    if !F::PHASE.is_whole_multiple().get() {
        return Bool::FALSE;
    }
    match cancelling_slot::<F>(Magnitude::SMALLEST) {
        Maybe::Is(slot) => contains::<F>(slot, Magnitude::SMALLEST),
        Maybe::Isnt => Bool::FALSE,
    }
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
//
// Asserted as `const` bindings rather than as runtime calls. A runtime assertion
// passes whether or not the function is const, and being const is the property:
// `ruling::never_a_runtime_check_and_one_lowered_path` wants the branching gone
// before the backend, and a predicate that cannot be evaluated at const time
// leaves it there. The magnitude search is inside these, so the search evaluates
// at const time too.

const _BIASED_IDENTITY: Bool = has_additive_identity::<Biased<7, -2, 2>>();
const _IDENTITY_ON_THE_GRID: Bool = has_additive_identity::<Integer<8>>();
const _IDENTITY_OFF_THE_GRID: Bool = has_additive_identity::<Biased<7, -2, 1>>();
const _CANCELLING_SLOT: Maybe<Slot> = cancelling_slot::<Biased<7, -2, 2>>(Magnitude::SMALLEST);
const _FOUND_BY_SEARCHING_HIGHER_UP: Bool =
    has_additive_identity::<Grid<BinaryRationals, Shrinking<2>, Signed<8>, 1, 2>>();
const _SEARCHED_AND_NOT_FOUND: Bool =
    has_additive_identity::<Grid<BinaryRationals, Shrinking<40>, Signed<62>, 1, 3>>();

#[test]
fn the_identity_is_decidable_at_const_time_including_the_search() {
    // The six bindings above are the assertion; if any were not const this file
    // would not compile. This body checks they are also correct rather than
    // merely evaluable, which the bindings alone do not say.
    assert!(_BIASED_IDENTITY.get());
    assert!(_IDENTITY_ON_THE_GRID.get());
    assert!(!_IDENTITY_OFF_THE_GRID.get());
    assert_eq!(_CANCELLING_SLOT, Maybe::Is(Slot::at(-1)));
    assert!(
        _FOUND_BY_SEARCHING_HIGHER_UP.get(),
        "the magnitude search did not run at const time, or ran and found nothing"
    );
    assert!(!_SEARCHED_AND_NOT_FOUND.get());
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
/// returns.
fn zero_is_a_member<F: Format>() -> bool {
    let magnitudes = <F::Quantum as Quantum>::MAGNITUDES.count();
    (0..magnitudes).any(|index| match cancelling_slot::<F>(Magnitude::at(index)) {
        Maybe::Is(slot) => contains::<F>(slot, Magnitude::at(index)).get(),
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
                    has_additive_identity::<$f>().get(),
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
    assert!(has_additive_identity::<Integer<8>>().get());
    assert!(!has_additive_identity::<Biased<7, -2, 1>>().get());
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
fn a_phase_keeps_the_value_it_was_declared_with() {
    // The coordinate holds the pair rather than a normalisation of it, and these
    // are the two pairs no normalisation inside the declared width can keep. A
    // constructor moving the sign to the numerator has to negate the denominator,
    // which the least value has no room for, so it read both as a denominator of
    // one and changed the value it was handed.
    let tiny_negative = Phase::of(3, i64::MIN);
    assert_eq!(tiny_negative.numerator(), 3);
    assert_eq!(tiny_negative.denominator(), i64::MIN);

    let large_positive = Phase::of(i64::MIN, -7);
    assert_eq!(large_positive.numerator(), i64::MIN);
    assert_eq!(large_positive.denominator(), -7);

    // Neither is a whole number of quanta, which is the question actually asked
    // of the pair, and the remainder that answers it is taken one width up
    // because the second pair overflows it in the declared one.
    assert!(!tiny_negative.is_whole_multiple().get());
    assert!(!large_positive.is_whole_multiple().get());

    // The control that says the reading is the divisibility rather than the sign:
    // the same magnitudes with a denominator that divides answer yes.
    assert!(Phase::of(i64::MIN, -1).is_whole_multiple().get());
    assert!(Phase::of(i64::MIN, i64::MIN).is_whole_multiple().get());
}

#[test]
fn the_extreme_phase_coordinates_are_answered_rather_than_overflowing() {
    // The least numerator over minus one. The phase is 2^63 quanta and the
    // cancelling slot is -2^63, which is representable as a slot index. No
    // admitted slot range reaches it, since the widest is 62 bits, so there is no
    // identity and the answer is a decided one.
    type MinOverMinusOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, -1>;
    assert_eq!(
        cancelling_slot::<MinOverMinusOne>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(i64::MIN))
    );
    assert!(!has_additive_identity::<MinOverMinusOne>().get());

    // The least numerator over one. The cancelling slot would be +2^63, which is
    // one past what a slot index carries, so it is `Isnt` rather than a wrapped
    // value landing inside somebody's range.
    type MinOverOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, 1>;
    assert_eq!(
        cancelling_slot::<MinOverOne>(Magnitude::SMALLEST),
        Maybe::Isnt
    );
    assert!(!has_additive_identity::<MinOverOne>().get());

    // The greatest numerator over minus one, which is the same shape without the
    // asymmetry that makes the pair above overflow.
    type MaxOverMinusOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MAX }, -1>;
    assert_eq!(
        cancelling_slot::<MaxOverMinusOne>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(i64::MAX))
    );
    assert!(!has_additive_identity::<MaxOverMinusOne>().get());

    // And the control, which is what keeps the three above from being a function
    // that answers `Isnt` or an unreachable slot for anything extreme: the two
    // ends divide to a phase of one, whose cancelling slot is -1 and is in range,
    // so this one does have an identity.
    //
    // **This is the arm a normalising constructor got wrong**, and it got it
    // wrong in the direction that reads as correct: it turned the pair into a
    // phase of -2^63 and the identity went away with no diagnostic.
    type MinOverMin = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, { i64::MIN }>;
    assert_eq!(
        cancelling_slot::<MinOverMin>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-1))
    );
    assert!(
        has_additive_identity::<MinOverMin>().get(),
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
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(i64::MIN))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(62)),
        Maybe::Is(Slot::at(-2))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(63)),
        Maybe::Is(Slot::at(-1))
    );
    assert!(
        has_additive_identity::<Growing>().get(),
        "a phase of 2^63 quanta was never divided down into an admitted slot"
    );

    // The control: the same phase with only one magnitude has nowhere to divide
    // down to, so the identity is off the grid.
    type OneMagnitude = Grid<BinaryRationals, Indexed<0, 1>, Signed<8>, { i64::MIN }, -1>;
    assert!(!has_additive_identity::<OneMagnitude>().get());
}

// --- the phase through the one shipped point that carries one ----------------

#[test]
fn a_zero_phase_puts_the_additive_identity_on_the_grid() {
    assert!(has_additive_identity::<Integer<8>>().get());
    assert!(has_additive_identity::<UFixed<13, -4>>().get());
    assert!(has_additive_identity::<Floating<11, -14, 30>>().get());
}

#[test]
fn a_fractional_phase_takes_the_additive_identity_off_the_grid() {
    // `Biased` fixes the denominator at two and the quantum at the constant
    // family, so an odd numerator is the half-step bias and no magnitude can
    // cancel it. The canon carries the phase coordinate precisely because this is
    // not a corner case: no exact sum lands on that grid, and when its quantum is
    // one the grid holds neither zero nor one, so it is not a monoid carrier.
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
    // The half an odd-numerator arm set cannot see. Every fractional arm above
    // writes an odd numerator against a denominator the type fixes at two, so
    // every phase tried there is fractional. An even numerator is a whole number
    // of quanta: the grid shifts onto itself and zero sits at the negated multiple
    // rather than at slot zero.
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

    // The control that keeps the four arms above from passing under a predicate
    // that ignores the numerator entirely.
    assert!(!has_additive_identity::<Biased<4, 0, 1>>().get());
}

#[test]
fn the_superseded_predicate_differs_exactly_at_a_nonzero_whole_multiple() {
    // One sentence about the whole disagreement rather than a list of the arms
    // that happen to show it. The old question was whether the numerator is zero;
    // the right one is whether the ratio is an integer the range can cancel, so
    // the two part company on a nonzero whole multiple and agree everywhere else.
    //
    // The quantum here is constant, which is what makes the two comparable at
    // all: over a moving one the right answer also depends on the magnitude and
    // the superseded predicate has no coordinate for it.
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
fn the_one_magnitude_predicate_differs_exactly_where_the_quantum_moves() {
    // The second superseded reading, and it is the one this crate shipped for
    // longer: whether the phase is a whole multiple at magnitude zero and the
    // slot it lands on is in range. It is exactly right in the constant family
    // and wrong in the other two, which is the whole content of the correction.
    macro_rules! agrees_in_the_constant_family {
        ($($f:ty),+ $(,)?) => {
            $(
                assert_eq!(
                    has_additive_identity::<$f>().get(),
                    the_one_magnitude_predicate::<$f>().get(),
                    "the reduction and the search disagree inside the constant family, \
                     where they are the same question"
                );
            )+
        };
    }
    agrees_in_the_constant_family!(
        Integer<8>,
        UFixed<13, -4>,
        Biased<7, -2, 1>,
        Biased<7, -2, 2>,
        Grid<BinaryRationals, Constant<0>, Signed<8>, 4, 1>,
        Grid<BinaryRationals, Constant<0>, Signed<2>, 4, 1>,
        Grid<DecimalRationals, Constant<0>, Signed<8>, 1, 5>,
    );

    // And it is wrong wherever the quantum moves. A growing law reaches a slot in
    // range at a higher magnitude; a shrinking one turns a fractional phase whole
    // there. Neither is visible from magnitude zero.
    type FoundHigherUp = Grid<BinaryRationals, Indexed<0, 2>, Signed<2>, 4, 1>;
    assert!(has_additive_identity::<FoundHigherUp>().get());
    assert!(!the_one_magnitude_predicate::<FoundHigherUp>().get());

    type WholeHigherUp = Grid<BinaryRationals, Shrinking<2>, Signed<8>, 1, 2>;
    assert!(has_additive_identity::<WholeHigherUp>().get());
    assert!(!the_one_magnitude_predicate::<WholeHigherUp>().get());

    // The control: the reduction is not simply weaker everywhere. Over a moving
    // quantum whose phase no magnitude cancels, the two agree, so the arms above
    // are about where the search finds something rather than about the mutant
    // answering no to everything.
    type NeverWhole = Grid<BinaryRationals, Shrinking<8>, Signed<16>, 1, 3>;
    assert!(!has_additive_identity::<NeverWhole>().get());
    assert!(!the_one_magnitude_predicate::<NeverWhole>().get());
}

#[test]
fn the_two_questions_about_a_phase_are_two_questions() {
    // `is_zero` was asked where `is_whole_multiple` was meant, and the coordinate
    // being a ratio is what lets the two come apart. Written down because the
    // names read alike and only one of them decides the additive identity in the
    // constant family.
    let whole = Phase::halves(2);
    assert!(!whole.is_zero().get());
    assert!(whole.is_whole_multiple().get());

    let fractional = Phase::halves(1);
    assert!(!fractional.is_zero().get());
    assert!(!fractional.is_whole_multiple().get());

    assert!(Phase::ZERO.is_zero().get());
    assert!(Phase::ZERO.is_whole_multiple().get());

    // The ratio is what is asked about rather than the numerator it was handed,
    // and the sign of the denominator does not enter it. `of(6, -3)` is `-2` and
    // whole; `of(1, -3)` is a third and is not.
    assert!(Phase::of(6, -3).is_whole_multiple().get());
    assert!(!Phase::of(1, -3).is_whole_multiple().get());
    assert!(Phase::of(0, 7).is_whole_multiple().get());

    // A denominator of zero names no position, so neither question about the
    // ratio has an answer and the divisibility one says no rather than dividing.
    assert!(!Phase::of(1, 0).denotes().get());
    assert!(!Phase::of(1, 0).is_whole_multiple().get());
    assert!(Phase::of(1, 2).denotes().get());

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
