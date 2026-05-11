//! Dulmage-Mendelsohn structural decomposition.
//!
//! Classifies each node of a `BitMatrix<W, N>` adjacency into one of
//! three disjoint classes based on the presence of incoming and
//! outgoing edges:
//!
//! - `horizontal` (class ID `0`): sinks. Node has incoming edges but
//!   no outgoing edges. Row reads but never writes (dead-end output).
//! - `vertical` (class ID `1`): sources and isolates. Node has no
//!   incoming edges. Covers pure producers and nodes with no edges at
//!   all (treated as read-only constants).
//! - `square` (class ID `2`): core. Node has both incoming and
//!   outgoing edges. The matched row-column pairs.
//!
//! The three classes partition the `N` node indices exactly: every
//! node `i` in `0..N` carries exactly one class ID.
//!
//! Generic over the bit-container word `W`; the result struct itself
//! is independent of `W`, so consumers downstream of the
//! classification do not thread W through their signatures.

use arvo::{Bool, Cap, USize};
use arvo_bitmask::{BitMatrix, Mask, NodeId, cap_size};
use arvo_bits_contracts::{BitAccess, BitLogic, BitPrim, BitSequence};
use notko::Maybe;

use crate::adjacency::BidirectionalSparseAdjacency;

/// Dulmage-Mendelsohn decomposition result.
///
/// `class[i]` is the class ID of node `i` in `0..N`. Class IDs:
/// `0` for horizontal, `1` for vertical, `2` for square. `class_count`
/// is the number of distinct classes used (always `3` for this
/// algorithm; carried for parity with other partitioner results).
#[derive(Copy, Clone)]
pub struct DulmageMendelsohn<const N: Cap>
where
    [(); cap_size(N)]:,
{
    /// Number of distinct classes used.
    pub class_count: USize,
    /// Class ID per node: `0` horizontal, `1` vertical, `2` square.
    pub class: [USize; cap_size(N)],
}

impl<const N: Cap> Default for DulmageMendelsohn<N>
where
    [(); cap_size(N)]:,
{
    #[inline]
    fn default() -> Self {
        DulmageMendelsohn {
            class_count: USize(0),
            class: [USize(0); cap_size(N)],
        }
    }
}

/// Classify each node in the adjacency into one of the three classes.
#[inline]
pub fn dulmage_mendelsohn<W, const N: Cap>(
    adjacency: &BitMatrix<W, N>,
) -> DulmageMendelsohn<N>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
    [(); cap_size(N)]:,
{
    let mut class: [USize; cap_size(N)] = [USize(0); cap_size(N)];

    let mut i = 0usize;
    while i < cap_size(N) {
        let node = NodeId::new(USize(i));
        let has_succ = !*adjacency.successors(node).is_empty();
        let has_pred = !*adjacency.predecessors(node).is_empty();

        if !has_pred {
            // No incoming edges: source or isolate.
            class[i] = USize(1);
        } else if !has_succ {
            // Has incoming but no outgoing: dead-end sink.
            class[i] = USize(0);
        } else {
            // Both directions: core.
            class[i] = USize(2);
        }
        i += 1;
    }

    DulmageMendelsohn {
        class_count: USize(3),
        class,
    }
}

/// Trait-driven variant of `dulmage_mendelsohn`.
///
/// Operates through the `BidirectionalSparseAdjacency<N>` contract,
/// so any consumer that implements the trait (BitMatrix, Csr's
/// `CsrBidirectional`, future representations) gets the same
/// classification without depending on bit-storage representation.
/// The trade is one method call per direction-test per node; the
/// mask-based `dulmage_mendelsohn` is strictly faster on `BitMatrix`.
#[inline]
pub fn dulmage_mendelsohn_via<T, const N: Cap>(adjacency: &T) -> DulmageMendelsohn<N>
where
    T: BidirectionalSparseAdjacency<N>,
    [(); cap_size(N)]:,
{
    let mut class: [USize; cap_size(N)] = [USize(0); cap_size(N)];

    let mut i = 0usize;
    while i < cap_size(N) {
        let node = NodeId::new(USize(i));
        let has_succ = adjacency.successors(node).next().is_some(); // lint:allow(no-bare-option) reason: core::iter::Iterator::next returns Option; tracked: #115
        let has_pred = adjacency.predecessors(node).next().is_some(); // lint:allow(no-bare-option) reason: core::iter::Iterator::next returns Option; tracked: #115

        if !has_pred {
            class[i] = USize(1);
        } else if !has_succ {
            class[i] = USize(0);
        } else {
            class[i] = USize(2);
        }
        i += 1;
    }

    DulmageMendelsohn {
        class_count: USize(3),
        class,
    }
}

/// Project a class into a fits-in-W bitmask.
///
/// Returns `Maybe::Is(mask)` when `cap_size(N) <= W::WIDTH` so every
/// node index can be represented in one `Mask<W>`. Returns
/// `Maybe::Isnt` when the node count exceeds W's bit width; in
/// that case the consumer should walk `dm.class` directly or pick
/// a wider `W`.
#[inline]
pub fn classification_to_mask<W, const N: Cap>(
    dm: &DulmageMendelsohn<N>,
    class_id: USize,
) -> Maybe<Mask<W>> // lint:allow(no-bare-option) reason: Maybe is notko, not bare Option; tracked: #115
where
    W: BitPrim + BitSequence + BitAccess + BitLogic + arvo::Identity + Copy + Default,
    [(); cap_size(N)]:,
{
    if cap_size(N) > *<W as BitPrim>::WIDTH {
        return Maybe::Isnt;
    }
    let mut mask: Mask<W> = Mask::<W>::empty();
    let mut i = 0usize;
    while i < cap_size(N) {
        if dm.class[i] == class_id {
            mask.insert(USize(i));
        }
        i += 1;
    }
    Maybe::Is(mask)
}
