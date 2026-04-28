//! XxHash3-64 streaming hasher with N-bit output.
//!
//! `XxHash3<const N: u8>` wraps the `xxhash3_64` algorithm and projects
//! its 64-bit state into the requested width via a per-N mask plus
//! `as` cast to the dispatched `<Hot as UContainerFor<N>>::T` container,
//! then `Bits::from_raw`. Same shape as `Fnv1a<N>`; consumer-facing
//! ergonomics are identical.
//!
//! Round 202604281000 Pass B.1: ships XxHash3 as the new default hash
//! family. FNV-1a remains as `Fnv1a<N>` for known-good fits where its
//! 8-16 byte band performance edge holds.
//!
//! Width is constrained to `1..=64` implicitly by `Hot: UContainerFor<N>`.
//! Wider widths (XxHash3-128) are tracked as a future-round concern.

use crate::Hasher;
use arvo::strategy::UContainerFor;
use arvo::{Bits, Hot};
use xxhash_rust::const_xxh3::xxh3_64;

/// XxHash3-64 over a byte slice (free const fn).
///
/// Returns the raw 64-bit state. `XxHash3<N>::hash_const` masks to N
/// bits via `Bits::<N, Hot>::from_raw`. The `&[u8]` parameter is the
/// boundary input from raw bytes; the `u64` return is the algorithm's
/// state-width.
// lint:allow(no-bare-numeric) reason: xxhash3 state is u64 by algorithm spec; mirrors fnv1a_64; tracked: #259
pub const fn xxhash3_64(bytes: &[u8]) -> u64 {
    xxh3_64(bytes)
}

/// Streaming XxHash3-64 hasher with N-bit output.
///
/// Default hash family for the substrate's content-addressing
/// workload. `N` must satisfy `1 <= N <= 64`. Wider widths require a
/// different state width (`XxHash3_128`, deferred).
///
/// ```ignore
/// use arvo_hash::{XxHash3, HasherExt};
/// let h: arvo::Bits<64, arvo::Hot> = XxHash3::<64>::new().hash(b"hello");
/// ```
///
/// The streaming impl buffers bytes and computes the hash on
/// `finalize()`. xxhash-rust's streaming Xxh3 is alloc-using; the
/// const-friendly API is one-shot only. For hot-path streaming, use
/// the `hash_const` entry point or accumulate into a single `&[u8]`
/// before calling `update` once.
pub struct XxHash3<const N: u8>
where
    Hot: UContainerFor<N>,
{
    /// Buffered bytes. Bounded by stack-allocated array; size matches
    /// the substrate's typical content-addressing payload (16-128
    /// bytes covers the documented consumer band per Pass B.1 design).
    /// Beyond this, consumers chunk.
    // lint:allow(no-bare-numeric) reason: streaming buffer is bare-byte algorithm internal; tracked: #259
    buffer: [u8; 256],
    // lint:allow(no-bare-numeric) reason: usize-shaped position cursor for the bare-byte buffer; tracked: #259
    pos: usize,
}

impl<const N: u8> XxHash3<N>
where
    Hot: UContainerFor<N>,
{
    /// Construct a fresh hasher in its initial state.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            // lint:allow(no-bare-numeric) reason: zero-init of streaming buffer; tracked: #259
            buffer: [0u8; 256],
            pos: 0,
        }
    }
}

impl<const N: u8> Default for XxHash3<N>
where
    Hot: UContainerFor<N>,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Per-N `Hasher<N>` impls plus per-N `hash_const` inherents.
///
/// Generated for `N` in `1..=64`, mirroring the `Fnv1a` table shape.
macro_rules! impl_xxhash3 {
    ($ty:ty, $($n:literal),+ $(,)?) => {
        $(
            impl XxHash3<$n> {
                /// Compile-time hash construction.
                #[inline]
                pub const fn hash_const(bytes: &[u8]) -> Bits<$n, Hot> {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: xxh3 state is u64 by algorithm spec; mask + container cast per D-7; tracked: #259
                    let raw: u64 = xxhash3_64(bytes);
                    let mask: u64 = if $n == 64 { u64::MAX } else { (1u64 << $n) - 1 };
                    Bits::<$n, Hot>::from_raw((raw & mask) as $ty)
                }
            }

            impl Hasher<$n> for XxHash3<$n> {
                #[inline]
                fn update(&mut self, bytes: &[u8]) {
                    // lint:allow(no-bare-numeric) reason: streaming-buffer copy; bounded by 256-byte stack buffer; tracked: #259
                    let mut i = 0;
                    while i < bytes.len() && self.pos < self.buffer.len() {
                        self.buffer[self.pos] = bytes[i];
                        self.pos += 1;
                        i += 1;
                    }
                }

                #[inline]
                fn finalize(self) -> Bits<$n, Hot> {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: hash buffered slice + mask to N bits; tracked: #259
                    let raw: u64 = xxhash3_64(&self.buffer[..self.pos]);
                    let mask: u64 = if $n == 64 { u64::MAX } else { (1u64 << $n) - 1 };
                    Bits::<$n, Hot>::from_raw((raw & mask) as $ty)
                }
            }
        )+
    };
}

// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-size-class container dispatch table; tracked: #259
impl_xxhash3!(u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_xxhash3!(u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_xxhash3!(u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
#[rustfmt::skip]
impl_xxhash3!(
    u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
