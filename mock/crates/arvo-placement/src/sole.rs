//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The carrier at sole occupancy as a type.
//!
//! `derive_sole` answers with a width, which a value cannot be stored in. A value
//! needs the container itself, chosen at compile time, so the same answer is
//! given here a second time in the form a type can be built from: each slot
//! family names the ladder container its declared width sits in alone.
//!
//! The two forms are one derivation. The type is checked against the width the
//! derivation computes, every row here at check time. `ADMITTED` is provided, so
//! an outside row may write one of its own that checks nothing; a crate that
//! needs the agreement of an outside row asserts `carrier_agrees` itself.

use arvo_format::slots::{Signed, Unsigned};
use arvo_format::{Bool, Slots};

use crate::{Carrier, Carrier8, Carrier16, Carrier32, Carrier64, narrowest_carrier};

/// A slot family, and the ladder container it sits in at sole occupancy.
///
/// Open: an outside slot family names its container here and owes `ADMITTED`,
/// that the container is the one the derivation picks for its declared width.
/// This crate writes the rows for `Signed<W>` and `Unsigned<W>` at every width
/// `arvo-format` admits.
#[diagnostic::on_unimplemented(
    message = "no ladder container holds the slot family `{Self}` alone",
    note = "the rows here cover `Signed<W>` and `Unsigned<W>` with `W` from 1 to 64; a slot family of \
            another crate names its container by implementing this trait"
)]
pub trait SoleCarrier: Slots {
    /// The narrowest ladder container that holds the family's declared width.
    type Carrier: Carrier;

    /// What an implementor owes, checked rather than asked for: the container is
    /// the one `derive_sole` derives for the declared width.
    ///
    /// Forced by every row of this crate. An outside row may replace it with a
    /// constant of its own, so a crate building on an outside row asserts
    /// `carrier_agrees` where it needs the agreement.
    ///
    /// ```compile_fail
    /// use arvo_format::Width;
    /// use arvo_format::slots::{Slot, Slots};
    /// use arvo_placement::{Carrier16, SoleCarrier};
    ///
    /// struct Eight;
    ///
    /// impl Slots for Eight {
    ///     const MIN: Slot = Slot::at(0);
    ///     const MAX: Slot = Slot::at(255);
    ///     const WIDTH: Width = Width::bits(8);
    /// }
    ///
    /// impl SoleCarrier for Eight {
    ///     type Carrier = Carrier16;
    /// }
    ///
    /// const _: () = <Eight as SoleCarrier>::ADMITTED;
    ///
    /// fn main() {}
    /// ```
    ///
    /// The control, which says the refusal above is this obligation: the same row
    /// naming the eight-bit container builds.
    ///
    /// ```
    /// use arvo_format::Width;
    /// use arvo_format::slots::{Slot, Slots};
    /// use arvo_placement::{Carrier8, SoleCarrier};
    ///
    /// struct Eight;
    ///
    /// impl Slots for Eight {
    ///     const MAX: Slot = Slot::at(255);
    ///     const MIN: Slot = Slot::at(0);
    ///     const WIDTH: Width = Width::bits(8);
    /// }
    ///
    /// impl SoleCarrier for Eight {
    ///     type Carrier = Carrier8;
    /// }
    ///
    /// const _: () = <Eight as SoleCarrier>::ADMITTED;
    ///
    /// fn main() {}
    /// ```
    const ADMITTED: () = assert!(
        carrier_agrees::<Self>().get(),
        "the slot family names a container the placement does not derive for its width: at sole \
         occupancy the carrier is the narrowest ladder container that holds the declared width"
    );
}

/// Whether a slot family's container is the one the derivation picks for its
/// declared width.
///
/// The obligation as a verdict, so a wrong row can be reported on without
/// forcing the const that refuses it.
#[must_use]
pub const fn carrier_agrees<Sl: SoleCarrier + ?Sized>() -> Bool {
    <Sl::Carrier as Carrier>::BITS.equals(narrowest_carrier(Sl::WIDTH))
}

/// Writes the rows at every admitted width and forces each one's obligation.
macro_rules! sole {
    ($($carrier:ty => [$($w:literal)+];)+) => {
        $($(
            impl SoleCarrier for Signed<$w> {
                type Carrier = $carrier;
            }
            impl SoleCarrier for Unsigned<$w> {
                type Carrier = $carrier;
            }
            const _: () = <Signed<$w> as SoleCarrier>::ADMITTED;
            const _: () = <Unsigned<$w> as SoleCarrier>::ADMITTED;
        )+)+
    };
}

sole! {
    Carrier8 => [1 2 3 4 5 6 7 8];
    Carrier16 => [9 10 11 12 13 14 15 16];
    Carrier32 => [17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32];
    Carrier64 => [
        33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
        49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
    ];
}
