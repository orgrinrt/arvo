//! `WideBits<const BYTES: usize, A: Align = A1>` byte-sequence storage
//! primitive, parametric over alignment.
//!
//! Used by every Strategy for logical widths above the native
//! primitive ladder (above 128 bits). Alignment is selected per
//! Strategy by the wide-bucket projection in `container.rs`:
//!
//! - `Warm` / `Cold` / `Precise` use `WideBits<BYTES, A1>` (align-1
//!   byte-exact, no padding).
//! - `Hot` baseline uses `WideBits<BYTES, A16>` (SSE2 / NEON
//!   16-byte aligned, ubiquitous on x86_64 and aarch64).
//!
//! The alignment markers `A1` / `A16` / `A32` / `A64` cover the four
//! SIMD tiers we ship today. Wider markers (`A128` for AVX-512BW
//! variants, etc.) plug in by adding one ZST struct + `Align` impl;
//! no new container types are needed. `#320` lands the AVX-2 (`A32`)
//! and AVX-512 (`A64`) Hot variants behind cfg gates and consumer
//! opt-in.
//!
//! Validated by sketches 02 + 03. `repr(C)` over `[u8; BYTES]` with a
//! zero-sized alignment marker `[A; 0]` field gives align-of-A across
//! the full BYTES range without internal padding (the marker has zero
//! size; the byte array follows immediately and the struct's
//! alignment is `max(align_of::<A>(), align_of::<u8>()) = align_of::<A>()`).
//!
//! Round 202605031400 (#316) replaces the deleted
//! `MultiContainer<HiT, LoT>` heterogeneous pair. Sketch 01 verified
//! `MultiContainer<u64, u128>` was always 32 bytes physical
//! (alignment-of-largest-prim padded). Deletion does not regress
//! storage size for the canonical 129..=255 range; `WideBits<BYTES, A16>`
//! hits the same envelope at align-16 with explicit (not implicit)
//! intent.
//!
//! Per audit L2: when `BYTES` is not a multiple of `A::VALUE`, the
//! struct's physical size is padded to `round_up(BYTES, A::VALUE)`
//! but the trailing pad bytes after `[u8; BYTES]` are NOT part of
//! the safe field surface. Reading the full struct via raw-pointer
//! cast or `core::mem::transmute` is undefined behavior. Only
//! `as_bytes()` is safe.
//!
//! Byte-ordering convention (per audit C1): bit 0 is the LSB of
//! byte 0. `trailing_zeros` walks from byte 0 upward;
//! `leading_zeros` walks from byte BYTES-1 downward. Mirrors
//! `MultiContainer`'s lo-first cascade and matches `u128::trailing_zeros`
//! semantics.

use arvo_transparent::{ConstAsRef, ConstDeref, Transparent};

/// Alignment marker trait. Implementations are zero-sized types
/// whose `repr(C, align(N))` gives the desired alignment when
/// embedded as `[A; 0]` in a `WideBits<BYTES, A>`.
pub trait Align: Copy {
    /// The alignment in bytes.
    const VALUE: usize;
}

/// Align-1 marker. Default for Warm / Cold / Precise wide storage.
#[repr(C, align(1))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub struct A1;

impl Align for A1 {
    const VALUE: usize = 1;
}

/// Align-16 marker. Hot baseline (SSE2 / NEON).
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub struct A16;

impl Align for A16 {
    const VALUE: usize = 16;
}

/// Align-32 marker. Hot AVX-2 tier (lands via #320 cfg gate).
#[repr(C, align(32))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub struct A32;

impl Align for A32 {
    const VALUE: usize = 32;
}

/// Align-64 marker. Hot AVX-512 tier (lands via #320 cfg gate).
#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Hash)]
pub struct A64;

impl Align for A64 {
    const VALUE: usize = 64;
}

/// Byte-sequence storage, parametric over alignment.
///
/// Default alignment is `A1` (align-1, byte-exact). Pass `A16` /
/// `A32` / `A64` to opt into SIMD-aligned storage. The struct's
/// physical alignment is the alignment of `A`; the byte array
/// follows immediately with no internal padding.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct WideBits<const BYTES: usize, A: Align = A1> {
    /// Zero-sized alignment marker. Determines the struct's
    /// alignment; takes no space.
    _align: [A; 0],
    /// Little-endian byte sequence. Bit 0 of the logical value is
    /// the LSB of `bytes[0]` per the byte-ordering convention.
    pub bytes: [u8; BYTES],
}

impl<const BYTES: usize, A: Align> WideBits<BYTES, A> {
    /// Zero value: all bytes zero.
    #[inline(always)]
    pub const fn zero() -> Self {
        Self {
            _align: [],
            bytes: [0u8; BYTES],
        }
    }

    /// Construct from a byte sequence.
    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; BYTES]) -> Self {
        Self { _align: [], bytes }
    }

    /// Borrow the underlying byte sequence.
    ///
    /// Only the initialized prefix `[u8; BYTES]`. Trailing pad bytes
    /// (when `BYTES` is not a multiple of `A::VALUE`) are not
    /// reachable through this accessor and must not be read by any
    /// other means. Per audit L2.
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; BYTES] {
        &self.bytes
    }
}

// SAFETY: `repr(C)` over `([A; 0], [u8; BYTES])`. The marker `[A; 0]`
// has size 0 (zero-element array) and alignment `A::VALUE`; the byte
// array follows immediately. The struct's layout is identical to
// `[u8; BYTES]` with the alignment lifted to `A::VALUE`. The single
// non-ZST field is the byte array.
unsafe impl<const BYTES: usize, A: Align + 'static> const Transparent for WideBits<BYTES, A> {
    type Inner = [u8; BYTES];
}

impl<const BYTES: usize, A: Align> Default for WideBits<BYTES, A> {
    #[inline(always)]
    fn default() -> Self {
        Self::zero()
    }
}

/// Canonical `ConstDeref` impl for `WideBits<BYTES, A>`.
///
/// Round 4 (#314) follow-up: ships the bridge family with at least one
/// canonical impl per trait so the trait shape is exercised. Mirrors
/// `core::ops::Deref` for the const-callable path; consumers reach for
/// the underlying byte sequence in const context via `const_deref()`.
impl<const BYTES: usize, A: Align> const ConstDeref for WideBits<BYTES, A> {
    type Target = [u8; BYTES];
    #[inline(always)]
    fn const_deref(&self) -> &Self::Target {
        &self.bytes
    }
}

/// Canonical `ConstAsRef<[u8; BYTES]>` impl for `WideBits<BYTES, A>`.
///
/// Same shape as the `ConstDeref` impl above; the as-ref form is the
/// idiomatic choice when consumer code carries a `&[u8; BYTES]` bound
/// rather than relying on auto-deref.
impl<const BYTES: usize, A: Align> const ConstAsRef<[u8; BYTES]> for WideBits<BYTES, A> {
    #[inline(always)]
    fn const_as_ref(&self) -> &[u8; BYTES] {
        &self.bytes
    }
}
