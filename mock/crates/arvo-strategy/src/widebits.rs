//! `WideBits<const BYTES: usize>` byte-sequence storage primitive.
//!
//! Align-1, exact size = BYTES. Used by Warm / Cold / Precise
//! strategies for logical widths above 128 bits, where rustc's primitive
//! ladder ends. The const-generic BYTES is `bytes_for(N)` from
//! `arvo_strategy::width::bytes_for` for a given `Width`.
//!
//! Validated by sketch 02 in
//! `mock/research/sketches/202605031400_hlist_heterogeneous_container/`.
//! `repr(C)` over `[u8; BYTES]` is align-1 across the full BYTES range
//! with no internal padding regardless of N.
//!
//! Hot above 128 uses `AlignedWideBits16` instead (align(16) baseline
//! for SSE2 / NEON friendly storage; AVX-2 / AVX-512 follow-up
//! tracked under #320).
//!
//! Round 202605031400 (#316) introduces `WideBits` to replace the
//! deleted `MultiContainer<HiT, LoT>` heterogeneous pair. Sketch 01
//! verified that `MultiContainer<u64, u128>` was always 32 bytes
//! physical (alignment-of-largest-prim padded). The deletion does
//! not regress storage size for the canonical 129..=255 logical
//! range; WideBits hits the same envelope at align-1.
//!
//! Byte-ordering convention (per audit C1): bit 0 is the LSB of
//! byte 0. `trailing_zeros` walks from byte 0 upward;
//! `leading_zeros` walks from byte BYTES-1 downward. Mirrors
//! `MultiContainer`'s lo-first cascade and matches `u128::trailing_zeros`
//! semantics where bit 0 is the low bit. The BitPrim impl ships in
//! `arvo-bits-contracts::widebits` and composes byte-by-byte through
//! this storage shape.

use arvo_transparent::Transparent;

/// Byte-sequence storage primitive, align-1, exact size BYTES.
///
/// Used by Warm / Cold / Precise strategies for N > 128.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct WideBits<const BYTES: usize> {
    /// Little-endian byte sequence. Bit 0 of the logical value is the
    /// LSB of `bytes[0]` per the byte-ordering convention.
    pub bytes: [u8; BYTES],
}

impl<const BYTES: usize> WideBits<BYTES> {
    /// Zero value: all bytes zero.
    #[inline(always)]
    pub const fn zero() -> Self {
        Self {
            bytes: [0u8; BYTES],
        }
    }

    /// Borrow the underlying byte sequence.
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; BYTES] {
        &self.bytes
    }

    /// Construct from a byte sequence.
    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; BYTES]) -> Self {
        Self { bytes }
    }
}

// SAFETY: `repr(C)` over `[u8; BYTES]`. Layout-identical by Rust spec
// (single non-ZST field; align is 1; size is BYTES).
unsafe impl<const BYTES: usize> const Transparent for WideBits<BYTES> {
    type Inner = [u8; BYTES];
}
