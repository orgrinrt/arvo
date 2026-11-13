//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The map from a slot family to its ladder carrier.
//!
//! One list of admitted widths writes the carrier for `Signed<W>` and
//! `Unsigned<W>`, and each row asserts at check time that the carrier's size is
//! what `arvo_placement::derive_sole` computes for the family's canonical point,
//! so the map is a transcription of the placement and cannot drift from it.

use core::mem::size_of;

use arvo_format::overflow::Wrap;
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::slots::{Signed, Unsigned};
use arvo_format::{Adapt, Format, Signature, Slots};
use arvo_placement::derive_sole;
use arvo_placement::objective::Access;

use crate::machine::Machine;

mod sealed {
    pub trait Sealed {}
}

/// A slot family the ladder holds, and the carrier it is held in.
///
/// Sealed: the rows below are the whole map, so a carrier is always the one the
/// placement derives for the family's width.
#[diagnostic::on_unimplemented(
    message = "no ladder carrier holds the slot family `{Self}`",
    note = "a value is held for `Signed<W>` or `Unsigned<W>` with `W` from 1 to 64; zero bits has no slot range and no carrier holds more than 64"
)]
pub trait Carried: Slots + sealed::Sealed {
    /// The narrowest ladder carrier that holds the family's declared width,
    /// signed exactly where the family is.
    type Repr: Machine;
}

/// The canonical point a slot family is the slot range of, so the map can ask
/// the placement what it derives: `derive_sole` reads a declared signature, and
/// the slot family alone is not one.
trait PointOf: Slots {
    type Point: Format;
}

impl<const W: u32> PointOf for Signed<W>
where
    Signed<W>: Slots,
{
    type Point = Integer<W>;
}

impl<const W: u32> PointOf for Unsigned<W>
where
    Unsigned<W>: Slots,
{
    type Point = UFixed<W, 0>;
}

/// How many bits `derive_sole` puts a slot family's canonical point in.
const fn derived_bits<Sl: PointOf>() -> u32 {
    derive_sole::<Signature<Sl::Point, Adapt<TowardZero, Wrap>>, Access>()
        .carrier
        .count()
}

/// How many bits a carrier has.
const fn carrier_bits<R: Machine>() -> u32 {
    size_of::<R>() as u32 * 8
}

macro_rules! map {
    ($($s:ty, $u:ty => [$($w:literal)+];)+) => {
        $($(
            impl sealed::Sealed for Signed<$w> {}
            impl Carried for Signed<$w> {
                type Repr = $s;
            }
            impl sealed::Sealed for Unsigned<$w> {}
            impl Carried for Unsigned<$w> {
                type Repr = $u;
            }
            const _: () = assert!(
                carrier_bits::<$s>() == derived_bits::<Signed<$w>>()
                    && carrier_bits::<$u>() == derived_bits::<Unsigned<$w>>(),
                "the carrier the map names is not the one the placement derives"
            );
        )+)+
    };
}

map! {
    i8, u8 => [1 2 3 4 5 6 7 8];
    i16, u16 => [9 10 11 12 13 14 15 16];
    i32, u32 => [17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32];
    i64, u64 => [
        33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
        49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
    ];
}
