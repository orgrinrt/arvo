//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The result type, and the widths it exists at.

use arvo_bits::Bits;

/// A digest width, as a type.
///
/// It carries nothing at run time. Its one job is to be the thing
/// [`Carrier`] is implemented for, so that the widths a digest exists at are
/// a closed fact the compiler checks rather than a value checked later.
pub struct Output<const BITS: u32>;

mod sealed {
    pub trait OutputWidth {}
    impl OutputWidth for super::Output<32> {}
    impl OutputWidth for super::Output<64> {}
    impl OutputWidth for super::Output<128> {}
}

/// The container a digest of a given width is held in.
///
/// Implemented for `Output<32>`, `Output<64>` and `Output<128>` and sealed, so
/// `Digest<BITS>` at any other width is not a type.
pub trait Carrier: sealed::OutputWidth {
    /// The `arvo_bits` container exactly as wide as the digest: one `Bits` up to
    /// sixty-four bits, and two `Bits<64>`, low half first, at 128.
    type Raw: Copy + Eq + core::fmt::Debug;
}

impl Carrier for Output<32> {
    type Raw = Bits<32>;
}

impl Carrier for Output<64> {
    type Raw = Bits<64>;
}

impl Carrier for Output<128> {
    type Raw = [Bits<64>; 2];
}

/// A digest, `BITS` wide.
///
/// Compares for equality and copies. There is no ordering, because the
/// order of two digests means nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Digest<const BITS: u32>(<Output<BITS> as Carrier>::Raw)
where
    Output<BITS>: Carrier;

impl<const BITS: u32> Digest<BITS>
where
    Output<BITS>: Carrier,
{
    /// Wraps a value an algorithm in this crate computed.
    pub(crate) const fn from_raw(raw: <Output<BITS> as Carrier>::Raw) -> Self {
        Self(raw)
    }

    /// The digest in its container, the one door out.
    #[must_use]
    pub const fn raw(self) -> <Output<BITS> as Carrier>::Raw {
        self.0
    }
}

impl Digest<32> {
    /// The low `N` bits, at the width a consumer names.
    ///
    /// `Bits::cast`, the masked cast `arvo-bits` owns. `N` wider than 32 is
    /// refused when this function is instantiated.
    #[must_use]
    pub const fn bits<const N: u32>(self) -> Bits<N> {
        const { assert!(N <= 32, "a 32 bit digest has no bits above 32 to give") };
        self.0.cast::<N>()
    }
}

impl Digest<64> {
    /// The low `N` bits, at the width a consumer names.
    ///
    /// `Bits::cast`, the masked cast `arvo-bits` owns. `Bits` holds at most 64 bits, so
    /// every `N` it admits fits a 64 bit digest.
    #[must_use]
    pub const fn bits<const N: u32>(self) -> Bits<N> {
        self.0.cast::<N>()
    }
}
