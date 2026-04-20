//! Block-diagonal detection via connected components.
//!
//! Treats the `BitMatrix64<N>` adjacency as an undirected graph
//! (successors ∪ predecessors) and assigns each node a component
//! identifier. The result permits permuting rows and columns into
//! block-diagonal form: each block is an independent sub-problem.
//!
//! DFS is iterative on a fixed-size `[NodeId; cap_size(N)]` stack
//! with a `Mask64` visited set. The implementation mirrors
//! `arvo-graph::components` but lives here to avoid a dependency edge
//! from `arvo-sparse` onto `arvo-graph` (the forbidden-imports lint
//! prohibits `arvo_graph::*` from `arvo-sparse`).

use arvo::newtype::{Bool, Cap, USize};
use arvo_bitmask::{BitMatrix64, Mask64, NodeId, cap_size};

/// Assign a component (block) ID to every node.
///
/// Returns `(block_count, per_node_block_ids)`. `block_count` is the
/// number of distinct components. `per_node_block_ids[i]` is the
/// block ID of node `i`; IDs start at `USize(0)` and increase by one
/// per distinct component.
#[inline]
pub fn block_diagonal<const N: Cap>(
    adjacency: &BitMatrix64<N>,
) -> (USize, [USize; cap_size(N)])
where
    [(); cap_size(N)]:,
{
    let mut block_id: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    let mut visited: Mask64 = Mask64::empty();
    let mut next_id: usize = 0;

    let mut seed = 0usize;
    while seed < cap_size(N) {
        if *visited.contains(USize(seed)) {
            seed += 1;
            continue;
        }

        let id = USize(next_id);
        next_id += 1;

        // Iterative DFS. Stack capacity = N is a safe bound: each
        // node enters the stack at most once.
        let mut stack: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
        let mut sp = 0usize;
        stack[sp] = NodeId::new(USize(seed));
        sp += 1;
        visited.insert(USize(seed));
        block_id[seed] = id;

        while sp > 0 {
            sp -= 1;
            let node = stack[sp];

            // Undirected neighbour set = successors ∪ predecessors.
            let neigh = adjacency
                .successors(node)
                .union(adjacency.predecessors(node));

            for n_pos in neigh.iter_set_bits() {
                let n_idx = n_pos.0;
                if n_idx >= cap_size(N) {
                    continue;
                }
                if let Bool(false) = visited.contains(USize(n_idx)) {
                    visited.insert(USize(n_idx));
                    block_id[n_idx] = id;
                    stack[sp] = NodeId::new(USize(n_idx));
                    sp += 1;
                }
            }
        }

        seed += 1;
    }

    (USize(next_id), block_id)
}
