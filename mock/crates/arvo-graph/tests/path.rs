//! longest_path DP correctness + predecessor tracking.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::{Cap, FBits, IBits, USize};
use arvo::strategy::Hot;
use arvo::traits::FromConstant;
use arvo::ufixed::UFixed;
use arvo_bitmask::{BitMatrix64, NodeId};
use arvo_graph::{longest_path, topo_sort};

type W = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C1: Cap = cap(1);
const C3: Cap = cap(3);
const C4: Cap = cap(4);

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

fn w(n: usize) -> W {
    W::from_constant(USize(n))
}

#[test]
fn linear_chain_longest_is_sum() {
    // 0 -> 1 -> 2 -> 3, weights 1, 2, 3, 4.
    // best[0] = 1; best[1] = 1 + 2 = 3; best[2] = 6; best[3] = 10.
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1), w(2), w(3), w(4)];
    let (_, order) = topo_sort(&dag);
    let (max, has_pred, preds) = longest_path(&dag, &weights, &order);
    assert_eq!(max.to_raw(), 10);
    // 0 has no predecessor; 1, 2, 3 do.
    assert!(!*has_pred.contains(USize(0)));
    assert!(*has_pred.contains(USize(1)));
    assert!(*has_pred.contains(USize(2)));
    assert!(*has_pred.contains(USize(3)));
    assert_eq!(preds[1], nid(0));
    assert_eq!(preds[2], nid(1));
    assert_eq!(preds[3], nid(2));
}

#[test]
fn diamond_picks_heavier_branch() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3. Weights: 1, 5, 2, 1.
    // best[0] = 1; best[1] = 1 + 5 = 6; best[2] = 1 + 2 = 3;
    // best[3] = max(6, 3) + 1 = 7. pred_of[3] = 1.
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(3));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1), w(5), w(2), w(1)];
    let (_, order) = topo_sort(&dag);
    let (max, has_pred, preds) = longest_path(&dag, &weights, &order);
    assert_eq!(max.to_raw(), 7);
    assert!(*has_pred.contains(USize(3)));
    assert_eq!(preds[3], nid(1));
}

#[test]
fn isolated_node_no_predecessor() {
    // Single root 0, weight 9. No edges.
    let dag: BitMatrix64<C1> = BitMatrix64::empty();
    let weights: [W; 1] = [w(9)];
    let (_, order) = topo_sort(&dag);
    let (max, has_pred, _preds) = longest_path(&dag, &weights, &order);
    assert_eq!(max.to_raw(), 9);
    assert!(!*has_pred.contains(USize(0)));
}

#[test]
fn two_roots_pick_heaviest_leaf_path() {
    // 0 -> 2, 1 -> 2. Weights: 1, 4, 2.
    // best[0] = 1; best[1] = 4; best[2] = max(1, 4) + 2 = 6.
    let mut dag: BitMatrix64<C3> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(2));
    let weights: [W; 3] = [w(1), w(4), w(2)];
    let (_, order) = topo_sort(&dag);
    let (max, _, preds) = longest_path(&dag, &weights, &order);
    assert_eq!(max.to_raw(), 6);
    assert_eq!(preds[2], nid(1));
}
