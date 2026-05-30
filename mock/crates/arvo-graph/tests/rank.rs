//! upward_rank / downward_rank correctness on known-shape DAGs.

use arvo::{FBits, ibits, USize};
use arvo::strategy::{Hot, Unsigned};
use arvo::Bits;
use arvo::ufixed::UFixed;
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_graph::{downward_rank, upward_rank};
use arvo_tensor::Dim;

// Small integer weight type. Hot strategy: wrapping arithmetic, u8
// container.
type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize→u8 cast for typed weight in concrete-W test scope; no runtime-FromConstant by design (round 202604271346); tracked: #256
    W::from_raw(n as u8)
}

#[test]
fn upward_rank_linear_chain() {
    // 0 -> 1 -> 2 -> 3, all weights = 1.
    // Upward: rank[3] = 1; rank[2] = 1 + 1 = 2; rank[1] = 3; rank[0] = 4.
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1); 4];
    let r = upward_rank(&dag, &weights);
    assert_eq!(r[3].to_raw(), 1);
    assert_eq!(r[2].to_raw(), 2);
    assert_eq!(r[1].to_raw(), 3);
    assert_eq!(r[0].to_raw(), 4);
}

#[test]
fn upward_rank_diamond_picks_max_branch() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3. Weights: 1, 5, 2, 1.
    // rank[3] = 1; rank[1] = 5 + 1 = 6; rank[2] = 2 + 1 = 3; rank[0] = 1 + max(6, 3) = 7.
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(3));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1), w(5), w(2), w(1)];
    let r = upward_rank(&dag, &weights);
    assert_eq!(r[3].to_raw(), 1);
    assert_eq!(r[1].to_raw(), 6);
    assert_eq!(r[2].to_raw(), 3);
    assert_eq!(r[0].to_raw(), 7);
}

#[test]
fn downward_rank_linear_chain() {
    // 0 -> 1 -> 2 -> 3, all weights = 1.
    // Downward: rank[0] = 1; rank[1] = 2; rank[2] = 3; rank[3] = 4.
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1); 4];
    let r = downward_rank(&dag, &weights);
    assert_eq!(r[0].to_raw(), 1);
    assert_eq!(r[1].to_raw(), 2);
    assert_eq!(r[2].to_raw(), 3);
    assert_eq!(r[3].to_raw(), 4);
}

#[test]
fn downward_rank_diamond_picks_max_predecessor() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3. Weights: 1, 5, 2, 1.
    // rank[0] = 1; rank[1] = 5 + 1 = 6; rank[2] = 2 + 1 = 3;
    // rank[3] = 1 + max(6, 3) = 7.
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(0), nid(2));
    dag.set_edge(nid(1), nid(3));
    dag.set_edge(nid(2), nid(3));
    let weights: [W; 4] = [w(1), w(5), w(2), w(1)];
    let r = downward_rank(&dag, &weights);
    assert_eq!(r[0].to_raw(), 1);
    assert_eq!(r[1].to_raw(), 6);
    assert_eq!(r[2].to_raw(), 3);
    assert_eq!(r[3].to_raw(), 7);
}

#[test]
fn rank_singleton() {
    // Single node, no edges. Rank is own weight in both directions.
    let dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<1>> = BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    let weights: [W; 1] = [w(7)];
    let up = upward_rank(&dag, &weights);
    let down = downward_rank(&dag, &weights);
    assert_eq!(up[0].to_raw(), 7);
    assert_eq!(down[0].to_raw(), 7);
}
