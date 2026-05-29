//! Reverse Cuthill-McKee reordering.
//!
//! Bandwidth minimisation on a `BitMatrix<W, N>` adjacency, generic
//! over the bit-container word `W`. The algorithm:
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

use arvo::{Identity, Bool, Cap, USize};
use arvo_bitmask::{BitMatrix, Mask, NodeId, cap_size};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use notko::Maybe;

use crate::adjacency::BidirectionalSparseAdjacency;

/// Reverse Cuthill-McKee permutation.
///
/// `result[new_pos] = old_NodeId`. Min-degree start, ascending-degree
/// BFS ordering, final reverse. Generic over the bit-container word
/// `W`; the algorithm operates entirely through the `Mask<W>` set
/// surface, so any W satisfying the bounds works.
#[inline]
pub fn rcm_reorder<W, const N: Cap>(adjacency: &BitMatrix<W, N>) -> [NodeId; cap_size(N)]
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(N)]:,
{
    let mut order: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
    let mut visited: Mask<W> = Mask::<W>::empty();
    let mut head = USize(0);

    // Main loop: keep seeding BFS from the remaining min-degree node
    // until every node is visited. Handles disconnected graphs.
    while head.0 < cap_size(N) {
        // Pick the unvisited node with the smallest combined degree.
        // Tie-break by lowest index.
        let start = match min_degree_unvisited(adjacency, &visited) {
            Maybe::Is(s) => s.0,
            Maybe::Isnt => break,
        };

        visited.insert(USize(start));
        order[*head] = NodeId::new(USize(start));
        head = head + USize::ONE;

        // BFS frontier pointers: [read, head) is the current queue.
        let mut read = head - USize::ONE;

        while read.0 < head.0 {
            let node = order[*read];
            read = read + USize::ONE;

            // Collect unvisited neighbours (successors + predecessors).
            let neigh = adjacency
                .successors(node)
                .union(adjacency.predecessors(node))
                .difference(visited);

            // Sort neighbours by ascending degree, tie-break by index.
            // Collect into a fixed-size scratch buffer.
            let mut scratch: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
            let mut scratch_len = 0usize;
            for pos in neigh.iter_set_bits() {
                let p = pos.0;
                if p >= cap_size(N) {
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
                    let swap = da.0 > db.0 || (da.0 == db.0 && (a.0).0 > (b.0).0);
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
                    order[*head] = n;
                    head = head + USize::ONE;
                }
                k += 1;
            }
        }
    }

    // Reverse in place.
    let mut l = 0usize;
    let mut r = if head.0 == 0 { 0 } else { head.0 - 1 };
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
fn degree<W, const N: Cap>(adj: &BitMatrix<W, N>, n: NodeId) -> USize
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(N)]:,
{
    adj.successors(n).union(adj.predecessors(n)).count()
}

