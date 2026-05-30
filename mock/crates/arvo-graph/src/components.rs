//! Connected components via iterative DFS.
//!
//! Two nodes share a component if either reaches the other through
//! directed edges (successors or predecessors). For each unvisited
//! node we allocate a fresh component ID, then DFS every node
//! reachable in either direction and tag it with that ID.
//!
//! Visited tracking is a `Mask<Bits<64, Hot, Unsigned>>`; the DFS stack is a fixed-size
//! `C::Array<NodeId>` with a head index, so no heap, no grow.

use arvo::{Identity, Bool, USize};
use arvo::{Bits, Hot, Unsigned};
use arvo_bitmask::{BitMatrix, Mask, NodeId, cap_size};
use arvo_tensor::Capacity;

/// Assign a component ID to every node.
///
/// `result[i]` is the component ID of node `i`. Component IDs start
/// at `USize(0)` and increase by one per distinct component. Two
/// nodes receive the same ID exactly when their DFS closure
/// (successors + predecessors, transitively) intersects.
#[inline]
pub fn components<C: Capacity>(dag: &BitMatrix<Bits<64, Hot, Unsigned>, C>) -> C::Array<USize>
where
    C::Array<USize>: Copy,
    C::Array<NodeId>: Copy,
{
    let mut comp: C::Array<USize> = C::filled(USize(0));
    let mut visited: Mask<Bits<64, Hot, Unsigned>> = Mask::<Bits<64, Hot, Unsigned>>::empty();
    let mut next_id = USize(0);

    let mut seed = 0usize;
    while seed < cap_size(C::CAP) {
        if *visited.contains(USize(seed)) {
            seed += 1;
            continue;
        }

        // Fresh component: seed is the root.
        let id = next_id;
        next_id = next_id + USize::ONE;

        // Iterative DFS over undirected adjacency (succ + pred).
        let mut stack: C::Array<NodeId> = C::filled(NodeId::new(USize(0)));
        let mut sp = 0usize;
        stack.as_mut()[sp] = NodeId::new(USize(seed));
        sp += 1;
        visited.insert(USize(seed));
        comp.as_mut()[seed] = id;

        while sp > 0 {
            sp -= 1;
            let node = stack.as_ref()[sp];

            // Undirected neighbour set = successors ∪ predecessors.
            let neigh = dag.successors(node).union(dag.predecessors(node));

            for n_pos in neigh.iter_set_bits() {
                let n_idx = n_pos.0;
                if n_idx >= cap_size(C::CAP) {
                    continue;
                }
                if let Bool(false) = visited.contains(USize(n_idx)) {
                    visited.insert(USize(n_idx));
                    comp.as_mut()[n_idx] = id;
                    stack.as_mut()[sp] = NodeId::new(USize(n_idx));
                    sp += 1;
                }
            }
        }

        seed += 1;
    }

    comp
}
