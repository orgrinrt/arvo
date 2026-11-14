//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The divisor's two doors, where `of` is admitted, and every condition of the
//! quotient's obligation on both sides of its boundary.
//!
//! The refusals themselves are build failures and live in `tests/ui` as pairs;
//! here each door is used where it is admitted, the reading `of` is admitted by
//! is checked against the oracle's own search for a slot whose value is zero,
//! and each condition is read through `is_divisible` at a format on each side of
//! where it flips.

use notko::Maybe;

use super::{exact_quotient, members, value};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::division::{
    Divisor,
    conditions,
    is_divisible,
    no_slot_names_the_zero,
    quotient_position,
};
use crate::format::Format;
use crate::points::{Floating, Integer, UFixed};
use crate::quantum::Constant;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::grid::Grid;
use crate::width::{Bool, Width};

/// The slots one to 255, a consumer's range that leaves the zero out.
struct OneTo255;

impl Slots for OneTo255 {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(8);
}

/// Integers over `OneTo255`: no zero member, and the non-member slot zero names
/// the zero.
type Positive = Grid<BinaryRationals, Constant<0>, OneTo255, 0, 1>;

/// A half-offset grid, whose members are all odd halves.
type Halves = Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 2>;

/// A whole phase of one over the unsigned range, whose zero is the slot `-1`.
type FromOne = Grid<BinaryRationals, Constant<0>, Unsigned<8>, 1, 1>;

/// Whether the oracle finds a slot whose value is zero, members and the slots
/// for the range's own span beyond each end alike. The zero's slot `-n / d` lies
/// within the phase's whole part of zero, so for every format below the window
/// holds it wherever it exists.
fn oracle_finds_a_zero_slot<F: Format>() -> bool {
    let min = <F::Slots as Slots>::MIN.index();
    let max = <F::Slots as Slots>::MAX.index();
    let span = max - min + 1;
    (min - span ..= max + span).any(|s| value::<F>(Slot::at(s)).num == 0)
}

#[test]
fn of_is_admitted_exactly_where_the_oracle_finds_no_slot_naming_the_zero() {
    macro_rules! check {
        ($($f:ty),+ $(,)?) => {
            $(
                assert_eq!(
                    no_slot_names_the_zero::<$f>().get(),
                    !oracle_finds_a_zero_slot::<$f>(),
                    "{}",
                    stringify!($f)
                );
            )+
        };
    }
    check!(
        Positive,
        Halves,
        FromOne,
        Integer<8>,
        UFixed<8, -3>,
        Grid<BinaryRationals, Constant<0>, Signed<8>, -3, 1>,
        Grid<BinaryRationals, Constant<0>, Unsigned<8>, -3, 1>,
        Grid<BinaryRationals, Constant<0>, Unsigned<8>, 6, 4>,
        Grid<DecimalRationals, Constant<-1>, Unsigned<8>, 2, 3>,
    );
    // A range without a zero member is not enough: its zero slot is a non-member
    // a runtime slot could still name.
    assert!(!members::<Positive>().any(|s| value::<Positive>(s).num == 0));
    assert!(!no_slot_names_the_zero::<Positive>().get());
    assert!(!no_slot_names_the_zero::<FromOne>().get());
    assert!(no_slot_names_the_zero::<Halves>().get());
    // Six quarters reduce to three halves, so the reduced denominator decides.
    assert!(
        no_slot_names_the_zero::<Grid<BinaryRationals, Constant<0>, Unsigned<8>, 6, 4>>().get()
    );
    assert!(
        !no_slot_names_the_zero::<Grid<BinaryRationals, Constant<0>, Unsigned<8>, 4, 4>>().get()
    );
}

