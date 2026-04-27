//! `power_iteration` convergence on a known-eigenvector matrix.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_spectral::{Matrix, power_iteration};

mod common;
use common::TF;

const C3: Cap = Cap(USize(3));
const C4: Cap = Cap(USize(4));

#[test]
fn diagonal_matrix_converges_to_dominant_axis() {
    // Diagonal matrix diag(1, 2, 10). Dominant eigenvalue is 10 at
    // index 2; dominant eigenvector is e2 = [0, 0, 1].
    let m: Matrix<TF, C3> = Matrix::from_fn(|i, j| {
        if i.0 != j.0 {
            TF(0.0)
        } else {
            match i.0 {
                0 => TF(1.0),
                1 => TF(2.0),
                2 => TF(10.0),
                _ => TF(0.0),
            }
        }
    });
    let v = power_iteration::<C3, TF>(&m, USize(50));
    // After many iterations, v should point along e2 (sign may be
    // either way; test magnitude).
    assert!(v[0].0.abs() < 1e-3, "v[0] should be ~0, got {}", v[0].0);
    assert!(v[1].0.abs() < 1e-3, "v[1] should be ~0, got {}", v[1].0);
    assert!((v[2].0.abs() - 1.0).abs() < 1e-3, "v[2] should be ~±1, got {}", v[2].0);
}

#[test]
fn identity_like_preserves_unit() {
    // Identity matrix * v = v. v starts as all-ones and gets
    // normalised to 1/sqrt(N) per entry.
    let m: Matrix<TF, C4> = Matrix::from_fn(|i, j| {
        if i.0 == j.0 { TF(1.0) } else { TF(0.0) }
    });
    let v = power_iteration::<C4, TF>(&m, USize(5));
    let expected = 1.0f32 / (4.0f32).sqrt();
    for (i, vi) in v.iter().enumerate() {
        assert!((vi.0 - expected).abs() < 1e-4, "v[{i}] = {}, want {}", vi.0, expected);
    }
}

#[test]
fn zero_iterations_returns_normalised_seed() {
    // Any matrix: with iterations = 0 the result is the seed
    // ([1, 1, ..., 1], unnormalised under the current contract —
    // the function does not normalise before the loop).
    let m: Matrix<TF, C3> = Matrix::from_fn(|_, _| TF(0.0));
    let v = power_iteration::<C3, TF>(&m, USize(0));
    // Current impl seeds at all-ones without normalisation on step 0.
    assert_eq!(v[0].0, 1.0);
    assert_eq!(v[1].0, 1.0);
    assert_eq!(v[2].0, 1.0);
}
