//! BitMatrix adjacency semantics, successors / predecessors, and
//! Warshall's transitive closure.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_bitmask::{BitMatrix64, BitMatrix256, NodeId};

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C4: Cap = cap(4);
const C5: Cap = cap(5);
const C8: Cap = cap(8);

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

#[test]
fn empty_matrix_has_no_edges() {
    let m: BitMatrix64<C8> = BitMatrix64::empty();
    for i in 0..8 {
        for j in 0..8 {
            assert!(!*m.edge(nid(i), nid(j)));
        }
    }
}

#[test]
fn set_edge_then_edge_reads_true() {
    let mut m: BitMatrix64<C8> = BitMatrix64::empty();
    m.set_edge(nid(3), nid(5));
    assert!(*m.edge(nid(3), nid(5)));
    assert!(!*m.edge(nid(5), nid(3)));
}

#[test]
fn clear_edge_removes_edge() {
    let mut m: BitMatrix64<C8> = BitMatrix64::empty();
    m.set_edge(nid(1), nid(2));
    m.clear_edge(nid(1), nid(2));
    assert!(!*m.edge(nid(1), nid(2)));
}

#[test]
fn successors_returns_outgoing() {
    let mut m: BitMatrix64<C8> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(0), nid(3));
    m.set_edge(nid(0), nid(7));
    let succ = m.successors(nid(0));
    assert_eq!(succ.count(), USize(3));
    assert!(*succ.contains(USize(1)));
    assert!(*succ.contains(USize(3)));
    assert!(*succ.contains(USize(7)));
}

#[test]
fn predecessors_scans_column() {
    let mut m: BitMatrix64<C8> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(5));
    m.set_edge(nid(2), nid(5));
    m.set_edge(nid(4), nid(5));
    m.set_edge(nid(4), nid(6));
    let pred = m.predecessors(nid(5));
    assert_eq!(pred.count(), USize(3));
    assert!(*pred.contains(USize(0)));
    assert!(*pred.contains(USize(2)));
    assert!(*pred.contains(USize(4)));
}

#[test]
fn transitive_closure_chain() {
    // 0 -> 1 -> 2 -> 3. Closure should yield direct + transitive edges.
    let mut m: BitMatrix64<C4> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(1), nid(2));
    m.set_edge(nid(2), nid(3));
    m.transitive_closure();
    // 0 reaches 1, 2, 3.
    assert!(*m.edge(nid(0), nid(1)));
    assert!(*m.edge(nid(0), nid(2)));
    assert!(*m.edge(nid(0), nid(3)));
    // 1 reaches 2, 3.
    assert!(*m.edge(nid(1), nid(2)));
    assert!(*m.edge(nid(1), nid(3)));
    // 2 reaches 3.
    assert!(*m.edge(nid(2), nid(3)));
    // 3 reaches nothing.
    for j in 0..4 {
        assert!(!*m.edge(nid(3), nid(j)));
    }
}

#[test]
fn transitive_closure_fan_out() {
    // 0 -> 1, 0 -> 2; 1 -> 3; 2 -> 4.
    let mut m: BitMatrix64<C5> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(0), nid(2));
    m.set_edge(nid(1), nid(3));
    m.set_edge(nid(2), nid(4));
    m.transitive_closure();
    // 0 reaches 1, 2, 3, 4.
    let s0 = m.successors(nid(0));
    assert_eq!(s0.count(), USize(4));
    for j in &[1usize, 2, 3, 4] {
        assert!(*s0.contains(USize(*j)));
    }
}

#[test]
fn transitive_closure_disconnected() {
    // Two disjoint edges: 0 -> 1, 2 -> 3.
    let mut m: BitMatrix64<C4> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(2), nid(3));
    m.transitive_closure();
    assert!(*m.edge(nid(0), nid(1)));
    assert!(*m.edge(nid(2), nid(3)));
    assert!(!*m.edge(nid(0), nid(2)));
    assert!(!*m.edge(nid(0), nid(3)));
    assert!(!*m.edge(nid(1), nid(2)));
}

#[test]
fn out_of_range_edge_is_false() {
    let m: BitMatrix64<C4> = BitMatrix64::empty();
    assert!(!*m.edge(nid(10), nid(0)));
    assert!(!*m.edge(nid(0), nid(10))); // bit index beyond width is still false
}

// --- BitMatrix256 -------------------------------------------------------

#[test]
fn bitmatrix256_edge_set_clear() {
    let mut m: BitMatrix256<C4> = BitMatrix256::empty();
    m.set_edge(nid(0), nid(200));
    assert!(*m.edge(nid(0), nid(200)));
    m.clear_edge(nid(0), nid(200));
    assert!(!*m.edge(nid(0), nid(200)));
}

#[test]
fn bitmatrix256_closure_chain() {
    let mut m: BitMatrix256<C4> = BitMatrix256::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(1), nid(2));
    m.set_edge(nid(2), nid(3));
    m.transitive_closure();
    assert!(*m.edge(nid(0), nid(3)));
    assert!(*m.edge(nid(0), nid(2)));
    assert!(*m.edge(nid(1), nid(3)));
}
