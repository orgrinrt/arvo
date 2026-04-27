//! `laplacian` correctness on a known weighted graph.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_spectral::{Matrix, laplacian};

mod common;
use common::TF;

const C4: Cap = Cap(USize(4));

/// Build a symmetric weighted adjacency matrix.
fn sample_weights() -> Matrix<u32, C4> {
    // Graph:
    //   0 -- 1  weight 2
    //   0 -- 2  weight 3
    //   1 -- 2  weight 1
    //   2 -- 3  weight 4
    // Diagonal is zero (no self-loops).
    let mut m: Matrix<u32, C4> = Matrix::from_fn(|_, _| 0u32);
    m.set(USize(0), USize(1), 2);
    m.set(USize(1), USize(0), 2);
    m.set(USize(0), USize(2), 3);
    m.set(USize(2), USize(0), 3);
    m.set(USize(1), USize(2), 1);
    m.set(USize(2), USize(1), 1);
    m.set(USize(2), USize(3), 4);
    m.set(USize(3), USize(2), 4);
    m
}

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

#[test]
fn off_diagonal_is_negated_weight() {
    let w = sample_weights();
    let lap: Matrix<TF, C4> = laplacian::<C4, u32, TF>(&w);
    // L[0][1] = -w[0][1] = -2.
    assert!((lap.get(USize(0), USize(1)).0 - (-2.0)).abs() < 1e-6);
    // L[2][3] = -w[2][3] = -4.
    assert!((lap.get(USize(2), USize(3)).0 - (-4.0)).abs() < 1e-6);
    // L[0][3] = -w[0][3] = 0.
    assert!(lap.get(USize(0), USize(3)).0.abs() < 1e-6);
}

#[test]
fn diagonal_is_weighted_degree() {
    let w = sample_weights();
    let lap: Matrix<TF, C4> = laplacian::<C4, u32, TF>(&w);
    // Node 0: connects to 1 (w=2) and 2 (w=3). Degree = 5.
    assert!((lap.get(USize(0), USize(0)).0 - 5.0).abs() < 1e-6);
    // Node 1: connects to 0 (w=2) and 2 (w=1). Degree = 3.
    assert!((lap.get(USize(1), USize(1)).0 - 3.0).abs() < 1e-6);
    // Node 2: connects to 0 (w=3), 1 (w=1), 3 (w=4). Degree = 8.
    assert!((lap.get(USize(2), USize(2)).0 - 8.0).abs() < 1e-6);
    // Node 3: connects to 2 (w=4). Degree = 4.
    assert!((lap.get(USize(3), USize(3)).0 - 4.0).abs() < 1e-6);
}

#[test]
fn row_sums_to_zero() {
    // Property of any Laplacian: rows sum to zero.
    let w = sample_weights();
    let lap: Matrix<TF, C4> = laplacian::<C4, u32, TF>(&w);
    for i in 0..4 {
        let mut row_sum = 0.0f32;
        for j in 0..4 {
            row_sum += lap.get(USize(i), USize(j)).0;
        }
        assert!(row_sum.abs() < 1e-5, "row {i} sum = {row_sum}");
    }
}
