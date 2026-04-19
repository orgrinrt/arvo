//! Waist detection on known-shape DAGs.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::USize;
use arvo_bitmask::{BitMatrix64, NodeId};
use arvo_graph::{topo_sort, waist_detect};

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

#[test]
fn linear_chain_has_no_waist() {
    // 0 -> 1 -> 2 -> 3. Each level has width 1. No strict local
    // minimum.
    let mut dag: BitMatrix64<4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let (_, order) = topo_sort(&dag);
    let waist = waist_detect(&dag, &order);
    assert_eq!(waist.count(), USize(0));
}

#[test]
fn hourglass_waist_detected() {
    // Level 0: {0, 1} width 2.
    // Level 1: {2}     width 1   (waist).
    // Level 2: {3, 4}  width 2.
    // Edges: 0->2, 1->2, 2->3, 2->4.
    let mut dag: BitMatrix64<5> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    dag.set_edge(nid(2), nid(4));
    let (_, order) = topo_sort(&dag);
    let waist = waist_detect(&dag, &order);
    // Exactly one waist bit — the position in topo order where node 2
    // lands.
    assert_eq!(waist.count(), USize(1));
    let pos2 = order.iter().position(|&n| n == nid(2)).unwrap();
    assert!(*waist.contains(USize(pos2)));
}

#[test]
fn no_waist_when_width_monotone() {
    // Level 0: {0}         width 1.
    // Level 1: {1, 2}      width 2.
    // Level 2: {3, 4, 5}   width 3.
    // Edges: 0 -> 1, 0 -> 2, 1 -> 3, 1 -> 4, 2 -> 5.
    let mut dag: BitMatrix64<6> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(3));
    dag.set_edge(nid(1), nid(4));
    dag.set_edge(nid(2), nid(5));
    let (_, order) = topo_sort(&dag);
    let waist = waist_detect(&dag, &order);
    assert_eq!(waist.count(), USize(0));
}
