//! Connected components via iterative DFS.
//!
//! Two nodes share a component if either reaches the other through
//! directed edges (successors or predecessors). For each unvisited
//! node we allocate a fresh component ID, then DFS every node
//! reachable in either direction and tag it with that ID.
//!
//! Visited tracking is a `Mask64`; the DFS stack is a fixed-size
//! `[NodeId; cap_size(N)]` with a head index — no heap, no grow.

use arvo::{Bool, Cap, USize};
use arvo_bitmask::{BitMatrix64, Mask64, NodeId, cap_size};

/// Assign a component ID to every node.
///
/// `result[i]` is the component ID of node `i`. Component IDs start
/// at `USize(0)` and increase by one per distinct component. Two
/// nodes receive the same ID exactly when their DFS closure
/// (successors + predecessors, transitively) intersects.
#[inline]
pub fn components<const N: Cap>(dag: &BitMatrix64<N>) -> [USize; cap_size(N)]
where
    [(); cap_size(N)]:,
{
    let mut comp: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    let mut visited: Mask64 = Mask64::empty();
    let mut next_id = USize(0);

    let mut seed = 0usize;
    while seed < cap_size(N) {
        if *visited.contains(USize(seed)) {
            seed += 1;
            continue;
        }

        // Fresh component: seed is the root.
        let id = next_id;
        next_id = USize(next_id.0 + 1);

        // Iterative DFS over undirected adjacency (succ + pred).
        let mut stack: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
        let mut sp = 0usize;
        stack[sp] = NodeId::new(USize(seed));
        sp += 1;
        visited.insert(USize(seed));
        comp[seed] = id;

        while sp > 0 {
            sp -= 1;
            let node = stack[sp];

            // Undirected neighbour set = successors ∪ predecessors.
            let neigh = dag.successors(node).union(dag.predecessors(node));

            for n_pos in neigh.iter_set_bits() {
                let n_idx = n_pos.0;
                if n_idx >= cap_size(N) {
                    continue;
                }
                if let Bool(false) = visited.contains(USize(n_idx)) {
                    visited.insert(USize(n_idx));
                    comp[n_idx] = id;
                    stack[sp] = NodeId::new(USize(n_idx));
                    sp += 1;
                }
            }
        }

        seed += 1;
    }

    comp
}
