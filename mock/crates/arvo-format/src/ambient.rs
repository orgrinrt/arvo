//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The ambient domain a format's values are drawn from.
//!
//! Half of a format's identity, the other half being the representable set. Two
//! formats with the same representable set under different ambient algebras are
//! two formats, which is why the domain is carried rather than assumed.

/// The domain a representable set is a subset of.
///
/// The radix sits here because it is a property of the domain rather than of the
/// step law: the same law at radix 2 and radix 10 describes different values.
pub trait Ambient {
    /// The base positional notation counts in.
    const RADIX: u32;

    /// Whether the domain carries values below zero.
    ///
    /// Read as a coordinate of the domain rather than of the storage. A format
    /// over an unsigned domain has no negative member regardless of what any
    /// carrier could hold.
    const SIGNED: bool;
}

/// The rationals at radix two, which is where the fixed-point and binary floating
/// families live.
pub struct BinaryRationals;

impl Ambient for BinaryRationals {
    const RADIX: u32 = 2;
    const SIGNED: bool = true;
}

/// The non-negative rationals at radix two.
pub struct UnsignedBinaryRationals;

impl Ambient for UnsignedBinaryRationals {
    const RADIX: u32 = 2;
    const SIGNED: bool = false;
}

/// The rationals at radix ten, which is where the decimal conventions live.
///
/// Present because the radix is a coordinate and a design that carried only one
/// value of it would have hardcoded a choice the canon leaves open. Nothing in
/// this crate is specialised to radix two.
pub struct DecimalRationals;

impl Ambient for DecimalRationals {
    const RADIX: u32 = 10;
    const SIGNED: bool = true;
}
