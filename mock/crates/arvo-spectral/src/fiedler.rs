//! Fiedler vector via shifted, deflated power iteration.
//!
//! The Fiedler vector is the eigenvector of the second-smallest
//! eigenvalue of the graph Laplacian `L = D - W`. To compute it with
//! plain power iteration (which converges to the largest-magnitude
//! eigenvector), the Laplacian is shifted: `M = sigma * I - L` where
//! `sigma >= lambda_max(L)`. Power iteration on `M` with orthogonal
//! deflation against the all-ones vector (the zero-eigenvalue
//! eigenvector of `L`) converges to the eigenvector of `M`'s second-
//! largest eigenvalue, which is `L`'s second-smallest, the Fiedler
//! vector.
//!
//! Operator surface: `fiedler_vector` takes any `LinearOperator<F, C>`
//! whose action is `y = L * x`. The caller supplies sigma. Convenience
//! helpers (`dense_laplacian_lambda_max_bound` and
//! `SparseLaplacian::lambda_max_bound`) compute the Gershgorin upper
//! bound `sigma = max_i 2 * L[i][i]` for the two shipped operator
//! types; consumers with custom operators provide their own.
//!
//! Deflation: `v = v - (sum(v) / N) * [1, 1, ..., 1]`. This is
//! Gram-Schmidt against the normalised all-ones direction (eigenvector
//! of `M`'s largest eigenvalue `sigma`).
//!
//! Only the sign pattern of the result is meaningful for
//! `spectral_bisection`; magnitude is L2-normalised after the final
//! step. The capacity is a TYPE (`C: Capacity`); buffers are
//! `C::Array<F>`.

use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo::USize;
use arvo_tensor::{Capacity, cap_size};

use crate::matrix::Matrix;
use crate::operator::LinearOperator;

/// Compute the Fiedler vector via shifted, deflated power iteration on
/// any Laplacian-shaped operator.
///
/// `operator` applies `L * x`. `sigma` is the shift, an upper bound on
/// `lambda_max(L)`. The shipped operator types both expose a Gershgorin
/// bound helper:
///
/// - dense `Matrix<F, C>` representing a Laplacian:
///   `dense_laplacian_lambda_max_bound(&lap)`.
/// - sparse `SparseLaplacian<'_, R, NNZ, W, F>`:
///   `lap.lambda_max_bound()`.
///
/// The returned array carries only signs that `spectral_bisection`
/// consumes; magnitudes are L2-normalised but not otherwise
/// calibrated.
#[inline]
pub fn fiedler_vector<Op, C: Capacity, F>(
    operator: &Op,
    sigma: F,
    iterations: USize,
) -> C::Array<F>
where
    Op: LinearOperator<F, C>,
    F: Add<Output = F>
        + Sub<Output = F>
        + Mul<Output = F>
        + Sqrt<Output = F>
        + Recip<Output = F>
        + TotalOrd
        + Copy
        + FromConstant,
{
    let n = operator.live_dim().0;
    let one = F::from_constant::<{ USize(1) }>();
    let zero = F::from_constant::<{ USize(0) }>();

    // Seed: alternating +1 / -1 over the live span (orthogonal to the
    // all-ones vector for even live counts; for odd counts the
    // deflation step pulls out the residual projection on the first
    // pass). The slack tail stays zero so the empty rows of a loose
    // CSR never enter the deflation mean or the L2 norm.
    let mut v: C::Array<F> = C::from_fn(|i| {
        if i.0 >= n {
            zero
        } else if i.0 & 1 == 0 {
            one
        } else {
            zero - one
        }
    });

    // Reciprocal of N built via fold-counted addition; `n` is runtime.
    let mut n_f = zero;
    let mut k = 0usize;
    while k < n {
        n_f = n_f + one;
        k += 1;
    }
    let n_inv = n_f.recip();

    let mut step = 0usize;
    while step < iterations.0 {
        // lv = L * v
        let mut lv: C::Array<F> = C::filled(zero);
        operator.apply(&v, &mut lv);

        // next = sigma * v - lv  (shifted: M * v)
        let mut next: C::Array<F> = C::filled(zero);
        {
            let vs = v.as_ref();
            let lvs = lv.as_ref();
            let ns = next.as_mut();
            let mut i = 0usize;
            while i < n {
                ns[i] = sigma * vs[i] - lvs[i];
                i += 1;
            }

            // Deflate: next = next - (sum(next) / N) * [1, ..., 1].
            let mut sum = zero;
            let mut k = 0usize;
            while k < n {
                sum = sum + ns[k];
                k += 1;
            }
            let mean = sum * n_inv;
            let mut k = 0usize;
            while k < n {
                ns[k] = ns[k] - mean;
                k += 1;
            }

            // L2-normalise.
            let mut sq_sum = zero;
            let mut k = 0usize;
            while k < n {
                sq_sum = sq_sum + ns[k] * ns[k];
                k += 1;
            }
            let inv_norm = sq_sum.sqrt().recip();
            let mut k = 0usize;
            while k < n {
                ns[k] = ns[k] * inv_norm;
                k += 1;
            }
        }

        v = next;
        step += 1;
    }

    v
}

/// Gershgorin upper bound on `lambda_max(L)` for a dense Laplacian.
///
/// For a Laplacian `L = D - W`, the row sum of absolute values equals
/// `2 * L[i][i]` (the diagonal value equals the off-diagonal absolute
/// sum by construction). The Gershgorin disc upper bound is then
/// `max_i 2 * L[i][i]`. Use this for the `sigma` parameter of
/// `fiedler_vector` when the operator wraps a dense Laplacian matrix.
#[inline]
pub fn dense_laplacian_lambda_max_bound<C: Capacity, F>(lap: &Matrix<F, C>) -> F
where
    F: Add<Output = F> + Mul<Output = F> + TotalOrd + Copy + FromConstant,
{
    let n = cap_size(C::CAP);
    let two = F::from_constant::<{ USize(2) }>();
    let mut sigma = F::from_constant::<{ USize(0) }>();
    let mut i = 0usize;
    while i < n {
        let candidate = two * lap.get(USize(i), USize(i));
        if sigma.total_cmp(candidate) == core::cmp::Ordering::Less {
            sigma = candidate;
        }
        i += 1;
    }
    sigma
}
