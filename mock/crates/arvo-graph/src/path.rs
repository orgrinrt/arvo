//! Longest-path DP over a DAG.
//!
//! One pass in forward topological order. For each node `v`,
//! `best[v] = weight[v] + max(best[p] for p in predecessors(v))`;
//! roots ground at `weight[v]`. The best predecessor is recorded in
//! `pred_of[v]`, with `has_predecessor` tracking which nodes actually
//! have one. The overall return is the max `best[v]` across all
//! nodes.

use core::cmp::Ordering;
use core::ops::Add;

use arvo::newtype::{Cap, USize};
use arvo::traits::{FromConstant, TotalOrd};
use arvo_bitmask::{BitMatrix64, Mask64, NodeId, cap_size};

/// Longest path in a DAG as a DP over forward topo order.
///
/// Returns `(overall_max, has_predecessor, pred_of)`:
///
/// - `overall_max` is the maximum path weight ending at any node.
/// - `has_predecessor` is a `Mask64` with bit `i` set when node `i`
///   has a best predecessor. Roots (and unreached nodes past the
///   valid topo prefix) have their bit unset.
/// - `pred_of[i]` is the chosen predecessor of node `i` when
///   `has_predecessor` bit `i` is set; otherwise undefined.
#[inline]
pub fn longest_path<const N: Cap, W>(
    dag: &BitMatrix64<N>,
    weights: &[W; cap_size(N)],
    topo_order: &[NodeId; cap_size(N)],
) -> (W, Mask64, [NodeId; cap_size(N)])
where
    W: Add<Output = W> + TotalOrd + Copy + FromConstant,
    [(); cap_size(N)]:,
{
    let zero = <W as FromConstant>::from_constant(0);
    let mut best: [W; cap_size(N)] = [zero; cap_size(N)];
    let mut pred_of: [NodeId; cap_size(N)] = [NodeId::new(USize(0)); cap_size(N)];
    let mut has_pred: Mask64 = Mask64::empty();

    let mut overall = zero;
    let mut any_node = false;

    let mut idx = 0usize;
    while idx < cap_size(N) {
        let node = topo_order[idx];
        let node_i = (node.0).0;
        if node_i >= cap_size(N) {
            idx += 1;
            continue;
        }

        let w = weights[node_i];

        // Scan predecessors; track the one that maximises best.
        let preds = dag.predecessors(node);
        let mut top = zero;
        let mut top_p: NodeId = NodeId::new(USize(0));
        let mut any_pred = false;
        for p_pos in preds.iter_set_bits() {
            let p_idx = p_pos.0;
            if p_idx >= cap_size(N) {
                continue;
            }
            let candidate = best[p_idx];
            if !any_pred {
                top = candidate;
                top_p = NodeId::new(USize(p_idx));
                any_pred = true;
            } else if matches!(candidate.total_cmp(&top), Ordering::Greater) {
                top = candidate;
                top_p = NodeId::new(USize(p_idx));
            }
        }

        let this_best = if any_pred { w + top } else { w };
        best[node_i] = this_best;
        if any_pred {
            has_pred.insert(USize(node_i));
            pred_of[node_i] = top_p;
        }

        if !any_node {
            overall = this_best;
            any_node = true;
        } else if matches!(this_best.total_cmp(&overall), Ordering::Greater) {
            overall = this_best;
        }

        idx += 1;
    }

    (overall, has_pred, pred_of)
}
