//! Generic-threading proof for the `Capacity`-migrated `BitMatrix`.
//!
//! `BitMatrix<W, C>` builds and walks when the row-count capacity is threaded
//! through a caller's OWN generic `C: Capacity` (not a concrete `Dim<N>` fixed
//! at the call site). That is the shape #652 needs and the form that overflowed
//! `generic_const_exprs` under `const N: Cap`. No `#![feature(...)]` gate: its
//! absence is the proof the chassis escaped the GCE surface.

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, Mask, NodeId, propagate_dirty};
use arvo_tensor::{Capacity, Dim};

type W = Bits<64, Hot, Unsigned>;

// Build a BitMatrix generic over the row capacity, set edges, propagate a dirty
// seed, count reached nodes. Generic over `C: Capacity` end to end.
fn dirty_reach<C: Capacity>(edges: &[(usize, usize)], seed: usize) -> usize {
    let mut m: BitMatrix<W, C> = BitMatrix::empty();
    for &(a, b) in edges {
        m.set_edge(NodeId(USize(a)), NodeId(USize(b)));
    }
    let mut dirty = Mask::<W>::empty();
    dirty.insert(USize(seed));
    propagate_dirty(&m, &mut dirty);
    dirty.count().0
}

#[test]
fn bitmatrix_threads_generically() {
    // 0->1->2->3 chain at Dim<4>: dirty 0 reaches all 4.
    assert_eq!(dirty_reach::<Dim<4>>(&[(0, 1), (1, 2), (2, 3)], 0), 4);
    // 0->1->2 at Dim<8>: dirty 0 reaches 3, the slack rows untouched.
    assert_eq!(dirty_reach::<Dim<8>>(&[(0, 1), (1, 2)], 0), 3);
}
