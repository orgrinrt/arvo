//! Hash algorithm contracts + free FNV-1a-64 helper.
//!
//! Two trait surfaces:
//!
//! - `Hasher<N>`: streaming. Feed bytes via `update`; finalise to a
//!   `Bits<N, Hot>`.
//! - `ConstHash<N, S, Sign>`: one-shot, const-callable. Produces a
//!   typed `Bits<N, S, Sign>` from a byte slice in const context.
//!
//! `ConstHash` replaces the prior per-N `hash_const` inherent method
//! pattern (round 4 / #314). `HasherExt` is removed; one-shot
//! ergonomics ride on `ConstHash`. Streaming and one-shot are now
//! independent trait surfaces; consumers pick the one that fits.

use arvo::strategy::{BitsContainerFor, Signed, Signedness, Strategy, Unsigned};
use arvo::{Bits, Hot};
use arvo_bits_contracts::NarrowFromU64;

/// Streaming N-bit hasher. Feed bytes via `update`, finalise to a
/// `Bits<N, Hot>`.
///
/// Algorithms implement this trait. The bounded-generic `impl<const N: u16>
/// Hasher<N>` per algorithm replaces the prior 64-impl macro paste:
/// the width-dispatched narrowing step lifts to `NarrowFromU64<N, S, Sign>`
/// (declared in `arvo-bits-contracts`).
///
/// `N` is `u16` directly (matches the substrate cap; round 202605031400
/// removed the `Width` newtype layer).
pub trait Hasher<const N: u16>
where
    Hot: BitsContainerFor<N, Unsigned>,
{
    /// Feed a byte chunk into the hasher.
    fn update(&mut self, bytes: &[u8]);

    /// Consume the hasher and produce the final N-bit digest.
    fn finalize(self) -> Bits<N, Hot>;
}

/// Compile-time, one-shot hash construction.
///
/// Produces a typed `Bits<N, S, Sign>` from a byte slice in const
/// context. Mirrors the strategy-aware `BitsContainerFor<N, Sign>`
/// cascade.
///
/// The `[const]` host-effect bound on the supertrait constraint is
/// required for cross-crate const dispatch. Sketch 02 (round
/// 202605031548) validates the chain `ConstHash → NarrowFromU64
/// → BitsContainerFor → Project` resolves at both compile time and
/// runtime in a downstream crate.
///
/// Algorithms implement once each, generic over `N`. Consumer code
/// reaches for `<Fnv1a<N> as ConstHash<N, Hot, Unsigned>>::hash_const(bytes)`,
/// or imports the trait and writes `Fnv1a::<N>::hash_const(bytes)`
/// resolved through trait associated-fn lookup.
pub const trait ConstHash<const N: u16, S: Strategy, Sign: Signedness>: Sized
where
    S: BitsContainerFor<N, Sign>,
    <S as BitsContainerFor<N, Sign>>::T: [const] NarrowFromU64<N, S, Sign>,
{
    /// Hash the byte slice and produce the typed N-bit digest.
    fn hash_const(bytes: &[u8]) -> Bits<N, S, Sign>;
}

/// FNV-1a-64 over a byte slice (free const fn).
///
/// Returns the raw 64-bit state. Concrete `Hasher<N>` implementors
/// mask to N bits via `NarrowFromU64`. The `&[u8]` parameter is
/// workspace-rule exception #4 (boundary input from raw bytes;
/// canonical hash input shape); the `u64` return is the algorithm's
/// state-width.
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a-64 algorithm boundary; raw byte slice in, raw u64 state out per algorithm contract; tracked: #256
pub const fn fnv1a_64(bytes: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x100_0000_01b3;
    let mut hash: u64 = FNV_OFFSET_BASIS;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

// Suppress unused-import lint for `Signed`: it appears in the
// `ConstHash<N, S, Sign>` Sign bound but rustc currently does not
// flag the import as used because the trait is generic. Keep the
// import so consumers writing `impl ConstHash<N, S, Signed>` can
// resolve the marker through this crate's re-export surface.
#[allow(dead_code)]
const _: fn() = || {
    fn _bound<T: Signedness>() {}
    _bound::<Signed>();
};
