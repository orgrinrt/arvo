//! Dulmage-Mendelsohn classification correctness.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::USize;
use arvo_bitmask::{BitMatrix64, NodeId};
use arvo_sparse::dulmage_mendelsohn;

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

#[test]
fn three_masks_partition_the_nodes() {
    // Chain 0 -> 1 -> 2, plus isolated 3.
    // Node 0: source (no preds) -> vertical.
    // Node 1: has preds + has succs -> square.
    // Node 2: has preds, no succs -> horizontal.
    // Node 3: isolated -> vertical (no preds).
    let mut adj: BitMatrix64<4> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    for i in 0..4 {
        let in_h = *dm.horizontal.contains(USize(i));
        let in_v = *dm.vertical.contains(USize(i));
        let in_s = *dm.square.contains(USize(i));
        let count = (in_h as u8) + (in_v as u8) + (in_s as u8);
        assert_eq!(count, 1, "node {} not in exactly one mask", i);
    }
}

#[test]
fn chain_classification() {
    let mut adj: BitMatrix64<3> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    // 0 is a pure source.
    assert!(*dm.vertical.contains(USize(0)));
    // 1 is matched (both directions).
    assert!(*dm.square.contains(USize(1)));
    // 2 is a pure sink.
    assert!(*dm.horizontal.contains(USize(2)));
}

#[test]
fn isolated_node_is_vertical() {
    let adj: BitMatrix64<3> = BitMatrix64::empty();
    let dm = dulmage_mendelsohn(&adj);
    // Per impl decision: isolated nodes classify as vertical.
    for i in 0..3 {
        assert!(*dm.vertical.contains(USize(i)));
        assert!(!*dm.horizontal.contains(USize(i)));
        assert!(!*dm.square.contains(USize(i)));
    }
}

#[test]
fn fan_out_source_is_vertical() {
    // 0 -> 1, 0 -> 2. Node 0 is a source (no preds), nodes 1 and 2
    // are sinks (no succs).
    let mut adj: BitMatrix64<3> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(0), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    assert!(*dm.vertical.contains(USize(0)));
    assert!(*dm.horizontal.contains(USize(1)));
    assert!(*dm.horizontal.contains(USize(2)));
}

#[test]
fn fan_in_sink_is_horizontal() {
    // 0 -> 2, 1 -> 2. Nodes 0 and 1 are sources, 2 is a sink.
    let mut adj: BitMatrix64<3> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(2));
    adj.set_edge(nid(1), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    assert!(*dm.vertical.contains(USize(0)));
    assert!(*dm.vertical.contains(USize(1)));
    assert!(*dm.horizontal.contains(USize(2)));
}