#[test]
fn both_doors_give_the_quotient_the_oracle_names() {
    const HALF: Divisor<Halves> = Divisor::of(Slot::at(-1));
    const SEVEN: Divisor<Positive> = Divisor::constant::<7>();
    const MINUS_THREE: Divisor<Integer<8>> = Divisor::constant::<-3>();
    assert_eq!(HALF.slot(), Slot::at(-1));
    assert_eq!(MINUS_THREE.slot(), Slot::at(-3));
    assert_eq!(
        quotient_position::<Positive>(Slot::at(200), SEVEN),
        exact_quotient::<Positive>(Slot::at(200), Slot::at(7)).position()
    );
    // `(-1/2 + 1/2)` is the zero of the dividend's side, `-1/2` the divisor.
    for a in [-128, -1, 0, 5, 127] {
        assert_eq!(
            quotient_position::<Halves>(Slot::at(a), HALF),
            exact_quotient::<Halves>(Slot::at(a), Slot::at(-1)).position(),
            "{a}"
        );
        assert_eq!(
            quotient_position::<Integer<8>>(Slot::at(a), MINUS_THREE),
            exact_quotient::<Integer<8>>(Slot::at(a), Slot::at(-3)).position(),
            "{a}"
        );
    }
}

#[test]
fn a_runtime_divisor_outside_the_range_is_placed_as_any_other_slot() {
    // `of` takes any slot. Over a half-offset grid none names the zero, so a slot
    // past either end is an ordinary divisor the step places exactly.
    for b in [-129, 128, 1000, -1000] {
        let divisor = Divisor::<Halves>::of(Slot::at(b));
        for a in [-128, 0, 127] {
            assert_eq!(
                quotient_position::<Halves>(Slot::at(a), divisor),
                exact_quotient::<Halves>(Slot::at(a), Slot::at(b)).position(),
                "{a} / {b}"
            );
        }
    }
}

#[test]
fn every_condition_flips_at_its_boundary() {
    // A phase that names no member.
    assert!(!is_divisible::<Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 0>>().get());
    assert!(is_divisible::<Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 1>>().get());
    // A magnitude-indexed quantum, beside the constant one.
    assert!(!is_divisible::<Floating<4, -3, 4>>().get());
    assert!(is_divisible::<UFixed<4, -3>>().get());
    // The denominator `|b| 2^e` at the range's far end: `(2^64 - 1) 2^192` is
    // formed in the four limbs and `2^63 2^193` is not, where every corner is a
    // small position.
    assert!(is_divisible::<UFixed<64, 192>>().get());
    assert!(!is_divisible::<UFixed<64, 193>>().get());
    assert!(is_divisible::<Grid<BinaryRationals, Constant<192>, Signed<64>, 0, 1>>().get());
    assert!(!is_divisible::<Grid<BinaryRationals, Constant<193>, Signed<64>, 0, 1>>().get());
    // A phase of a third over the unsigned 64-bit range: `3 (1 + 3 b)` near
    // `9 * 2^64` fits.
    assert!(is_divisible::<Grid<BinaryRationals, Constant<0>, Unsigned<64>, 1, 3>>().get());
    // No condition reads the slot index: at the divisor next to the pole,
    // `MIN / -1` scaled by `2^63` is `2^126`, and by `2^64` it is `2^127`, one
    // past the index, and both are admitted.
    assert!(is_divisible::<Grid<BinaryRationals, Constant<-63>, Signed<64>, 0, 1>>().get());
    assert!(is_divisible::<Grid<BinaryRationals, Constant<-64>, Signed<64>, 0, 1>>().get());
    assert!(is_divisible::<Integer<64>>().get());
    assert!(is_divisible::<UFixed<64, 0>>().get());
}

#[test]
fn the_verdict_is_false_exactly_at_the_condition_that_fails() {
    // Each held-out format fails one condition and holds the ones before it, so
    // the obligation's first refusal is the one the boundary test names.
    fn first_failing(verdicts: [Bool; 3]) -> Maybe<usize> {
        match verdicts.iter().position(|v| !v.get()) {
            Some(i) => Maybe::Is(i),
            None => Maybe::Isnt,
        }
    }
    assert_eq!(
        first_failing(conditions::<
            Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 0>,
        >()),
        Maybe::Is(0)
    );
    assert_eq!(
        first_failing(conditions::<Floating<4, -3, 4>>()),
        Maybe::Is(1)
    );
    assert_eq!(first_failing(conditions::<UFixed<64, 193>>()), Maybe::Is(2));
    assert_eq!(first_failing(conditions::<Integer<64>>()), Maybe::Isnt);
}

/// Bound in a `const` item, so a format the quotient does not admit is read as
/// a verdict rather than a build failure.
const _HELD_OUT: Bool = is_divisible::<UFixed<64, 193>>();
