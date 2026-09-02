//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A format this crate declares, and the placement derived from it.
//!
//! `proposal::the_concept_is_closed_and_the_inventory_is_open`, ratified through
//! `ruling::the_format_spine_is_canon`, says a new instance earns admission by
//! supplying the concept's obligations, and that closing the concept while opening
//! the inventory is what makes admission a check rather than a negotiation.
//!
//! **Where this file sits is half of what it asserts.** `arvo-format` introduces
//! the numeric category, so the bare-primitive lints skip it whole, and a format
//! declared in that crate's own tests is written under a rule with nothing to
//! refuse. This crate is read by those lints like any other consumer: a coordinate
//! written as a host integer here stops the gate, so the declaration below
//! standing is the check the ratified clause asks for rather than a demonstration
//! that the types compile somewhere.
//!
//! The other half is that it supplies all of them. An ambient domain, a quantum
//! law and a slot range nothing ships, plus the phase the format carries. Reusing
//! the shipped parts exercises the one coordinate `Format` itself demands and says
//! nothing about the nine underneath it.
//!
//! What a format owes and what refuses a bad one are stated in `arvo-format` and
//! are not restated here, because two copies of one rule are two things that can
//! disagree and only one of them would be the rule.

use crate::objective;
use crate::{declared_width, derive_shared, derive_sole, narrowest_carrier, Occupancy};
use arvo_format::ambient::Ambient;
use arvo_format::format::Format;
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::quantum::{Exponent, MagnitudeCount, Quantum};
use arvo_format::rounding::Floor;
use arvo_format::slots::{Slot, Slots};
use arvo_format::{Adapt, Bool, Phase, Radix, Signature, Width};

/// A domain nothing in the stack ships: the rationals at radix seven.
struct SeptenaryRationals;

impl Ambient for SeptenaryRationals {
    const RADIX: Radix = Radix::of(7);
    const SIGNED: Bool = Bool::TRUE;
}

/// A step law nothing in the stack ships: three exponents per magnitude.
struct TripleStepped;

impl Quantum for TripleStepped {
    const BASE: Exponent = Exponent::of(-5);
    const SLOPE: Exponent = Exponent::of(3);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(6);
}

/// A slot range nothing in the stack ships: twenty-seven bits, offset so it is
/// neither of the two shapes `arvo-format` writes.
struct OffsetTwentySeven;

impl Slots for OffsetTwentySeven {
    const MIN: Slot = Slot::at(-1000);
    const MAX: Slot = Slot::at(66108863);
    const WIDTH: Width = Width::bits(27);
}

/// The format, out of the three above plus a phase of its own.
///
/// The phase is two whole quanta, which is a value no shipped point carries: the
/// four `arvo-format` writes set it to zero or to a count of half steps. What that
/// means for the representable set is `arvo-format`'s law and is asserted there.
struct Septenary;

impl Format for Septenary {
    type Ambient = SeptenaryRationals;
    type Quantum = TripleStepped;
    type Slots = OffsetTwentySeven;
    const PHASE: Phase = Phase::of(4, 2);
}

type Foreign = Signature<Septenary, Adapt<Floor, Wrap>>;

#[test]
fn the_derivation_reads_a_format_this_crate_declared_itself() {
    // The declared width comes from the foreign slot range and from nothing else,
    // which is the mechanical form of the rule that the derivation is a function
    // of the declaration.
    assert_eq!(declared_width::<Foreign>(), Width::bits(27));

    // And both arms produce a placement from it. At twenty-seven bits the ladder
    // lands on the thirty-two bit carrier, and the packed stride is the declared
    // width rather than the carrier.
    let sole = derive_sole::<Foreign, objective::Footprint>();
    let shared = derive_shared::<Foreign, objective::Footprint>();

    assert_eq!(sole.carrier, narrowest_carrier(Width::bits(27)));
    assert_eq!(sole.occupancy, Occupancy::Sole);
    assert_eq!(shared.stride, declared_width::<Foreign>());
    assert_ne!(shared.stride, shared.carrier);
    assert_eq!(shared.occupancy, Occupancy::Shared);

    // The two objectives behave for a format this crate wrote exactly as the
    // design says they behave for one it did not: together at sole occupancy and
    // apart at shared.
    assert_eq!(
        derive_sole::<Foreign, objective::Footprint>(),
        derive_sole::<Foreign, objective::Access>()
    );
    assert_ne!(
        derive_shared::<Foreign, objective::Footprint>(),
        derive_shared::<Foreign, objective::Access>()
    );
}

#[test]
fn the_control_the_foreign_width_is_not_one_a_shipped_point_here_produces() {
    // Without this the arms above could be reading a width they already had from
    // some other signature in this crate and agreeing by coincidence. Twenty-seven
    // bits is not a width any `Integer` in the sweeps reaches, and the foreign
    // coordinates differ from the shipped ones they would otherwise borrow.
    assert_ne!(
        declared_width::<Foreign>(),
        declared_width::<Signature<Integer<13>, Adapt<Floor, Wrap>>>()
    );
    assert_ne!(
        declared_width::<Foreign>(),
        declared_width::<Signature<Integer<32>, Adapt<Floor, Wrap>>>()
    );

    // And every coordinate under `Format` is one this crate supplied rather than
    // one it borrowed, which is what makes this a member of the inventory rather
    // than a rearrangement of the shipped points.
    assert_ne!(
        <SeptenaryRationals as Ambient>::RADIX,
        <<Integer<13> as Format>::Ambient as Ambient>::RADIX
    );
    assert_ne!(
        <TripleStepped as Quantum>::SLOPE,
        <<Integer<13> as Format>::Quantum as Quantum>::SLOPE
    );
    assert_ne!(
        <OffsetTwentySeven as Slots>::MIN,
        <<Integer<13> as Format>::Slots as Slots>::MIN
    );
    assert_ne!(<Septenary as Format>::PHASE, <Integer<13> as Format>::PHASE);
}

#[test]
fn the_foreign_format_carries_the_coordinates_the_contract_asks_for() {
    // Read back through the contract rather than through the impls, so a
    // coordinate the crate ignored would not pass here. This is the placement
    // crate's side of the open inventory: it names every coordinate a format is
    // declared with, and what refuses a bad one is stated in `arvo-format`.
    assert_eq!(<SeptenaryRationals as Ambient>::RADIX, Radix::of(7));
    assert!(<SeptenaryRationals as Ambient>::SIGNED.get());
    assert_eq!(<TripleStepped as Quantum>::BASE, Exponent::of(-5));
    assert_eq!(<TripleStepped as Quantum>::SLOPE, Exponent::of(3));
    assert_eq!(
        <TripleStepped as Quantum>::MAGNITUDES,
        MagnitudeCount::of(6)
    );
    assert_eq!(<OffsetTwentySeven as Slots>::MIN, Slot::at(-1000));
    assert_eq!(<OffsetTwentySeven as Slots>::MAX, Slot::at(66108863));
    assert_eq!(<OffsetTwentySeven as Slots>::WIDTH, Width::bits(27));
    assert_eq!(<Septenary as Format>::PHASE.numerator(), 4);
    assert_eq!(<Septenary as Format>::PHASE.denominator(), 2);
}
