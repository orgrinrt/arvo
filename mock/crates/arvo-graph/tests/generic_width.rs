//! DAG algorithms over row-words wider than 64 nodes (#663).
//!
//! Every algorithm used to pin `Bits<64, Hot, Unsigned>` as the adjacency
//! word, capping graphs at 64 nodes. These tests run graphs past that cap
//! through the generalized signatures at two wider words: a native 128-bit
//! container (`Bits<128, Hot, Unsigned>`, 100-plus nodes) and a wide-bucket
//! container (`Bits<256, Hot, Unsigned>`, 129 nodes addressing bit 128).
//!
//! Red first: against the pinned `Bits<64>` signatures these calls fail to
//! typecheck (the `BitMatrix` word does not match). They compile once the
//! algorithms carry a row-word type parameter, and the assertions then prove
//! the wider word is genuinely used: a `Bits<64>` mask could not represent
//! node indices 64 through 128, so the connected-component closure and the
//! waist position would be wrong if any body still narrowed to 64.
//!
//! Lives under `tests/` so the bare numeric node indices read as graph
//! structure without the src-tree primitive discipline.

use arvo::{Bits, FBits, Hot, UFixed, USize, Unsigned, ibits};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_graph::{
    components, downward_rank, longest_path, spanning_tree, topo_sort, upward_rank, waist_detect,
};
use arvo_tensor::Dim;

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

// Small wrapping 8-bit weight, matching the per-function rank/spanning tests.
type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;

fn w(n: usize) -> W {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper; runtime usize->u8 cast for typed weight in concrete-W test scope; tracked: #256
    W::from_raw(n as u8)
}

/// The 129-node bowtie at the wide-bucket word: 64 sources (0..63) fan into the
/// middle (node 64), which fans out to 64 sinks (65..128). Capacity is exactly
/// 129 so no isolated padding shifts the topo order.
fn bowtie_256() -> BitMatrix<Bits<256, Hot, Unsigned>, Dim<129>> {
    let mut dag: BitMatrix<Bits<256, Hot, Unsigned>, Dim<129>> = BitMatrix::empty();
    let mut s = 0usize;
    while s < 64 {
        dag.set_edge(nid(s), nid(64));
        s += 1;
    }
    let mut t = 65usize;
    while t < 129 {
        dag.set_edge(nid(64), nid(t));
        t += 1;
    }
    dag
}

/// A full 128-node chain at the native 128-bit word. Every node index 0..127
/// is on the chain (no isolated padding), so the topo order is the identity
/// permutation and the whole chain is one connected component. Node 127 sits
/// at bit 127, beyond a `Bits<64>` mask.
#[test]
fn chain_128_native_word() {
    let mut dag: BitMatrix<Bits<128, Hot, Unsigned>, Dim<128>> = BitMatrix::empty();
    let mut i = 0usize;
    while i < 127 {
        dag.set_edge(nid(i), nid(i + 1));
        i += 1;
    }

    // Topological sort: acyclic, so all 128 nodes place, and a pure chain
    // sorts into the identity order.
    let (valid, order) = topo_sort(&dag);
    assert_eq!(valid, USize(128), "the 128-node chain is acyclic; all place");
    let mut k = 0usize;
    while k < 128 {
        assert_eq!(order[k], nid(k), "a pure chain sorts in identity order");
        k += 1;
    }

    // Connected components: one undirected closure over the whole chain. The
    // DFS visited mask must reach bit 127, impossible at `Bits<64>`.
    let comp = components(&dag);
    let mut j = 1usize;
    while j < 128 {
        assert_eq!(comp[j], comp[0], "the chain is a single component");
        j += 1;
    }
}

