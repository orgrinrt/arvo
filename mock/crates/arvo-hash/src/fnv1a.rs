//! FNV-1a-64 streaming hasher with N-bit output.
//!
//! `Fnv1a<const N: u8>` wraps the `fnv1a_64` algorithm and projects
//! its 64-bit state into the requested width via `Bits::from_raw_u64`.
//!
//! `N` is `u8` directly rather than the `Width` meta-newtype for the
//! same reason `Hasher<const N: u8>` does (see `algo.rs`): nested
//! const-fn evaluation at trait-bound resolution is unreliable on
//! current nightly.
//!
//! Width is constrained to `1..=64` implicitly by `Hot:
//! UContainerFor<N>`: that impl exists only for those Ns, so a
//! consumer using `Fnv1a<N>` outside that range gets a missing-impl
//! error pointing at strategy choice. Wider widths (FNV state >= 128
//! bits) are tracked in `BACKLOG.md` as a separate `Fnv1a128` type.
//! The per-N `Hasher<N> for Fnv1a<N>` impls below cover N=1..=64
//! exhaustively, mirroring `Bits<N, Hot>::from_raw_u64`'s per-N
//! macro expansion in `arvo::bits`.

use crate::Hasher;
use crate::algo::fnv1a_64;
use arvo::strategy::UContainerFor;
use arvo::{Bits, Hot};

/// Streaming FNV-1a-64 hasher with N-bit output.
///
/// `N` must satisfy `1 <= N <= 64`. Wider widths require a different
/// state width (`Fnv1a128`, deferred).
///
/// ```ignore
/// use arvo_hash::{Fnv1a, HasherExt};
/// let h: arvo::Bits<28, arvo::Hot> = Fnv1a::<28>::new().hash(b"hello");
/// ```
pub struct Fnv1a<const N: u8>
where
    Hot: UContainerFor<N>,
{
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a-64 internal state width is fixed by the algorithm specification; tracked: #256
    state: u64,
}

impl<const N: u8> Fnv1a<N>
where
    Hot: UContainerFor<N>,
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

impl<const N: u8> Default for Fnv1a<N>
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
/// Generated for `N` in `1..=64`. The per-N expansion is necessary
/// because `Bits::<N, Hot>::from_raw_u64` is itself defined per-N
/// (see `arvo::bits::impl_bits_u64`); generic-over-N callers cannot
/// see it without a helper trait, which would just push the per-N
/// expansion one layer deeper.
macro_rules! impl_fnv1a {
    ($($n:literal),+ $(,)?) => {
        $(
            impl Fnv1a<$n> {
                /// Compile-time hash construction.
                ///
                /// Equivalent to `Fnv1a::new().hash(bytes)` but
                /// callable from `const` context.
                #[inline]
                pub const fn hash_const(bytes: &[u8]) -> Bits<$n, Hot> {
                    // lint:allow(no-bare-numeric) reason: FNV state width is u64 by algorithm spec; tracked: #256
                    let raw: u64 = fnv1a_64(bytes);
                    Bits::<$n, Hot>::from_raw_u64(raw)
                }
            }

            impl Hasher<$n> for Fnv1a<$n> {
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
                fn finalize(self) -> Bits<$n, Hot> {
                    Bits::<$n, Hot>::from_raw_u64(self.state)
                }
            }
        )+
    };
}

impl_fnv1a!(1, 2, 3, 4, 5, 6, 7, 8);
impl_fnv1a!(9, 10, 11, 12, 13, 14, 15, 16);
impl_fnv1a!(17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
#[rustfmt::skip]
impl_fnv1a!(
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
