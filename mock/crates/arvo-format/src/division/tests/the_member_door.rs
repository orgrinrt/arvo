//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The member door: the quotient by a member of a zero-free range against the
//! oracle, each of the door's conditions on both sides of its boundary, and the
//! planted verdicts the table has to report.
//!
//! The divisors are reached through the doors a consumer has, a member adapted
//! at its own slot on the grid and then `Divisor::member`, never `assumed`. The
//! refusals themselves are build failures and live in `tests/ui` as pairs.

use core::marker::PhantomData;

use super::members;
use super::the_oracle::{Tally, pair};
use crate::adapt::{DeclaredSignature, Signature};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Dither, Exact};
use crate::division::{
    Divisor,
    is_divisible,
    member_conditions,
    no_member_names_the_zero,
    range_inside,
    same_phase,
};
use crate::format::{Format, radix};
use crate::member::Member;
use crate::overflow::SHIPPED_POLICIES;
use crate::quantum::{Constant, is_constant_family};
use crate::rounding::ALL_MODES;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::dispatch::{self, PerSignature};
use crate::tests::grid::Grid;
use crate::width::{Bool, Width};

/// The slots one to 255, a consumer's range that leaves the zero out.
struct OneTo255;

impl Slots for OneTo255 {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(8);
}

/// The slots one to 65535, a range past a byte's.
struct OneTo65535;

impl Slots for OneTo65535 {
    const MAX: Slot = Slot::at(65535);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(16);
}

/// The bytes, integers over the unsigned 8-bit range.
type Byte = Grid<BinaryRationals, Constant<0>, Unsigned<8>, 0, 1>;
/// Integers over `OneTo255`: no member names the zero.
type Positive = Grid<BinaryRationals, Constant<0>, OneTo255, 0, 1>;
/// `OneTo255` at radix ten.
type Decimal = Grid<DecimalRationals, Constant<0>, OneTo255, 0, 1>;
/// `OneTo255` in halves.
type Halves = Grid<BinaryRationals, Constant<-1>, OneTo255, 0, 1>;
/// The byte range offset by a half, where no slot names the zero.
type Biased = Grid<BinaryRationals, Constant<0>, Unsigned<8>, 1, 2>;
/// `OneTo255` offset by two quarters, the same phase as `Biased` unreduced.
type Quarters = Grid<BinaryRationals, Constant<0>, OneTo255, 2, 4>;
/// Integers over `OneTo65535`.
type Wider = Grid<BinaryRationals, Constant<0>, OneTo65535, 0, 1>;
/// A whole phase of one over the bytes, whose zero is the slot `-1`, outside.
type FromOne = Grid<BinaryRationals, Constant<0>, Unsigned<8>, 1, 1>;
/// The same phase over the signed range, which holds the slot `-1`.
type SignedFromOne = Grid<BinaryRationals, Constant<0>, Signed<8>, 1, 1>;

/// The quotient of every member of `S`'s format by every member of `G`, each
/// reached as a consumer reaches it, against the oracle.
struct ByMembers<G>(PhantomData<G>);

impl<G: Format> PerSignature for ByMembers<G> {
    type Out = Tally;

    #[inline(never)]
    fn run<S: DeclaredSignature>(&self) -> Tally {
        let mut tally = Tally::default();
        for b in members::<G>() {
            let member = Member::<G>::adapted::<Signature<G, S::Adaptation>>(
                Exact::on_grid(b),
                Dither::UNUSED,
            );
            let divisor = Divisor::<S::Format>::member(member);
            assert_eq!(divisor.slot(), b, "the member door moved the divisor");
            for a in members::<S::Format>() {
                tally.add(pair::<S>(a, divisor, false));
            }
        }
        tally
    }
}

/// Every mode and shipped policy at `F`, divided by every member of `G`.
fn sweep<F: Format, G: Format>() -> (Tally, u64) {
    assert!(
        is_divisible::<F>().get(),
        "the sweep runs at admitted formats only"
    );
    let mut tally = Tally::default();
    let mut cells = 0;
    for mode in ALL_MODES {
        for policy in SHIPPED_POLICIES {
            tally.add(dispatch::at::<F, ByMembers<G>>(
                mode,
                policy,
                &ByMembers(PhantomData),
            ));
            cells += 1;
        }
    }
    (tally, cells)
}

#[test]
fn a_zero_free_range_divides_by_its_own_members_as_the_oracle_does() {
    let (tally, cells) = sweep::<Positive, Positive>();
    assert_eq!(cells, 6 * 3);
    // 255 dividends by 255 divisors in each cell, the stochastic mode's at four
    // dithers.
    assert_eq!(tally.checked, 255 * 255 * (5 * 3 + 4 * 3));
    assert_eq!(tally.wrong, 0);
}

