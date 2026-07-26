//! FNV-1a-64 streaming hasher with N-bit output.
//!
//! `Fnv1a<const N: u16>` wraps the `fnv1a_64` algorithm and projects
//! its 64-bit state into the requested width. The width-dispatched
//! narrowing step lifts to `NarrowFromU64<N, S, Sign>` (declared in
//! arvo-bits-contracts); this module ships one bounded-generic
//! `Hasher<N>` impl plus one bounded-generic `ConstHash<N, Hot, Unsigned>`
//! impl per algorithm, replacing the prior 64-impl macro paste.
//!
//! Width is constrained to `1..=64` implicitly by `Hot:
//! BitsContainerFor<N, Unsigned>` plus FNV-1a-64's 64-bit state. Wider
//! widths (FNV state >= 128 bits) are tracked in `BACKLOG.md` as a
//! separate `Fnv1a128` type.

use crate::algo::fnv1a_64;
use crate::{ConstHash, Hasher};
use arvo::strategy::{BitsContainerFor, Unsigned};
use arvo::{Bits, Hot};
use arvo_bits_contracts::NarrowFromU64;

/// Streaming FNV-1a-64 hasher with N-bit output.
///
/// `N` must satisfy `1 <= N <= 64`. Wider widths require a different
/// state width (`Fnv1a128`, deferred).
///
/// ```ignore
/// use arvo_hash::{Fnv1a, ConstHash};
/// /// use arvo::strategy::Unsigned;
///
/// let h: arvo::Bits<28, Hot> =
///     <Fnv1a<28> as ConstHash<28, Hot, Unsigned>>::hash_const(b"hello");
/// ```
pub struct Fnv1a<const N: u16>
where
    Hot: BitsContainerFor<N, Unsigned>,
{
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a-64 internal state width is fixed by the algorithm specification; tracked: #256
    state: u64,
}

impl<const N: u16> Fnv1a<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
{
    /// FNV-1a-64 offset basis, the algorithm's initial state value.
    // lint:allow(no-bare-numeric) reason: FNV offset basis; algorithm-fixed constant; tracked: #256
    const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

    /// FNV-1a-64 multiplicative prime.
    // lint:allow(no-bare-numeric) reason: FNV prime; algorithm-fixed constant; tracked: #256
    const PRIME: u64 = 0x100_0000_01b3;

    /// Construct a fresh hasher in its initial state.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            state: Self::OFFSET_BASIS,
        }
    }
}

impl<const N: u16> Default for Fnv1a<N>
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
///
/// `update` runs the FNV-1a round byte-by-byte; `finalize` narrows the
/// 64-bit state to N bits via `NarrowFromU64`.
impl<const N: u16> Hasher<N> for Fnv1a<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
    <Hot as BitsContainerFor<N, Unsigned>>::T: NarrowFromU64<N, Hot, Unsigned>,
{
    #[inline]
    fn update(&mut self, bytes: &[u8]) {
        let mut i = 0;
        while i < bytes.len() {
            // lint:allow(no-bare-numeric) reason: FNV-1a-64 round; algorithm-fixed u8/u64 arithmetic; tracked: #256
            self.state ^= bytes[i] as u64;
            self.state = self.state.wrapping_mul(Self::PRIME);
            i += 1;
        }
    }

    #[inline]
    fn finalize(self) -> Bits<N, Hot> {
        let raw = <<Hot as BitsContainerFor<N, Unsigned>>::T as NarrowFromU64<
            N,
            Hot,
            Unsigned,
        >>::narrow_u64(self.state);
        Bits::<N, Hot>::from_raw(raw)
    }
}

/// One-shot `ConstHash<N, Hot, Unsigned>` impl. Const-callable;
/// equivalent to `Fnv1a::new()`, then `update(bytes)`, then `finalize()`.
///
/// Replaces the prior per-N inherent `hash_const` method pattern; the
/// trait identity is now focused on hash-as-const-construction rather
/// than bolted onto every per-N implementor as inherents.
const impl<const N: u16> ConstHash<N, Hot, Unsigned> for Fnv1a<N>
where
    Hot: BitsContainerFor<N, Unsigned>,
    <Hot as BitsContainerFor<N, Unsigned>>::T: [const] NarrowFromU64<N, Hot, Unsigned>,
{
    #[inline]
    fn hash_const(bytes: &[u8]) -> Bits<N, Hot, Unsigned> {
        let raw_u64 = fnv1a_64(bytes);
        let raw = <<Hot as BitsContainerFor<N, Unsigned>>::T as NarrowFromU64<
            N,
            Hot,
            Unsigned,
        >>::narrow_u64(raw_u64);
        Bits::<N, Hot, Unsigned>::from_raw(raw)
    }
}
