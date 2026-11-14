//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The door for a host integer, stated over the slot index.
//!
//! A host integer names a slot index, and an index is on the grid, so nothing
//! rounds on the way in and the door needs only the completion. That is done at
//! 64 bits, by the 64-bit rung's own adaptation, and the format's exact step is
//! never on this path.
//!
//! Beside it sits the door for the host's byte image: a consumer that reads bytes, such as a
//! digest, takes them as [`Bytes`] and does its arithmetic in [`Word`], so it names no host integer of
//! its own. None of those is a number: a `Word` has no declared signature, no adaptation and no
//! debug check, only the host word and the operations named on it.

use core::mem::size_of;

use arvo_format::{Bool, Policy, Slots};
use notko::Maybe;

use crate::admit::Sole;
use crate::check::DebugCheck;
use crate::machine::{Machine, copy};
use crate::num::{Num, SlotsOf};
use crate::rung;

/// A host integer a value takes its slot index from and gives it as.
///
/// The host's machine integers of 8 to 64 bits and the two pointer-width ones.
/// Sealed: the door's arithmetic is written for these and no others.
pub trait HostInteger: Machine {}

macro_rules! host_integer {
    ($($t:ty),+ $(,)?) => {
        $( impl HostInteger for $t {} )+
    };
}

host_integer! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize }

/// A host integer at 64 bits: its two's complement pattern, extended by its
/// own sign, and the two facts a 64-bit comparison of the other sign would
/// lose.
#[derive(Clone, Copy)]
struct Wide {
    bits:     u64,
    /// Below zero, which only a signed host integer is.
    negative: bool,
    /// Above the signed 64-bit maximum, which only an unsigned one is.
    above:    bool,
}

const fn signed(x: i64) -> Wide {
    Wide {
        bits:     x as u64,
        negative: x < 0,
        above:    false,
    }
}

const fn unsigned(x: u64) -> Wide {
    Wide {
        bits:     x,
        negative: false,
        above:    x > i64::MAX as u64,
    }
}

/// The host integer widened. One arm survives monomorphisation.
const fn widen<H: HostInteger>(host: H) -> Wide {
    match const { (size_of::<H>(), H::SIGNED) } {
        (1, true) => signed(copy::<H, i8>(host) as i64),
        (1, false) => unsigned(copy::<H, u8>(host) as u64),
        (2, true) => signed(copy::<H, i16>(host) as i64),
        (2, false) => unsigned(copy::<H, u16>(host) as u64),
        (4, true) => signed(copy::<H, i32>(host) as i64),
        (4, false) => unsigned(copy::<H, u32>(host) as u64),
        (8, true) => signed(copy::<H, i64>(host)),
        (8, false) => unsigned(copy::<H, u64>(host)),
        _ => unreachable!(),
    }
}

impl<S: Sole, C: DebugCheck> Num<S, C> {
    /// The member at the slot index a host integer names, completed by the
    /// declared overflow policy where the index is outside the range.
    ///
    /// Equal to `adapted(Exact::on_grid(Slot::at(host)))` for every host value,
    /// and computed at 64 bits. Under the `Abort` check a debug build aborts on
    /// an index outside the range; under `Quiet` no build does.
    #[must_use]
    pub const fn from_index<H: HostInteger>(host: H) -> Self {
        if cfg!(debug_assertions) && const { C::ABORTS.get() } {
            assert!(
                Self::holds_index(host).get(),
                "the host integer names a slot index outside the signature's range, and the `Abort` \
                 check aborts on that in a debug build. Name the `Quiet` check where the completion \
                 the overflow policy gives is what you mean"
            );
        }
        let wide = widen(host);
        let width = const { S::WIDTH.count() };
        // The two cases in which the host's sign and the range's differ are
        // answered before any comparison, since a cast would lose them.
        let index = if const { S::SIGNED.get() } {
            let completed = match S::POLICY {
                Policy::Wrap => rung::adapt_s64(wide.bits as i64, width, Policy::Wrap),
                policy => {
                    let x = if wide.above { i64::MAX } else { wide.bits as i64 };
                    rung::adapt_s64(x, width, policy)
                },
            };
            rung::index_s64(completed)
        } else {
            let completed = match S::POLICY {
                Policy::Wrap => rung::adapt_u64(wide.bits, width, Policy::Wrap),
                policy => {
                    let x = if wide.negative {
                        if cfg!(feature = "mutant-index") { u64::MAX } else { 0 }
                    } else {
                        wide.bits
                    };
                    rung::adapt_u64(x, width, policy)
                },
            };
            rung::index_u64(completed)
        };
        Self::at_index(index)
    }

    /// Whether the slot index a host integer names is a member, so `from_index`
    /// needs no completion. Private: it is what the debug check asks, and no
    /// verdict on a host integer is offered to a consumer.
    const fn holds_index<H: HostInteger>(host: H) -> Bool {
        let () = S::ADMITTED;
        let wide = widen(host);
        let shift = 64 - const { S::WIDTH.count() };
        Bool::of(if const { S::SIGNED.get() } {
            let x = wide.bits as i64;
            !wide.above && x >= i64::MIN >> shift && x <= i64::MAX >> shift
        } else {
            !wide.negative && wide.bits <= u64::MAX >> shift
        })
    }

    /// The member's slot index as the host integer `H`. An unwrap door.
    ///
    /// Refused at build where the signature's slot range is not inside `H`'s,
    /// so it never truncates.
    #[must_use]
    pub const fn index<H: HostInteger>(self) -> H {
        const {
            let bits = 8 * size_of::<H>() as u32;
            let (lowest, highest) = if H::SIGNED {
                (-(1i128 << (bits - 1)), (1i128 << (bits - 1)) - 1)
            } else {
                (0, (1i128 << bits) - 1)
            };
            assert!(
                <SlotsOf<S> as Slots>::MIN.index() >= lowest
                    && <SlotsOf<S> as Slots>::MAX.index() <= highest,
                "`index::<H>()` names a host integer that does not hold every slot index of the \
                 signature's range. Name one that does: a wider one, or one of the range's sign"
            );
        }
        let index = self.slot().index();
        match const { (size_of::<H>(), H::SIGNED) } {
            (1, true) => copy::<i8, H>(index as i8),
            (1, false) => copy::<u8, H>(index as u8),
            (2, true) => copy::<i16, H>(index as i16),
            (2, false) => copy::<u16, H>(index as u16),
            (4, true) => copy::<i32, H>(index as i32),
            (4, false) => copy::<u32, H>(index as u32),
            (8, true) => copy::<i64, H>(index as i64),
            (8, false) => copy::<u64, H>(index as u64),
            _ => unreachable!(),
        }
    }
}

// The door for the host's byte image and its words.

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
