//! Probe 2: `Reduce` cannot be composed inside another trait's generic
//! machinery, only invoked as a bare alias at a concrete numeral pair.
//!
//! This is the finding that decided how `BiasProduct` in `vu_bias.rs` is
//! shaped. The first attempt at `bias = B1 * B2` wrote a fully generic
//! `BiasMul<N1, D1, N2, D2>` trait, mirroring `Adjustment`'s own shape
//! (`impl<N: Pos + Gcd<D, Out = H>, D: Pos> Adjustment for Ratio<N, D>`).
//! It does not compile, and this file isolates why down to the smallest
//! case: nothing about `Bias`, nothing about multiplication, nothing about
//! sign. A bare function with an unused where-clause naming `Reduce` for two
//! fully abstract `Pos` parameters is refused before anything calls it.
//!
//! `(a)` is the control: `Gcd`, the bound `Adjustment` already ships,
//! composes fine generically, unused, no call site. `(b)` and `(c)` are
//! `Reduce` in the same position, as a bare bound and as an associated-type
//! equality to a concrete type respectively; both fail identically.
//!
//! The mechanism, read from the diagnostic rather than guessed: `Reduce`'s
//! only impl is a blanket `impl<N: Pos, D: Pos> Reduce for Ratio<N, D>`,
//! matching any `Ratio<_,_>` unconditionally, so the solver commits to it
//! and must then discharge its own where-clauses (`Strip2`, transitively
//! `Gcd`, `ExactDivOdd`). Those are defined over `Nat`'s `Pz<P>` wrapper by
//! explicit pattern (`Pz<O<P>>`, `Pz<I<P>>`, `Pz<H>`), and for an abstract
//! `Pz<X>` the solver can unify `X` against `O<P>` by inventing a fresh `P`,
//! then repeat on that fresh `P`, with no base case to stop it: hence the
//! ever-deepening `Pz<O<O<O<...>>>>` in the trace and the eventual overflow.
//! `Gcd` never reaches this, because its own impls pattern-match directly on
//! `Pos`'s three constructors with no such wrapper position to unify a fresh
//! variable into.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_2_generic_reduce_composition_is_refused.rs
//! Outcome: (a) WORKS. (b) and (c) FAIL WITH E0275 ("overflow evaluating
//! the requirement `Pz<O<_>>: ExactDivOdd<_>`"), verbatim in OUTCOMES.md.
//! rustc 1.98.0-nightly (57d06900f 2026-05-27). Committed refusing, on
//! purpose. Do not "fix" this file; probe 2b is the fix, in a different
//! shape.

#![allow(dead_code)]
#[path = "vu_nat.rs"]
mod nat;
use nat::{Gcd, Pos, Ratio, Reduce, H};

/// (a) control: a bound already shipped in `Adjustment`, unused, no call
/// site. Compiles.
fn gcd_bound_only<N: Pos + Gcd<D, Out = H>, D: Pos>() {}

/// (b) the same shape with `Reduce` instead of `Gcd`, as a bare bound,
/// unused, no call site. Does not compile.
fn reduce_bound_only<N: Pos, D: Pos>()
where
    Ratio<N, D>: Reduce,
{
}

/// (c) `Reduce` with an associated-type equality to a concrete type on one
/// side, mirroring `Gcd<Out = H>`'s own shape exactly. Does not compile
/// either: the failure is not about which side is concrete.
fn reduce_concrete_n<N: Pos, D: Pos>()
where
    Ratio<N, D>: Reduce<N = H, D = D>,
{
}
