//! XxHash3-64 streaming hasher with N-bit output.
//!
//! `XxHash3<const N: u16>` wraps the `xxhash3_64` algorithm and projects
//! its 64-bit state into the requested width via `NarrowFromU64<N, S, Sign>`
//! (declared in arvo-bits-contracts). Same shape as `Fnv1a<N>` post
//! round 4 (#314): one bounded-generic `Hasher<N>` impl plus one
//! bounded-generic `ConstHash<N, Hot, Unsigned>` impl per algorithm.
//!
//! Round 202604281000 Pass B.1 ships XxHash3 as the new default hash
//! family. FNV-1a remains as `Fnv1a<N>` for known-good fits where its
//! 8-16 byte band performance edge holds.
//!
//! Width is constrained to `1..=64` implicitly by `Hot: BitsContainerFor<N, Unsigned>`.
//! Wider widths (XxHash3-128) are tracked as a future-round concern.

use crate::{ConstHash, Hasher};
use arvo::strategy::{BitsContainerFor, Unsigned};
use arvo::{Bits, Hot};
use arvo_bits_contracts::NarrowFromU64;
use xxhash_rust::const_xxh3::xxh3_64;

/// XxHash3-64 over a byte slice (free const fn).
///
/// Returns the raw 64-bit state. Concrete `Hasher<N>` / `ConstHash<N, S, Sign>`
/// implementors mask to N bits via `NarrowFromU64`. The `&[u8]`
/// parameter is the boundary input from raw bytes; the `u64` return
/// is the algorithm's state-width.
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
/// use arvo_hash::{XxHash3, ConstHash};
/// /// use arvo::strategy::Unsigned;
///
/// let h: arvo::Bits<64, Hot> =
///     <XxHash3<64> as ConstHash<64, Hot, Unsigned>>::hash_const(b"hello");
/// ```
///
/// The streaming impl buffers bytes and computes the hash on
/// `finalize()`. xxhash-rust's streaming Xxh3 is alloc-using; the
/// const-friendly API is one-shot only. For hot-path streaming, use
/// `ConstHash::hash_const` or accumulate into a single `&[u8]` before
/// calling `update` once.
pub struct XxHash3<const N: u16>
where
    Hot: BitsContainerFor<N, Unsigned>,
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

impl<const N: u16> XxHash3<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
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

impl<const N: u16> Default for XxHash3<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming `Hasher<N>` impl. Single bounded-generic block replaces
/// the prior 64-impl macro paste.
impl<const N: u16> Hasher<N> for XxHash3<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
    <Hot as BitsContainerFor<N, Unsigned>>::T: NarrowFromU64<N, Hot, Unsigned>,
{
    #[inline]
    fn update(&mut self, bytes: &[u8]) {
        // Buffer overflow is a contract violation (the streaming surface
        // bounds at 256 bytes by design; consumers above that band reach
        // for `ConstHash::hash_const`). Visible in debug, silent in
        // release to match the no-overhead substrate philosophy.
        debug_assert!(
            self.pos + bytes.len() <= self.buffer.len(),
            "XxHash3<N>::update exceeds 256-byte buffer; use ConstHash::hash_const for the full byte slice"
        );
        // lint:allow(no-bare-numeric) reason: streaming-buffer copy; bounded by 256-byte stack buffer; tracked: #259
        let mut i = 0;
        while i < bytes.len() && self.pos < self.buffer.len() {
            self.buffer[self.pos] = bytes[i];
            self.pos += 1;
            i += 1;
        }
    }

    #[inline]
    fn finalize(self) -> Bits<N, Hot> {
        let raw_u64 = xxhash3_64(&self.buffer[..self.pos]);
        let raw = <<Hot as BitsContainerFor<N, Unsigned>>::T as NarrowFromU64<
            N,
            Hot,
            Unsigned,
        >>::narrow_u64(raw_u64);
        Bits::<N, Hot>::from_raw(raw)
    }
}

/// One-shot `ConstHash<N, Hot, Unsigned>` impl. Const-callable.
///
/// xxhash-rust's `xxh3_64` is const-callable, so this trait impl
/// composes through `NarrowFromU64` cleanly without per-N
/// specialisation.
const impl<const N: u16> ConstHash<N, Hot, Unsigned> for XxHash3<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
    <Hot as BitsContainerFor<N, Unsigned>>::T: [const] NarrowFromU64<N, Hot, Unsigned>,
{
    #[inline]
    fn hash_const(bytes: &[u8]) -> Bits<N, Hot, Unsigned> {
        let raw_u64 = xxhash3_64(bytes);
        let raw = <<Hot as BitsContainerFor<N, Unsigned>>::T as NarrowFromU64<
            N,
            Hot,
            Unsigned,
        >>::narrow_u64(raw_u64);
        Bits::<N, Hot, Unsigned>::from_raw(raw)
    }
}
