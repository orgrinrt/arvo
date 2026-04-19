//! Block-diagonal (connected component) detection.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::newtype::USize;
use arvo_bitmask::{BitMatrix64, NodeId};
use arvo_sparse::block_diagonal;

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

#[test]
fn single_chain_is_one_block() {
    let mut adj: BitMatrix64<4> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));
    adj.set_edge(nid(2), nid(3));

    let (count, ids) = block_diagonal(&adj);
    assert_eq!(count.0, 1);
    for i in 0..4 {
        assert_eq!(ids[i], ids[0]);
    }
}

#[test]
fn two_disjoint_components() {
    // Chain 0-1-2 and isolated 3.
    let mut adj: BitMatrix64<4> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));

    let (count, ids) = block_diagonal(&adj);
    assert_eq!(count.0, 2);
    assert_eq!(ids[0], ids[1]);
    assert_eq!(ids[1], ids[2]);
    assert!(ids[3] != ids[0]);
}

#[test]
fn all_isolated() {
    let adj: BitMatrix64<4> = BitMatrix64::empty();
    let (count, ids) = block_diagonal(&adj);
    assert_eq!(count.0, 4);
    // Every pair must be distinct.
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert!(ids[i] != ids[j]);
        }
    }
}

#[test]
fn diamond_is_one_block() {
    let mut adj: BitMatrix64<4> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(0), nid(2));
    adj.set_edge(nid(1), nid(3));
    adj.set_edge(nid(2), nid(3));

    let (count, ids) = block_diagonal(&adj);
    assert_eq!(count.0, 1);
    for i in 1..4 {
        assert_eq!(ids[i], ids[0]);
    }
}

#[test]
fn two_disjoint_edges() {
    // 0 -> 1, 2 -> 3.
    let mut adj: BitMatrix64<4> = BitMatrix64::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(2), nid(3));

    let (count, ids) = block_diagonal(&adj);
    assert_eq!(count.0, 2);
    assert_eq!(ids[0], ids[1]);
    assert_eq!(ids[2], ids[3]);
    assert!(ids[0] != ids[2]);
}
