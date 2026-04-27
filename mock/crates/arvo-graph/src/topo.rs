//! Topological sort and node renumbering.
//!
//! `topo_sort` runs Kahn's algorithm over a `BitMatrix64` adjacency.
//! Start with the set of in-degree-zero nodes, pop one, record it in
//! the output order, and decrement each successor's in-degree. A
//! successor hitting zero joins the frontier. The loop stops when the
//! frontier empties.
//!
//! If every node was popped, the graph was acyclic and
//! `valid_count == N`. If some in-degrees never hit zero (a cycle),
//! `valid_count < N` and the unsorted nodes' positions in the output
//! array are left defaulted.
//!
//! `renumber` turns a valid topo order into the inverse permutation:
//! `new_to_old[k]` is the `NodeId` that now sits at position `k`. The
//! function is a pure index-shuffle; it does not rewrite the adjacency
//! matrix.

use arvo::{Cap, USize};
use arvo_bitmask::{BitMatrix64, NodeId, cap_size};

/// Topologically sort a DAG via Kahn's algorithm.
///
/// Returns `(valid_count, topo_order)`. On a well-formed DAG,
/// `valid_count == N` and `topo_order` is a valid linear extension.
/// On a cyclic graph, `valid_count < N` and only the first
/// `valid_count` entries of `topo_order` are meaningful; the rest are
/// defaulted to `NodeId::new(USize(0))`.
///
/// All working storage is stack-allocated: a `[USize; cap_size(N)]`
/// in-degree table and a `[NodeId; cap_size(N)]` frontier queue.
#[inline]
pub fn topo_sort<const N: Cap>(
    dag: &BitMatrix64<N>,
) -> (USize, [NodeId; cap_size(N)])
where
    [(); cap_size(N)]:,
{
    // In-degree per node. Computed by counting bits set in each
    // node's predecessor mask.
    let mut in_deg: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    let mut i = 0usize;
    while i < cap_size(N) {
        in_deg[i] = dag.predecessors(NodeId::new(USize(i))).count();
        i += 1;
    }

    // Frontier queue holds in-degree-zero nodes. Fixed array with
    // head / tail indices; no heap grow.
    let mut queue: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
    let mut q_head = 0usize;
    let mut q_tail = 0usize;

    // Seed the frontier with every node whose in-degree is zero.
    let mut j = 0usize;
    while j < cap_size(N) {
        if in_deg[j].0 == 0 {
            queue[q_tail] = NodeId::new(USize(j));
            q_tail += 1;
        }
        j += 1;
    }

    // Output order and how many nodes have been sorted so far.
    let mut order: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
    let mut sorted = 0usize;

    while q_head < q_tail {
        let node = queue[q_head];
        q_head += 1;

        order[sorted] = node;
        sorted += 1;

        // Decrement successor in-degrees. When one hits zero, enqueue.
        let succ = dag.successors(node);
        for s_pos in succ.iter_set_bits() {
            let s_idx = s_pos.0;
            if s_idx < cap_size(N) {
                let cur = in_deg[s_idx].0;
                if cur > 0 {
                    in_deg[s_idx] = USize(cur - 1);
                    if in_deg[s_idx].0 == 0 {
                        queue[q_tail] = NodeId::new(USize(s_idx));
                        q_tail += 1;
                    }
                }
            }
        }
    }

    (USize(sorted), order)
}

/// Inverse permutation: map new topo position to old `NodeId`.
///
/// Given a valid topo order, `renumber` returns an array where
/// `new_to_old[k]` is the `NodeId` that should sit at position `k`
/// in the renumbered sequence. For a consumer that wants to lay out
/// weights sequentially in topo order, the pattern is:
/// `new_weights[k] = old_weights[new_to_old[k].0.0]`.
#[inline]
pub fn renumber<const N: Cap>(
    topo_order: &[NodeId; cap_size(N)],
) -> [NodeId; cap_size(N)]
where
    [(); cap_size(N)]:,
{
    let mut new_to_old: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
    let mut k = 0usize;
    while k < cap_size(N) {
        new_to_old[k] = topo_order[k];
        k += 1;
    }
    new_to_old
}