/// A 129-node bowtie at the wide-bucket word: 64 sources fan into one middle
/// (node 64), which fans out to 64 sinks (nodes 65..128). The capacity is
/// exactly 129 so no isolated padding shifts the topo order. The middle is
/// the sole waist; node 128 forces the visited mask past bit 127 into the
/// wide bucket.
#[test]
fn bowtie_129_wide_bucket_word() {
    let mut dag: BitMatrix<Bits<256, Hot, Unsigned>, Dim<129>> = BitMatrix::empty();
    let mut s = 0usize;
    while s < 64 {
        dag.set_edge(nid(s), nid(64));
        s += 1;
    }
    let mut t = 65usize;
    while t < 129 {
        dag.set_edge(nid(64), nid(t));
        t += 1;
    }

    let (valid, order) = topo_sort(&dag);
    assert_eq!(valid, USize(129), "the bowtie is acyclic; all 129 place");

    // Connected components: every node reaches every other through the
    // middle, so all 129 share one component. Visiting node 128 needs a mask
    // wider than 128 bits (the wide bucket).
    let comp = components(&dag);
    let mut j = 1usize;
    while j < 129 {
        assert_eq!(comp[j], comp[0], "the bowtie is a single component");
        j += 1;
    }

    // Waist detection: source level width 64, middle level width 1, sink
    // level width 64. The middle (topo position 64) is the sole strict local
    // minimum, so exactly one waist bit is set, at position 64.
    let waist = waist_detect(&dag, &order);
    assert_eq!(waist.count(), USize(1), "the bowtie has exactly one waist");
    assert!(
        *waist.contains(USize(64)),
        "the waist is the bowtie middle at topo position 64",
    );
}

/// The four weighted/rank algorithms (`upward_rank`, `downward_rank`,
/// `longest_path`, `spanning_tree`) on the wide-bucket bowtie with unit
/// weights. Every assertion reads node indices at or beyond 64 (and beyond 128
/// for the wide-bucket path), so a row-word narrowed back to 64 would fail it.
#[test]
fn rank_path_spanning_wide_bucket() {
    let dag = bowtie_256();
    let weights: [W; 129] = [w(1); 129];

    // upward_rank = weight + max successor rank. Sinks ground at 1, the middle
    // adds the max sink rank (2), a source adds the middle rank (3).
    let up = upward_rank(&dag, &weights);
    assert_eq!(up[65].to_raw(), 1, "a sink grounds at its own weight");
    assert_eq!(up[64].to_raw(), 2, "the middle adds the max sink rank");
    assert_eq!(up[0].to_raw(), 3, "a source adds the middle rank");

    // downward_rank = weight + max predecessor rank. Sources ground at 1, the
    // middle adds the max source rank (2), a sink adds the middle rank (3). The
    // sink read is at index 128, in the wide bucket.
    let down = downward_rank(&dag, &weights);
    assert_eq!(down[0].to_raw(), 1, "a source grounds at its own weight");
    assert_eq!(down[64].to_raw(), 2, "the middle adds the max source rank");
    assert_eq!(down[128].to_raw(), 3, "a sink adds the middle rank");

    // longest_path = source -> middle -> sink, weight 3. The middle and the
    // sinks carry a predecessor; the sources do not. has_pred reaches bit 128.
    let (placed, topo) = topo_sort(&dag);
    assert_eq!(placed, USize(129), "the bowtie is acyclic; all 129 place");
    let (overall, has_pred, _pred_of) = longest_path(&dag, &weights, &topo);
    assert_eq!(overall.to_raw(), 3, "the longest weighted path is source-middle-sink");
    assert!(*has_pred.contains(USize(64)), "the middle has a predecessor");
    assert!(
        *has_pred.contains(USize(128)),
        "a wide-bucket sink has a predecessor"
    );
    assert!(!*has_pred.contains(USize(0)), "a source has no predecessor");

    // spanning_tree: the middle (64 predecessors) is the sole fan-in bridge;
    // the head source, the middle, and a wide-bucket sink all lie on a trunk.
    let tree = spanning_tree(&dag, &up);
    assert_eq!(tree.bridges.count(), USize(1), "the middle is the only fan-in node");
    assert!(*tree.bridges.contains(USize(64)), "the bridge is the bowtie middle");
    assert!(*tree.on_trunk.contains(USize(0)), "the head source is on a trunk");
    assert!(*tree.on_trunk.contains(USize(64)), "the middle is on a trunk");
    assert!(
        *tree.on_trunk.contains(USize(128)),
        "a wide-bucket sink is on a trunk"
    );
}
