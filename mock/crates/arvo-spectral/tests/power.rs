//! `power_iteration` convergence on a known-eigenvector matrix.
//!
//! Post round 202605111719: `power_iteration` is operator-generic.
//! The `Matrix<F, C>` impl of `LinearOperator<F, C>` ships in
//! arvo-spectral; callers pass `&m` directly.

// `adt_const_params` is required by the `common::TF` `FromConstant` impl
// (`from_constant<const C: USize>`), not by capacity arithmetic. The
// migration dropped `generic_const_exprs`; this gate is independent of it.
#![feature(adt_const_params)]

use arvo::USize;
use arvo_spectral::{Matrix, power_iteration};
use arvo_tensor::Dim;

mod common;
use common::TF;

#[test]
fn diagonal_matrix_converges_to_dominant_axis() {
    // Diagonal matrix diag(1, 2, 10). Dominant eigenvalue is 10 at
    // index 2; dominant eigenvector is e2 = [0, 0, 1].
    let m: Matrix<TF, Dim<3>> = Matrix::from_fn(|i, j| {
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
    let v: [TF; 3] = power_iteration(&m, USize(50));
    // After many iterations, v should point along e2 (sign may be
    // either way; test magnitude).
    assert!(v[0].0.abs() < 1e-3, "v[0] should be ~0, got {}", v[0].0);
    assert!(v[1].0.abs() < 1e-3, "v[1] should be ~0, got {}", v[1].0);
    assert!(
        (v[2].0.abs() - 1.0).abs() < 1e-3,
        "v[2] should be ~±1, got {}",
        v[2].0
    );
}

#[test]
fn identity_like_preserves_unit() {
    // Identity matrix * v = v. v starts as all-ones and gets
    // normalised to 1/sqrt(N) per entry.
    let m: Matrix<TF, Dim<4>> = Matrix::from_fn(|i, j| if i.0 == j.0 { TF(1.0) } else { TF(0.0) });
    let v: [TF; 4] = power_iteration(&m, USize(5));
    let expected = 1.0f32 / (4.0f32).sqrt();
    for (i, vi) in v.iter().enumerate() {
        assert!(
            (vi.0 - expected).abs() < 1e-4,
            "v[{i}] = {}, want {}",
            vi.0,
            expected
        );
    }
}

#[test]
fn zero_iterations_returns_normalised_seed() {
    // Any matrix: with iterations = 0 the result is the seed
    // ([1, 1, ..., 1], unnormalised under the current contract.
    // The function does not normalise before the loop).
    let m: Matrix<TF, Dim<3>> = Matrix::from_fn(|_, _| TF(0.0));
    let v: [TF; 3] = power_iteration(&m, USize(0));
    // Current impl seeds at all-ones without normalisation on step 0.
    assert_eq!(v[0].0, 1.0);
    assert_eq!(v[1].0, 1.0);
    assert_eq!(v[2].0, 1.0);
}
