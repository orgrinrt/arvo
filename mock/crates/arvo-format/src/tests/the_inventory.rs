//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The open inventory, and what a member supplies to join it.
//!
//! `proposal::the_concept_is_closed_and_the_inventory_is_open` says a new instance
//! earns admission by supplying the concept's obligations rather than by amending
//! anything. So the members here are declared in this file, outside the shipped
//! points, and every one of them is reached through the crate's own free functions
//! rather than through its fields.
//!
//! The wholly foreign format is the shape this round measured against: written into
//! a crate that is not exempt from the bare-primitive lints, it was refused at ten
//! positions before every coordinate carried a type this crate owns.

use crate::ambient::{Ambient, DecimalRationals, Radix};
use crate::format::{contains, has_additive_identity, radix, step_exponent, Format, Phase};
use crate::points::Integer;
use crate::quantum::{
    is_constant_family, Constant, Exponent, Indexed, Magnitude, MagnitudeCount, Quantum,
};
use crate::slots::{slot_count, slot_in_range, Signed, Slot, SlotCount, Slots, Unsigned};
use crate::width::{Bool, Width};

// --- a format the crate does not know about ----------------------------------

/// A format declared outside the shipped points, which is what an open inventory
/// means for the format concept.
struct Ternary;

impl Format for Ternary {
    type Ambient = DecimalRationals;
    type Quantum = Constant<-1>;
    type Slots = Signed<3>;
    const PHASE: Phase = Phase::ZERO;
}

#[test]
fn the_format_inventory_admits_a_member_this_crate_does_not_know_about() {
    assert_eq!(radix::<Ternary>(), Radix::DECIMAL);
    assert_eq!(
        step_exponent::<Ternary>(Magnitude::SMALLEST),
        Exponent::of(-1)
    );
    assert!(has_additive_identity::<Ternary>().get());
    assert!(contains::<Ternary>(Slot::ZERO, Magnitude::SMALLEST).get());
    assert!(!contains::<Ternary>(Slot::at(4), Magnitude::SMALLEST).get());
}

// --- the whole contract, supplied from outside the shipped inventory ---------
//
// `Ternary` above reuses every part under `Format`, so it never exercised what an
// implementor supplying the concept's obligations has to write. The four members
// below supply all of them, and they are what the round measured against: this
// exact shape, written into a crate that is not exempt from the bare-primitive
// lints, was refused at ten positions before every coordinate carried a type this
// crate owns.

/// A domain nothing here ships: the rationals at radix three.
struct TernaryRationals;

impl Ambient for TernaryRationals {
    const RADIX: Radix = Radix::of(3);
    const SIGNED: Bool = Bool::TRUE;
}

/// A step law nothing here ships: two exponents per magnitude.
struct DoubleStepped;

impl Quantum for DoubleStepped {
    const BASE: Exponent = Exponent::of(-2);
    const SLOPE: Exponent = Exponent::of(2);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(4);
}

/// A slot range nothing here ships: five bits, offset so it is neither of the two
/// shipped shapes.
struct OffsetFive;

impl Slots for OffsetFive {
    const MIN: Slot = Slot::at(-8);
    const MAX: Slot = Slot::at(23);
    const WIDTH: Width = Width::bits(5);
}

/// A format built entirely out of the three above, plus a phase of its own.
struct WhollyForeign;

impl Format for WhollyForeign {
    type Ambient = TernaryRationals;
    type Quantum = DoubleStepped;
    type Slots = OffsetFive;
    const PHASE: Phase = Phase::of(1, 3);
}

#[test]
fn the_whole_contract_is_supplied_from_outside_and_every_coordinate_reads_back() {
    // Each of the ten coordinates the contract asks for, reached through the
    // crate's own free functions rather than through the fields, so a coordinate
    // the functions ignored would not pass here.
    assert_eq!(radix::<WhollyForeign>(), Radix::of(3));
    assert!(<TernaryRationals as Ambient>::SIGNED.get());

    assert_eq!(
        step_exponent::<WhollyForeign>(Magnitude::SMALLEST),
        Exponent::of(-2)
    );
    assert_eq!(
        step_exponent::<WhollyForeign>(Magnitude::at(3)),
        Exponent::of(4)
    );
    assert_eq!(
        <DoubleStepped as Quantum>::MAGNITUDES,
        MagnitudeCount::of(4)
    );
    assert!(!is_constant_family::<DoubleStepped>().get());

    assert_eq!(slot_count::<OffsetFive>(), SlotCount::of(32));
    assert!(slot_in_range::<OffsetFive>(Slot::at(-8)).get());
    assert!(slot_in_range::<OffsetFive>(Slot::at(23)).get());
    assert!(!slot_in_range::<OffsetFive>(Slot::at(24)).get());
    assert_eq!(
        crate::slots::declared_slot_width::<OffsetFive>(),
        Width::bits(5)
    );

    // The phase is nonzero, so the grid carries no additive identity, and that is
    // the coordinate doing work rather than sitting in the declaration.
    assert!(!has_additive_identity::<WhollyForeign>().get());
    assert_eq!(<WhollyForeign as Format>::PHASE.numerator(), 1);
    assert_eq!(<WhollyForeign as Format>::PHASE.denominator(), 3);

    // And it is a member of the set the predicate decides, at a magnitude the
    // shipped points do not reach.
    assert!(contains::<WhollyForeign>(Slot::at(20), Magnitude::at(3)).get());
    assert!(!contains::<WhollyForeign>(Slot::at(20), Magnitude::at(4)).get());
}