/// Lowest-index unvisited node with minimum combined degree, or
/// `Maybe::Isnt` if every node in `0..N` is already visited.
#[inline]
fn min_degree_unvisited<W, const N: Cap>(
    adj: &BitMatrix<W, N>,
    visited: &Mask<W>,
) -> Maybe<USize>
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(N)]:,
{
    let mut best: Maybe<(USize, USize)> = Maybe::Isnt;
    let mut i = 0usize;
    while i < cap_size(N) {
        if let Bool(false) = visited.contains(USize(i)) {
            let d = degree(adj, NodeId::new(USize(i)));
            match best {
                Maybe::Isnt => best = Maybe::Is((USize(i), d)),
                Maybe::Is((_, bd)) if d < bd => best = Maybe::Is((USize(i), d)),
                _ => {}
            }
        }
        i += 1;
    }
    match best {
        Maybe::Is((idx, _)) => Maybe::Is(idx),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// Trait-driven variant of `rcm_reorder`.
///
/// Operates through the `BidirectionalSparseAdjacency<N>` contract.
/// Visited tracking uses a `[Bool; cap_size(N)]` flag array; degree
/// computation walks both iterators and dedupes through a
/// `[Bool; cap_size(N)]` set buffer. Algorithmic shape mirrors
/// `rcm_reorder` (min-degree start, BFS with ascending-degree
/// neighbour ordering, final reverse).
///
/// The mask-based `rcm_reorder` is strictly faster on `BitMatrix`;
/// this version is the right call for CSR-shaped or other
/// iterator-only adjacency representations.
#[inline]
pub fn rcm_reorder_via<T, const N: Cap>(adjacency: &T) -> [NodeId; cap_size(N)]
where
    T: BidirectionalSparseAdjacency<N>,
    [(); cap_size(N)]:,
{
    let mut order: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
    let mut visited: [Bool; cap_size(N)] = [Bool(false); cap_size(N)];
    let mut head = USize(0);

    // Seed BFS over the live node range only. `node_count()` is the
    // cap for BitMatrix / packed Csr (so packed consumers are
    // unchanged) and the smaller live count for a loose Csr.
    let node_count = adjacency.node_count().0;
    while head.0 < node_count {
        let start = match min_degree_unvisited_via(adjacency, &visited) {
            Maybe::Is(s) => s.0,
            Maybe::Isnt => break,
        };

        visited[start] = Bool(true);
        order[*head] = NodeId::new(USize(start));
        head = head + USize::ONE;

        let mut read = head - USize::ONE;

        while read.0 < head.0 {
            let node = order[*read];
            read = read + USize::ONE;

            // Collect neighbours (successors ∪ predecessors) into a
            // scratch buffer, deduping through a local flag array.
            let mut scratch: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
            let mut scratch_len = 0usize;
            let mut in_scratch: [Bool; cap_size(N)] = [Bool(false); cap_size(N)];

            for n in adjacency.successors(node) {
                let n_idx = (n.0).0;
                if n_idx < cap_size(N) && !visited[n_idx].0 && !in_scratch[n_idx].0 {
                    in_scratch[n_idx] = Bool(true);
                    scratch[scratch_len] = n;
                    scratch_len += 1;
                }
            }
            for n in adjacency.predecessors(node) {
                let n_idx = (n.0).0;
                if n_idx < cap_size(N) && !visited[n_idx].0 && !in_scratch[n_idx].0 {
                    in_scratch[n_idx] = Bool(true);
                    scratch[scratch_len] = n;
                    scratch_len += 1;
                }
            }

            // Insertion sort by ascending degree, tie-break by index.
            let mut i = 1usize;
            while i < scratch_len {
                let mut j = i;
                while j > 0 {
                    let a = scratch[j - 1];
                    let b = scratch[j];
                    let da = degree_via(adjacency, a);
                    let db = degree_via(adjacency, b);
                    let swap = da.0 > db.0 || (da.0 == db.0 && (a.0).0 > (b.0).0);
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

            let mut k = 0usize;
            while k < scratch_len {
                let n = scratch[k];
                let n_idx = (n.0).0;
                if !visited[n_idx].0 {
                    visited[n_idx] = Bool(true);
                    order[*head] = n;
                    head = head + USize::ONE;
                }
                k += 1;
            }
        }
    }

    let mut l = 0usize;
    let mut r = if head.0 == 0 { 0 } else { head.0 - 1 };
    while l < r {
        let tmp = order[l];
        order[l] = order[r];
        order[r] = tmp;
        l += 1;
        r -= 1;
    }

    order
}

/// Undirected-view degree via the trait contract.
///
/// `|successors ∪ predecessors|`, computed by walking both iterators
/// and counting unique node IDs through a `[Bool; cap_size(N)]` set
/// buffer.
#[inline]
fn degree_via<T, const N: Cap>(adj: &T, n: NodeId) -> USize
where
    T: BidirectionalSparseAdjacency<N>,
    [(); cap_size(N)]:,
{
    let mut seen: [Bool; cap_size(N)] = [Bool(false); cap_size(N)];
    let mut count = USize(0);
    for s in adj.successors(n) {
        let idx = (s.0).0;
        if idx < cap_size(N) && !seen[idx].0 {
            seen[idx] = Bool(true);
            count = count + USize::ONE;
        }
    }
    for p in adj.predecessors(n) {
        let idx = (p.0).0;
        if idx < cap_size(N) && !seen[idx].0 {
            seen[idx] = Bool(true);
            count = count + USize::ONE;
        }
    }
    count
}

/// Trait-driven counterpart to `min_degree_unvisited`.
#[inline]
fn min_degree_unvisited_via<T, const N: Cap>(
    adj: &T,
    visited: &[Bool; cap_size(N)],
) -> Maybe<USize>
where
    T: BidirectionalSparseAdjacency<N>,
    [(); cap_size(N)]:,
{
    let mut best: Maybe<(USize, USize)> = Maybe::Isnt;
    let mut i = 0usize;
    let node_count = adj.node_count().0;
    while i < node_count {
        if !visited[i].0 {
            let d = degree_via(adj, NodeId::new(USize(i)));
            match best {
                Maybe::Isnt => best = Maybe::Is((USize(i), d)),
                Maybe::Is((_, bd)) if d < bd => best = Maybe::Is((USize(i), d)),
                _ => {}
            }
        }
        i += 1;
    }
    match best {
        Maybe::Is((idx, _)) => Maybe::Is(idx),
        Maybe::Isnt => Maybe::Isnt,
    }
}
