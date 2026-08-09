//! Probe 5: `49:868-871` records the composition wall's residual as
//! "recorded, not resolved", and the consolidation names it explicitly:
//! whether a member with deeper diagnostic knowledge can do better in
//! twenty minutes was "named and left untried" (`49:839-842`). This is
//! that attempt.
//!
//! The wall's error is E0275, "overflow evaluating the requirement", from
//! confirming an unconditional blanket impl's obligations against an
//! abstract, non-constructor-headed operand (`Reduce`'s recursive
//! definition has no base case there). `#[diagnostic::on_unimplemented]`
//! is documented (and every other diagnostic instrument this review has
//! used) as attaching to a TRAIT and customising the message when that
//! trait's bound is NOT SATISFIED, which is E0277's shape: the solver
//! tried every candidate and found none. E0275 is a different failure: the
//! solver did not finish trying, because the candidate chain recursed past
//! the depth limit. This probe tests directly whether the attribute
//! reaches that failure at all, on the exact reproduction 48's probe 1
//! built.
//!
//! EXPECTED: unknown going in, though the a-priori read (E0275 is a solver
//! resource limit, not a "no impl found" outcome) predicts no.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern
//!   tower=libtower.rlib probe_5_does_on_unimplemented_reach_e0275.rs

#![allow(dead_code)]
#![feature(diagnostic_namespace)]

use tower::nat::{Pos, Ratio, Reduce as TowerReduce};

// Cannot attach #[diagnostic::on_unimplemented] to a trait defined in
// another crate (it has to sit on the trait declaration). Reproduce the
// wall on a LOCAL trait with the identical shape (one unconditional
// blanket impl, recursive, no base case for an abstract input) so the
// attribute can actually be tested, and confirm the local reproduction
// hits the same E0275 signature first.

pub trait LocalReduce {
    type N;
    type D;
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be reduced to lowest terms here",
    label = "this ratio needs a concrete numerator and denominator to reduce",
    note = "`LocalReduce` recurses on the pair's own constructors and has no \
            base case for an abstract, unconstrained operand"
)]
pub trait LocalReduceBounded: LocalReduce {}

impl<N, D> LocalReduce for Ratio<N, D>
where
    Ratio<N, D>: LocalReduceStep,
{
    type N = <Ratio<N, D> as LocalReduceStep>::N;
    type D = <Ratio<N, D> as LocalReduceStep>::D;
}
impl<N, D> LocalReduceBounded for Ratio<N, D> where Ratio<N, D>: LocalReduce {}

// The recursive step, deliberately with no base case reachable from an
// abstract N/D, reproducing the shape that makes Reduce diverge.
pub trait LocalReduceStep {
    type N;
    type D;
}
impl<N, D> LocalReduceStep for Ratio<N, D>
where
    Ratio<N, D>: LocalReduceStep,
{
    type N = N;
    type D = D;
}

pub fn regroup_fold_reduced_headroom_local<Hd, Am1>(_xs: &[i32])
where
    Hd: Pos,
    Am1: Pos,
    Ratio<Hd, Am1>: LocalReduceBounded,
{
}

// The tower's own real Reduce, undecorated, as the direct baseline
// reproduction of 48's probe 1 (same crate, same E0275, unmodified).
pub fn regroup_fold_reduced_headroom_real<Hd, Am1>(_xs: &[i32])
where
    Hd: Pos,
    Am1: Pos,
    Ratio<Hd, Am1>: TowerReduce,
{
}
