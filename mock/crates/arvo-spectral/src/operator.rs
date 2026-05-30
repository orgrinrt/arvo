//! Linear-operator contract for spectral iteration.
//!
//! Spectral algorithms (power_iteration, fiedler_vector) need exactly
//! one capability from their input: the matrix-vector product
//! `y = A * x`. Making that the trait surface lifts the algorithms
//! across every concrete representation. Dense `Matrix<F, C>` runs the
//! classic `O(N^2)` matvec. CSR-backed `SparseLaplacian` computes
//! `(L * x)[i] = sum over j of w(i, j) * (x[i] - x[j])` walking the
//! row directly, `O(NNZ)` per apply.
//!
//! The trait contract: `apply(x, y)` writes `A * x` into `y`. `y` must
//! not alias `x`. Algorithms keep `y` as an exclusive scratch buffer.
//! The capacity is a TYPE (`C: Capacity`); vectors are the associated
//! array `C::Array<F>`.

use core::marker::PhantomData;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, TotalOrd};
use arvo::USize;
use arvo_sparse::{Csr, SparseAdjacency};
use arvo_tensor::{Capacity, cap_size};

use crate::matrix::Matrix;

/// Linear operator `y = A * x` on a fixed-capacity vector space.
///
/// `apply(x, y)` writes `A * x` into `y`. The two arrays must not
/// alias; spectral algorithms keep `y` as exclusive scratch.
pub trait LinearOperator<F, C: Capacity> {
    /// Write `A * x` into `y`. `y` must not alias `x`.
    fn apply(&self, x: &C::Array<F>, y: &mut C::Array<F>);

    /// Number of nodes the operator spans.
    ///
    /// `cap_size(C::CAP)` for a fully packed operator (dense `Matrix`),
    /// the live row count for a `SparseLaplacian` over a loose CSR. The
    /// iterative algorithms iterate `[0, live_dim())`, so a loose
    /// graph's empty slack rows stay out of the Fiedler iteration and
    /// the partition budget. Required, not defaulted: every operator
    /// declares its span so a loose consumer cannot silently inherit a
    /// full-capacity dimension.
    fn live_dim(&self) -> USize;
}

// Dense Matrix<F, C>: classic O(N^2) matvec.
impl<F, C: Capacity> LinearOperator<F, C> for Matrix<F, C>
where
    F: Add<Output = F> + Mul<Output = F> + Copy + FromConstant,
{
    #[inline]
    fn apply(&self, x: &C::Array<F>, y: &mut C::Array<F>) {
        let n = cap_size(C::CAP);
        let zero = F::from_constant::<{ USize(0) }>();
        let xs = x.as_ref();
        let ys = y.as_mut();
        let mut i = 0usize;
        while i < n {
            let mut acc = zero;
            let mut j = 0usize;
            while j < n {
                acc = acc + self.get(USize(i), USize(j)) * xs[j];
                j += 1;
            }
            ys[i] = acc;
            i += 1;
        }
    }

    #[inline]
    fn live_dim(&self) -> USize {
        // Dense matrices are always fully packed: the span is the cap.
        USize(cap_size(C::CAP))
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
/// `W: Into<F>` at apply time. The row capacity `R` is the operator's
/// `Capacity`; the nnz capacity `NNZ` is the CSR's storage axis.
///
/// Diagonal entries in the CSR are ignored (a self-edge does not
/// contribute to either the off-diagonal or the degree, by the
/// Laplacian's definition).
pub struct SparseLaplacian<'data, R: Capacity, NNZ: Capacity, W, F>
where
    W: Copy + Into<F>,
{
    csr: &'data Csr<R, NNZ, W>,
    _phantom: PhantomData<F>,
}

impl<'data, R: Capacity, NNZ: Capacity, W, F> SparseLaplacian<'data, R, NNZ, W, F>
where
    W: Copy + Into<F>,
{
    /// Build a Laplacian operator over the borrowed CSR.
    #[inline]
    pub fn new(csr: &'data Csr<R, NNZ, W>) -> Self {
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
        // Iterate live rows only; slack rows are empty (zero row-sum)
        // and would not raise the bound, but skipping them is cheaper.
        let n = self.csr.node_count().0;
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

impl<'data, R: Capacity, NNZ: Capacity, W, F> LinearOperator<F, R>
    for SparseLaplacian<'data, R, NNZ, W, F>
where
    W: Copy + Into<F>,
    F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + Copy + FromConstant,
{
    #[inline]
    fn apply(&self, x: &R::Array<F>, y: &mut R::Array<F>) {
        let n = cap_size(R::CAP);
        let zero = F::from_constant::<{ USize(0) }>();
        let xs = x.as_ref();
        let ys = y.as_mut();
        let mut i = 0usize;
        while i < n {
            let cols = self.csr.row_col_indices(USize(i));
            let vals = self.csr.row_values(USize(i));
            let xi = xs[i];
            let mut acc = zero;
            let mut k = 0usize;
            while k < cols.len() {
                let j_idx = (cols[k].0).0;
                // Skip self-edges and out-of-range indices.
                if j_idx != i && j_idx < n {
                    let wf: F = vals[k].into();
                    acc = acc + wf * (xi - xs[j_idx]);
                }
                k += 1;
            }
            ys[i] = acc;
            i += 1;
        }
    }

    #[inline]
    fn live_dim(&self) -> USize {
        // The live row count: loose-CSR slack rows are not real nodes.
        self.csr.node_count()
    }
}
