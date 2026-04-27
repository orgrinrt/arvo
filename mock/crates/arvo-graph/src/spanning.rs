//! Spanning-tree decomposition of a DAG.
//!
//! Pick the highest-ranked root and follow its highest-ranked
//! successor at every step — that's the trunk. At each fan-out the
//! non-trunk successors become branch roots; every branch is
//! decomposed the same way (highest-rank descent within the branch
//! sub-DAG). Fan-in nodes — nodes with two or more predecessors —
//! are recorded as bridges.
//!
//! The decomposition is stored flat in `SpanningTree<N>`: masks for
//! trunk membership, branch roots, and bridges, plus a
//! `trunk_next[i]` successor map valid at `on_trunk` positions.

use core::cmp::Ordering;

use arvo::{Cap, USize};
use arvo::traits::TotalOrd;
use arvo_bitmask::{BitMatrix64, Mask64, NodeId, cap_size};

/// Flat spanning-tree decomposition.
///
/// - `on_trunk`: bit set for every node that lies on the trunk or a
///   branch trunk.
/// - `trunk_next[i]`: the successor chosen when node `i` is on
///   `on_trunk`. Defaulted to `NodeId::new(USize(0))` elsewhere.
/// - `branch_roots`: bit set at every node that starts a branch
///   (non-trunk successor of a fan-out point, or a secondary root).
/// - `bridges`: bit set at every node with two or more predecessors.
#[derive(Copy, Clone)]
pub struct SpanningTree<const N: Cap>
where
    [(); cap_size(N)]:,
{
    /// Nodes that sit on a trunk (main or branch).
    pub on_trunk: Mask64,
    /// Successor of each on-trunk node. Undefined off the trunk.
    pub trunk_next: [NodeId; cap_size(N)],
    /// Starting nodes of branches.
    pub branch_roots: Mask64,
    /// Fan-in nodes (predecessor count >= 2).
    pub bridges: Mask64,
}

impl<const N: Cap> SpanningTree<N>
where
    [(); cap_size(N)]:,
{
    /// Empty decomposition.
    #[inline]
    pub fn empty() -> Self {
        Self {
            on_trunk: Mask64::empty(),
            trunk_next: [NodeId::new(USize(0)); cap_size(N)],
            branch_roots: Mask64::empty(),
            bridges: Mask64::empty(),
        }
    }
}

/// Decompose a DAG into trunks, branches, and bridges.
///
/// `ranks` supplies the rank used for selection — typically the
/// result of `upward_rank`. A fan-out (successor count >= 2) routes
/// its highest-ranked successor onto the current trunk and pushes the
/// rest as branch roots. A fan-in (predecessor count >= 2) flags a
/// bridge regardless of which side reached it first.
///
/// The trunk starts at the highest-ranked source (no predecessors).
/// Every source ever reached becomes either the initial trunk start
/// or a branch root. Unreached sources are skipped — a cyclic input
/// silently yields an empty decomposition for the cyclic remainder.
#[inline]
pub fn spanning_tree<const N: Cap, W>(
    dag: &BitMatrix64<N>,
    ranks: &[W; cap_size(N)],
) -> SpanningTree<N>
where
    W: TotalOrd + Copy,
    [(); cap_size(N)]:,
{
    let mut out: SpanningTree<N> = SpanningTree::empty();

    // Every node with predecessor count >= 2 is a bridge.
    let mut i = 0usize;
    while i < cap_size(N) {
        let preds_count = dag.predecessors(NodeId::new(USize(i))).count();
        if preds_count.0 >= 2 {
            out.bridges.insert(USize(i));
        }
        i += 1;
    }

    // Identify sources: predecessors().count() == 0.
    let mut sources: Mask64 = Mask64::empty();
    let mut src_any = false;
    let mut j = 0usize;
    while j < cap_size(N) {
        if dag.predecessors(NodeId::new(USize(j))).count().0 == 0 {
            sources.insert(USize(j));
            src_any = true;
        }
        j += 1;
    }
    if !src_any {
        return out;
    }

    // Highest-ranked source = trunk head. Remaining sources become
    // branch roots.
    let mut head_idx = USize(0);
    let mut have_head = false;
    for s_pos in sources.iter_set_bits() {
        let s = s_pos.0;
        if s >= cap_size(N) {
            continue;
        }
        if !have_head {
            head_idx = USize(s);
            have_head = true;
        } else if matches!(ranks[s].total_cmp(&ranks[head_idx.0]), Ordering::Greater) {
            head_idx = USize(s);
        }
    }
    if !have_head {
        return out;
    }

    // BFS-like queue of trunk seeds. Each entry is the start of a
    // trunk (main trunk or a branch trunk).
    let mut queue: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
    let mut q_head = 0usize;
    let mut q_tail = 0usize;
    queue[q_tail] = NodeId::new(head_idx);
    q_tail += 1;

    // Non-head sources become branch roots. Enqueue them so the
    // descent below walks their sub-trunks. Without this step,
    // disconnected components past the main trunk would be missed.
    for s_pos in sources.iter_set_bits() {
        let s = s_pos.0;
        if s < cap_size(N) && s != head_idx.0 {
            out.branch_roots.insert(USize(s));
            if q_tail < cap_size(N) {
                queue[q_tail] = NodeId::new(USize(s));
                q_tail += 1;
            }
        }
    }

    let mut visited: Mask64 = Mask64::empty();

    while q_head < q_tail {
        let start = queue[q_head];
        q_head += 1;
        let start_i = (start.0).0;
        if start_i >= cap_size(N) || *visited.contains(USize(start_i)) {
            continue;
        }

        // Walk the trunk by following the highest-ranked successor.
        let mut current_i = start_i;
        loop {
            if *visited.contains(USize(current_i)) {
                break;
            }
            visited.insert(USize(current_i));
            out.on_trunk.insert(USize(current_i));

            let succ = dag.successors(NodeId::new(USize(current_i)));
            if succ.is_empty().0 {
                break;
            }

            // Choose highest-ranked successor as trunk continuation.
            let mut top_i = USize(0);
            let mut have_top = false;
            for s_pos in succ.iter_set_bits() {
                let s = s_pos.0;
                if s >= cap_size(N) {
                    continue;
                }
                if !have_top {
                    top_i = USize(s);
                    have_top = true;
                } else if matches!(ranks[s].total_cmp(&ranks[top_i.0]), Ordering::Greater) {
                    top_i = USize(s);
                }
            }
            if !have_top {
                break;
            }

            out.trunk_next[current_i] = NodeId::new(top_i);

            // Seed branches for the remaining (non-top) successors.
            for s_pos in succ.iter_set_bits() {
                let s = s_pos.0;
                if s >= cap_size(N) || s == top_i.0 {
                    continue;
                }
                out.branch_roots.insert(USize(s));
                if !*visited.contains(USize(s)) && q_tail < cap_size(N) {
                    queue[q_tail] = NodeId::new(USize(s));
                    q_tail += 1;
                }
            }

            current_i = top_i.0;
        }
    }

    out
}
