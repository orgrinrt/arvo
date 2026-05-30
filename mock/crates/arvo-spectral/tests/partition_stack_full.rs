//! Pins the post-review fix (F2) for `k_way_partition`: when the
//! push-back stack fills, the algorithm must NOT silently drop a
//! half. It must either push both (room for two) or fall back to
//! the larger half (room for one). Never admit one and lose the
//! other.

// `adt_const_params` is required by the `common::TF` `FromConstant` impl
// (`from_constant<const C: USize>`), not by capacity arithmetic. The
// migration dropped `generic_const_exprs`; this gate is independent of it.
#![feature(adt_const_params)]

use arvo::USize;
use arvo_spectral::{Matrix, dense_laplacian_lambda_max_bound, k_way_partition, laplacian};
use arvo_tensor::Dim;

mod common;
use common::TF;

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

fn four_cluster_weights_8() -> Matrix<u32, Dim<8>> {
    let mut m: Matrix<u32, Dim<8>> = Matrix::from_fn(|_, _| 0u32);
    // Four 2-node clusters with heavy intra-cluster edges.
    for pair in [(0, 1), (2, 3), (4, 5), (6, 7)] {
        m.set(USize(pair.0), USize(pair.1), 20);
        m.set(USize(pair.1), USize(pair.0), 20);
    }
    // Weak inter-cluster bridges.
    for pair in [(1, 2), (3, 4), (5, 6)] {
        m.set(USize(pair.0), USize(pair.1), 1);
        m.set(USize(pair.1), USize(pair.0), 1);
    }
    m
}

#[test]
fn k_way_fills_stack_without_dropping_halves() {
    // K=4 on a 4-cluster, 8-node graph. The recursion stack reaches
    // maximum depth mid-run. Pre-fix the algorithm silently dropped
    // a half; post-fix every node is assigned to some partition.
    let w = four_cluster_weights_8();
    let lap: Matrix<TF, Dim<8>> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let (count, ids) = k_way_partition::<_, Dim<8>, Dim<4>, TF>(&lap, sigma, USize(100));
    assert!(*count >= 2, "should produce at least 2 partitions, got {count:?}");
    assert!(*count <= 4, "partition count capped by K=4, got {count:?}");
    // Every node has a valid partition id (< count).
    for i in 0..8usize {
        assert!(
            *ids[i] < *count,
            "node {i} has id {:?} >= count {count:?}",
            ids[i]
        );
    }
}

#[test]
fn k_way_k_equals_one_produces_single_partition() {
    // Edge case: K=1 means no bisection happens at all. Stack
    // never grows past the initial component, so the fix's new
    // guard must still let this case through cleanly.
    let w = four_cluster_weights_8();
    let lap: Matrix<TF, Dim<8>> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let (count, _ids) = k_way_partition::<_, Dim<8>, Dim<1>, TF>(&lap, sigma, USize(100));
    assert_eq!(count, USize(1), "K=1 must produce exactly one partition");
}
