//! Hash algorithm contracts + free FNV-1a-64 helper.
//!
//! Concrete algorithms implement `Hasher<N>`. The blanket impl of
//! `HasherExt<N>` over every `Hasher<N>` exposes a `hash(bytes)`
//! one-shot form without extra work at the impl site.

use arvo::strategy::UContainerFor;
use arvo::{Bits, Hot};

/// Streaming N-bit hasher. Feed bytes via `update`, finalise to a
/// `Bits<N, Hot>`.
///
/// Algorithms implement this trait. One-shot ergonomics come from
/// the `HasherExt<N>` ext trait via its blanket impl.
///
/// `N` is `u8` directly rather than the `Width` meta-newtype because
/// nested const-fn evaluation at trait-bound resolution
/// (`where Hot: UContainerFor<{ width_u8(N) }>`) is unreliable on
/// current nightly under generic_const_exprs. The bound resolves
/// cleanly with bare `const N: u8`. The Width newtype is preserved
/// for typed const-generic positions where evaluation is local.
pub trait Hasher<const N: u8>
where
    Hot: UContainerFor<N>,
{
    /// Feed a byte chunk into the hasher.
    fn update(&mut self, bytes: &[u8]);

    /// Consume the hasher and produce the final N-bit digest.
    fn finalize(self) -> Bits<N, Hot>;
}

/// One-shot convenience: update with all bytes, then finalize.
///
/// Blanket-implemented for every `Hasher<N>`. Consumers bind
/// `H: HasherExt<N>`, or `H: Hasher<N>` with the ext trait in
/// scope, to call `.hash(bytes)` on any hasher.
pub trait HasherExt<const N: u8>: Hasher<N> + Sized
where
    Hot: UContainerFor<N>,
{
    /// Hash a complete byte slice in one pass.
    fn hash(mut self, bytes: &[u8]) -> Bits<N, Hot> {
        self.update(bytes);
        self.finalize()
    }
}

impl<H, const N: u8> HasherExt<N> for H
where
    H: Hasher<N> + Sized,
    Hot: UContainerFor<N>,
{
}

/// FNV-1a-64 over a byte slice (free const fn).
///
/// Returns the raw 64-bit state. Concrete `Hasher<N>` implementors
/// mask to N bits via `Bits::<N, Hot>::from_raw_u64`. The `&[u8]`
/// parameter is workspace-rule exception #4 (boundary input from raw
/// bytes); the `u64` return is the algorithm's state-width.
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
