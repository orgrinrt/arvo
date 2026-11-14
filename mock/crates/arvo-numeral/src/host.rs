//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The door where the host's bytes and words enter the stack.
//!
//! A consumer that reads the host's byte image, such as a digest, takes it as [`Bytes`] and does its
//! arithmetic in [`Word`], so it names no host integer of its own. Nothing else in this crate uses
//! these types, and none of them is a number: a `Word` has no declared signature, no adaptation and no
//! debug check, only the host word and the operations named here.

use arvo_format::Bool;
use notko::Maybe;

/// One byte of the host's byte image.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Byte(u8);

impl Byte {
    /// A byte from the host's.
    #[must_use]
    pub const fn of(host: u8) -> Self {
        Self(host)
    }

    /// The host's byte, an unwrap door.
    #[must_use]
    pub const fn host(self) -> u8 {
        self.0
    }
}

/// A borrowed run of the host's bytes.
///
/// Walked by [`Bytes::split_first`], and its length asked only by [`Bytes::shorter_than`], so no index
/// or length type is needed to read one.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bytes<'a>(&'a [u8]);

impl<'a> Bytes<'a> {
    /// A run over the host's bytes.
    #[must_use]
    pub const fn of(host: &'a [u8]) -> Self {
        Self(host)
    }

    /// The host's bytes, an unwrap door.
    #[must_use]
    pub const fn host(self) -> &'a [u8] {
        self.0
    }

    /// The first byte and the rest of the run, or nothing when the run is empty.
    #[must_use]
    pub const fn split_first(self) -> Maybe<(Byte, Bytes<'a>)> {
        match self.0 {
            [first, rest @ ..] => Maybe::Is((Byte(*first), Bytes(rest))),
            [] => Maybe::Isnt,
        }
    }

    /// Whether the run holds fewer than `N` bytes.
    #[must_use]
    pub const fn shorter_than<const N: usize>(self) -> Bool {
        Bool::of(self.0.len() < N)
    }
}

mod sealed {
    pub trait WordWidth {}
    impl WordWidth for super::At<32> {}
    impl WordWidth for super::At<64> {}
    impl WordWidth for super::At<128> {}
}

/// The width marker a [`Word`] is admitted through.
pub struct At<const BITS: u32>;

/// The host integer a word of a width is held in.
///
/// Implemented for `At<32>`, `At<64>` and `At<128>` and sealed, so no other width of `Word` exists.
pub trait HostWord: sealed::WordWidth {
    /// The host's unsigned integer of this width.
    type Host: Copy + Eq + core::fmt::Debug;
}

impl HostWord for At<32> {
    type Host = u32;
}

impl HostWord for At<64> {
    type Host = u64;
}

impl HostWord for At<128> {
    type Host = u128;
}

/// A host word of `BITS` bits.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Word<const BITS: u32>(<At<BITS> as HostWord>::Host)
where
    At<BITS>: HostWord;

impl<const BITS: u32> Word<BITS>
where
    At<BITS>: HostWord,
{
    /// A word from the host's integer of this width.
    #[must_use]
    pub const fn of(host: <At<BITS> as HostWord>::Host) -> Self {
        Self(host)
    }

    /// The host's integer, an unwrap door.
    #[must_use]
    pub const fn host(self) -> <At<BITS> as HostWord>::Host {
        self.0
    }
}

impl Word<32> {
    /// The same value as a 64 bit word.
    #[must_use]
    pub const fn widen(self) -> Word<64> {
        Word(self.0 as u64)
    }

    /// The byte, zero-extended, xored into the low bits.
    #[must_use]
    pub const fn xor_byte(self, byte: Byte) -> Self {
        Self(self.0 ^ byte.0 as u32)
    }

    /// The product, reduced modulo two to the 32.
    #[must_use]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self(self.0.wrapping_mul(rhs.0))
    }
}

impl Word<64> {
    /// The byte, zero-extended, xored into the low bits.
    #[must_use]
    pub const fn xor_byte(self, byte: Byte) -> Self {
        Self(self.0 ^ byte.0 as u64)
    }

    /// The product, reduced modulo two to the 64.
    #[must_use]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self(self.0.wrapping_mul(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use notko::Maybe;

    use super::{Byte, Bytes, Word};

    #[test]
    fn a_byte_and_a_run_give_back_what_they_were_made_of() {
        assert_eq!(Byte::of(0xA5).host(), 0xA5);
        let run = Bytes::of(b"abc");
        assert_eq!(run.host(), b"abc");
        assert_eq!(Word::<128>::of(u128::MAX).host(), u128::MAX);
    }

    #[test]
    fn split_first_walks_the_run_in_order_and_ends_on_nothing() {
        let mut run = Bytes::of(b"xyz");
        let mut seen = [0u8; 3];
        let mut at = 0;
        while let Maybe::Is((byte, rest)) = run.split_first() {
            seen[at] = byte.host();
            at += 1;
            run = rest;
        }
        assert_eq!(&seen, b"xyz");
        assert_eq!(at, 3);
        assert!(matches!(Bytes::of(b"").split_first(), Maybe::Isnt));
    }

    #[test]
    fn shorter_than_compares_the_length_strictly() {
        let run = Bytes::of(b"four");
        assert!(run.shorter_than::<5>().get());
        assert!(!run.shorter_than::<4>().get());
        assert!(!run.shorter_than::<0>().get());
        assert!(Bytes::of(b"").shorter_than::<1>().get());
    }

    #[test]
    fn xor_byte_touches_only_the_low_eight_bits() {
        assert_eq!(
            Word::<32>::of(0xFFFF_FF00).xor_byte(Byte::of(0x0F)).host(),
            0xFFFF_FF0F
        );
        assert_eq!(
            Word::<64>::of(0x0123_4567_89AB_CDEF)
                .xor_byte(Byte::of(0xFF))
                .host(),
            0x0123_4567_89AB_CD10
        );
    }

    #[test]
    fn wrapping_mul_reduces_modulo_the_width() {
        // 0x8000_0001 * 3 = 0x1_8000_0003, whose low 32 bits are 0x8000_0003.
        assert_eq!(
            Word::<32>::of(0x8000_0001).wrapping_mul(Word::of(3)).host(),
            0x8000_0003
        );
        // 2^63 * 2 = 2^64, which is zero modulo 2^64.
        assert_eq!(Word::<64>::of(1 << 63).wrapping_mul(Word::of(2)).host(), 0);
    }

    #[test]
    fn widen_keeps_the_value() {
        assert_eq!(
            Word::<32>::of(u32::MAX).widen().host(),
            0x0000_0000_FFFF_FFFF
        );
    }

    #[test]
    fn the_door_works_in_a_const() {
        const W: Word<64> = Word::<64>::of(7)
            .wrapping_mul(Word::of(6))
            .xor_byte(Byte::of(1));
        assert_eq!(W.host(), 43);
    }

    #[test]
    #[should_panic(expected = "assertion")]
    fn planted_a_wrong_product_is_caught() {
        assert_eq!(
            Word::<32>::of(0x8000_0001).wrapping_mul(Word::of(3)).host(),
            0x8000_0004
        );
    }
}
