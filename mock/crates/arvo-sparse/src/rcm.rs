//! Reverse Cuthill-McKee reordering.
//!
//! Bandwidth minimisation on a `BitMatrix64<N>` adjacency. The
//! algorithm:
//!
//! 1. Pick the start node as the one with the lowest combined
//!    (successors + predecessors) degree, tie-broken by lowest index.
//! 2. BFS from the start. At each frontier, order newly-discovered
//!    neighbours by ascending degree (tie-break by lowest index).
//!    Append them to a permutation buffer in that order.
//! 3. Reverse the permutation.
//!
//! Disconnected nodes are handled by continuing with the
//! next-unvisited min-degree node once a BFS completes. The
//! pseudo-peripheral heuristic (BFS-diameter) is deferred; min-degree
//! start is adequate for this round's scope.
//!
//! Returns `[NodeId; N]` mapping new position to old node id:
//! `result[new_pos] = old_NodeId`.

use arvo::newtype::{Bool, USize};
use arvo_bitmask::{BitMatrix64, Mask64, NodeId};

/// Reverse Cuthill-McKee permutation.
///
/// `result[new_pos] = old_NodeId`. Min-degree start, ascending-degree
/// BFS ordering, final reverse.
#[inline]
pub fn rcm_reorder<const N: usize>(adjacency: &BitMatrix64<N>) -> [NodeId; N] {
    let mut order: [NodeId; N] = [NodeId::new(USize(0)); N];
    let mut visited: Mask64 = Mask64::empty();
    let mut head: usize = 0;

    // Main loop: keep seeding BFS from the remaining min-degree node
    // until every node is visited. Handles disconnected graphs.
    while head < N {
        // Pick the unvisited node with the smallest combined degree.
        // Tie-break by lowest index.
        let start = match min_degree_unvisited(adjacency, &visited) {
            Some(s) => s,
            None => break,
        };

        visited.insert(USize(start));
        order[head] = NodeId::new(USize(start));
        head += 1;

        // BFS frontier pointers: [read, head) is the current queue.
        let mut read = head - 1;

        while read < head {
            let node = order[read];
            read += 1;

            // Collect unvisited neighbours (successors + predecessors).
            let neigh = adjacency
                .successors(node)
                .union(adjacency.predecessors(node))
                .difference(visited);

            // Sort neighbours by ascending degree, tie-break by index.
            // Collect into a fixed-size scratch buffer.
            let mut scratch: [NodeId; N] = [NodeId::new(USize(0)); N];
            let mut scratch_len = 0usize;
            for pos in neigh.iter_set_bits() {
                let p = pos.0;
                if p >= N {
                    continue;
                }
                scratch[scratch_len] = NodeId::new(USize(p));
                scratch_len += 1;
            }

            // Insertion sort by ascending degree. Small frontiers so
            // quadratic is fine and simpler than a heap.
            let mut i = 1usize;
            while i < scratch_len {
                let mut j = i;
                while j > 0 {
                    let a = scratch[j - 1];
                    let b = scratch[j];
                    let da = degree(adjacency, a);
                    let db = degree(adjacency, b);
                    let swap = da > db || (da == db && (a.0).0 > (b.0).0);
                    if swap {
                        scratch[j - 1] = b;
                        scratch[j] = a;
                        j -= 1;
                    } else {
                        break;
                    }
                }
                i += 1;
            }

            // Append sorted neighbours to the permutation.
            let mut k = 0usize;
            while k < scratch_len {
                let n = scratch[k];
                let n_idx = (n.0).0;
                if let Bool(false) = visited.contains(USize(n_idx)) {
                    visited.insert(USize(n_idx));
                    order[head] = n;
                    head += 1;
                }
                k += 1;
            }
        }
    }

    // Reverse in place.
    let mut l = 0usize;
    let mut r = if head == 0 { 0 } else { head - 1 };
    while l < r {
        let tmp = order[l];
        order[l] = order[r];
        order[r] = tmp;
        l += 1;
        r -= 1;
    }

    order
}

/// Degree of `n` in the undirected view (successors + predecessors).
#[inline(always)]
fn degree<const N: usize>(adj: &BitMatrix64<N>, n: NodeId) -> usize {
    adj.successors(n).union(adj.predecessors(n)).count().0
}

/// Lowest-index unvisited node with minimum combined degree, or
/// `None` if every node in `0..N` is already visited.
#[inline]
fn min_degree_unvisited<const N: usize>(
    adj: &BitMatrix64<N>,
    visited: &Mask64,
) -> Option<usize> {
    let mut best: Option<(usize, usize)> = None;
    let mut i = 0usize;
    while i < N {
        if let Bool(false) = visited.contains(USize(i)) {
            let d = degree(adj, NodeId::new(USize(i)));
            match best {
                None => best = Some((i, d)),
                Some((_, bd)) if d < bd => best = Some((i, d)),
                _ => {}
            }
        }
        i += 1;
    }
    best.map(|(i, _)| i)
}

