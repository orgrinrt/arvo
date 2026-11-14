//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The result type, and the widths it exists at.

use arvo_bits::Bits;
use arvo_numeral::Word;

/// A digest width, as a type.
///
/// It carries nothing at run time. Its one job is to be the thing
/// [`Carrier`] is implemented for, so that the widths a digest exists at are
/// a closed fact the compiler checks rather than a value checked later.
pub struct Width<const BITS: u16>;

mod sealed {
    pub trait DigestWidth {}
    impl DigestWidth for super::Width<32> {}
    impl DigestWidth for super::Width<64> {}
    impl DigestWidth for super::Width<128> {}
}

/// The host word a digest of a given width is held in.
///
/// Implemented for `Width<32>`, `Width<64>` and `Width<128>` and sealed, so
/// `Digest<BITS>` at any other width is not a type.
pub trait Carrier: sealed::DigestWidth {
    /// The `arvo_numeral::Word` exactly as wide as the digest.
    type Raw: Copy + Eq + core::fmt::Debug;
}

impl Carrier for Width<32> {
    type Raw = Word<32>;
}

impl Carrier for Width<64> {
    type Raw = Word<64>;
}

impl Carrier for Width<128> {
    type Raw = Word<128>;
}

/// A digest, `BITS` wide.
///
/// Compares for equality and copies. There is no ordering, because the
/// order of two digests means nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Digest<const BITS: u16>(<Width<BITS> as Carrier>::Raw)
where
    Width<BITS>: Carrier;

impl<const BITS: u16> Digest<BITS>
where
    Width<BITS>: Carrier,
{
    /// Wraps a value an algorithm in this crate computed.
    pub(crate) const fn from_raw(raw: <Width<BITS> as Carrier>::Raw) -> Self {
        Self(raw)
    }

    /// The digest as its host word, the one door out.
    #[must_use]
    pub const fn raw(self) -> <Width<BITS> as Carrier>::Raw {
        self.0
    }
}

impl Digest<32> {
    /// The low `N` bits, at the width a consumer names.
    ///
    /// The masked cast `arvo-bits` owns. `N` wider than 32 is refused when
    /// this function is instantiated.
    #[must_use]
    pub const fn bits<const N: u32>(self) -> Bits<N> {
        const { assert!(N <= 32, "a 32 bit digest has no bits above 32 to give") };
        Bits::<N>::masked(self.0.widen().host())
    }
}

impl Digest<64> {
    /// The low `N` bits, at the width a consumer names.
    ///
    /// The masked cast `arvo-bits` owns. `Bits` holds at most 64 bits, so
    /// every `N` it admits fits a 64 bit digest.
    #[must_use]
    pub const fn bits<const N: u32>(self) -> Bits<N> {
        Bits::<N>::masked(self.0.host())
    }
}