#[test]
fn a_byte_divides_by_the_members_of_a_zero_free_range_as_the_oracle_does() {
    let (tally, _) = sweep::<Byte, Positive>();
    assert_eq!(tally.checked, 256 * 255 * (5 * 3 + 4 * 3));
    assert_eq!(tally.wrong, 0);
}

/// One way of answering the door's five conditions.
trait Verdict {
    fn conditions<F: Format, G: Format>() -> [Bool; 5];
}

/// The verdict the door ships.
struct Shipped;

impl Verdict for Shipped {
    fn conditions<F: Format, G: Format>() -> [Bool; 5] {
        member_conditions::<F, G>()
    }
}

/// A zero check that reads `F`'s range and phase rather than `G`'s.
struct ZeroOfTheDivisorsFormat;

impl Verdict for ZeroOfTheDivisorsFormat {
    fn conditions<F: Format, G: Format>() -> [Bool; 5] {
        let [radix, exponent, phase, inside, _] = member_conditions::<F, G>();
        [radix, exponent, phase, inside, no_member_names_the_zero::<F>()]
    }
}

/// A grid comparison that asks both quanta to be constant and never compares
/// their exponents.
struct NoExponent;

impl Verdict for NoExponent {
    fn conditions<F: Format, G: Format>() -> [Bool; 5] {
        let constant =
            is_constant_family::<F::Quantum>().get() && is_constant_family::<G::Quantum>().get();
        [
            radix::<F>().equals(radix::<G>()),
            Bool::of(constant),
            same_phase::<F, G>(),
            range_inside::<F, G>(),
            no_member_names_the_zero::<G>(),
        ]
    }
}

/// A verdict that drops the range lying inside.
struct NoInside;

impl Verdict for NoInside {
    fn conditions<F: Format, G: Format>() -> [Bool; 5] {
        let [radix, exponent, phase, _, zero_free] = member_conditions::<F, G>();
        [radix, exponent, phase, Bool::TRUE, zero_free]
    }
}

/// How many of the stated pairs `V` answers otherwise than the table.
///
/// Each row is a pair and the five answers stated by hand from the two
/// declarations: the radix, the constant exponent, the reduced phase, the range
/// inside, and no member of the second naming the zero.
fn disagreements<V: Verdict>() -> usize {
    let mut wrong = 0;
    macro_rules! table {
        ($($f:ty, $g:ty => [$($want:literal),+];)+) => {
            $(
                let got = V::conditions::<$f, $g>().map(Bool::get);
                wrong += usize::from(got != [$($want),+]);
            )+
        };
    }
    table! {
        Byte, Positive => [true, true, true, true, true];
        Positive, Positive => [true, true, true, true, true];
        Byte, Byte => [true, true, true, true, false];
        Byte, Decimal => [false, true, true, true, true];
        Byte, Halves => [true, false, true, true, true];
        Byte, Biased => [true, true, false, true, true];
        Biased, Quarters => [true, true, true, true, true];
        Byte, Wider => [true, true, true, false, true];
        FromOne, FromOne => [true, true, true, true, true];
        FromOne, SignedFromOne => [true, true, true, false, false];
    }
    wrong
}

#[test]
fn each_condition_of_the_door_reads_as_stated_on_both_sides() {
    assert_eq!(disagreements::<Shipped>(), 0);
    // The table's own control: every condition flips in at least one row, so a
    // verdict answering true everywhere is not mistaken for the shipped one.
    let flips = [
        (member_conditions::<Byte, Decimal>(), 0),
        (member_conditions::<Byte, Halves>(), 1),
        (member_conditions::<Byte, Biased>(), 2),
        (member_conditions::<Byte, Wider>(), 3),
        (member_conditions::<Byte, Byte>(), 4),
    ];
    for (row, condition) in flips {
        assert!(!row[condition].get(), "condition {condition} does not flip");
        let others = row
            .iter()
            .enumerate()
            .filter(|&(i, c)| i != condition && !c.get())
            .count();
        assert_eq!(others, 0, "a row flips more than condition {condition}");
    }
}

#[test]
fn the_table_reports_each_planted_verdict() {
    assert!(
        disagreements::<ZeroOfTheDivisorsFormat>() > 0,
        "the table missed a zero check that reads the divisor's own range"
    );
    assert!(
        disagreements::<NoExponent>() > 0,
        "the table missed a grid comparison without the exponent"
    );
    assert!(
        disagreements::<NoInside>() > 0,
        "the table missed a verdict without the range lying inside"
    );
}
