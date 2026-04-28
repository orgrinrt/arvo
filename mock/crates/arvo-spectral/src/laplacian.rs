//! Graph Laplacian construction.
//!
//! The Laplacian `L = D - W` of a weighted undirected graph, where `W`
//! is the weighted adjacency matrix and `D` is the diagonal degree
//! matrix. For `i != j`, `L[i][j] = -W[i][j]`. On the diagonal,
//! `L[i][i] = sum over j != i of W[i][j]` (the node's weighted
//! degree).
//!
//! The input uses weight type `W`; the output uses float type `F`.
//! The per-cell conversion happens through `Into<F>`, so weights can
//! be fixed-point and eigenvectors can be floats.

use core::ops::{Add, Sub};

use arvo::{Cap, USize};
use arvo::traits::FromConstant;

use crate::matrix::{Matrix, cap_size};

/// Build the graph Laplacian `L = D - W`.
///
/// `L[i][j] = -weights[i][j] converted to F` for `i != j`.
/// `L[i][i] = sum over j != i of weights[i][j] converted to F`.
///
/// The weight-to-float conversion runs per-cell via `Into<F>`. Diagonal
/// weights in the input are ignored; only the off-diagonal edges
/// contribute to the degree sum.
#[inline]
pub fn laplacian<const N: Cap, W, F>(weights: &Matrix<W, N>) -> Matrix<F, N>
where
    [(); cap_size(N)]:,
    W: Into<F> + Copy,
    F: Add<Output = F> + Sub<Output = F> + Copy + FromConstant,
{
    let n = cap_size(N);
    Matrix::from_fn(|i, j| {
        let i_raw = i.0;
        let j_raw = j.0;
        if i_raw == j_raw {
            // Diagonal: weighted degree (sum of off-diagonal row).
            let mut sum = F::from_constant::<{ USize(0) }>();
            let mut k = 0usize;
            while k < n {
                if k != i_raw {
                    let w: F = weights.get(USize(i_raw), USize(k)).into();
                    sum = sum + w;
                }
                k += 1;
            }
            sum
        } else {
            // Off-diagonal: negated weight.
            let w: F = weights.get(USize(i_raw), USize(j_raw)).into();
            F::from_constant::<{ USize(0) }>() - w
        }
    })
}
