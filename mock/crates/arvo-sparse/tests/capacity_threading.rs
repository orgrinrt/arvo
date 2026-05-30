//! Generic-threading proof for the `Capacity`-migrated arvo-sparse algorithms.
//!
//! The migrated functions run when the capacity is threaded through a caller's
//! OWN generic `C: Capacity` (not a concrete `Dim<N>` fixed at the call site).
//! That is the shape #652 (hilavitkutin threading `PlanDims` capacities) needs,
//! and the exact shape that overflowed `generic_const_exprs` under the
//! `const N: Cap` form. No `#![feature(...)]` gate: its absence is the proof
//! the algorithms escaped the GCE surface.

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_sparse::{Csr, SparseAdjacency, block_diagonal, rcm_reorder};
use arvo_tensor::{Capacity, Dim};

type W = Bits<64, Hot, Unsigned>;

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

// rcm_reorder threaded through a caller's own `C: Capacity` over a BitMatrix
// adjacency. Builds a single-chain 0 -> 1 -> ... and confirms the reversed
// permutation covers every live node exactly once. Generic in `C`, no concrete
// cap fixed here.
fn rcm_chain_covers_all<C: Capacity>(n: usize) -> bool
where
    C::Array<NodeId>: Copy,
{
    let mut adj: BitMatrix<W, C> = BitMatrix::<W, _>::empty();
    let mut e = 0usize;
    while e + 1 < n {
        adj.set_edge(nid(e), nid(e + 1));
        e += 1;
    }
    let perm = rcm_reorder::<W, C>(&adj);
    // Every index in 0..n must appear exactly once in the permutation prefix.
    let slice = perm.as_ref();
    let mut seen = [false; 64];
    let mut i = 0usize;
    while i < n {
        let idx = (slice[i].0).0;
        if idx >= n || seen[idx] {
            return false;
        }
        seen[idx] = true;
        i += 1;
    }
    let mut all = true;
    let mut j = 0usize;
    while j < n {
        all = all && seen[j];
        j += 1;
    }
    all
}

#[test]
fn rcm_threads_generically() {
    assert!(rcm_chain_covers_all::<Dim<4>>(4));
    assert!(rcm_chain_covers_all::<Dim<6>>(6));
}

// block_diagonal threaded through a caller's own `C: Capacity`. Two disjoint
// edges in an N-node graph produce two components; this exercises the
// `C::Array<USize>` block-id storage generically.
fn block_count_two_edges<C: Capacity>() -> usize
where
    C::Array<USize>: Copy,
{
    let mut adj: BitMatrix<W, C> = BitMatrix::<W, _>::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(2), nid(3));
    let (count, _ids) = block_diagonal::<W, C>(&adj);
    count.0
}

#[test]
fn block_diagonal_threads_generically() {
    // Dim<4>: nodes {0,1} and {2,3} are the only two components.
    assert_eq!(block_count_two_edges::<Dim<4>>(), 2);
    // Dim<5>: same two edges plus isolated node 4 = three components.
    assert_eq!(block_count_two_edges::<Dim<5>>(), 3);
}

// Csr build + SparseAdjacency walk threaded through caller-owned R / NNZ
// capacities. Confirms the GAT-backed `Csr` storage and the trait-driven
// successor walk run generically (no concrete cap fixed at the call site).
fn csr_successor_count<R: Capacity, NNZ: Capacity>() -> usize
where
    R::Array<USize>: Copy,
    NNZ::Array<NodeId>: Copy,
    NNZ::Array<u32>: Copy,
{
    // Row 0 has two successors (cols 1, 2); rows beyond keep the packed
    // default. Build a 3-row, 2-nnz CSR with row 0 -> {1, 2}.
    let mut csr: Csr<R, NNZ, u32> = Csr::new();
    csr.row_ptr.as_mut()[0] = USize(0);
    // pack the two edges into row 0; remaining rows empty by pointing at nnz end.
    let nnz = NNZ::CAP;
    let mut r = 1usize;
    while r < arvo_bitmask::cap_size(R::CAP) {
        csr.row_ptr.as_mut()[r] = USize(arvo_bitmask::cap_size(nnz));
        r += 1;
    }
    if arvo_bitmask::cap_size(NNZ::CAP) >= 2 {
        csr.col_idx.as_mut()[0] = nid(1);
        csr.col_idx.as_mut()[1] = nid(2);
    }
    csr.successors(nid(0)).count()
}

#[test]
fn csr_threads_generically() {
    assert_eq!(csr_successor_count::<Dim<3>, Dim<2>>(), 2);
    assert_eq!(csr_successor_count::<Dim<4>, Dim<2>>(), 2);
}
