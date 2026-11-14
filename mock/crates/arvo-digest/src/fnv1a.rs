//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! FNV-1a, by Fowler, Noll and Vo, at 32 and 64 bits.

use arvo_numeral::{Bytes, Word};
use notko::Maybe;

use crate::Digest;

/// FNV-1a, `BITS` wide.
///
/// Each byte is xored into the state and the state multiplied by the
/// width's prime, wrapping. The offset bases and primes are the reference
/// ones, so a value here equals the value any other FNV-1a gives for the
/// same bytes, which is what a consumer with stored FNV-1a values needs.
pub struct Fnv1a<const BITS: u16>;

impl Fnv1a<32> {
    const OFFSET: Word<32> = Word::of(0x811C_9DC5);
    const PRIME: Word<32> = Word::of(0x0100_0193);

    /// The 32 bit FNV-1a digest of `bytes`.
    #[must_use]
    pub const fn of(bytes: Bytes<'_>) -> Digest<32> {
        let mut state = Self::OFFSET;
        let mut rest = bytes;
        while let Maybe::Is((byte, after)) = rest.split_first() {
            state = state.xor_byte(byte).wrapping_mul(Self::PRIME);
            rest = after;
        }
        Digest::from_raw(state)
    }
}

impl Fnv1a<64> {
    const OFFSET: Word<64> = Word::of(0xCBF2_9CE4_8422_2325);
    const PRIME: Word<64> = Word::of(0x0000_0100_0000_01B3);

    /// The 64 bit FNV-1a digest of `bytes`.
    #[must_use]
    pub const fn of(bytes: Bytes<'_>) -> Digest<64> {
        let mut state = Self::OFFSET;
        let mut rest = bytes;
        while let Maybe::Is((byte, after)) = rest.split_first() {
            state = state.xor_byte(byte).wrapping_mul(Self::PRIME);
            rest = after;
        }
        Digest::from_raw(state)
    }
}
