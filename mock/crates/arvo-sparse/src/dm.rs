//! Dulmage-Mendelsohn structural decomposition.
//!
//! Classifies each node of a `BitMatrix<W, C>` adjacency into one of
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

use arvo::USize;
use arvo_bitmask::{BitMatrix, Mask, NodeId, cap_size};
use arvo_bits_contracts::{BitAccess, BitLogic, BitPrim, BitSequence};
use arvo_tensor::Capacity;
use notko::Maybe;

use crate::adjacency::BidirectionalSparseAdjacency;

/// Dulmage-Mendelsohn decomposition result.
///
/// `class[i]` is the class ID of node `i` in `0..N`. Class IDs:
/// `0` for horizontal, `1` for vertical, `2` for square. `class_count`
/// is the number of distinct classes used (always `3` for this
/// algorithm; carried for parity with other partitioner results).
pub struct DulmageMendelsohn<C: Capacity> {
    /// Number of distinct classes used.
    pub class_count: USize,
    /// Class ID per node: `0` horizontal, `1` vertical, `2` square.
    pub class: C::Array<USize>,
}

impl<C: Capacity> Copy for DulmageMendelsohn<C> where C::Array<USize>: Copy {}

impl<C: Capacity> Clone for DulmageMendelsohn<C>
where
    C::Array<USize>: Copy,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Capacity> Default for DulmageMendelsohn<C> {
    #[inline]
    fn default() -> Self {
        DulmageMendelsohn {
            class_count: USize(0),
            class: C::filled(USize(0)),
        }
    }
}

/// Classify each node in the adjacency into one of the three classes.
#[inline]
pub fn dulmage_mendelsohn<W, C: Capacity>(
    adjacency: &BitMatrix<W, C>,
) -> DulmageMendelsohn<C>
where
    W: BitSequence + BitAccess + BitLogic + Copy + Default,
{
    let mut class: C::Array<USize> = C::filled(USize(0));

    let mut i = 0usize;
    while i < cap_size(C::CAP) {
        let node = NodeId::new(USize(i));
        let has_succ = !*adjacency.successors(node).is_empty();
        let has_pred = !*adjacency.predecessors(node).is_empty();

        if !has_pred {
            // No incoming edges: source or isolate.
            class.as_mut()[i] = USize(1);
        } else if !has_succ {
            // Has incoming but no outgoing: dead-end sink.
            class.as_mut()[i] = USize(0);
        } else {
            // Both directions: core.
            class.as_mut()[i] = USize(2);
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
/// Operates through the `BidirectionalSparseAdjacency<C>` contract,
/// so any consumer that implements the trait (BitMatrix, Csr's
/// `CsrBidirectional`, future representations) gets the same
/// classification without depending on bit-storage representation.
/// The trade is one method call per direction-test per node; the
/// mask-based `dulmage_mendelsohn` is strictly faster on `BitMatrix`.
#[inline]
pub fn dulmage_mendelsohn_via<T, C: Capacity>(adjacency: &T) -> DulmageMendelsohn<C>
where
    T: BidirectionalSparseAdjacency<C>,
{
    let mut class: C::Array<USize> = C::filled(USize(0));

    let mut i = 0usize;
    // Classify the live node range only. Packed (BitMatrix / packed
    // Csr) reports the cap, so packed consumers are unchanged; the tail
    // past the live count keeps the default class and is never read.
    let node_count = adjacency.node_count().0;
    while i < node_count {
        let node = NodeId::new(USize(i));
        let has_succ = adjacency.successors(node).next().is_some(); // lint:allow(no-bare-option) reason: core::iter::Iterator::next returns Option; tracked: #115
        let has_pred = adjacency.predecessors(node).next().is_some(); // lint:allow(no-bare-option) reason: core::iter::Iterator::next returns Option; tracked: #115

        if !has_pred {
            class.as_mut()[i] = USize(1);
        } else if !has_succ {
            class.as_mut()[i] = USize(0);
        } else {
            class.as_mut()[i] = USize(2);
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
/// Returns `Maybe::Is(mask)` when `cap_size(C::CAP) <= W::WIDTH` so every
/// node index can be represented in one `Mask<W>`. Returns
/// `Maybe::Isnt` when the node count exceeds W's bit width; in
/// that case the consumer should walk `dm.class` directly or pick
/// a wider `W`.
#[inline]
pub fn classification_to_mask<W, C: Capacity>(
    dm: &DulmageMendelsohn<C>,
    class_id: USize,
) -> Maybe<Mask<W>> // lint:allow(no-bare-option) reason: Maybe is notko, not bare Option; tracked: #115
where
    W: BitPrim + BitSequence + BitAccess + BitLogic + arvo::Identity + Copy + Default,
{
    if cap_size(C::CAP) > *<W as BitPrim>::WIDTH {
        return Maybe::Isnt;
    }
    let mut mask: Mask<W> = Mask::<W>::empty();
    let class = dm.class.as_ref();
    let mut i = 0usize;
    while i < cap_size(C::CAP) {
        if class[i] == class_id {
            mask.insert(USize(i));
        }
        i += 1;
    }
    Maybe::Is(mask)
}