#[test]
fn the_control_the_foreign_contract_differs_from_every_shipped_one() {
    // If it agreed with a shipped point on every coordinate the test above would
    // be re-testing `Integer` under another name.
    assert_ne!(radix::<WhollyForeign>(), radix::<Integer<8>>());
    assert_ne!(
        <DoubleStepped as Quantum>::SLOPE,
        <Constant<0> as Quantum>::SLOPE
    );
    assert_ne!(
        <DoubleStepped as Quantum>::SLOPE,
        <Indexed<-14, 30> as Quantum>::SLOPE
    );
    assert_ne!(<OffsetFive as Slots>::MIN, <Signed<5> as Slots>::MIN);
    assert_ne!(
        <WhollyForeign as Format>::PHASE,
        <Integer<8> as Format>::PHASE
    );
}

// --- the width bound is the impl set, and these are the properties it is about -

#[test]
fn the_declared_width_is_read_rather_than_recovered() {
    // The coordinate the declaration stated, not a number counted back out of the
    // slot bounds. This is what removed the class where a 63-bit declaration
    // derived a placement of zero bits: no count is formed, so nothing can wrap.
    assert_eq!(<Unsigned<13> as Slots>::WIDTH, Width::bits(13));
    assert_eq!(<Signed<13> as Slots>::WIDTH, Width::bits(13));
    assert_eq!(<Unsigned<1> as Slots>::WIDTH, Width::bits(1));
    assert_eq!(<Unsigned<62> as Slots>::WIDTH, Width::bits(62));
}

#[test]
fn every_admitted_width_has_a_coherent_range() {
    // The property the bound is about, asserted over the whole admitted set
    // rather than about the constant that used to name it. A width whose impl
    // inverted its own range would fail here, and the previous cut of this file
    // asserted a constant against its own literal instead, which could not.
    macro_rules! coherent {
        ($($w:literal),+ $(,)?) => {
            $(
                {
                    assert!(
                        <Unsigned<$w> as Slots>::MIN
                            .is_at_most(<Unsigned<$w> as Slots>::MAX)
                            .get(),
                        "unsigned {} inverted its range", $w
                    );
                    assert!(
                        <Signed<$w> as Slots>::MIN
                            .is_at_most(<Signed<$w> as Slots>::MAX)
                            .get(),
                        "signed {} inverted its range", $w
                    );
                    assert!(
                        slot_count::<Unsigned<$w>>().count() > 0,
                        "unsigned {} counted nothing", $w
                    );
                    assert!(
                        slot_count::<Signed<$w>>().count() > 0,
                        "signed {} counted nothing", $w
                    );
                    assert!(
                        slot_in_range::<Unsigned<$w>>(Slot::ZERO).get(),
                        "unsigned {} excludes zero", $w
                    );
                    assert!(
                        slot_in_range::<Signed<$w>>(Slot::ZERO).get(),
                        "signed {} excludes zero", $w
                    );
                    assert_eq!(slot_count::<Unsigned<$w>>(), SlotCount::of(1i64 << $w));
                    assert_eq!(slot_count::<Signed<$w>>(), SlotCount::of(1i64 << $w));
                }
            )+
        };
    }
    coherent!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
    );
}

#[test]
fn the_admitted_set_is_the_contiguous_run_the_macro_names() {
    // The list in `slots` is the bound, so what it contains is a fact worth
    // pinning: a contiguous run from one to the widest admitted width, with no
    // gap and nothing past the end.
    let widths = crate::slots::ADMITTED_WIDTHS;
    assert_eq!(widths.first(), Some(&Width::bits(1)));
    assert_eq!(widths.last(), Some(&Width::bits(62)));
    assert_eq!(widths.len(), 62);
    for (i, w) in widths.iter().enumerate() {
        assert_eq!(
            w.count() as usize,
            i + 1,
            "the admitted set has a gap at {}",
            w.count()
        );
    }
}

#[test]
fn the_widest_admitted_width_is_where_the_count_stops_fitting() {
    // Why 62 and not 63, derived rather than restated. The count is two to the
    // power of the width; at the widest admitted width it fits a signed 64-bit
    // integer and one above it does not. If somebody widens the impl set without
    // this being true, this fails.
    let widest = crate::slots::ADMITTED_WIDTHS.last().unwrap().count();
    let at_bound = 1u128 << widest;
    let one_over = 1u128 << (widest + 1);
    assert!(
        at_bound <= i64::MAX as u128,
        "the widest admitted width does not fit, so the impl set is too wide"
    );
    assert!(
        one_over > i64::MAX as u128,
        "one width past the widest admitted still fits, so the impl set is too narrow"
    );
    assert!(slot_count::<Unsigned<62>>().count() > 0);
    assert_eq!(slot_count::<Unsigned<62>>(), SlotCount::of(at_bound as i64));
}

