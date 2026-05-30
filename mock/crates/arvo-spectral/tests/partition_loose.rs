//! Loose-CSR live-node-count awareness for spectral partitioning.
//!
//! A `SparseLaplacian` over a loose `Csr` (live rows below the const
//! cap) must partition only the live nodes. The empty slack rows are
//! degree-0 isolated nodes; if the algorithms iterated `cap_size(C::CAP)`
//! they would each be a zero-eigenvalue connected component, degenerate
//! the Fiedler vector, and consume the partition budget. `live_dim()`
//! excludes them.

// `adt_const_params` is required by the `common::TF` `FromConstant` impl
// (`from_constant<const C: USize>`), not by capacity arithmetic. The
// migration dropped `generic_const_exprs`; this gate is independent of it.
#![feature(adt_const_params)]

use arvo::USize;
use arvo_bitmask::NodeId;
use arvo_sparse::Csr;
use arvo_spectral::{SparseLaplacian, k_way_partition};
use arvo_tensor::Dim;

mod common;
use common::TF;

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

/// Two heavy 2-node clusters joined by one weak bridge, stored as a
/// symmetric loose CSR: 4 live nodes in an 8-row cap (4 isolated slack
/// rows). Edges 0-1 and 2-3 carry weight 10; the bridge 1-2 carries
/// weight 1. The Fiedler cut of the live graph runs through the bridge.
fn loose_bridged_clusters() -> Csr<Dim<8>, Dim<16>, u32> {
    // live_rows = 4, live_nnz = 6 (1 + 2 + 2 + 1 directed entries).
    let mut csr: Csr<Dim<8>, Dim<16>, u32> = Csr::with_live_counts(USize(4), USize(6));
    csr.row_ptr[0] = USize(0);
    csr.row_ptr[1] = USize(1);
    csr.row_ptr[2] = USize(3);
    csr.row_ptr[3] = USize(5);
    // row 0: neighbour 1 (w10)
    csr.col_idx[0] = NodeId::new(USize(1));
    csr.values[0] = 10;
    // row 1: neighbours 0 (w10), 2 (w1, the bridge)
    csr.col_idx[1] = NodeId::new(USize(0));
    csr.values[1] = 10;
    csr.col_idx[2] = NodeId::new(USize(2));
    csr.values[2] = 1;
    // row 2: neighbours 1 (w1, the bridge), 3 (w10)
    csr.col_idx[3] = NodeId::new(USize(1));
    csr.values[3] = 1;
    csr.col_idx[4] = NodeId::new(USize(3));
    csr.values[4] = 10;
    // row 3: neighbour 2 (w10)
    csr.col_idx[5] = NodeId::new(USize(2));
    csr.values[5] = 10;
    csr
}

#[test]
fn loose_csr_partitions_only_live_nodes() {
    let csr = loose_bridged_clusters();
    let lap: SparseLaplacian<Dim<8>, Dim<16>, u32, TF> = SparseLaplacian::new(&csr);
    let sigma = lap.gershgorin_lambda_max();
    let (count, ids) = k_way_partition::<_, Dim<8>, Dim<2>, TF>(&lap, sigma, USize(100));

    // Exactly two partitions: the live graph is one connected component
    // (the bridge keeps it connected), so the Fiedler cut splits it in
    // two through the weak bridge. The 4 slack rows never form their own
    // partitions because `live_dim()` is 4, not the cap of 8.
    assert_eq!(count, USize(2), "live graph bisects into two partitions");
    assert_eq!(ids[0], ids[1], "cluster A (nodes 0,1) stays together");
    assert_eq!(ids[2], ids[3], "cluster B (nodes 2,3) stays together");
    assert!(ids[0] != ids[2], "the two clusters land in different partitions");

    // The 4 slack rows (4..8) are never partitioned: live_dim() = 4
    // excludes them, so they keep the initial partition id 0. Under the
    // pre-fix cap_size(N) bug they are isolated zero-eigenvalue nodes
    // that the bisection splits off into a non-zero partition; this is
    // the assertion that flips when the algorithms iterate the cap
    // instead of the live count.
    assert_eq!(ids[4], USize(0), "slack row 4 not partitioned");
    assert_eq!(ids[5], USize(0), "slack row 5 not partitioned");
    assert_eq!(ids[6], USize(0), "slack row 6 not partitioned");
    assert_eq!(ids[7], USize(0), "slack row 7 not partitioned");
}
