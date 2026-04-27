//! Waist (narrow-level) detection on a topologically ordered DAG.
//!
//! For each node we compute a topo depth: zero at roots (no
//! predecessors), otherwise `1 + max(depth[pred])` in forward topo
//! order. Nodes at the same depth form a level. The level width is
//! the node count at that depth.
//!
//! A depth is a local minimum (waist) when its width is strictly less
//! than both neighbours in the occupied-depth sequence. Bits are set
//! on the returned mask at the TOPO-ORDER POSITIONS of every node
//! whose depth is a local minimum — so consumers can re-map the
//! waist mask back through the same `topo_order` they passed in.

use arvo::{Bool, Cap, USize};
use arvo_bitmask::{BitMatrix64, Mask64, NodeId, cap_size};

/// Detect waist levels in a DAG.
///
/// Returns a `Mask64` whose bit `k` is set when the node at
/// `topo_order[k]` sits at a depth whose level width is a strict
/// local minimum relative to the occupied depths on either side.
/// Nodes outside the valid prefix (e.g. when a cycle clipped the
/// topo sort) contribute nothing.
#[inline]
pub fn waist_detect<const N: Cap>(
    dag: &BitMatrix64<N>,
    topo_order: &[NodeId; cap_size(N)],
) -> Mask64
where
    [(); cap_size(N)]:,
{
    // Depth per node (index = original NodeId).
    let mut depth: [USize; cap_size(N)] = [USize(0); cap_size(N)];

    // Single forward pass in the given topo order computes depths.
    let mut idx = 0usize;
    while idx < cap_size(N) {
        let node = topo_order[idx];
        let node_i = (node.0).0;
        if node_i >= cap_size(N) {
            idx += 1;
            continue;
        }

        let preds = dag.predecessors(node);
        let mut max_d = 0usize;
        let mut any = false;
        for p_pos in preds.iter_set_bits() {
            let p_idx = p_pos.0;
            if p_idx >= cap_size(N) {
                continue;
            }
            let d = depth[p_idx].0;
            if !any || d > max_d {
                max_d = d;
                any = true;
            }
        }

        depth[node_i] = if any { USize(max_d + 1) } else { USize(0) };
        idx += 1;
    }

    // Level widths. Max possible depth is N-1 (a straight chain).
    let mut width: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    let mut max_depth_seen = USize(0);
    let mut j = 0usize;
    while j < cap_size(N) {
        let d = depth[j].0;
        if d < cap_size(N) {
            width[d] = USize(width[d].0 + 1);
            if d > max_depth_seen.0 {
                max_depth_seen = USize(d);
            }
        }
        j += 1;
    }

    // Collect the occupied depths in order. Occupied means width > 0.
    let mut occupied: [USize; cap_size(N)] = [USize(0); cap_size(N)];
    let mut occ_n = USize(0);
    let mut d = 0usize;
    while d <= max_depth_seen.0 && d < cap_size(N) {
        if width[d].0 > 0 {
            occupied[occ_n.0] = USize(d);
            occ_n = USize(occ_n.0 + 1);
        }
        d += 1;
    }

    // Depths that are strict local minima among the occupied depths.
    // A length-one or length-two occupied list has no interior;
    // no minima are emitted.
    let mut is_waist: [Bool; cap_size(N)] = [Bool::FALSE; cap_size(N)];
    if occ_n.0 >= 3 {
        let mut k = 1usize;
        while k + 1 < occ_n.0 {
            let prev_w = width[occupied[k - 1].0].0;
            let cur_w = width[occupied[k].0].0;
            let next_w = width[occupied[k + 1].0].0;
            if cur_w < prev_w && cur_w < next_w {
                is_waist[occupied[k].0] = Bool::TRUE;
            }
            k += 1;
        }
    }

    // Emit bits at the topo-order POSITIONS of nodes whose depth is a
    // waist depth.
    let mut out = Mask64::empty();
    let mut k = 0usize;
    while k < cap_size(N) {
        let node = topo_order[k];
        let node_i = (node.0).0;
        if node_i < cap_size(N) {
            let d = depth[node_i].0;
            if d < cap_size(N) && is_waist[d].0 {
                out.insert(USize(k));
            }
        }
        k += 1;
    }

    let _ = NodeId::new(USize(0));
    out
}
