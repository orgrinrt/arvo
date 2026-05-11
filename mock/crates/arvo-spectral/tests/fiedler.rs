//! `fiedler_vector` sign partitions a known-bipartition graph.
//!
//! Post round 202605111719: `fiedler_vector` is operator-generic and
//! takes `sigma` (the Gershgorin upper bound) as an explicit
//! parameter. The caller builds the Laplacian and passes
//! `dense_laplacian_lambda_max_bound(&lap)` as sigma.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_spectral::{Matrix, dense_laplacian_lambda_max_bound, fiedler_vector, laplacian};

mod common;
use common::TF;

const C4: Cap = Cap(USize(4));

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

/// Two clusters {0, 1} and {2, 3} linked by a weak bridge 1 -- 2.
fn two_cluster_weights() -> Matrix<u32, C4> {
    let mut m: Matrix<u32, C4> = Matrix::from_fn(|_, _| 0u32);
    // Cluster A: 0 -- 1 heavy.
    m.set(USize(0), USize(1), 10);
    m.set(USize(1), USize(0), 10);
    // Cluster B: 2 -- 3 heavy.
    m.set(USize(2), USize(3), 10);
    m.set(USize(3), USize(2), 10);
    // Bridge: 1 -- 2 light.
    m.set(USize(1), USize(2), 1);
    m.set(USize(2), USize(1), 1);
    m
}

#[test]
fn bipartition_shows_sign_split() {
    // Signs of the Fiedler vector should agree on intra-cluster nodes
    // and disagree across the cut.
    let w = two_cluster_weights();
    let lap: Matrix<TF, C4> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let v: [TF; 4] = fiedler_vector(&lap, sigma, USize(100));
    let s0 = v[0].0.signum();
    let s1 = v[1].0.signum();
    let s2 = v[2].0.signum();
    let s3 = v[3].0.signum();
    // Nodes 0 and 1 share a sign.
    assert_eq!(s0, s1, "nodes 0, 1 differ: v = {v:?}");
    // Nodes 2 and 3 share a sign.
    assert_eq!(s2, s3, "nodes 2, 3 differ: v = {v:?}");
    // Clusters are on opposite sides.
    assert!(s0 != s2, "clusters not split: v = {v:?}");
}

#[test]
fn sum_close_to_zero_after_deflation() {
    // Because of the deflation step, the Fiedler vector should be
    // orthogonal to the all-ones vector; i.e. its sum is ~0.
    let w = two_cluster_weights();
    let lap: Matrix<TF, C4> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let v: [TF; 4] = fiedler_vector(&lap, sigma, USize(100));
    let s: f32 = v[0].0 + v[1].0 + v[2].0 + v[3].0;
    assert!(s.abs() < 1e-4, "sum = {s}, v = {v:?}");
}
