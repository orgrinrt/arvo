//! Connected-component DFS correctness.

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_graph::components;
use arvo_tensor::Dim;

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

#[test]
fn all_isolated_nodes_get_distinct_ids() {
    // No edges: each node is its own component.
    let dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    let c = components(&dag);
    // 4 distinct IDs total.
    let mut seen = [false; 4];
    for i in 0..4 {
        let id = c[i].0;
        assert!(id < 4);
        assert!(!seen[id]);
        seen[id] = true;
    }
}

#[test]
fn linear_chain_is_one_component() {
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let c = components(&dag);
    assert_eq!(c[0], c[1]);
    assert_eq!(c[1], c[2]);
    assert_eq!(c[2], c[3]);
}

#[test]
fn two_disjoint_edges_two_components() {
    // 0 -> 1, 2 -> 3.
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(2), nid(3));
    let c = components(&dag);
    assert_eq!(c[0], c[1]);
    assert_eq!(c[2], c[3]);
    assert!(c[0] != c[2], "disjoint edges must be distinct components");
}

#[test]
fn diamond_is_one_component() {
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(3));
    dag.set_edge(nid(2), nid(3));
    let c = components(&dag);
    for i in 1..4 {
        assert_eq!(c[0], c[i]);
    }
}

#[test]
fn isolated_plus_chain() {
    // 0 -> 1 -> 2 ; 3 isolated.
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    let c = components(&dag);
    assert_eq!(c[0], c[1]);
    assert_eq!(c[1], c[2]);
    assert!(c[3] != c[0]);
}
