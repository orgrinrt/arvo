//! The second half of the residual, run only if probe_4 compiles clean:
//! even granted `impl Gcd<LocalRhs> for H`, the impl is unusable, because
//! every position in the tower that consumes a `Gcd` verdict types BOTH
//! operands `Pos` (`Ratio<N, D>`'s blanket: `N: Pos + Gcd<D, Out = H>,
//! D: Pos`; `BPos<N, D>` identically), and `LocalRhs: Pos` is what the
//! seal refuses. EXPECTED: E0277, `LocalRhs: Pos` (or the sealed
//! supertrait) unsatisfied at the D position.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_4b_local_rhs_cannot_enter.rs

#![allow(dead_code)]

use vu_core::nat::{Adjustment, Gcd, Ratio, H};

pub struct LocalRhs;

impl Gcd<LocalRhs> for H {
    type Out = H;
}

pub struct AdjPos<A: Adjustment>(core::marker::PhantomData<A>);

// H: Gcd<LocalRhs, Out = H> holds by the impl above; D: Pos does not.
// Forced through a fn signature; the bare-alias form defers the check.
pub fn attack(_: AdjPos<Ratio<H, LocalRhs>>) {}
