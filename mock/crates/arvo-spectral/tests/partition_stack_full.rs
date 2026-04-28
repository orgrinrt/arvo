//! Pins the post-review fix (F2) for `k_way_partition`: when the
//! push-back stack fills, the algorithm must NOT silently drop a
//! half. It must either push both (room for two) or fall back to
//! the larger half (room for one) — never admit one and lose the
//! other.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_spectral::{Matrix, k_way_partition};

mod common;
use common::TF;

const C8: Cap = Cap(USize(8));
const K4: Cap = Cap(USize(4));

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

fn four_cluster_weights_8() -> Matrix<u32, C8> {
    let mut m: Matrix<u32, C8> = Matrix::from_fn(|_, _| 0u32);
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
    let (count, ids) = k_way_partition::<C8, K4, u32, TF>(&w, USize(100));
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
    const K1: Cap = Cap(USize(1));
    let w = four_cluster_weights_8();
    let (count, _ids) = k_way_partition::<C8, K1, u32, TF>(&w, USize(100));
    assert_eq!(count, USize(1), "K=1 must produce exactly one partition");
}
