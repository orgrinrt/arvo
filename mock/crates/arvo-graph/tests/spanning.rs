//! SpanningTree decomposition correctness.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, FBits, IBits, ibits, fbits, USize};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo_bitmask::{BitMatrix64, NodeId};
use arvo_graph::{spanning_tree, upward_rank};

type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C3: Cap = cap(3);
const C4: Cap = cap(4);

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize→u8 cast for typed weight in concrete-W test scope; no runtime-FromConstant by design (round 202604271346); tracked: #256
    W::from_raw(n as u8)
}

#[test]
fn linear_chain_is_all_trunk() {
    // 0 -> 1 -> 2 -> 3, weights 1 each. The whole chain is the trunk.
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1); 4];
    let ranks = upward_rank(&dag, &weights);
    let tree = spanning_tree(&dag, &ranks);
    for i in 0..4 {
        assert!(*tree.on_trunk.contains(USize(i)));
    }
    assert_eq!(tree.branch_roots.count(), USize(0));
    assert_eq!(tree.bridges.count(), USize(0));
    assert_eq!(tree.trunk_next[0], nid(1));
    assert_eq!(tree.trunk_next[1], nid(2));
    assert_eq!(tree.trunk_next[2], nid(3));
}

#[test]
fn diamond_has_trunk_and_bridge() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3. Weights: 1, 5, 2, 1.
    // upward_rank: r[3] = 1; r[2] = 3; r[1] = 6; r[0] = 7.
    // Trunk from 0: highest-ranked successor is 1 (r=6 > r=3). Then
    // from 1: only successor 3. Trunk: 0, 1, 3. Node 2 is a branch
    // root. Node 3 is a bridge (two predecessors).
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(3));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1), w(5), w(2), w(1)];
    let ranks = upward_rank(&dag, &weights);
    let tree = spanning_tree(&dag, &ranks);
    assert!(*tree.on_trunk.contains(USize(0)));
    assert!(*tree.on_trunk.contains(USize(1)));
    assert!(*tree.on_trunk.contains(USize(3)));
    assert!(*tree.branch_roots.contains(USize(2)));
    assert!(*tree.bridges.contains(USize(3)));
    assert_eq!(tree.trunk_next[0], nid(1));
    assert_eq!(tree.trunk_next[1], nid(3));
}

#[test]
fn two_sources_highest_ranked_wins() {
    // 0 -> 2, 1 -> 2. Weights 1, 5, 1.
    // upward_rank: r[2] = 1; r[0] = 1 + 1 = 2; r[1] = 5 + 1 = 6.
    // Trunk head = 1 (rank 6). Trunk: 1, 2. 0 is a branch root.
    // 2 is a bridge.
    let mut dag: BitMatrix64<C3> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(2));
    let weights: [W; 3] = [w(1), w(5), w(1)];
    let ranks = upward_rank(&dag, &weights);
    let tree = spanning_tree(&dag, &ranks);
    assert!(*tree.on_trunk.contains(USize(1)));
    assert!(*tree.on_trunk.contains(USize(2)));
    assert!(*tree.branch_roots.contains(USize(0)));
    assert!(*tree.bridges.contains(USize(2)));
}

#[test]
fn disconnected_components_both_yield_trunks() {
    // 0 -> 1 ; 2 -> 3. No shared nodes. Two separate trunks.
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1); 4];
    let ranks = upward_rank(&dag, &weights);
    let tree = spanning_tree(&dag, &ranks);
    // All four nodes on a trunk (main or branch). 0 and 2 are sources;
    // one starts the main trunk, the other becomes a branch root.
    for i in 0..4 {
        assert!(*tree.on_trunk.contains(USize(i)));
    }
    // No bridges (every node has <= 1 predecessor).
    assert_eq!(tree.bridges.count(), USize(0));
}
