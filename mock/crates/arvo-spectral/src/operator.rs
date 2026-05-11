//! Linear-operator contract for spectral iteration.
//!
//! Spectral algorithms (power_iteration, fiedler_vector) need exactly
//! one capability from their input: the matrix-vector product
//! `y = A * x`. Making that the trait surface lifts the algorithms
//! across every concrete representation. Dense `Matrix<F, N>` runs the
//! classic `O(N^2)` matvec. CSR-backed `SparseLaplacian` computes
//! `(L * x)[i] = sum over j of w(i, j) * (x[i] - x[j])` walking the
//! row directly, `O(NNZ)` per apply.
//!
//! The trait contract: `apply(x, y)` writes `A * x` into `y`. `y` must
//! not alias `x`. Algorithms keep `y` as an exclusive scratch buffer.

use core::marker::PhantomData;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, TotalOrd};
use arvo::{Cap, USize};
use arvo_sparse::Csr;

use crate::matrix::{Matrix, cap_size};

/// Linear operator `y = A * x` on a fixed-size vector space.
///
/// `apply(x, y)` writes `A * x` into `y`. The two slices must not
/// alias; spectral algorithms keep `y` as exclusive scratch.
pub trait LinearOperator<F, const N: Cap>
where
    [(); cap_size(N)]:,
{
    /// Write `A * x` into `y`. `y` must not alias `x`.
    fn apply(&self, x: &[F; cap_size(N)], y: &mut [F; cap_size(N)]);
}

// Dense Matrix<F, N>: classic O(N^2) matvec.
impl<F, const N: Cap> LinearOperator<F, N> for Matrix<F, N>
where
    [(); cap_size(N)]:,
    F: Add<Output = F> + Mul<Output = F> + Copy + FromConstant,
{
    #[inline]
    fn apply(&self, x: &[F; cap_size(N)], y: &mut [F; cap_size(N)]) {
        let n = cap_size(N);
        let zero = F::from_constant::<{ USize(0) }>();
        let mut i = 0usize;
        while i < n {
            let mut acc = zero;
            let mut j = 0usize;
            while j < n {
                acc = acc + self.get(USize(i), USize(j)) * x[j];
                j += 1;
            }
            y[i] = acc;
            i += 1;
        }
    }
}

/// Graph Laplacian over a CSR-backed weighted adjacency.
///
/// `L = D - A` where `A` is the weighted adjacency and `D` is the
/// weighted-degree diagonal. The matvec rearranges to
/// `(L * x)[i] = sum over j of w(i, j) * (x[i] - x[j])`, computed by
/// walking row `i`'s non-zeros directly. Each apply is `O(NNZ)`, not
/// `O(N^2)`; spectral iteration on sparse graphs reaches the dominant
/// eigenvector in proportionally less work.
///
/// The borrow over `Csr` keeps the operator weightless: no copy of the
/// adjacency, no rebuild on each iteration. `F` is the spectral
/// eigenvector type (typically `FastFloat<f32>` / `StrictFloat<f32>`),
/// `W` is the edge weight type. The per-cell conversion runs through
/// `W: Into<F>` at apply time.
///
/// Diagonal entries in the CSR are ignored (a self-edge does not
/// contribute to either the off-diagonal or the degree, by the
/// Laplacian's definition).
pub struct SparseLaplacian<'data, const ROWS: Cap, const NNZ: Cap, W, F>
where
    [(); cap_size(ROWS)]:,
    [(); cap_size(NNZ)]:,
    W: Copy + Into<F>,
{
    csr: &'data Csr<ROWS, NNZ, W>,
    _phantom: PhantomData<F>,
}

impl<'data, const ROWS: Cap, const NNZ: Cap, W, F> SparseLaplacian<'data, ROWS, NNZ, W, F>
where
    [(); cap_size(ROWS)]:,
    [(); cap_size(NNZ)]:,
    W: Copy + Into<F>,
{
    /// Build a Laplacian operator over the borrowed CSR.
    #[inline]
    pub fn new(csr: &'data Csr<ROWS, NNZ, W>) -> Self {
        Self {
            csr,
            _phantom: PhantomData,
        }
    }

    /// Gershgorin upper bound on `lambda_max(L)`.
    ///
    /// For the Laplacian `L = D - A`, the Gershgorin disc bound is
    /// `max_i 2 * D[i] = max_i 2 * sum_{j != i} w(i, j)`. The result
    /// is suitable as the `sigma` parameter of `fiedler_vector`.
    ///
    /// Assumes non-negative weights (the standard graph-Laplacian
    /// case). For signed weights, the bound is `max_i 2 * sum_{j != i}
    /// |w(i, j)|`; consumers with signed weights compute it
    /// themselves.
    ///
    /// The same computation is exposed via the
    /// `SpectralBipartitioner::lambda_max_bound` trait method; this
    /// inherent is the one to call when the caller does not want to
    /// bring the trait into scope.
    #[inline]
    pub fn gershgorin_lambda_max(&self) -> F
    where
        F: Add<Output = F> + Mul<Output = F> + TotalOrd + Copy + FromConstant,
    {
        let n = cap_size(ROWS);
        let zero = F::from_constant::<{ USize(0) }>();
        let two = F::from_constant::<{ USize(2) }>();
        let mut sigma = zero;
        let mut i = 0usize;
        while i < n {
            let cols = self.csr.row_col_indices(USize(i));
            let vals = self.csr.row_values(USize(i));
            let mut row_sum = zero;
            let mut k = 0usize;
            while k < cols.len() {
                let j_idx = (cols[k].0).0;
                if j_idx != i && j_idx < n {
                    let wf: F = vals[k].into();
                    row_sum = row_sum + wf;
                }
                k += 1;
            }
            let candidate = two * row_sum;
            if sigma.total_cmp(candidate) == core::cmp::Ordering::Less {
                sigma = candidate;
            }
            i += 1;
        }
        sigma
    }
}

impl<'data, const ROWS: Cap, const NNZ: Cap, W, F> LinearOperator<F, ROWS>
    for SparseLaplacian<'data, ROWS, NNZ, W, F>
where
    [(); cap_size(ROWS)]:,
    [(); cap_size(NNZ)]:,
    W: Copy + Into<F>,
    F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + Copy + FromConstant,
{
    #[inline]
    fn apply(&self, x: &[F; cap_size(ROWS)], y: &mut [F; cap_size(ROWS)]) {
        let n = cap_size(ROWS);
        let zero = F::from_constant::<{ USize(0) }>();
        let mut i = 0usize;
        while i < n {
            let cols = self.csr.row_col_indices(USize(i));
            let vals = self.csr.row_values(USize(i));
            let xi = x[i];
            let mut acc = zero;
            let mut k = 0usize;
            while k < cols.len() {
                let j_idx = (cols[k].0).0;
                // Skip self-edges and out-of-range indices.
                if j_idx != i && j_idx < n {
                    let wf: F = vals[k].into();
                    acc = acc + wf * (xi - x[j_idx]);
                }
                k += 1;
            }
            y[i] = acc;
            i += 1;
        }
    }
}
