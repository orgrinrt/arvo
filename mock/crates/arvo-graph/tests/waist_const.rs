//! Tests for the const-callable `waist_detect_const`.
//!
//! Two lenses. First a forced-const-eval test: an hourglass DAG with
//! const-built successor words and a const topo order, run through
//! `waist_detect_const` in a `const` item, proving the const fn const-evaluates
//! over the `ConstCapacity` GAT + the const `BitAccess` contract and lands the
//! waist flag at the waist node's topo position. Second a runtime cross-check
//! that `waist_detect_const` agrees with the runtime `waist_detect`
//! position-for-position on shared fixtures.

#![feature(const_trait_impl)]

use arvo::{Additive, Bits, Hot, Identity, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_bits_contracts::BitAccess;
use arvo_graph::{topo_sort, waist_detect, waist_detect_const};
use arvo_tensor::Dim;

type W = Bits<64, Hot, Unsigned>;

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

// Build a successor word with the bits in `targets` set, const, via the const
// BitAccess contract. `targets` is a bitmask of successor node indices. The
// `j < 8` scan is fixture-scoped to this 8-node DAG (W = Bits<64> holds far
// more); extending the fixture past 8 nodes means widening this bound.
const fn mk_row(targets: u64) -> W {
    let mut w = <W as Identity<Additive>>::IDENTITY;
    let mut j = 0usize;
    while j < 8 {
        if (targets >> j) & 1 == 1 {
            w = w.with_bit_set(USize(j));
        }
        j += 1;
    }
    w
}

// Hourglass: level 0 {0} width 1, level 1 {1,2,3} width 3, level 2 {4} width 1
// (the waist), level 3 {5,6,7} width 3. Edges 0->{1,2,3}, {1,2,3}->4, 4->{5,6,7}.
const SUCC: [W; 8] = [
    mk_row(0b0000_1110), // 0 -> 1,2,3
    mk_row(0b0001_0000), // 1 -> 4
    mk_row(0b0001_0000), // 2 -> 4
    mk_row(0b0001_0000), // 3 -> 4
    mk_row(0b1110_0000), // 4 -> 5,6,7
    mk_row(0),
    mk_row(0),
    mk_row(0),
];

// Identity order respects every edge (sources precede targets), so it is a valid
// topo order. Node 4 (the waist) sits at position 4.
const ORDER: [NodeId; 8] = [
    NodeId(USize(0)),
    NodeId(USize(1)),
    NodeId(USize(2)),
    NodeId(USize(3)),
    NodeId(USize(4)),
    NodeId(USize(5)),
    NodeId(USize(6)),
    NodeId(USize(7)),
];

// Forced const evaluation of the whole detector.
const FLAGS: [arvo::Bool; 8] = waist_detect_const::<Dim<8>, W>(&SUCC, &ORDER);

#[test]
fn const_eval_lands_waist_flag_at_waist_position() {
    let mut set = 0;
    let mut i = 0;
    while i < 8 {
        if FLAGS[i].0 {
            set += 1;
        }
        i += 1;
    }
    assert_eq!(set, 1, "exactly one waist");
    assert!(
        FLAGS[4].0,
        "the waist flag lands at node 4's topo position (4)"
    );
}

// Build the raw successor words from a BitMatrix's rows (Mask is repr-transparent
// over `word`), so the const detector sees exactly the runtime detector's graph.
fn succ_from<const N: usize, C>(dag: &BitMatrix<W, C>) -> [W; N]
where
    C: arvo_tensor::Capacity<Array<W> = [W; N]>,
{
    let rows = dag.rows.as_ref();
    core::array::from_fn(|i| rows[i].word)
}

#[test]
fn cross_check_linear_chain_has_no_waist() {
    let mut dag: BitMatrix<W, Dim<4>> = BitMatrix::<W, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let (_, order) = topo_sort(&dag);
    let rt = waist_detect(&dag, &order);
    let succ = succ_from::<4, Dim<4>>(&dag);
    let ct = waist_detect_const::<Dim<4>, W>(&succ, &order);
    let mut k = 0;
    while k < 4 {
        assert_eq!(
            (*rt.contains(USize(k))),
            ct[k].0,
            "position {k} disagrees on linear chain"
        );
        k += 1;
    }
    assert_eq!(rt.count(), USize(0));
}

#[test]
fn cross_check_hourglass_matches_runtime() {
    // Level 0 {0,1} width 2, level 1 {2} width 1 (waist), level 2 {3,4} width 2.
    let mut dag: BitMatrix<W, Dim<5>> = BitMatrix::<W, _>::empty();
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    dag.set_edge(nid(2), nid(4));
    let (_, order) = topo_sort(&dag);
    let rt = waist_detect(&dag, &order);
    let succ = succ_from::<5, Dim<5>>(&dag);
    let ct = waist_detect_const::<Dim<5>, W>(&succ, &order);
    let mut k = 0;
    while k < 5 {
        assert_eq!(
            (*rt.contains(USize(k))),
            ct[k].0,
            "position {k} disagrees on hourglass"
        );
        k += 1;
    }
    assert_eq!(rt.count(), USize(1));
}
