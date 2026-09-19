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
//! The wholly foreign format is the shape the coordinate types were measured
//! against: written into a crate that is not exempt from the bare-primitive lints,
//! it was refused at ten positions before every coordinate carried a type this
//! crate owns.
//!
//! Two contracts state obligations and both are checked here, each against
//! constructions that meet the shape and not the conditions. Those constructions
//! are kept rather than deleted, because a construction that compiles and is wrong
//! is exactly what a verdict function exists to be able to report on.

use crate::ambient::{Ambient, DecimalRationals, Radix};
use crate::format::{Format, Phase, contains, has_additive_identity, radix, step_exponent};
use crate::points::Integer;
use crate::quantum::{
    Constant,
    Exponent,
    Indexed,
    Magnitude,
    MagnitudeCount,
    Quantum,
    is_constant_family,
};
use crate::slots::{
    Signed,
    Slot,
    SlotCount,
    Slots,
    Unsigned,
    declared_slot_width,
    slot_count,
    slot_in_range,
};
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
// below supply all of them, and they are what the coordinate types were measured
// against: this exact shape, written into a crate that is not exempt from the
// bare-primitive lints, was refused at ten positions before every coordinate
// carried a type this crate owns.

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
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(4);
    const SLOPE: Exponent = Exponent::of(2);
}

/// A slot range nothing here ships: five bits, offset so it is neither of the two
/// shipped shapes.
struct OffsetFive;

impl Slots for OffsetFive {
    const MAX: Slot = Slot::at(23);
    const MIN: Slot = Slot::at(-8);
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
    assert_eq!(declared_slot_width::<OffsetFive>(), Width::bits(5));

    // The phase is a third of a quantum, so nothing cancels it and the grid
    // carries no additive identity. Fractional rather than merely nonzero is what
    // decides that, and this is the coordinate doing work rather than sitting in
    // the declaration.
    assert!(!<WhollyForeign as Format>::PHASE.is_whole_multiple().get());
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

/// A slot range declaring more bits than its span needs.
///
/// Admissible on every count: not inverted, its width is in range, and a span of
/// four sits well inside two to the thirteenth. Its bounds imply two bits and
/// its declaration says thirteen, which is the only shape that separates a width
/// that is read from one recovered by counting. Every range this crate ships has
/// the two agreeing by construction, so no shipped range can.
struct WiderThanItsSpan;

impl Slots for WiderThanItsSpan {
    const MAX: Slot = Slot::at(3);
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::bits(13);
}

#[test]
fn the_declared_width_is_read_rather_than_recovered() {
    // The coordinate the declaration stated, not a number counted back out of the
    // slot bounds. No count is formed, so a 63-bit declaration cannot derive a
    // placement of zero bits by wrapping.
    //
    // Reading `<Unsigned<13> as Slots>::WIDTH == Width::bits(13)` instead would
    // check a macro whose body is `Width::bits($w)` against its own literal, the
    // class the comment two tests down condemns.
    let declared = declared_slot_width::<WiderThanItsSpan>();
    let recovered = Width::bits(slot_count::<WiderThanItsSpan>().count().ilog2());

    assert_eq!(declared, Width::bits(13));
    assert_eq!(
        recovered,
        Width::bits(2),
        "the span is four slots, so counting it back gives two bits"
    );
    assert_ne!(
        declared, recovered,
        "the declared width and the one its bounds imply agree here, so this arm \
         cannot tell a width that is read from one that is counted"
    );

    // The control: where a declaration is tight the two agree, so the arm above is
    // about which of them comes back rather than about the function refusing to
    // answer at all. Every range this crate ships is of that kind.
    macro_rules! agree_where_tight {
        ($($w:literal),+ $(,)?) => {
            $(
                assert_eq!(
                    declared_slot_width::<Unsigned<$w>>(),
                    Width::bits(slot_count::<Unsigned<$w>>().count().ilog2()),
                    "unsigned {} disagrees with its own span", $w
                );
                assert_eq!(
                    declared_slot_width::<Signed<$w>>(),
                    Width::bits(slot_count::<Signed<$w>>().count().ilog2()),
                    "signed {} disagrees with its own span", $w
                );
            )+
        };
    }
    agree_where_tight!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
}

#[test]
fn every_admitted_width_has_a_coherent_range() {
    // The property the bound is about, asserted over the whole admitted set
    // rather than about a constant naming it. A width whose impl inverted its
    // own range fails here, which a constant asserted against its own literal
    // could not.
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
                    assert_eq!(slot_count::<Unsigned<$w>>(), SlotCount::of(1i128 << $w));
                    assert_eq!(slot_count::<Signed<$w>>(), SlotCount::of(1i128 << $w));
                    // The ends, derived here from the 128-bit index rather than
                    // from the 64-bit shifts the impls are written in, so a wrong
                    // shift in either spelling disagrees with the other.
                    assert_eq!(<Unsigned<$w> as Slots>::MIN, Slot::ZERO);
                    assert_eq!(<Unsigned<$w> as Slots>::MAX, Slot::at((1i128 << $w) - 1));
                    assert_eq!(<Signed<$w> as Slots>::MIN, Slot::at(-(1i128 << ($w - 1))));
                    assert_eq!(<Signed<$w> as Slots>::MAX, Slot::at((1i128 << ($w - 1)) - 1));
                }
            )+
        };
    }
    coherent!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );
}

