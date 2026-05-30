//! Power iteration for the dominant eigenvector.
//!
//! Repeatedly multiply a vector by the operator and renormalise to the
//! L2 unit. After `iterations` rounds, the vector converges toward the
//! eigenvector of the largest-magnitude eigenvalue. No convergence
//! ratio check this round: the caller picks the iteration count.
//!
//! The operator surface is `LinearOperator<F, C>`, so the same routine
//! runs across dense `Matrix<F, C>`, sparse `SparseLaplacian`, or any
//! consumer-supplied operator that implements the trait. The capacity
//! is a TYPE (`C: Capacity`); the ping-pong buffers are `C::Array<F>`.
//!
//! Initial vector is the all-ones vector `[1, 1, ..., 1]`. Constant
//! construction uses `F::from_constant(1)`.

use core::ops::{Add, Mul};

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo::USize;
use arvo_tensor::Capacity;

use crate::operator::LinearOperator;

/// Run power iteration on `operator` for `iterations` rounds.
///
/// Returns the approximate dominant eigenvector, L2-normalised. The
/// initial guess is the all-ones vector. Each round applies the
/// operator then divides by the L2 norm. `iterations = 0` returns the
/// normalised all-ones vector.
///
/// Behaviour when the vector collapses to zero (e.g. the operator has
/// a zero eigenvector along the all-ones direction): the normalisation
/// multiplies by `recip(sqrt(0))`, which for float types is infinity
/// or NaN. Callers sensitive to this case should deflate the operator
/// against the known zero-eigenvector direction first. `fiedler.rs`
/// does exactly that.
#[inline]
pub fn power_iteration<Op, C: Capacity, F>(
    operator: &Op,
    iterations: USize,
) -> C::Array<F>
where
    Op: LinearOperator<F, C>,
    F: Add<Output = F>
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
    // Seed the live span with ones; the slack tail stays zero so it
    // never enters the norm or the matvec output.
    let mut v: C::Array<F> = C::from_fn(|i| if i.0 < n { one } else { zero });

    let mut step = 0usize;
    while step < iterations.0 {
        // next = operator.apply(v)
        let mut next: C::Array<F> = C::filled(zero);
        operator.apply(&v, &mut next);

        {
            let ns = next.as_mut();
            // L2 norm: sqrt of sum of squares.
            let mut sq_sum = zero;
            let mut k = 0usize;
            while k < n {
                sq_sum = sq_sum + ns[k] * ns[k];
                k += 1;
            }
            let inv = sq_sum.sqrt().recip();

            // Normalise in place.
            let mut k = 0usize;
            while k < n {
                ns[k] = ns[k] * inv;
                k += 1;
            }
        }

        v = next;
        step += 1;
    }

    v
}
