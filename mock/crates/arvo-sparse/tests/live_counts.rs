//! Live-count CSR: slack-vs-tight equivalence and transpose pollution.
//!
//! A capacity-with-slack `Csr` (a small graph living in a larger cap)
//! must drive the trait-driven algorithms identically to the same graph
//! in a tight cap, and `with_transpose` must not count the unfilled tail
//! (default `NodeId(0)` slots) as edges into node 0.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_bitmask::NodeId;
use arvo_sparse::{
    BidirectionalSparseAdjacency, Csr, CsrBidirectional, SparseAdjacency, block_diagonal_via,
    dulmage_mendelsohn_via, rcm_reorder_via,
};

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

const R2: Cap = cap(2);
const R3: Cap = cap(3);
const R4: Cap = cap(4);
const NNZ2: Cap = cap(2);
const NNZ3: Cap = cap(3);
const NNZ8: Cap = cap(8);

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

/// Collect predecessor node indices into a fixed buffer (no alloc).
/// Returns the count plus the buffer (sources arrive in ascending order
/// from the count-prefix-scatter, so a direct prefix compare is valid).
fn preds<const ROWS: Cap, const NNZ: Cap>(
    bidi: &CsrBidirectional<ROWS, NNZ, u32>,
    node: usize,
) -> (usize, [usize; 8])
where
    [(); arvo_bitmask::cap_size(ROWS)]:,
    [(); arvo_bitmask::cap_size(NNZ)]:,
{
    let mut buf = [usize::MAX; 8];
    let mut n = 0;
    for p in bidi.predecessors(nid(node)) {
        buf[n] = (p.0).0;
        n += 1;
    }
    (n, buf)
}

/// Two-node cycle (0 -> 1, 1 -> 0) living in a cap(4)/cap(8) buffer with
/// only 2 live rows and 2 live edges. The 6 trailing `col_idx` slots are
/// the constructor default `NodeId(0)`.
fn build_loose_2cycle() -> CsrBidirectional<R4, NNZ8, u32> {
    let mut csr: Csr<R4, NNZ8, u32> = Csr::with_live_counts(USize(2), USize(2));
    csr.row_ptr = [USize(0), USize(1), USize(0), USize(0)];
    csr.col_idx = [
        nid(1),
        nid(0),
        nid(0),
        nid(0),
        nid(0),
        nid(0),
        nid(0),
        nid(0),
    ];
    csr.with_transpose()
}

/// The same two-node cycle in a tight cap(2)/cap(2) packed buffer.
fn build_tight_2cycle() -> CsrBidirectional<R2, NNZ2, u32> {
    let mut csr: Csr<R2, NNZ2, u32> = Csr::new();
    csr.row_ptr = [USize(0), USize(1)];
    csr.col_idx = [nid(1), nid(0)];
    csr.with_transpose()
}

/// Three-node cycle (0 -> 1 -> 2 -> 0) packed in a tight cap(3)/cap(3).
fn build_tight_3cycle() -> CsrBidirectional<R3, NNZ3, u32> {
    let mut csr: Csr<R3, NNZ3, u32> = Csr::new();
    csr.row_ptr = [USize(0), USize(1), USize(2)];
    csr.col_idx = [nid(1), nid(2), nid(0)];
    csr.with_transpose()
}

#[test]
fn transpose_packed_unchanged() {
    // Reverse edges of 0->1->2->0: 0<-2, 1<-0, 2<-1.
    let bidi = build_tight_3cycle();
    let (c0, b0) = preds(&bidi, 0);
    assert_eq!(c0, 1);
    assert_eq!(b0[0], 2);
    let (c1, b1) = preds(&bidi, 1);
    assert_eq!(c1, 1);
    assert_eq!(b1[0], 0);
    let (c2, b2) = preds(&bidi, 2);
    assert_eq!(c2, 1);
    assert_eq!(b2[0], 1);
}

