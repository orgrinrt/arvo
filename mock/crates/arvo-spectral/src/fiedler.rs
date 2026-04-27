//! Fiedler vector via shifted, deflated power iteration.
//!
//! The Fiedler vector is the eigenvector of the second-smallest
//! eigenvalue of the graph Laplacian `L = D - W`. To compute it with
//! plain power iteration (which converges to the largest-magnitude
//! eigenvector), the Laplacian is shifted: `M = sigma * I - L` where
//! `sigma >= lambda_max(L)`. Power iteration on `M` with orthogonal
//! deflation against the all-ones vector (the zero-eigenvalue
//! eigenvector of `L`) converges to the eigenvector of `M`'s second-
//! largest eigenvalue — which is `L`'s second-smallest — the Fiedler
//! vector.
//!
//! Shift budget. `lambda_max(L)` is bounded above by `2 * max_i
//! (L[i][i])` (Gershgorin circle theorem applied to the Laplacian's
//! non-positive off-diagonal structure). We pick `sigma` as exactly
//! that bound; any tighter value would risk the shifted matrix
//! retaining a negative-sign mode that outruns Fiedler.
//!
//! Deflation step: `v = v - (sum(v) / N) * [1, 1, ..., 1]`. This is
//! Gram-Schmidt against the normalised all-ones direction (eigenvector
//! of `M`'s largest eigenvalue `sigma`).
//!
//! Only the sign pattern of the result is meaningful for
//! `spectral_bisection`; magnitude is L2-normalised after the final
//! step.

use core::ops::{Add, Mul, Sub};

use arvo::{Cap, USize};
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
    // Promote the documented "N <= 64 via Mask64" invariant to a
    // check that fires well before the silent `as u8` truncation
    // below. `const { ... }` on a generic const parameter is not
    // supported under the current generic_const_exprs feature, so
    // we use a `debug_assert!` — zero release cost, catches mis-
    // instantiation in debug. Promote to compile-time when the
    // const-block restriction lifts.
    debug_assert!(
        cap_size(N) <= 64,
        "fiedler_vector requires N <= 64 (Mask64 partition surface)"
    );

    let n = cap_size(N);
    let lap: Matrix<F, N> = laplacian(weights);

    // Seed: alternating +1 / -1 (orthogonal to the all-ones vector for
    // even N; for odd N the deflation step pulls out the residual
    // projection on the first pass). Using all-ones as a seed would be
    // entirely in the null space and get zeroed by the first deflation.
    let one = F::from_constant(USize(1));
    let zero = F::from_constant(USize(0));
    let mut v: [F; cap_size(N)] = core::array::from_fn(|i| {
        if i & 1 == 0 { one } else { zero - one }
    });

    // Reciprocal of N, used each iteration for deflation mean.
    let n_f = F::from_constant(USize(n));
    let n_inv = n_f.recip();

    // Gershgorin upper bound on lambda_max(L): max over i of 2 * L[i][i]
    // (the diagonal value equals the off-diagonal absolute sum by
    // Laplacian construction). Shift via sigma >= lambda_max.
    let two = F::from_constant(USize(2));
    let mut sigma = zero;
    let mut i = 0usize;
    while i < n {
        let candidate = two * lap.get(USize(i), USize(i));
        if sigma.total_cmp(&candidate) == core::cmp::Ordering::Less {
            sigma = candidate;
        }
        i += 1;
    }

    let mut step = 0usize;
    while step < iterations.0 {
        // v_new = (sigma * I - L) * v = sigma * v - L * v.
        let mut next: [F; cap_size(N)] = [zero; cap_size(N)];
        let mut i = 0usize;
        while i < n {
            let mut acc = zero;
            let mut j = 0usize;
            while j < n {
                acc = acc + lap.get(USize(i), USize(j)) * v[j];
                j += 1;
            }
            // Shifted product: sigma * v[i] - (L * v)[i].
            next[i] = sigma * v[i] - acc;
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
