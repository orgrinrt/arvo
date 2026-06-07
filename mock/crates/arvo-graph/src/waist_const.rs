//! Const-callable waist detection (compile-time DAG analysis).
//!
//! The const analog of [`crate::waist::waist_detect`]. Same algorithm (topo
//! depth pass, level widths, strict-local-minimum occupied depths, waist flags
//! at topo positions), expressed as a `const fn` over the `ConstCapacity`
//! contract and the const `BitAccess` bit contract, so a const consumer
//! computes waist boundaries at compile time. The runtime `waist_detect` is
//! untouched.

use arvo::{Bool, USize};
use arvo_bits_contracts::BitAccess;
use arvo_bitmask::{cap_size, NodeId};
use arvo_tensor::ConstCapacity;

/// Detect waist levels in a DAG at compile time.
///
/// `succ_rows[i]` is the successor bit-word of node `i` (bit `j` set means edge
/// `i -> j`), the raw-word analog of a `BitMatrix` row. `topo_order` is the
/// topological order. Returns a `C::Array<Bool>` whose slot `k` is true when the
/// node at `topo_order[k]` sits at a depth whose level width is a strict local
/// minimum among the occupied depths on either side, matching
/// [`crate::waist::waist_detect`]'s output mask position-for-position (flag `k`
/// true iff bit `k` set). Nodes outside the valid prefix contribute nothing.
///
/// The predecessor set of a node is found by scanning every row and testing the
/// node's column via the const `BitAccess::bit`, so neither the non-const
/// `BitMatrix::predecessors` nor the `Mask::iter_set_bits` iterator is needed.
#[inline]
pub const fn waist_detect_const<C, W>(
    succ_rows: &C::Array<W>,
    topo_order: &C::Array<NodeId>,
) -> C::Array<Bool>
where
    C: [const] ConstCapacity,
    W: [const] BitAccess,
{
    let n = cap_size(C::CAP);

    // Depth per node (index = original NodeId), single forward topo pass.
    let mut depth = C::filled(USize(0));
    let mut idx = 0usize;
    while idx < n {
        let node = C::get(topo_order, USize(idx));
        let node_i = (node.0).0;
        if node_i >= n {
            idx += 1;
            continue;
        }
        // Predecessors of node_i: every i whose successor word has bit node_i.
        let mut max_d = 0usize;
        let mut any = false;
        let mut i = 0usize;
        while i < n {
            let row = C::get(succ_rows, USize(i));
            if row.bit(USize(node_i)).0 {
                let d = C::get(&depth, USize(i)).0;
                if !any || d > max_d {
                    max_d = d;
                    any = true;
                }
            }
            i += 1;
        }
        let nd = if any { USize(max_d + 1) } else { USize(0) };
        C::set(&mut depth, USize(node_i), nd);
        idx += 1;
    }

    // Level widths + max occupied depth. Max possible depth is N-1 (a chain).
    let mut width = C::filled(USize(0));
    let mut max_depth_seen = 0usize;
    let mut j = 0usize;
    while j < n {
        let d = C::get(&depth, USize(j)).0;
        if d < n {
            let w = C::get(&width, USize(d)).0;
            C::set(&mut width, USize(d), USize(w + 1));
            if d > max_depth_seen {
                max_depth_seen = d;
            }
        }
        j += 1;
    }

    // Occupied depths in order. Occupied means width > 0.
    let mut occupied = C::filled(USize(0));
    let mut occ_n = 0usize;
    let mut d = 0usize;
    while d <= max_depth_seen && d < n {
        if C::get(&width, USize(d)).0 > 0 {
            C::set(&mut occupied, USize(occ_n), USize(d));
            occ_n += 1;
        }
        d += 1;
    }

    // Depths that are strict local minima among the occupied depths. A
    // length-one or length-two occupied list has no interior; no minima.
    let mut is_waist = C::filled(Bool::FALSE);
    if occ_n >= 3 {
        let mut k = 1usize;
        while k + 1 < occ_n {
            let prev_w = C::get(&width, USize(C::get(&occupied, USize(k - 1)).0)).0;
            let cur_w = C::get(&width, USize(C::get(&occupied, USize(k)).0)).0;
            let next_w = C::get(&width, USize(C::get(&occupied, USize(k + 1)).0)).0;
            if cur_w < prev_w && cur_w < next_w {
                C::set(&mut is_waist, USize(C::get(&occupied, USize(k)).0), Bool::TRUE);
            }
            k += 1;
        }
    }

    // Emit flags at the topo-order positions of nodes whose depth is a waist.
    let mut out = C::filled(Bool::FALSE);
    let mut k = 0usize;
    while k < n {
        let node = C::get(topo_order, USize(k));
        let node_i = (node.0).0;
        if node_i < n {
            let dd = C::get(&depth, USize(node_i)).0;
            if dd < n && C::get(&is_waist, USize(dd)).0 {
                C::set(&mut out, USize(k), Bool::TRUE);
            }
        }
        k += 1;
    }

    out
}
