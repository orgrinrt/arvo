//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! XXH3 at 64 and 128 bits, through xxhash-rust.

use arvo_numeral::{Bytes, Word};

use crate::Digest;

/// XXH3, `BITS` wide, with the default secret and no seed.
///
/// Two entry points per width, because one function cannot be both. `of`
/// runs xxhash-rust's runtime path, whose SIMD width `cfg(target_feature)`
/// picked when this crate was compiled. `of_const` runs its `const fn` path.
/// The two give the same value for the same bytes.
pub struct Xxh3<const BITS: u16>;

impl Xxh3<64> {
    /// The 64 bit XXH3 digest of `bytes`, on the runtime path.
    #[must_use]
    #[inline]
    pub fn of(bytes: Bytes<'_>) -> Digest<64> {
        Digest::from_raw(Word::of(xxhash_rust::xxh3::xxh3_64(bytes.host())))
    }

    /// The 64 bit XXH3 digest of `bytes`, in const context.
    #[must_use]
    pub const fn of_const(bytes: Bytes<'_>) -> Digest<64> {
        Digest::from_raw(Word::of(xxhash_rust::const_xxh3::xxh3_64(bytes.host())))
    }
}

impl Xxh3<128> {
    /// The 128 bit XXH3 digest of `bytes`, on the runtime path.
    #[must_use]
    #[inline]
    pub fn of(bytes: Bytes<'_>) -> Digest<128> {
        Digest::from_raw(Word::of(xxhash_rust::xxh3::xxh3_128(bytes.host())))
    }

    /// The 128 bit XXH3 digest of `bytes`, in const context.
    #[must_use]
    pub const fn of_const(bytes: Bytes<'_>) -> Digest<128> {
        Digest::from_raw(Word::of(xxhash_rust::const_xxh3::xxh3_128(bytes.host())))
    }
}