#[test]
fn the_ranges_at_the_host_widths_are_the_host_integers_own() {
    // A third derivation of the ends, from the host's own integer types, at the
    // four widths that have one. The ladder's widest rung is the unsigned 64-bit
    // range, whose top no signed 64-bit index holds, which is why the index is
    // wider than that.
    macro_rules! host {
        ($(($w:literal, $u:ty, $i:ty)),+ $(,)?) => {
            $(
                assert_eq!(<Unsigned<$w> as Slots>::MIN, Slot::at(<$u>::MIN as i128));
                assert_eq!(<Unsigned<$w> as Slots>::MAX, Slot::at(<$u>::MAX as i128));
                assert_eq!(<Signed<$w> as Slots>::MIN, Slot::at(<$i>::MIN as i128));
                assert_eq!(<Signed<$w> as Slots>::MAX, Slot::at(<$i>::MAX as i128));
            )+
        };
    }
    host!((8, u8, i8), (16, u16, i16), (32, u32, i32), (64, u64, i64));
    assert!(<Unsigned<64> as Slots>::MAX.index() > i64::MAX as i128);
}

#[test]
fn the_admitted_set_is_the_contiguous_run_the_macro_names() {
    // The list in `slots` is the bound, so what it contains is a fact worth
    // pinning: a contiguous run from one to the widest admitted width, with no
    // gap and nothing past the end.
    let widths = crate::slots::ADMITTED_WIDTHS;
    assert_eq!(widths.first(), Some(&Width::bits(1)));
    assert_eq!(widths.last(), Some(&Width::bits(64)));
    assert_eq!(widths.len(), 64);
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
fn the_widest_admitted_width_is_the_widest_pointer_width() {
    // Why 64, derived rather than restated: the ladder has to reach the running
    // target's pointer width, since that is where the platform-width points sit,
    // and 64 is the widest a target has. Nothing past it is asked for.
    let widest = crate::slots::ADMITTED_WIDTHS.last().unwrap().count();
    assert!(
        widest >= usize::BITS,
        "the running target's pointer width is past the ladder, so `USize` has no range here"
    );
    assert_eq!(widest, 64);

    // At that width both ranges count `2^64` slots, which no 64-bit integer holds
    // and the count's own integer does.
    assert_eq!(slot_count::<Unsigned<64>>(), SlotCount::of(1i128 << 64));
    assert_eq!(slot_count::<Signed<64>>(), SlotCount::of(1i128 << 64));
    assert!(slot_count::<Unsigned<64>>().count() > u64::MAX as i128);
}

// --- what an outside implementor owes, and the constructions that do not ----

mod the_range_obligation;

pub(crate) use the_range_obligation::{AtTheBottom, AtTheTop};

// --- what a quantum law owes, and the constructions that do not --------------

mod the_quantum_obligation;
