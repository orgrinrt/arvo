//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The carrier as a type is the carrier as a width, at every row.
//!
//! Every row is held against `derive_sole` on the slot family's canonical point,
//! at every width `arvo-format` admits, and an outside row naming the wrong
//! container is reported by the verdict beside a control naming the right one.

use arvo_format::overflow::Wrap;
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::Floor;
use arvo_format::slots::{Signed, Slot, Slots, Unsigned};
use arvo_format::{Adapt, Signature, Width};

use crate::{Carrier, Carrier8, Carrier16, SoleCarrier, carrier_agrees, derive_sole, objective};

/// The width `derive_sole` gives a signature, beside the width of the
/// container the row names for its slot family.
macro_rules! both_forms {
    ($point:ty, $family:ty) => {
        (
            derive_sole::<Signature<$point, Adapt<Floor, Wrap>>, objective::Access>().carrier,
            <<$family as SoleCarrier>::Carrier as Carrier>::BITS,
        )
    };
}

macro_rules! every_width {
    ($($w:literal)+) => {
        [$(
            both_forms!(Integer<$w>, Signed<$w>),
            both_forms!(UFixed<$w, 0>, Unsigned<$w>),
        )+]
    };
}

#[test]
fn every_row_names_the_container_derive_sole_derives_at_every_admitted_width() {
    let rows = every_width!(
        1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
        33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62
        63 64
    );
    assert_eq!(rows.len(), 128);
    for (derived, named) in rows {
        assert_eq!(derived, named);
    }
}

/// An eight-bit slot family of this file's own.
struct OutsideEight;

impl Slots for OutsideEight {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(0);
    const WIDTH: Width = Width::bits(8);
}

/// The same family, as another crate would declare it, with the right container.
struct RightRow;

impl Slots for RightRow {
    const MAX: Slot = OutsideEight::MAX;
    const MIN: Slot = OutsideEight::MIN;
    const WIDTH: Width = OutsideEight::WIDTH;
}

impl SoleCarrier for RightRow {
    type Carrier = Carrier8;
}

/// The same family with a container one rung too wide, which the obligation
/// refuses: alone, eight bits sit in the eight-bit container.
struct WrongRow;

impl Slots for WrongRow {
    const MAX: Slot = OutsideEight::MAX;
    const MIN: Slot = OutsideEight::MIN;
    const WIDTH: Width = OutsideEight::WIDTH;
}

impl SoleCarrier for WrongRow {
    type Carrier = Carrier16;
}

#[test]
fn an_outside_row_naming_another_container_than_the_derivation_is_reported() {
    assert!(carrier_agrees::<RightRow>().get());
    assert!(!carrier_agrees::<WrongRow>().get());
}