#[test]
fn transpose_slack_no_node0_pollution() {
    let loose = build_loose_2cycle();
    // Node 0's only predecessor is node 1 (1 -> 0). The 6 unfilled tail
    // slots (default NodeId(0)) must NOT be counted as edges into node
    // 0; a packed transpose over the full cap would report 7.
    let (count0, buf0) = preds(&loose, 0);
    assert_eq!(count0, 1, "node 0 predecessor count polluted by slack tail");
    assert_eq!(buf0[0], 1);
    let (count1, buf1) = preds(&loose, 1);
    assert_eq!(count1, 1);
    assert_eq!(buf1[0], 0);
    // Nodes beyond the live range have no predecessors.
    assert_eq!(preds(&loose, 2).0, 0);
    assert_eq!(preds(&loose, 3).0, 0);
}

#[test]
fn transpose_slack_equals_tight() {
    let loose = build_loose_2cycle();
    let tight = build_tight_2cycle();
    for node in 0..2 {
        let (lc, lb) = preds(&loose, node);
        let (tc, tb) = preds(&tight, node);
        assert_eq!(lc, tc, "predecessor count differs at node {}", node);
        assert_eq!(lb[..lc], tb[..tc], "predecessor set differs at node {}", node);
    }
}

#[test]
fn node_count_reflects_live() {
    // Loose bidirectional reports its live row count, not the cap.
    let loose = build_loose_2cycle();
    assert_eq!(loose.node_count().0, 2);
    // A packed Csr reports the cap (existing consumers unchanged).
    let packed: Csr<R4, NNZ8, u32> = Csr::new();
    assert_eq!(packed.node_count().0, 4);
}

#[test]
fn rcm_via_slack_equals_tight() {
    let loose = build_loose_2cycle();
    let tight = build_tight_2cycle();
    let loose_perm = rcm_reorder_via::<CsrBidirectional<R4, NNZ8, u32>, R4>(&loose);
    let tight_perm = rcm_reorder_via::<CsrBidirectional<R2, NNZ2, u32>, R2>(&tight);
    let l = [(loose_perm[0].0).0, (loose_perm[1].0).0];
    let t = [(tight_perm[0].0).0, (tight_perm[1].0).0];
    // Min-degree start (node 0), BFS, reverse: [1, 0].
    assert_eq!(l, [1, 0]);
    assert_eq!(l, t);
}

#[test]
fn block_via_slack_equals_tight() {
    let loose = build_loose_2cycle();
    let tight = build_tight_2cycle();
    let (loose_n, loose_blocks) = block_diagonal_via::<CsrBidirectional<R4, NNZ8, u32>, R4>(&loose);
    let (tight_n, tight_blocks) = block_diagonal_via::<CsrBidirectional<R2, NNZ2, u32>, R2>(&tight);
    // Both nodes are connected: one block.
    assert_eq!(loose_n.0, 1);
    assert_eq!(tight_n.0, 1);
    assert_eq!([loose_blocks[0].0, loose_blocks[1].0], [0, 0]);
    assert_eq!(
        [loose_blocks[0].0, loose_blocks[1].0],
        [tight_blocks[0].0, tight_blocks[1].0]
    );
}

#[test]
fn dm_via_slack_equals_tight() {
    let loose = build_loose_2cycle();
    let tight = build_tight_2cycle();
    let loose_dm = dulmage_mendelsohn_via::<CsrBidirectional<R4, NNZ8, u32>, R4>(&loose);
    let tight_dm = dulmage_mendelsohn_via::<CsrBidirectional<R2, NNZ2, u32>, R2>(&tight);
    assert_eq!(loose_dm.class_count.0, 3);
    // Both nodes have incoming and outgoing edges: class 2 (square).
    assert_eq!(loose_dm.class[0].0, 2);
    assert_eq!(loose_dm.class[1].0, 2);
    assert_eq!(
        [loose_dm.class[0].0, loose_dm.class[1].0],
        [tight_dm.class[0].0, tight_dm.class[1].0]
    );
}
