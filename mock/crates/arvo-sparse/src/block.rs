//! Block-diagonal detection via connected components.
//!
//! Treats the `BitMatrix<W, C>` adjacency as an undirected graph
//! (successors ∪ predecessors) and assigns each node a component
//! identifier. The result permits permuting rows and columns into
//! block-diagonal form: each block is an independent sub-problem.
//!
//! DFS is iterative on a fixed-size `C::Array<NodeId>` stack
//! with a `Mask<W>` visited set. Generic over the bit-container word
//! `W`. The implementation mirrors `arvo-graph::components` but lives
//! here to avoid a dependency edge from `arvo-sparse` onto
//! `arvo-graph` (the forbidden-imports lint prohibits `arvo_graph::*`
//! from `arvo-sparse`).

use arvo::{Additive, Bool, Identity, Multiplicative, USize};
use arvo_bitmask::{cap_size, BitMatrix, Mask, NodeId};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use arvo_tensor::Capacity;

use crate::adjacency::BidirectionalSparseAdjacency;

/// Assign a component (block) ID to every node.
///
/// Returns `(block_count, per_node_block_ids)`. `block_count` is the
/// number of distinct components. `per_node_block_ids[i]` is the
/// block ID of node `i`; IDs start at `USize(0)` and increase by one
/// per distinct component.
#[inline]
pub fn block_diagonal<W, C: Capacity>(adjacency: &BitMatrix<W, C>) -> (USize, C::Array<USize>)
where
    W: BitSequence + BitAccess + BitLogic + Identity<Additive> + Copy + Default,
{
    let mut block_id: C::Array<USize> = C::filled(USize(0));
    let mut visited: Mask<W> = Mask::<W>::empty();
    let mut next_id = USize(0);

    let mut seed = 0usize;
    while seed < cap_size(C::CAP) {
        if *visited.contains(USize(seed)) {
            seed += 1;
            continue;
        }

        let id = next_id;
        next_id = next_id + <USize as Identity<Multiplicative>>::IDENTITY;

        // Iterative DFS. Stack capacity = N is a safe bound: each
        // node enters the stack at most once.
        let mut stack: C::Array<NodeId> = C::filled(NodeId::new(USize(0)));
        let mut sp = 0usize;
        stack.as_mut()[sp] = NodeId::new(USize(seed));
        sp += 1;
        visited.insert(USize(seed));
        block_id.as_mut()[seed] = id;

        while sp > 0 {
            sp -= 1;
            let node = stack.as_ref()[sp];

            // Undirected neighbour set = successors ∪ predecessors.
            let neigh = adjacency
                .successors(node)
                .union(adjacency.predecessors(node));

            for n_pos in neigh.iter_set_bits() {
                let n_idx = n_pos.0;
                if n_idx >= cap_size(C::CAP) {
                    continue;
                }
                if let Bool(false) = visited.contains(USize(n_idx)) {
                    visited.insert(USize(n_idx));
                    block_id.as_mut()[n_idx] = id;
                    stack.as_mut()[sp] = NodeId::new(USize(n_idx));
                    sp += 1;
                }
            }
        }

        seed += 1;
    }

    (next_id, block_id)
}

/// Trait-driven variant of `block_diagonal`.
///
/// Operates through the `BidirectionalSparseAdjacency<C>` contract.
/// Visited tracking uses a `C::Array<Bool>` flag array instead
/// of `Mask<W>`; otherwise the DFS structure mirrors
/// `block_diagonal`. The mask-based version is strictly faster on
/// `BitMatrix` adjacencies; this version is the right call for
/// representations (CSR's `CsrBidirectional`, future shapes) that
/// don't expose a cheap mask.
#[inline]
pub fn block_diagonal_via<T, C: Capacity>(adjacency: &T) -> (USize, C::Array<USize>)
where
    T: BidirectionalSparseAdjacency<C>,
{
    let mut block_id: C::Array<USize> = C::filled(USize(0));
    let mut visited: C::Array<Bool> = C::filled(Bool(false));
    let mut next_id = USize(0);

    let mut seed = 0usize;
    // Seed DFS over the live node range only. Packed (BitMatrix /
    // packed Csr) reports the cap, so packed consumers are unchanged.
    let node_count = adjacency.node_count().0;
    while seed < node_count {
        if visited.as_ref()[seed].0 {
            seed += 1;
            continue;
        }

        let id = next_id;
        next_id = next_id + <USize as Identity<Multiplicative>>::IDENTITY;

        let mut stack: C::Array<NodeId> = C::filled(NodeId::new(USize(0)));
        let mut sp = 0usize;
        stack.as_mut()[sp] = NodeId::new(USize(seed));
        sp += 1;
        visited.as_mut()[seed] = Bool(true);
        block_id.as_mut()[seed] = id;

        while sp > 0 {
            sp -= 1;
            let node = stack.as_ref()[sp];

            // Walk successors, then predecessors. Dedup via visited
            // array; the same neighbour may appear in both iterators
            // for an asymmetric adjacency, but visited gates the
            // second visit.
            for n in adjacency.successors(node) {
                let n_idx = (n.0).0;
                if n_idx < cap_size(C::CAP) && !visited.as_ref()[n_idx].0 {
                    visited.as_mut()[n_idx] = Bool(true);
                    block_id.as_mut()[n_idx] = id;
                    stack.as_mut()[sp] = NodeId::new(USize(n_idx));
                    sp += 1;
                }
            }
            for n in adjacency.predecessors(node) {
                let n_idx = (n.0).0;
                if n_idx < cap_size(C::CAP) && !visited.as_ref()[n_idx].0 {
                    visited.as_mut()[n_idx] = Bool(true);
                    block_id.as_mut()[n_idx] = id;
                    stack.as_mut()[sp] = NodeId::new(USize(n_idx));
                    sp += 1;
                }
            }
        }

        seed += 1;
    }

    (next_id, block_id)
}
