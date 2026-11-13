//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The divisor's two doors, the non-member that names the zero, and every
//! condition of the quotient's obligation on both sides of its boundary.
//!
//! The refusals themselves are build failures and live in `tests/ui` as pairs;
//! here each door is used where it is admitted, the zero-member reading is
//! checked against the oracle's own count of the zero, and each condition is
//! read through `is_divisible` at a format on each side of where it flips.

use super::{exact_quotient, members, value};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::Exact;
use crate::division::{Divisor, is_divisible, quotient_position};
use crate::format::{Format, has_additive_identity};
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

/// Integers over `OneTo255`.
type Positive = Grid<BinaryRationals, Constant<0>, OneTo255, 0, 1>;

/// A half-offset grid, whose members are all odd halves.
type Halves = Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 2>;

/// A whole phase of one over the unsigned range, whose zero is the slot `-1`.
type FromOne = Grid<BinaryRationals, Constant<0>, Unsigned<8>, 1, 1>;

/// Whether the oracle finds a member whose value is zero.
fn oracle_has_zero<F: Format>() -> bool {
    members::<F>().any(|s| value::<F>(s).num == 0)
}

#[test]
fn a_format_has_a_zero_member_exactly_where_the_oracle_finds_one() {
    // `Divisor::of` reads `has_additive_identity`; the oracle counts values.
    macro_rules! check {
        ($($f:ty),+ $(,)?) => {
            $(
                assert_eq!(
                    has_additive_identity::<$f>().get(),
                    oracle_has_zero::<$f>(),
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
        Grid<DecimalRationals, Constant<-1>, Unsigned<8>, 2, 3>,
    );
    assert!(!oracle_has_zero::<Positive>() && !oracle_has_zero::<Halves>());
    assert!(oracle_has_zero::<Integer<8>>());
}

#[test]
fn both_doors_give_the_quotient_the_oracle_names() {
    const SEVEN: Divisor<Positive> = Divisor::of(Slot::at(7));
    const HALF: Divisor<Halves> = Divisor::of(Slot::at(-1));
    const MINUS_THREE: Divisor<Integer<8>> = Divisor::constant::<-3>();
    assert_eq!(SEVEN.slot(), Slot::at(7));
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
fn the_non_member_that_names_the_zero_saturates_toward_the_positive_end() {
    // `FromOne` has no zero member, so `of` is admitted, and the slot `-1` is
    // not a member but is the one slot where `phase + b` is zero.
    assert!(!has_additive_identity::<FromOne>().get());
    assert_eq!(value::<FromOne>(Slot::at(-1)).num, 0);
    let top = Exact::on_grid(Slot::at(i128::MAX));
    for a in [0, 1, 255] {
        assert_eq!(
            quotient_position::<FromOne>(Slot::at(a), Divisor::of(Slot::at(-1))),
            top,
            "{a}"
        );
    }
    // Its neighbours are ordinary quotients, so only the zero saturates.
    assert_eq!(
        quotient_position::<FromOne>(Slot::at(3), Divisor::of(Slot::at(-2))),
        exact_quotient::<FromOne>(Slot::at(3), Slot::at(-2)).position()
    );
}

#[test]
fn every_condition_flips_at_its_boundary() {
    // A phase that names no member.
    assert!(!is_divisible::<Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 0>>().get());
    assert!(is_divisible::<Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 1>>().get());
    // A magnitude-indexed quantum, beside the constant one.
    assert!(!is_divisible::<Floating<4, -3, 4>>().get());
    assert!(is_divisible::<UFixed<4, -3>>().get());
    // The denominator `|b| 2^e` at the range's far end: `(2^64 - 1) 2^64` fits
    // and `2^63 2^65` does not, where every corner is a small position.
    assert!(is_divisible::<UFixed<64, 64>>().get());
    assert!(!is_divisible::<UFixed<64, 65>>().get());
    assert!(!is_divisible::<Grid<BinaryRationals, Constant<65>, Signed<64>, 0, 1>>().get());
    // A phase of a third over the unsigned 64-bit range: `3 (1 + 3 b)` near
    // `9 * 2^64` fits.
    assert!(is_divisible::<Grid<BinaryRationals, Constant<0>, Unsigned<64>, 1, 3>>().get());
    // The slot index, at the divisor next to the pole: `MIN / -1` scaled by
    // `2^63` is `2^126`, and by `2^64` it is `2^127`, one past the index.
    assert!(is_divisible::<Grid<BinaryRationals, Constant<-63>, Signed<64>, 0, 1>>().get());
    assert!(!is_divisible::<Grid<BinaryRationals, Constant<-64>, Signed<64>, 0, 1>>().get());
    assert!(is_divisible::<Integer<64>>().get());
    assert!(is_divisible::<UFixed<64, 0>>().get());
}

/// Bound in a `const` item, so a format the quotient does not admit is read as
/// a verdict rather than a build failure.
const _HELD_OUT: Bool = is_divisible::<Grid<BinaryRationals, Constant<-64>, Signed<64>, 0, 1>>();
