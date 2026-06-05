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
//! whose depth is a local minimum, so consumers can re-map the
//! waist mask back through the same `topo_order` they passed in.

use arvo::{Identity, Bool, USize};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use arvo_bitmask::{BitMatrix, Mask, NodeId, cap_size};
use arvo_tensor::Capacity;

/// Detect waist levels in a DAG.
///
/// Returns a `Mask<B>` whose bit `k` is set when the node at
/// `topo_order[k]` sits at a depth whose level width is a strict
/// local minimum relative to the occupied depths on either side.
/// Nodes outside the valid prefix (e.g. when a cycle clipped the
/// topo sort) contribute nothing.
#[inline]
pub fn waist_detect<C: Capacity, B>(
    dag: &BitMatrix<B, C>,
    topo_order: &C::Array<NodeId>,
) -> Mask<B>
where
    B: BitSequence + BitAccess + BitLogic + Copy + Default + Identity,
    C::Array<USize>: Copy,
    C::Array<Bool>: Copy,
{
    // Depth per node (index = original NodeId).
    let mut depth: C::Array<USize> = C::filled(USize(0));

    // Single forward pass in the given topo order computes depths.
    let mut idx = 0usize;
    while idx < cap_size(C::CAP) {
        let node = topo_order.as_ref()[idx];
        let node_i = (node.0).0;
        if node_i >= cap_size(C::CAP) {
            idx += 1;
            continue;
        }

        let preds = dag.predecessors(node);
        let mut max_d = 0usize;
        let mut any = false;
        for p_pos in preds.iter_set_bits() {
            let p_idx = p_pos.0;
            if p_idx >= cap_size(C::CAP) {
                continue;
            }
            let d = depth.as_ref()[p_idx].0;
            if !any || d > max_d {
                max_d = d;
                any = true;
            }
        }

        depth.as_mut()[node_i] = if any { USize(max_d + 1) } else { USize(0) };
        idx += 1;
    }

    // Level widths. Max possible depth is N-1 (a straight chain).
    let mut width: C::Array<USize> = C::filled(USize(0));
    let mut max_depth_seen = USize(0);
    let mut j = 0usize;
    while j < cap_size(C::CAP) {
        let d = depth.as_ref()[j].0;
        if d < cap_size(C::CAP) {
            width.as_mut()[d] = width.as_ref()[d] + USize::ONE;
            if d > max_depth_seen.0 {
                max_depth_seen = USize(d);
            }
        }
        j += 1;
    }

    // Collect the occupied depths in order. Occupied means width > 0.
    let mut occupied: C::Array<USize> = C::filled(USize(0));
    let mut occ_n = USize(0);
    let mut d = 0usize;
    while d <= max_depth_seen.0 && d < cap_size(C::CAP) {
        if width.as_ref()[d].0 > 0 {
            occupied.as_mut()[*occ_n] = USize(d);
            occ_n = occ_n + USize::ONE;
        }
        d += 1;
    }

    // Depths that are strict local minima among the occupied depths.
    // A length-one or length-two occupied list has no interior;
    // no minima are emitted.
    let mut is_waist: C::Array<Bool> = C::filled(Bool::FALSE);
    if occ_n.0 >= 3 {
        let mut k = 1usize;
        while k + 1 < occ_n.0 {
            let prev_w = width.as_ref()[occupied.as_ref()[k - 1].0].0;
            let cur_w = width.as_ref()[occupied.as_ref()[k].0].0;
            let next_w = width.as_ref()[occupied.as_ref()[k + 1].0].0;
            if cur_w < prev_w && cur_w < next_w {
                is_waist.as_mut()[occupied.as_ref()[k].0] = Bool::TRUE;
            }
            k += 1;
        }
    }

    // Emit bits at the topo-order POSITIONS of nodes whose depth is a
    // waist depth.
    let mut out = Mask::<B>::empty();
    let mut k = 0usize;
    while k < cap_size(C::CAP) {
        let node = topo_order.as_ref()[k];
        let node_i = (node.0).0;
        if node_i < cap_size(C::CAP) {
            let d = depth.as_ref()[node_i].0;
            if d < cap_size(C::CAP) && is_waist.as_ref()[d].0 {
                out.insert(USize(k));
            }
        }
        k += 1;
    }

    let _ = NodeId::new(USize(0));
    out
}
