//! Fiedler vector via deflated power iteration on the Laplacian.
//!
//! Build the graph Laplacian `L = D - W`, then run power iteration
//! with each iterate orthogonalised against the all-ones vector. The
//! all-ones vector is the zero-eigenvalue eigenvector of any
//! Laplacian; projecting it out at every step keeps the iterate in the
//! orthogonal complement.
//!
//! Deflation step: `v = v - (sum(v) / N) * [1, 1, ..., 1]`. This is
//! Gram-Schmidt against the normalised all-ones direction.
//!
//! Sign of the resulting vector partitions the graph. Magnitude is not
//! meaningful after normalisation; only the sign pattern matters for
//! `spectral_bisection`.

use core::ops::{Add, Mul, Sub};

use arvo::newtype::{Cap, USize};
use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};

use crate::laplacian::laplacian;
use crate::matrix::{Matrix, cap_size};

/// Compute the Fiedler vector for a weighted adjacency matrix.
///
/// Builds the Laplacian `L = D - W`, then iterates `v <- L * v`,
/// deflating against the all-ones eigenvector and normalising in L2
/// after each step. Runs for `iterations` rounds; no convergence
/// check. Consumer-supplied `F` is typically `FastFloat<f32>` or
/// `StrictFloat<f32>`.
///
/// The returned array carries only signs that `spectral_bisection`
/// consumes; magnitudes are L2-normalised but not otherwise
/// calibrated.
#[inline]
pub fn fiedler_vector<const N: Cap, W, F>(
    weights: &Matrix<W, N>,
    iterations: USize,
) -> [F; cap_size(N)]
where
    [(); cap_size(N)]:,
    W: Into<F> + Copy,
    F: Add<Output = F>
        + Sub<Output = F>
        + Mul<Output = F>
        + Sqrt
        + Recip
        + TotalOrd
        + Copy
        + FromConstant,
{
    let n = cap_size(N);
    let lap: Matrix<F, N> = laplacian(weights);

    // Seed: alternating +1 / -1 (orthogonal to the all-ones vector for
    // even N; for odd N the deflation step pulls out the residual
    // projection on the first pass). Using all-ones as a seed would be
    // entirely in the null space and get zeroed by the first deflation.
    let one = F::from_constant(1);
    let zero = F::from_constant(0);
    let mut v: [F; cap_size(N)] = core::array::from_fn(|i| {
        if i & 1 == 0 { one } else { zero - one }
    });

    // Reciprocal of N, used each iteration for deflation mean.
    // `F::from_constant` takes a `u8`; we guarantee `N <= 64` via the
    // Mask64 partitioning surface, so the `as u8` is safe at every
    // shipping shape.
    let n_as_u8 = n as u8;
    let n_f = F::from_constant(n_as_u8);
    let n_inv = n_f.recip();

    let mut step = 0usize;
    while step < iterations.0 {
        // v_new = lap * v.
        let mut next: [F; cap_size(N)] = [zero; cap_size(N)];
        let mut i = 0usize;
        while i < n {
            let mut acc = zero;
            let mut j = 0usize;
            while j < n {
                acc = acc + lap.get(USize(i), USize(j)) * v[j];
                j += 1;
            }
            next[i] = acc;
            i += 1;
        }

        // Deflate: next = next - (sum(next) / N) * [1, ..., 1].
        let mut sum = zero;
        let mut k = 0usize;
        while k < n {
            sum = sum + next[k];
            k += 1;
        }
        let mean = sum * n_inv;
        let mut k = 0usize;
        while k < n {
            next[k] = next[k] - mean;
            k += 1;
        }

        // L2-normalise.
        let mut sq_sum = zero;
        let mut k = 0usize;
        while k < n {
            sq_sum = sq_sum + next[k] * next[k];
            k += 1;
        }
        let inv_norm = sq_sum.sqrt().recip();
        let mut k = 0usize;
        while k < n {
            next[k] = next[k] * inv_norm;
            k += 1;
        }

        v = next;
        step += 1;
    }

    v
}
