//! Dulmage-Mendelsohn classification correctness.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Bits, Cap, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_sparse::dulmage_mendelsohn;

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C3: Cap = cap(3);
const C4: Cap = cap(4);

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

// Class IDs in the new DulmageMendelsohn shape:
//   0 = horizontal (sinks: incoming + no outgoing)
//   1 = vertical (sources and isolates: no incoming)
//   2 = square (core: both directions).
const H: USize = USize(0);
const V: USize = USize(1);
const S: USize = USize(2);

#[test]
fn class_count_is_three() {
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, C4> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    assert_eq!(dm.class_count, USize(3));
    // Every node lands in exactly one class; class[i] is a single id,
    // not a set, so the partition is exact by construction.
    for i in 0..4 {
        let c = dm.class[i];
        assert!(
            c == H || c == V || c == S,
            "node {} has unknown class id {:?}",
            i,
            c
        );
    }
}

#[test]
fn chain_classification() {
    // Chain 0 -> 1 -> 2.
    // 0: source -> vertical.
    // 1: matched -> square.
    // 2: sink -> horizontal.
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, C3> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    assert_eq!(dm.class[0], V);
    assert_eq!(dm.class[1], S);
    assert_eq!(dm.class[2], H);
}

#[test]
fn isolated_node_is_vertical() {
    let adj: BitMatrix<Bits<64, Hot, Unsigned>, C3> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    let dm = dulmage_mendelsohn(&adj);
    // Isolated nodes have no predecessors and classify as vertical.
    for i in 0..3 {
        assert_eq!(dm.class[i], V);
    }
}

#[test]
fn fan_out_source_is_vertical() {
    // 0 -> 1, 0 -> 2. Node 0 is a source, nodes 1 and 2 are sinks.
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, C3> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(0), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    assert_eq!(dm.class[0], V);
    assert_eq!(dm.class[1], H);
    assert_eq!(dm.class[2], H);
}

#[test]
fn fan_in_sink_is_horizontal() {
    // 0 -> 2, 1 -> 2. Nodes 0 and 1 are sources, 2 is a sink.
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, C3> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    adj.set_edge(nid(0), nid(2));
    adj.set_edge(nid(1), nid(2));

    let dm = dulmage_mendelsohn(&adj);
    assert_eq!(dm.class[0], V);
    assert_eq!(dm.class[1], V);
    assert_eq!(dm.class[2], H);
}
