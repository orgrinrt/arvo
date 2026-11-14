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
//! oracle divides by the value the slot has in the divisor's own format, and the
//! sweep asserts it equal to the value the same slot has in the dividend's, so
//! the grid claim the door rests on is checked rather than assumed. Every
//! admitted pair of the table is swept, a negative exponent, radix ten, radix ten
//! sharing the bytes' grid at exponent zero, and both sides of a signed byte
//! among them. The zero column is checked against a search over every member. The
//! refusals themselves are build failures and live in `tests/ui` as pairs.

use core::marker::PhantomData;

use super::the_oracle::{Tally, pair};
use super::{members, value};
use crate::adapt::{DeclaredSignature, Signature};
use crate::ambient::{BinaryRationals, DecimalRationals};
use crate::apply::{Dither, Exact};
use crate::division::{Divisor, is_divisible, member_conditions, no_member_names_the_zero};
use crate::format::{Format, radix};
use crate::member::Member;
use crate::overflow::SHIPPED_POLICIES;
use crate::quantum::{Constant, Quantum, is_constant_family};
use crate::rounding::ALL_MODES;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::dispatch::{self, PerSignature};
use crate::tests::grid::Grid;
use crate::tests::ratio::Ratio;
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

/// The bytes in steps of a quarter, a negative exponent.
type ByteInQuarters = Grid<BinaryRationals, Constant<-2>, Unsigned<8>, 0, 1>;
/// `OneTo255` in steps of a quarter.
type PositiveInQuarters = Grid<BinaryRationals, Constant<-2>, OneTo255, 0, 1>;
/// The bytes in tenths, at radix ten.
type ByteInTenths = Grid<DecimalRationals, Constant<-1>, Unsigned<8>, 0, 1>;
/// `OneTo255` in tenths.
type PositiveInTenths = Grid<DecimalRationals, Constant<-1>, OneTo255, 0, 1>;

/// The slots one to 127, the positive side of a signed byte.
struct OneTo127;

impl Slots for OneTo127 {
    const MAX: Slot = Slot::at(127);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(8);
}

/// The slots `-128` to `-1`, the negative side of a signed byte.
struct DownFromMinusOne;

impl Slots for DownFromMinusOne {
    const MAX: Slot = Slot::at(-1);
    const MIN: Slot = Slot::at(-128);
    const WIDTH: Width = Width::bits(8);
}

/// Signed integers over the 8-bit range, a plain integer range holding its zero.
type SignedByte = Grid<BinaryRationals, Constant<0>, Signed<8>, 0, 1>;
/// Integers over `OneTo127`.
type UpTo127 = Grid<BinaryRationals, Constant<0>, OneTo127, 0, 1>;
/// Integers over `DownFromMinusOne`, a zero-free range below the zero.
type Negatives = Grid<BinaryRationals, Constant<0>, DownFromMinusOne, 0, 1>;
/// `OneTo255` at exponent one, where slot `b` names `2b`.
type Doubled = Grid<BinaryRationals, Constant<1>, OneTo255, 0, 1>;

/// The quotient of every member of `S`'s format by every member of `G`, each
/// reached as a consumer reaches it, against the oracle fed `G`'s value.
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
            let by = value::<G>(b);
            assert_eq!(
                by,
                value::<S::Format>(b),
                "the slot {b:?} denotes another value in the divisor's format than in the dividend's"
            );
            for a in members::<S::Format>() {
                tally.add(pair::<S>(a, divisor, by, false));
            }
        }
        tally
    }
}

/// Every mode and shipped policy at `F`, divided by every member of `G`, and
/// nothing wrong at any of them.
///
/// Each cell checks every pair once, and the stochastic mode's three cells at
/// four dithers each, so the count is fixed by the two ranges alone.
fn swept<F: Format, G: Format>() {
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
    assert_eq!(cells, 6 * 3);
    let pairs = (members::<F>().count() * members::<G>().count()) as u64;
    assert_eq!(tally.checked, pairs * (5 * 3 + 4 * 3));
    assert_eq!(tally.wrong, 0);
}

#[test]
fn a_zero_free_range_divides_by_its_own_members_as_the_oracle_does() {
    swept::<Positive, Positive>();
}

#[test]
fn a_byte_divides_by_the_members_of_a_zero_free_range_as_the_oracle_does() {
    swept::<Byte, Positive>();
}

#[test]
fn a_phase_divides_by_the_members_of_its_unreduced_spelling_as_the_oracle_does() {
    swept::<Biased, Quarters>();
}

#[test]
fn a_whole_phase_divides_by_its_own_members_as_the_oracle_does() {
    swept::<FromOne, FromOne>();
}

