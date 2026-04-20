//! topo_sort Kahn's algorithm + renumber tests.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::{Cap, USize};
use arvo_bitmask::{BitMatrix64, NodeId};
use arvo_graph::{renumber, topo_sort};

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C3: Cap = cap(3);
const C4: Cap = cap(4);

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

#[test]
fn empty_dag_sorts_all_nodes() {
    // No edges: every node is a root. Valid count is N; any order
    // returned is a valid extension.
    let dag: BitMatrix64<C4> = BitMatrix64::empty();
    let (valid, _order) = topo_sort(&dag);
    assert_eq!(valid, USize(4));
}

#[test]
fn linear_chain_sort_is_stable() {
    // 0 -> 1 -> 2 -> 3. Only valid topo order is 0,1,2,3.
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let (valid, order) = topo_sort(&dag);
    assert_eq!(valid, USize(4));
    assert_eq!(order[0], nid(0));
    assert_eq!(order[1], nid(1));
    assert_eq!(order[2], nid(2));
    assert_eq!(order[3], nid(3));
}

#[test]
fn diamond_sort_respects_order() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3.
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(3));
    dag.set_edge(nid(2), nid(3));
    let (valid, order) = topo_sort(&dag);
    assert_eq!(valid, USize(4));
    // 0 must precede 1, 2, 3; 1 and 2 precede 3.
    let pos_of = |n: NodeId| order.iter().position(|&x| x == n).unwrap();
    assert!(pos_of(nid(0)) < pos_of(nid(1)));
    assert!(pos_of(nid(0)) < pos_of(nid(2)));
    assert!(pos_of(nid(1)) < pos_of(nid(3)));
    assert!(pos_of(nid(2)) < pos_of(nid(3)));
}

#[test]
fn cycle_is_detected() {
    // 0 -> 1 -> 2 -> 0. No in-degree-zero seed.
    let mut dag: BitMatrix64<C3> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(0));
    let (valid, _order) = topo_sort(&dag);
    assert!(valid.0 < 3, "cycle must leave valid_count < N, got {}", valid.0);
}

#[test]
fn partial_cycle_sorts_reachable_prefix() {
    // 0 -> 1, 2 -> 3 -> 2 (cycle between 2 and 3).
    let mut dag: BitMatrix64<C4> = BitMatrix64::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(2), nid(3));
    dag.set_edge(nid(3), nid(2));
    let (valid, _order) = topo_sort(&dag);
    // 0 and 1 get sorted; 2 and 3 remain in the cycle.
    assert_eq!(valid, USize(2));
}

#[test]
fn renumber_identity_pass_through() {
    let order = [nid(3), nid(1), nid(0), nid(2)];
    let out = renumber::<C4>(&order);
    assert_eq!(out[0], nid(3));
    assert_eq!(out[1], nid(1));
    assert_eq!(out[2], nid(0));
    assert_eq!(out[3], nid(2));
}
