//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What the same positions already carry from the stack.
//!
//! **230 host primitives at API positions is a numerator with no denominator.**
//! It says nothing about whether the obligation is nearly met or barely started,
//! because the same walk over the same positions counting the stack's own types
//! is the other half of that fraction, and nothing has ever counted it.
//!
//! So this runs the identical position classification over `type_identifier`
//! rather than `primitive_type`, and keeps the ones whose name is on the list
//! below.
//!
//! # The list is written down rather than inferred, and that is a limit
//!
//! There is no way to ask a tree "is this name one of the stack's numeric
//! types" without resolving imports, which needs a compiler. So the list is
//! maintained by hand, it is **printed in the report**, and a name missing from
//! it undercounts the supply side, which is the direction that makes the
//! obligation look further from met than it is.
//!
//! Two things about its contents. `Maybe`, `Outcome` and `Just` are notko's
//! fallibility carriers rather than numerals, and they are counted separately,
//! because a position holding one is a position that already went through the
//! stack even though no number is involved. And the list carries the arvo names
//! that exist **now**: the crate tree that held `UFixed`, `IFixed`, `FastFloat`,
//! `StrictFloat`, `Cap` and `Bits` was deleted, so those names are on the list
//! and currently match nothing, which is itself worth seeing.

/// What arvo exports today, read off the three crates' own `lib.rs` on `dev`.
///
/// `arvo-format`, `arvo-placement` and `arvo-strategy`, and nothing else,
/// because nothing else is there.
pub const ARVO_SHIPPED: &[&str] = &[
    // arvo-format
    "Width",
    "Bool",
    "Format",
    "Quantum",
    "Magnitude",
    "MagnitudeCount",
    "Slot",
    "SlotCount",
    "Slots",
    "Fraction",
    "Overflow",
    "Policy",
    "Mode",
    "Rounding",
    "Dither",
    "Exact",
    "Ambient",
    "Adapt",
    "Adaptation",
    "Arity",
    "Exponent",
    "Operation",
    "Phase",
    "Radix",
    "Signature",
    "DeclaredSignature",
    "Integer",
    "UFixed",
    "Biased",
    "Floating",
    // arvo-placement
    "Placement",
    "Carrier",
    "Access",
    "Footprint",
    "Occupancy",
    "Objective",
    "ObjectiveKind",
    // arvo-strategy
    "Strategy",
    "Hot",
    "Warm",
    "Cold",
    "Precise",
];

/// Names consumers write as arvo's that arvo no longer has.
///
/// **This is the finding the count exists to make visible.** The crate tree
/// holding these was deleted, and the consumers were not changed, so a position
/// carrying one of these is not a satisfied obligation: it is a position
/// pointing at nothing. Counting it on the supply side, which is what a single
/// arvo list does, makes the obligation look 85 percent met when most of the 85
/// does not resolve.
///
/// Established by reading the three shipped `lib.rs` files against what the
/// consumers import; `hilavitkutin` alone writes `use arvo::USize`,
/// `use arvo::Cap`, `use arvo::{Bits, Hot, Unsigned}`, `use arvo_bits::Bits`,
/// `use arvo_bitmask::BitAccess` and `use arvo_tensor::{cap_size, Capacity, Dim}`.
pub const ARVO_GONE: &[&str] = &[
    "USize",
    "Cap",
    "Capacity",
    "Bits",
    "BitPrim",
    "BitWidth",
    "BitAccess",
    "ContentHash",
    "IFixed",
    "FastFloat",
    "StrictFloat",
    "UWire",
    "Dim",
    "Unsigned",
];

/// notko's carriers, which are not numerals and are still the stack's own.
pub const NOTKO: &[&str] = &[
    "Just",
    "Maybe",
    "Outcome",
    "Boundable",
    "NonZeroable",
    "Lent",
];

/// Which side of the supply a type name is on, or `None` if it is not the
/// stack's at all.
///
/// Three answers rather than two, and the third is the point: `gone` is a
/// position that already stopped using a host primitive and now names a type
/// that is not there.
#[must_use]
pub fn supplier(name: &str) -> Option<&'static str> {
    if ARVO_SHIPPED.contains(&name) {
        Some("arvo")
    } else if ARVO_GONE.contains(&name) {
        Some("gone")
    } else if NOTKO.contains(&name) {
        Some("notko")
    } else {
        None
    }
}

/// Every name on any of the three lists, for the report's own accounting.
#[must_use]
pub fn every_name() -> Vec<&'static str> {
    ARVO_SHIPPED
        .iter()
        .chain(ARVO_GONE.iter())
        .chain(NOTKO.iter())
        .copied()
        .collect()
}
