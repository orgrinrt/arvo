//! Reverse Cuthill-McKee correctness.

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_sparse::rcm_reorder;
use arvo_tensor::Dim;

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

#[test]
fn permutation_is_a_permutation() {
    // Any non-empty graph: the output covers every node index
    // exactly once.
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, Dim<6>> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));
    adj.set_edge(nid(2), nid(3));
    adj.set_edge(nid(3), nid(4));
    adj.set_edge(nid(4), nid(5));

    let perm = rcm_reorder(&adj);
    let mut seen = [false; 6];
    for p in perm.iter() {
        let idx = (p.0).0;
        assert!(idx < 6);
        assert!(!seen[idx], "node {} visited twice", idx);
        seen[idx] = true;
    }
    for s in seen.iter() {
        assert!(*s);
    }
}

#[test]
fn linear_chain_is_reversed() {
    // 0 -> 1 -> 2 -> 3. Min-degree node by combined succ+pred is
    // either endpoint (degree 1). Tie-break picks node 0. BFS yields
    // [0, 1, 2, 3]; reversed gives [3, 2, 1, 0].
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));
    adj.set_edge(nid(2), nid(3));

    let perm = rcm_reorder(&adj);
    assert_eq!((perm[0].0).0, 3);
    assert_eq!((perm[1].0).0, 2);
    assert_eq!((perm[2].0).0, 1);
    assert_eq!((perm[3].0).0, 0);
}

#[test]
fn disconnected_graph_includes_all_nodes() {
    // Two separate chains. Every node still appears exactly once.
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, Dim<6>> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    adj.set_edge(nid(0), nid(1));
    adj.set_edge(nid(1), nid(2));
    adj.set_edge(nid(3), nid(4));
    adj.set_edge(nid(4), nid(5));

    let perm = rcm_reorder(&adj);
    let mut seen = [false; 6];
    for p in perm.iter() {
        let idx = (p.0).0;
        assert!(idx < 6);
        seen[idx] = true;
    }
    for (i, s) in seen.iter().enumerate() {
        assert!(*s, "node {} missing from permutation", i);
    }
}

#[test]
fn isolated_nodes_are_permuted() {
    // No edges: every node is visited via the "remaining min-degree"
    // seed loop.
    let adj: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    let perm = rcm_reorder(&adj);
    let mut seen = [false; 4];
    for p in perm.iter() {
        let idx = (p.0).0;
        assert!(idx < 4);
        seen[idx] = true;
    }
    for s in seen.iter() {
        assert!(*s);
    }
}
