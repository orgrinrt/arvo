//! Hash algorithm contracts. Streaming is canonical; oneshot is a
//! blanket-impl ext trait.
//!
//! Concrete algorithms implement `Hasher<N>`. The blanket impl of
//! `HasherExt<N>` over every `Hasher<N>` exposes a `hash(bytes)`
//! one-shot form without extra work at the impl site.

use arvo_bits::{Bits, Hot, UContainerFor};

/// Streaming N-bit hasher. Feed bytes via `update`, finalise to a
/// `Bits<N>`.
///
/// Algorithms implement this trait. One-shot ergonomics come from
/// the `HasherExt<N>` ext trait via its blanket impl.
pub trait Hasher<const N: u8>
where
    Hot: UContainerFor<N>,
{
    /// Feed a byte chunk into the hasher.
    fn update(&mut self, bytes: &[u8]);

    /// Consume the hasher and produce the final N-bit digest.
    fn finalize(self) -> Bits<N>;
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
    fn hash(mut self, bytes: &[u8]) -> Bits<N> {
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
