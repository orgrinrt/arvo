//! `AlignedWideBits16<const BYTES: usize>` byte-sequence storage at
//! align(16).
//!
//! `repr(C, align(16))` over `[u8; BYTES]`. Size is rounded up to a
//! multiple of 16; align is 16. Used by `Hot` strategy for logical
//! widths above 128 bits, where rustc's primitive ladder ends.
//!
//! 16-byte alignment is the SSE2 / NEON SIMD baseline. Wider tiers
//! (align(32) for AVX-2, align(64) for AVX-512) are deferred to #320
//! per audit H1 (bench-driven justification required before exposing
//! consumer-visible strategy variants).
//!
//! Validated by sketch 03 in
//! `mock/research/sketches/202605031400_hlist_heterogeneous_container/`.
//!
//! ## Trailing pad bytes are uninitialized
//!
//! Per audit L2: when BYTES is not a multiple of 16, the struct's
//! physical size is padded to `round_up(BYTES, 16)` but the trailing
//! pad bytes after `[u8; BYTES]` are NOT part of the safe field
//! surface. Reading the full struct via raw-pointer cast or
//! `core::mem::transmute` is undefined behavior. Only `as_bytes()`
//! is safe; it returns `&[u8; BYTES]` covering the initialized
//! region.
//!
//! BitPrim ops compose byte-by-byte through `as_bytes()`; the
//! alignment is purely for SIMD-friendly addressing of the
//! initialized prefix.

use arvo_transparent::Transparent;

/// Byte-sequence storage at align(16), used by `Hot` for N > 128.
///
/// Physical size is `round_up(BYTES, 16)` due to alignment padding.
/// Trailing pad bytes are uninitialized; only `as_bytes()` is safe.
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AlignedWideBits16<const BYTES: usize> {
    /// Little-endian byte sequence. Bit 0 of the logical value is the
    /// LSB of `bytes[0]` per the byte-ordering convention.
    pub bytes: [u8; BYTES],
}

impl<const BYTES: usize> AlignedWideBits16<BYTES> {
    /// Zero value: all bytes zero.
    #[inline(always)]
    pub const fn zero() -> Self {
        Self {
            bytes: [0u8; BYTES],
        }
    }

    /// Borrow the underlying byte sequence.
    ///
    /// Only the initialized prefix; trailing pad bytes are not
    /// reachable through this accessor (and must not be read by any
    /// other means).
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

// SAFETY: `repr(C, align(16))` over `[u8; BYTES]`. The single non-ZST
// field is the byte array; alignment overrides the natural align of 1
// to 16. Layout is well-defined per Rust spec; the inner type's size
// matches the field size. Trailing pad bytes are uninitialized but
// the Transparent contract concerns the field type, not the physical
// padding.
unsafe impl<const BYTES: usize> const Transparent for AlignedWideBits16<BYTES> {
    type Inner = [u8; BYTES];
}
