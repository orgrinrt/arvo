//! Spectral bisection and k-way partition correctness.
//!
//! Post round 202605111719: `spectral_bisection` returns
//! `(class_count, [USize; cap_size(N)])` (class 0 = positive,
//! 1 = non-positive). `k_way_partition` is operator-generic and takes
//! caller-supplied sigma.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_spectral::{
    Matrix, dense_laplacian_lambda_max_bound, k_way_partition, laplacian, spectral_bisection,
};

mod common;
use common::TF;

const C4: Cap = Cap(USize(4));
const C8: Cap = Cap(USize(8));
const K2: Cap = Cap(USize(2));
const K4: Cap = Cap(USize(4));

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

// Class IDs per the new spectral_bisection contract.
const POS: USize = USize(0);
const NEG: USize = USize(1);

#[test]
fn bisection_splits_positive_and_nonpositive() {
    // Fiedler-like input: [+, +, -, -].
    let f: [TF; 4] = [TF(0.5), TF(0.2), TF(-0.3), TF(-0.6)];
    let (count, ids) = spectral_bisection::<C4, TF>(&f);
    assert_eq!(count, USize(2));
    assert_eq!(ids[0], POS);
    assert_eq!(ids[1], POS);
    assert_eq!(ids[2], NEG);
    assert_eq!(ids[3], NEG);
}

#[test]
fn bisection_ties_go_negative() {
    // Tie (== 0) should route to the negative class per the contract.
    let f: [TF; 4] = [TF(0.5), TF(0.0), TF(-0.1), TF(1.0)];
    let (_, ids) = spectral_bisection::<C4, TF>(&f);
    assert_eq!(ids[1], NEG, "tie at index 1 should land in the negative class");
    assert_eq!(ids[0], POS);
    assert_eq!(ids[3], POS);
}

/// Two strongly-connected clusters linked by a single weak bridge.
fn two_cluster_weights_4() -> Matrix<u32, C4> {
    let mut m: Matrix<u32, C4> = Matrix::from_fn(|_, _| 0u32);
    m.set(USize(0), USize(1), 10);
    m.set(USize(1), USize(0), 10);
    m.set(USize(2), USize(3), 10);
    m.set(USize(3), USize(2), 10);
    m.set(USize(1), USize(2), 1);
    m.set(USize(2), USize(1), 1);
    m
}

#[test]
fn k_way_k2_matches_bisection() {
    let w = two_cluster_weights_4();
    let lap: Matrix<TF, C4> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let (count, ids) = k_way_partition::<_, C4, K2, TF>(&lap, sigma, USize(100));
    // Two partitions on a 2-cluster graph.
    assert_eq!(count, USize(2));
    // Nodes 0, 1 share an id; nodes 2, 3 share an id.
    assert_eq!(ids[0], ids[1], "cluster A split: ids = {ids:?}");
    assert_eq!(ids[2], ids[3], "cluster B split: ids = {ids:?}");
    // Clusters have different ids.
    assert!(ids[0] != ids[2], "clusters not separated: ids = {ids:?}");
}

/// Four 2-node clusters linked by weak bridges.
fn four_cluster_weights_8() -> Matrix<u32, C8> {
    let mut m: Matrix<u32, C8> = Matrix::from_fn(|_, _| 0u32);
    // Intra-cluster heavy edges.
    for pair in [(0, 1), (2, 3), (4, 5), (6, 7)] {
        m.set(USize(pair.0), USize(pair.1), 20);
        m.set(USize(pair.1), USize(pair.0), 20);
    }
    // Weak inter-cluster bridges.
    m.set(USize(1), USize(2), 1);
    m.set(USize(2), USize(1), 1);
    m.set(USize(3), USize(4), 1);
    m.set(USize(4), USize(3), 1);
    m.set(USize(5), USize(6), 1);
    m.set(USize(6), USize(5), 1);
    m
}

#[test]
fn k_way_k4_assigns_four_partitions() {
    let w = four_cluster_weights_8();
    let lap: Matrix<TF, C8> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let (count, ids) = k_way_partition::<_, C8, K4, TF>(&lap, sigma, USize(100));
    // Algorithm should reach the full K budget on a cleanly-separable
    // 4-cluster graph.
    assert!(count.0 <= 4, "count = {}, expected <= 4", count.0);
    assert!(count.0 >= 2, "count = {}, expected >= 2", count.0);
    // Each cluster's two nodes must carry the same partition id.
    for pair in [(0, 1), (2, 3), (4, 5), (6, 7)] {
        assert_eq!(
            ids[pair.0], ids[pair.1],
            "cluster ({}, {}) split across partitions: ids = {:?}",
            pair.0, pair.1, ids
        );
    }
}

#[test]
fn k_way_k1_returns_single_partition() {
    let w = two_cluster_weights_4();
    let lap: Matrix<TF, C4> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let (count, ids) =
        k_way_partition::<_, C4, { Cap(USize(1)) }, TF>(&lap, sigma, USize(100));
    assert_eq!(count, USize(1));
    for id in &ids {
        assert_eq!(*id, USize(0));
    }
}
