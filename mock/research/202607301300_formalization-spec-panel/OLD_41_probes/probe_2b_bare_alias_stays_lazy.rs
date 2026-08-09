//! Probe 2b: the escape from probe 2's wall, and the shape `vu_bias.rs`
//! actually uses.
//!
//! A bare, top-level type alias referencing `Reduce` for fully generic
//! `N, D` compiles and stays lazy: nothing forces the compiler to normalise
//! its body until something instantiates it with a concrete type, exactly
//! the way an ordinary generic function body is not type-checked against
//! every possible instantiation at its own definition. The difference from
//! probe 2 is not "unbounded vs bounded" or "projection vs equality"
//! (probe 2's variant `(c)` tried a concrete equality and still failed); it
//! is specifically that no TRAIT IMPL is elaborating the alias as one of
//! its own associated types. `Bias`'s own `type Out: Bias` bound, if
//! assigned from inside a trait impl, forces exactly the same eager check
//! probe 2 hits, independent of whether the assigned type is a raw
//! projection or a named alias pointing at one (checked separately, not
//! shown here to keep this file to the one claim).
//!
//! Build: rustc --edition 2021 --crate-type lib probe_2b_bare_alias_stays_lazy.rs
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#[path = "vu_nat.rs"]
mod nat;
use nat::{Gcd, Pos, Ratio, Reduce, H, I, O};

pub struct BPos<N, D>(core::marker::PhantomData<(N, D)>);
pub trait Bias {
    const NUM: i64;
}
impl<N: Pos + Gcd<D, Out = H>, D: Pos> Bias for BPos<N, D> {
    const NUM: i64 = N::VAL as i64;
}

/// Bare alias, no trait wrapping the assignment. This is the shape
/// `vu_bias.rs`'s `ReducedBiasPos`/`ReducedBiasNeg`/`BiasMagN`/`BiasMagD`
/// all use.
pub type ReducedBPos<N, D> = BPos<<Ratio<N, D> as Reduce>::N, <Ratio<N, D> as Reduce>::D>;

pub type P6 = O<I<H>>;
pub type P12 = O<O<I<H>>>;
const _: () = assert!(<ReducedBPos<P6, P12> as Bias>::NUM == 1);
