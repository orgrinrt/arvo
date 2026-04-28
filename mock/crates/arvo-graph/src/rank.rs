//! Upward and downward rank computation.
//!
//! Rank is a generic longest-path estimate over a DAG:
//!
//! - `upward_rank[v] = weight[v] + max(rank[succ] for succ in successors(v))`
//!   with leaves (no successors) grounded at `weight[v]`. Evaluated in
//!   reverse topo order so every successor is known when a node is
//!   reached.
//! - `downward_rank[v] = weight[v] + max(rank[pred] for pred in predecessors(v))`
//!   with roots (no predecessors) grounded at `weight[v]`. Evaluated
//!   in forward topo order.
//!
//! The weight type `W` only needs `Add`, `TotalOrd`, `Copy`, and
//! `FromConstant`. The algorithm does not multiply, divide, or take
//! square roots.

use core::cmp::Ordering;
use core::ops::Add;

use arvo::{Cap, USize};
use arvo::traits::{FromConstant, TotalOrd};
use arvo_bitmask::{BitMatrix64, cap_size};

use crate::topo::topo_sort;

/// Upward rank: weight plus the max successor rank.
///
/// Accepts a caller-supplied topo order. Walks it in reverse so
/// every successor's rank is already computed when a node is
/// visited. Leaves ground at their own weight.
#[inline]
pub fn upward_rank<const N: Cap, W>(
    dag: &BitMatrix64<N>,
    weights: &[W; cap_size(N)],
) -> [W; cap_size(N)]
where
    W: Add<Output = W> + TotalOrd + Copy + FromConstant,
    [(); cap_size(N)]:,
{
    let (valid, order) = topo_sort(dag);

    // Initialise ranks to zero; overwritten as we walk reverse topo.
    let zero = <W as FromConstant>::from_constant::<{ USize(0) }>();
    let mut rank: [W; cap_size(N)] = [zero; cap_size(N)];

    // Walk the valid prefix in reverse.
    let valid_n = valid.0;
    let mut idx = valid_n;
    while idx > 0 {
        idx -= 1;
        let node = order[idx];
        let node_i = (node.0).0;
        if node_i >= cap_size(N) {
            continue;
        }

        // Start with own weight.
        let w = weights[node_i];

        // Find max successor rank. If no successors, contribution is
        // zero so the total collapses to `w`.
        let succ = dag.successors(node);
        let mut best = zero;
        let mut any = false;
        for s_pos in succ.iter_set_bits() {
            let s_idx = s_pos.0;
            if s_idx >= cap_size(N) {
                continue;
            }
            let r = rank[s_idx];
            if !any {
                best = r;
                any = true;
            } else if matches!(r.total_cmp(best), Ordering::Greater) {
                best = r;
            }
        }

        rank[node_i] = if any { w + best } else { w };
    }

    rank
}

/// Downward rank: weight plus the max predecessor rank.
///
/// Walks the valid topo prefix forward so every predecessor is
/// computed when a node is reached. Roots ground at their own weight.
#[inline]
pub fn downward_rank<const N: Cap, W>(
    dag: &BitMatrix64<N>,
    weights: &[W; cap_size(N)],
) -> [W; cap_size(N)]
where
    W: Add<Output = W> + TotalOrd + Copy + FromConstant,
    [(); cap_size(N)]:,
{
    let (valid, order) = topo_sort(dag);

    let zero = <W as FromConstant>::from_constant::<{ USize(0) }>();
    let mut rank: [W; cap_size(N)] = [zero; cap_size(N)];

    let valid_n = valid.0;
    let mut idx = 0usize;
    while idx < valid_n {
        let node = order[idx];
        let node_i = (node.0).0;
        if node_i >= cap_size(N) {
            idx += 1;
            continue;
        }

        let w = weights[node_i];

        let pred = dag.predecessors(node);
        let mut best = zero;
        let mut any = false;
        for p_pos in pred.iter_set_bits() {
            let p_idx = p_pos.0;
            if p_idx >= cap_size(N) {
                continue;
            }
            let r = rank[p_idx];
            if !any {
                best = r;
                any = true;
            } else if matches!(r.total_cmp(best), Ordering::Greater) {
                best = r;
            }
        }

        rank[node_i] = if any { w + best } else { w };

        idx += 1;
    }

    rank
}
