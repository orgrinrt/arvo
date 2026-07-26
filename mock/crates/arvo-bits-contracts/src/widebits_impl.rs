//! `BitPrim` impl on `WideBits<BYTES, A>`.
//!
//! Per audit Finding 2 (architectural-dogfooding 2026-05-02): the
//! `BitsContainerFor<N, Sign>` projection routes N > 128 to wide
//! storage shapes (`WideBits<BYTES, A1>` for Warm/Cold/Precise,
//! `WideBits<BYTES, A16>` for Hot baseline). The blanket impls of
//! `BitAccess` / `BitSequence` / `BitLogic` on `Bits<N, S, Sign>`
//! bound the container on `[const] BitsBitPrim<Sign>`, which in turn
//! reaches `BitPrim` (Unsigned) / `IBitPrim` (Signed) via blanket
//! bridges. Without this impl, `Bits<256, Hot, Unsigned>` is
//! constructible but exposes no bit-level surface; the `Mask<W>`
//! chassis collapse for N=256 (Round 3) is unreachable.
//!
//! Round 202605031748 (#313) lands the impl. Body composes byte-by-
//! byte over `[u8; BYTES]`. Single-pass scans for `count_ones` /
//! `trailing_zeros` / `leading_zeros`. Element-wise byte ops for
//! the bitwise family.
//!
//! Byte-ordering convention (per audit C1, locked in WideBits docs):
//! bit 0 is the LSB of `bytes[0]`. `trailing_zeros` walks from byte 0
//! upward; `leading_zeros` walks from byte BYTES-1 downward. Matches
//! `u128::trailing_zeros` semantics over the same bit-significance
//! order.

use arvo_storage::{Bool, USize};
use arvo_strategy::{Align, WideBits};
use arvo_transparent::Transparent;

use crate::{sealed, BitPrim};

// SAFETY: WideBits is the byte-sequence storage primitive at the wide
// bucket of BitsContainerFor. Adding it to the sealed Bit marker
// extends the sealed BitPrim implementor set; the orphan rule is
// satisfied because BitPrim and sealed::Bit live in arvo-bits-contracts.
impl<const BYTES: usize, A: Align + 'static> sealed::Bit for WideBits<BYTES, A> {}

// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: WideBits inner storage is bare-byte by construction (see WideBits docs); this BitPrim impl bridges the typed surface to the byte-level op composition; tracked: #313
const impl<const BYTES: usize, A: Align + 'static> BitPrim for WideBits<BYTES, A> {
    const WIDTH: USize = USize(BYTES * 8);
    const ZERO: Self = Self::zero();
    const ONE: Self = {
        let mut bytes = [0u8; BYTES];
        if BYTES > 0 {
            bytes[0] = 1;
        }
        Self::from_bytes(bytes)
    };

    #[inline]
    fn count_ones(self) -> USize {
        let mut sum: usize = 0;
        let mut i = 0;
        while i < BYTES {
            sum += self.bytes[i].count_ones() as usize;
            i += 1;
        }
        USize(sum)
    }

    #[inline]
    fn trailing_zeros(self) -> USize {
        let mut i = 0;
        while i < BYTES {
            if self.bytes[i] != 0 {
                return USize(i * 8 + self.bytes[i].trailing_zeros() as usize);
            }
            i += 1;
        }
        USize(BYTES * 8)
    }

    #[inline]
    fn leading_zeros(self) -> USize {
        let mut i = BYTES;
        while i > 0 {
            i -= 1;
            if self.bytes[i] != 0 {
                return USize((BYTES - 1 - i) * 8 + self.bytes[i].leading_zeros() as usize);
            }
        }
        USize(BYTES * 8)
    }

    #[inline]
    fn get_bit(self, idx: USize) -> Bool {
        let i = <USize as Transparent>::raw(idx);
        if i >= BYTES * 8 {
            return Bool(false);
        }
        let byte = i / 8;
        let bit = i % 8;
        Bool((self.bytes[byte] >> bit) & 1 == 1)
    }

    #[inline]
    fn with_bit_set(self, idx: USize) -> Self {
        let i = <USize as Transparent>::raw(idx);
        if i >= BYTES * 8 {
            return self;
        }
        let byte = i / 8;
        let bit = i % 8;
        let mut bytes = self.bytes;
        bytes[byte] |= 1u8 << bit;
        Self::from_bytes(bytes)
    }

    #[inline]
    fn with_bit_cleared(self, idx: USize) -> Self {
        let i = <USize as Transparent>::raw(idx);
        if i >= BYTES * 8 {
            return self;
        }
        let byte = i / 8;
        let bit = i % 8;
        let mut bytes = self.bytes;
        bytes[byte] &= !(1u8 << bit);
        Self::from_bytes(bytes)
    }

    #[inline]
    fn with_bit_toggled(self, idx: USize) -> Self {
        let i = <USize as Transparent>::raw(idx);
        if i >= BYTES * 8 {
            return self;
        }
        let byte = i / 8;
        let bit = i % 8;
        let mut bytes = self.bytes;
        bytes[byte] ^= 1u8 << bit;
        Self::from_bytes(bytes)
    }

    #[inline]
    fn bitor(self, other: Self) -> Self {
        let mut bytes = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            bytes[i] = self.bytes[i] | other.bytes[i];
            i += 1;
        }
        Self::from_bytes(bytes)
    }

    #[inline]
    fn bitand(self, other: Self) -> Self {
        let mut bytes = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            bytes[i] = self.bytes[i] & other.bytes[i];
            i += 1;
        }
        Self::from_bytes(bytes)
    }

    #[inline]
    fn bitnot(self) -> Self {
        let mut bytes = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            bytes[i] = !self.bytes[i];
            i += 1;
        }
        Self::from_bytes(bytes)
    }

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        let mut bytes = [0u8; BYTES];
        let mut i = 0;
        while i < BYTES {
            bytes[i] = self.bytes[i] ^ other.bytes[i];
            i += 1;
        }
        Self::from_bytes(bytes)
    }

    #[inline]
    fn clear_lowest_set_bit(self) -> Self {
        let mut bytes = self.bytes;
        let mut i = 0;
        while i < BYTES {
            if bytes[i] != 0 {
                bytes[i] = bytes[i] & bytes[i].wrapping_sub(1);
                return Self::from_bytes(bytes);
            }
            i += 1;
        }
        self
    }

    #[inline]
    fn is_zero(self) -> Bool {
        let mut i = 0;
        while i < BYTES {
            if self.bytes[i] != 0 {
                return Bool(false);
            }
            i += 1;
        }
        Bool(true)
    }

    #[inline]
    fn mask_low(n: USize) -> Self {
        let n_bits = <USize as Transparent>::raw(n);
        let total_bits = BYTES * 8;
        let mut bytes = [0u8; BYTES];
        if n_bits == 0 {
            return Self::from_bytes(bytes);
        }
        let n_bits = if n_bits >= total_bits {
            total_bits
        } else {
            n_bits
        };
        let full_bytes = n_bits / 8;
        let rem_bits = n_bits % 8;
        let mut i = 0;
        while i < full_bytes {
            bytes[i] = 0xFFu8;
            i += 1;
        }
        if rem_bits > 0 && i < BYTES {
            bytes[i] = (1u8 << rem_bits) - 1;
        }
        Self::from_bytes(bytes)
    }
}