#[test]
fn a_negative_exponent_divides_by_the_members_of_a_zero_free_range_as_the_oracle_does() {
    swept::<ByteInQuarters, PositiveInQuarters>();
}

#[test]
fn radix_ten_divides_by_the_members_of_a_zero_free_range_as_the_oracle_does() {
    swept::<ByteInTenths, PositiveInTenths>();
}

#[test]
fn the_bytes_divide_by_a_range_at_radix_ten_on_their_grid_as_the_oracle_does() {
    swept::<Byte, Decimal>();
}

#[test]
fn a_signed_byte_divides_by_the_members_of_its_positive_side_as_the_oracle_does() {
    swept::<SignedByte, UpTo127>();
}

#[test]
fn a_signed_byte_divides_by_the_members_of_its_negative_side_as_the_oracle_does() {
    swept::<SignedByte, Negatives>();
}

/// One way of answering the door's four conditions.
trait Verdict {
    fn conditions<F: Format, G: Format>() -> [Bool; 4];
}

/// The verdict the door ships.
struct Shipped;

impl Verdict for Shipped {
    fn conditions<F: Format, G: Format>() -> [Bool; 4] {
        member_conditions::<F, G>()
    }
}

/// A zero check that reads `F`'s range and phase rather than `G`'s.
struct ZeroOfTheDivisorsFormat;

impl Verdict for ZeroOfTheDivisorsFormat {
    fn conditions<F: Format, G: Format>() -> [Bool; 4] {
        let [quantum, phase, inside, _] = member_conditions::<F, G>();
        [quantum, phase, inside, no_member_names_the_zero::<F>()]
    }
}

/// Whether both quanta are constant at one exponent, the radix aside.
fn constant_at_one_exponent<F: Format, G: Format>() -> bool {
    is_constant_family::<F::Quantum>().get()
        && is_constant_family::<G::Quantum>().get()
        && <F::Quantum as Quantum>::BASE.power() == <G::Quantum as Quantum>::BASE.power()
}

/// A grid comparison that reads the radix and the exponent apart, and so
/// refuses radixes that share a quantum.
struct RadixAndExponentApart;

impl Verdict for RadixAndExponentApart {
    fn conditions<F: Format, G: Format>() -> [Bool; 4] {
        let [_, phase, inside, zero_free] = member_conditions::<F, G>();
        let grid = radix::<F>().equals(radix::<G>()).get() && constant_at_one_exponent::<F, G>();
        [Bool::of(grid), phase, inside, zero_free]
    }
}

/// A grid comparison that reads the exponent and never the radix.
struct NoRadix;

impl Verdict for NoRadix {
    fn conditions<F: Format, G: Format>() -> [Bool; 4] {
        let [_, phase, inside, zero_free] = member_conditions::<F, G>();
        [Bool::of(constant_at_one_exponent::<F, G>()), phase, inside, zero_free]
    }
}

/// `V = r^max(-e, 0)` for a constant quantum, as a whole number.
fn v_half<F: Format>() -> i128 {
    let r = radix::<F>().base() as i128;
    r.pow(<F::Quantum as Quantum>::BASE.power().min(0).unsigned_abs())
}

/// A quantum comparison that reads the `V` halves alone, blind to `U`.
struct VHalvesAlone;

impl Verdict for VHalvesAlone {
    fn conditions<F: Format, G: Format>() -> [Bool; 4] {
        let [_, phase, inside, zero_free] = member_conditions::<F, G>();
        let constant =
            is_constant_family::<F::Quantum>().get() && is_constant_family::<G::Quantum>().get();
        [Bool::of(constant && v_half::<F>() == v_half::<G>()), phase, inside, zero_free]
    }
}

/// A phase comparison modulo a whole slot, which takes a grid as a set of points.
struct PhaseModuloASlot;

impl Verdict for PhaseModuloASlot {
    fn conditions<F: Format, G: Format>() -> [Bool; 4] {
        let [quantum, _, inside, zero_free] = member_conditions::<F, G>();
        let (f, g) = (Ratio::phase_of::<F>(), Ratio::phase_of::<G>());
        let phase = f.den == g.den && (f.num - g.num) % f.den == 0;
        [quantum, Bool::of(phase), inside, zero_free]
    }
}

/// A verdict that drops the range lying inside.
struct NoInside;

impl Verdict for NoInside {
    fn conditions<F: Format, G: Format>() -> [Bool; 4] {
        let [quantum, phase, _, zero_free] = member_conditions::<F, G>();
        [quantum, phase, Bool::TRUE, zero_free]
    }
}