// --- what an outside implementor owes, and the construction that does not ----

/// A slot range from outside this crate that does not meet the contract.
///
/// The reviewer's construction, values verbatim, kept permanently rather than in
/// a scratch file. It **compiles**, which is the point: the trait is open and
/// nothing stops it being written. What it does not do is pass the law below, and
/// using it does not build, which the `trybuild` case records.
struct RogueRange;

impl Slots for RogueRange {
    const MIN: Slot = Slot::at(4611686018427387904);
    const MAX: Slot = Slot::at(-4611686018427387905);
    const WIDTH: Width = Width::bits(63);
}

/// A width of zero, which admits nothing.
struct EmptyRange;

impl Slots for EmptyRange {
    const MIN: Slot = Slot::ZERO;
    const MAX: Slot = Slot::at(-1);
    const WIDTH: Width = Width::NONE;
}

#[test]
fn the_law_rejects_a_range_that_does_not_meet_the_contract() {
    // The law returns a verdict, so the wrong construction can be reported on
    // without forcing the const that refuses it. Asserting that it rejects is the
    // shape a construction that compiles and is wrong wants.
    assert!(
        !crate::slots::is_admissible::<RogueRange>().get(),
        "an inverted range was admitted, which is the finding returning"
    );
    assert!(
        !crate::slots::is_admissible::<EmptyRange>().get(),
        "a zero-width range was admitted"
    );
}

#[test]
fn the_law_admits_every_range_this_crate_ships() {
    // The control. A law that rejected everything would pass the test above and
    // establish nothing, so it has to accept the shipped set.
    macro_rules! admits {
        ($($w:literal),+ $(,)?) => {
            $(
                assert!(
                    crate::slots::is_admissible::<Unsigned<$w>>().get(),
                    "unsigned {} was refused by the law", $w
                );
                assert!(
                    crate::slots::is_admissible::<Signed<$w>>().get(),
                    "signed {} was refused by the law", $w
                );
            )+
        };
    }
    admits!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
    );

    // And the foreign range above, which is neither shipped shape and is
    // admissible, so the law is about the obligations rather than about the two
    // constructions this crate happens to write.
    assert!(crate::slots::is_admissible::<OffsetFive>().get());
}

#[test]
fn the_law_separates_the_two_constructions_rather_than_answering_one_way() {
    // Both directions in one place, so a law stuck at `true` or at `false` fails
    // here rather than passing one of the two tests above.
    let shipped = crate::slots::is_admissible::<Unsigned<13>>().get();
    let rogue = crate::slots::is_admissible::<RogueRange>().get();
    assert_ne!(
        shipped, rogue,
        "the law gives the same verdict to a shipped range and an inverted one"
    );
}

/// A range that meets the first three obligations and still cannot be counted.
///
/// `MIN <= MAX` holds and the width is in range, so an obligation checking only
/// those admits it. Its span is 2^63, which is what `slot_count` cannot carry.
/// Measured before the obligation was strengthened: under `overflow-checks` it
/// panicked at runtime, and without it `slot_count` returned
/// `-9223372036854775808`.
struct SpanTooWide;

impl Slots for SpanTooWide {
    const MIN: Slot = Slot::at(-4611686018427387904);
    const MAX: Slot = Slot::at(4611686018427387903);
    const WIDTH: Width = Width::bits(62);
}

/// A range whose declared width cannot address it.
struct WidthTooNarrow;

impl Slots for WidthTooNarrow {
    const MIN: Slot = Slot::ZERO;
    const MAX: Slot = Slot::at(1000);
    const WIDTH: Width = Width::bits(4);
}

#[test]
fn the_law_rejects_a_range_that_passes_the_easy_obligations() {
    // The case a weaker obligation admitted. Kept permanently because it is the
    // one that looks admissible: nothing about it is inverted and its width is in
    // range, and it still breaks the only thing the range is for.
    assert!(
        !crate::slots::is_admissible::<SpanTooWide>().get(),
        "a span of 2^63 was admitted, so counting it overflows"
    );
    assert!(
        !crate::slots::is_admissible::<WidthTooNarrow>().get(),
        "a width that cannot address its own range was admitted"
    );

    // And the reasons are distinct from the inverted case, so the law is not
    // rejecting everything that is not a shipped shape.
    assert!(<SpanTooWide as Slots>::MIN
        .is_at_most(<SpanTooWide as Slots>::MAX)
        .get());
    assert!(<WidthTooNarrow as Slots>::MIN
        .is_at_most(<WidthTooNarrow as Slots>::MAX)
        .get());
}
