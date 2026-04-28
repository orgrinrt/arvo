//! Power iteration for the dominant eigenvector.
//!
//! Repeatedly multiply a vector by the matrix and renormalise to the
//! L2 unit. After `iterations` rounds, the vector converges toward the
//! eigenvector of the largest-magnitude eigenvalue. No convergence
//! ratio check this round: the caller picks the iteration count.
//!
//! Initial vector is the all-ones vector `[1, 1, ..., 1]`. Constant
//! construction uses `F::from_constant(1)`.

use core::ops::{Add, Mul};

use arvo::{Cap, USize};
use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};

use crate::matrix::{Matrix, cap_size};

/// Run power iteration on `matrix` for `iterations` rounds.
///
/// Returns the approximate dominant eigenvector, L2-normalised. The
/// initial guess is the all-ones vector. Each round applies the
/// matrix then divides by the L2 norm. `iterations = 0` returns the
/// normalised all-ones vector.
///
/// Behaviour when the vector collapses to zero (e.g. the matrix has a
/// zero eigenvector along the all-ones direction): the normalisation
/// multiplies by `recip(sqrt(0))`, which for float types is infinity
/// or NaN. Callers sensitive to this case should deflate the matrix
/// against the known zero-eigenvector direction first — `fiedler.rs`
/// does exactly that.
#[inline]
pub fn power_iteration<const N: Cap, F>(
    matrix: &Matrix<F, N>,
    iterations: USize,
) -> [F; cap_size(N)]
where
    [(); cap_size(N)]:,
    F: Add<Output = F>
        + Mul<Output = F>
        + Sqrt<Output = F>
        + Recip<Output = F>
        + TotalOrd
        + Copy
        + FromConstant,
{
    let n = cap_size(N);
    let mut v: [F; cap_size(N)] = [F::from_constant::<{ USize(1) }>(); cap_size(N)];

    let mut step = 0usize;
    while step < iterations.0 {
        // v_new = matrix * v (dense matrix-vector product).
        let mut next: [F; cap_size(N)] = [F::from_constant::<{ USize(0) }>(); cap_size(N)];
        let mut i = 0usize;
        while i < n {
            let mut acc = F::from_constant::<{ USize(0) }>();
            let mut j = 0usize;
            while j < n {
                acc = acc + matrix.get(USize(i), USize(j)) * v[j];
                j += 1;
            }
            next[i] = acc;
            i += 1;
        }

        // L2 norm: sqrt of sum of squares.
        let mut sq_sum = F::from_constant::<{ USize(0) }>();
        let mut k = 0usize;
        while k < n {
            sq_sum = sq_sum + next[k] * next[k];
            k += 1;
        }
        let norm = sq_sum.sqrt();
        let inv = norm.recip();

        // Normalise in place.
        let mut k = 0usize;
        while k < n {
            next[k] = next[k] * inv;
            k += 1;
        }

        v = next;
        step += 1;
    }

    v
}
