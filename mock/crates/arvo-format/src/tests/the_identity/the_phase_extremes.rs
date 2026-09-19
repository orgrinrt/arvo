//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The extremes of the phase coordinates.
//!
//! The phase numerator and denominator are signed 64-bit and nothing bounds them,
//! so the ends of that range are reachable through the open trait. They are not
//! reachable through the shipped points: three of the four fix the phase at zero
//! over one, and `Biased` fixes the denominator at two.
//!
//! Solved in the width the phase is declared in, the cancellation has two pairs it
//! cannot answer. The least numerator over minus one overflows the remainder, and
//! that numerator over one produces a quotient whose negation overflows. Both
//! diverge rather than answering, and diverging on the value path is what
//! `ruling::never_a_runtime_check_and_one_lowered_path` forbids. Neither is
//! reachable by a guard placed before the arithmetic, because both are the
//! arithmetic.
//!
//! Carrying the division one width up is what makes them defined, because the only
//! overflowing pair in a signed division is the least value over minus one and the
//! phase's least value is nowhere near the wider one's. That wider width is the
//! slot index's own, so both quotients are slots, and a 64-bit range reaches one
//! of them.

use notko::Maybe;

use crate::ambient::{BinaryRationals, UnsignedBinaryRationals};
use crate::format::{Phase, cancelling_slot, has_additive_identity};
use crate::quantum::{Constant, Indexed, Magnitude};
use crate::slots::{Signed, Slot, Unsigned};
use crate::tests::grid::Grid;

#[test]
fn a_phase_keeps_the_value_it_was_declared_with() {
    // The coordinate holds the pair rather than a normalisation of it, and these
    // are the two pairs no normalisation inside the declared width can keep:
    // moving the sign to the numerator would negate the denominator, which the
    // least value has no room for, so the only normalised form available is a
    // denominator of one, a different position from the one declared.
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
    // cancelling slot is -2^63. `Signed<8>` does not reach it, so there is no
    // identity and the answer is a decided one.
    type MinOverMinusOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, -1>;
    assert_eq!(
        cancelling_slot::<MinOverMinusOne>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-(1i128 << 63)))
    );
    assert!(!has_additive_identity::<MinOverMinusOne>().get());

    // The widest signed range does reach it, at its own lowest slot, so the same
    // phase has an identity there. That is a range no slot index narrower than
    // the one this crate carries could have stated.
    type MinOverMinusOneAtSixtyFour =
        Grid<BinaryRationals, Constant<0>, Signed<64>, { i64::MIN }, -1>;
    assert!(has_additive_identity::<MinOverMinusOneAtSixtyFour>().get());

    // The least numerator over one. The cancelling slot is +2^63, one past what a
    // 64-bit integer carries and a slot the index holds, so it is that slot
    // rather than a wrapped value landing inside somebody's range.
    type MinOverOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MIN }, 1>;
    assert_eq!(
        cancelling_slot::<MinOverOne>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(1i128 << 63))
    );
    assert!(!has_additive_identity::<MinOverOne>().get());

    // Neither signed range reaches +2^63, the widest's top being one below it,
    // and the unsigned 64-bit range does.
    type MinOverOneSigned = Grid<BinaryRationals, Constant<0>, Signed<64>, { i64::MIN }, 1>;
    assert!(!has_additive_identity::<MinOverOneSigned>().get());
    type MinOverOneUnsigned =
        Grid<UnsignedBinaryRationals, Constant<0>, Unsigned<64>, { i64::MIN }, 1>;
    assert!(has_additive_identity::<MinOverOneUnsigned>().get());

    // The greatest numerator over minus one, which is the same shape without the
    // asymmetry that makes the pair above overflow.
    type MaxOverMinusOne = Grid<BinaryRationals, Constant<0>, Signed<8>, { i64::MAX }, -1>;
    assert_eq!(
        cancelling_slot::<MaxOverMinusOne>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(i64::MAX as i128))
    );
    assert!(!has_additive_identity::<MaxOverMinusOne>().get());

    // And the control, which is what keeps the three above from being a function
    // that answers `Isnt` or an unreachable slot for anything extreme: the two
    // ends divide to a phase of one, whose cancelling slot is -1 and is in range,
    // so this one does have an identity.
    //
    // This is the arm a normalising constructor got wrong, and it got it wrong in
    // the direction that reads as correct: it turned the pair into a phase of
    // -2^63 and the identity went away with no diagnostic.
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
        Maybe::Is(Slot::at(-(1i128 << 63)))
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
