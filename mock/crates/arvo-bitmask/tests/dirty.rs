//! Dirty propagation correctness on a DAG.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::{Cap, USize};
use arvo_bitmask::{
    BitMatrix64, BitMatrix256, Mask256, Mask64, NodeId, propagate_dirty_256,
    propagate_dirty_64,
};

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const C4: Cap = cap(4);
const C6: Cap = cap(6);

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

#[test]
fn single_dirty_bit_propagates_to_successors() {
    // 0 -> 1 -> 2 -> 3. Dirty 0 should propagate to all.
    let mut m: BitMatrix64<C4> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(1), nid(2));
    m.set_edge(nid(2), nid(3));
    let mut dirty = Mask64::empty();
    dirty.insert(USize(0));
    propagate_dirty_64(&m, &mut dirty);
    assert_eq!(dirty.count(), USize(4));
    for j in 0..4 {
        assert!(*dirty.contains(USize(j)));
    }
}

#[test]
fn dirty_does_not_touch_unrelated_nodes() {
    // Two chains: 0 -> 1, 2 -> 3. Dirty 0 should reach only 0, 1.
    let mut m: BitMatrix64<C4> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(2), nid(3));
    let mut dirty = Mask64::empty();
    dirty.insert(USize(0));
    propagate_dirty_64(&m, &mut dirty);
    assert_eq!(dirty.count(), USize(2));
    assert!(*dirty.contains(USize(0)));
    assert!(*dirty.contains(USize(1)));
    assert!(!*dirty.contains(USize(2)));
    assert!(!*dirty.contains(USize(3)));
}

#[test]
fn dirty_fan_out_reaches_all_descendants() {
    // 0 -> 1, 0 -> 2; 1 -> 3; 2 -> 4; 3 -> 5.
    let mut m: BitMatrix64<C6> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(0), nid(2));
    m.set_edge(nid(1), nid(3));
    m.set_edge(nid(2), nid(4));
    m.set_edge(nid(3), nid(5));
    let mut dirty = Mask64::empty();
    dirty.insert(USize(0));
    propagate_dirty_64(&m, &mut dirty);
    assert_eq!(dirty.count(), USize(6));
}

#[test]
fn dirty_mid_chain_reaches_only_downstream() {
    // 0 -> 1 -> 2 -> 3; dirty 2 should reach only 2, 3.
    let mut m: BitMatrix64<C4> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(1), nid(2));
    m.set_edge(nid(2), nid(3));
    let mut dirty = Mask64::empty();
    dirty.insert(USize(2));
    propagate_dirty_64(&m, &mut dirty);
    assert_eq!(dirty.count(), USize(2));
    assert!(*dirty.contains(USize(2)));
    assert!(*dirty.contains(USize(3)));
    assert!(!*dirty.contains(USize(0)));
    assert!(!*dirty.contains(USize(1)));
}

#[test]
fn empty_dirty_stays_empty() {
    let mut m: BitMatrix64<C4> = BitMatrix64::empty();
    m.set_edge(nid(0), nid(1));
    let mut dirty = Mask64::empty();
    propagate_dirty_64(&m, &mut dirty);
    assert!(*dirty.is_empty());
}

#[test]
fn dirty_terminates_on_no_successors() {
    // 0 has no outgoing edges; dirty 0 should stay {0}.
    let m: BitMatrix64<C4> = BitMatrix64::empty();
    let mut dirty = Mask64::empty();
    dirty.insert(USize(0));
    propagate_dirty_64(&m, &mut dirty);
    assert_eq!(dirty.count(), USize(1));
    assert!(*dirty.contains(USize(0)));
}

#[test]
fn mask256_dirty_propagation() {
    // 0 -> 1 -> 2 -> 3 in a Mask256-backed matrix.
    let mut m: BitMatrix256<C4> = BitMatrix256::empty();
    m.set_edge(nid(0), nid(1));
    m.set_edge(nid(1), nid(2));
    m.set_edge(nid(2), nid(3));
    let mut dirty = Mask256::empty();
    dirty.insert(USize(0));
    propagate_dirty_256(&m, &mut dirty);
    assert_eq!(dirty.count(), USize(4));
}