/// The stated pairs, handed to the macro named.
///
/// Each row is a pair and the four answers stated by hand from the two
/// declarations: one constant quantum, the reduced phase, the range inside, and
/// no member of the second naming the zero. The rows answering true throughout
/// are the pairs the sweeps above divide.
macro_rules! the_table {
    ($then:ident) => {
        $then! {
            Byte, Positive => [true, true, true, true];
            Positive, Positive => [true, true, true, true];
            Byte, Decimal => [true, true, true, true];
            Biased, Quarters => [true, true, true, true];
            FromOne, FromOne => [true, true, true, true];
            ByteInQuarters, PositiveInQuarters => [true, true, true, true];
            ByteInTenths, PositiveInTenths => [true, true, true, true];
            SignedByte, UpTo127 => [true, true, true, true];
            SignedByte, Negatives => [true, true, true, true];
            Byte, Halves => [false, true, true, true];
            Halves, PositiveInTenths => [false, true, true, true];
            Byte, Doubled => [false, true, true, true];
            Byte, Biased => [true, false, true, true];
            Byte, FromOne => [true, false, true, true];
            Byte, Wider => [true, true, false, true];
            Byte, Byte => [true, true, true, false];
            SignedByte, SignedByte => [true, true, true, false];
            FromOne, SignedFromOne => [true, true, false, false];
        }
    };
}

/// How many of the stated pairs `V` answers otherwise than the table.
fn disagreements<V: Verdict>() -> usize {
    let mut wrong = 0;
    macro_rules! compare {
        ($($f:ty, $g:ty => [$($want:literal),+];)+) => {
            $(
                let got = V::conditions::<$f, $g>().map(Bool::get);
                wrong += usize::from(got != [$($want),+]);
            )+
        };
    }
    the_table!(compare);
    wrong
}

#[test]
fn each_condition_of_the_door_reads_as_stated_on_both_sides() {
    assert_eq!(disagreements::<Shipped>(), 0);
    // The table's own control, read from its literals: every condition is
    // stated false in a row where every other is stated true, so a verdict
    // answering true everywhere, or a condition standing in for another, does
    // not agree with it.
    macro_rules! stated {
        ($($f:ty, $g:ty => [$($want:literal),+];)+) => {
            [$([$($want),+]),+]
        };
    }
    let rows: &[[bool; 4]] = &the_table!(stated);
    for condition in 0 .. 4 {
        let alone = rows.iter().any(|row| {
            row.iter()
                .enumerate()
                .all(|(i, &stated)| stated == (i != condition))
        });
        assert!(alone, "no row states condition {condition} false alone");
    }
}

#[test]
fn the_zero_column_agrees_with_a_search_over_every_member() {
    // The hand statement is checked against the search beside it, at every
    // format the table names.
    fn agrees<G: Format>() -> bool {
        no_member_names_the_zero::<G>().get() == members::<G>().all(|b| value::<G>(b).num != 0)
    }
    let every = [
        agrees::<Byte>(),
        agrees::<Positive>(),
        agrees::<Decimal>(),
        agrees::<Halves>(),
        agrees::<Biased>(),
        agrees::<Quarters>(),
        agrees::<Wider>(),
        agrees::<FromOne>(),
        agrees::<SignedFromOne>(),
        agrees::<ByteInQuarters>(),
        agrees::<PositiveInQuarters>(),
        agrees::<ByteInTenths>(),
        agrees::<PositiveInTenths>(),
        agrees::<SignedByte>(),
        agrees::<UpTo127>(),
        agrees::<Negatives>(),
        agrees::<Doubled>(),
    ];
    for (at, agreed) in every.into_iter().enumerate() {
        assert!(
            agreed,
            "the zero check disagrees with the search at format {at}"
        );
    }
}

#[test]
fn the_table_reports_each_planted_verdict() {
    assert!(
        disagreements::<ZeroOfTheDivisorsFormat>() > 0,
        "the table missed a zero check that reads the divisor's own range"
    );
    assert!(
        disagreements::<RadixAndExponentApart>() > 0,
        "the table missed a grid comparison that refuses radixes sharing a quantum"
    );
    assert!(
        disagreements::<NoRadix>() > 0,
        "the table missed a grid comparison without the radix"
    );
    assert!(
        disagreements::<VHalvesAlone>() > 0,
        "the table missed a quantum comparison blind to exponents above zero"
    );
    assert!(
        disagreements::<PhaseModuloASlot>() > 0,
        "the table missed a phase comparison modulo a whole slot"
    );
    assert!(
        disagreements::<NoInside>() > 0,
        "the table missed a verdict without the range lying inside"
    );
}
