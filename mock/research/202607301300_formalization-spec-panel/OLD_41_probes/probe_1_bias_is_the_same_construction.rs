//! Probe 1: `Bias` construction, value uniqueness, and the MATLAB witness
//! that made the plain-integer reading (`36:222`) wrong.
//!
//! `BZero | BPos<N, D> | BNeg<N, D>` with `N: Pos + Gcd<D, Out = H>, D: Pos`
//! for the two non-zero variants. Uniqueness by the same induction as
//! `Pos`/`Nat`/`Adjustment`: `BZero` denotes zero and nothing else, because
//! `BPos<N,D>`/`BNeg<N,D>` require `N: Pos`, which excludes zero by
//! construction (`Pos` has no zero constructor), so neither variant can
//! ever denote zero. A sign-magnitude encoding's classic defect, a
//! spellable negative zero, has no type here for the same reason `O<Z>` is
//! not a `Pos`: there is no constructor that could produce it.
//!
//! The magnitude bound is written directly against the reduction condition
//! rather than against the `Adjustment` trait; probe 4 is why.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1_bias_is_the_same_construction.rs
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#[path = "vu_bias.rs"]
mod bias;

use bias::nat::{Pos, H, I, O};
use bias::{BNeg, BPos, BZero, Bias, ReducedBiasNeg, ReducedBiasPos};

pub type P1 = H;
pub type P2 = O<H>;
pub type P3 = I<H>;
pub type P4 = O<O<H>>;
pub type P5 = I<O<H>>;

fn same_type<T>(_: core::marker::PhantomData<T>, _: core::marker::PhantomData<T>) {}

// --- CLAIM A: construction and sign. ---

const _: () = assert!(<BPos<P1, P2> as Bias>::NUM == 1);
const _: () = assert!(<BPos<P1, P2> as Bias>::DEN == 2);
const _: () = assert!(<BNeg<P1, P2> as Bias>::NUM == -1);
const _: () = assert!(<BNeg<P1, P2> as Bias>::DEN == 2);
const _: () = assert!(<BZero as Bias>::NUM == 0);
const _: () = assert!(<BZero as Bias>::DEN == 1);

// --- CLAIM B: the MATLAB witness (`39:135-136`). Slope 1, bias 1/2 is a ---
// --- legal MATLAB numerictype, and `Bias = Int` (`36:222`) could not name ---
// --- it: for every integer bias the value set is the integers, and 1/2's ---
// --- value set is the half-integers, disjoint from every one of them. ---
// --- Here it is a representable Bias, directly. ---

pub type MatlabBiasHalf = BPos<P1, P2>;
const _: () = assert!(<MatlabBiasHalf as Bias>::NUM == 1);
const _: () = assert!(<MatlabBiasHalf as Bias>::DEN == 2);

// file 39 probe 1's second witness, biases 1/2 and 5/2, both representable.
pub type MatlabBiasFiveHalves = BPos<P5, P2>;
const _: () = assert!(<MatlabBiasFiveHalves as Bias>::NUM == 5);
const _: () = assert!(<MatlabBiasFiveHalves as Bias>::DEN == 2);

// --- CLAIM C: normalisation at the naming site. Two spellings of one ---
// --- signed rational unify before anything asks whether they do, the ---
// --- same discipline `Reduced<N, D>` gives `Adjustment`. ---

pub type P6 = O<I<H>>;
pub type P12 = O<O<I<H>>>;

const _: () = assert!(<ReducedBiasPos<P6, P12> as Bias>::NUM == 1);
const _: () = assert!(<ReducedBiasPos<P6, P12> as Bias>::DEN == 2);
const _: () = assert!(<ReducedBiasNeg<P6, P12> as Bias>::NUM == -1);
const _: () = assert!(<ReducedBiasNeg<P6, P12> as Bias>::DEN == 2);

pub fn both_spellings_of_one_half_are_one_bias_type() {
    same_type(
        core::marker::PhantomData::<ReducedBiasPos<P6, P12>>,
        core::marker::PhantomData::<BPos<P1, P2>>,
    );
}

// --- CLAIM D: zero has one spelling. No construction reaches a distinct ---
// --- type that also denotes zero; there is exactly one Bias type with ---
// --- NUM == 0, and it is BZero. (Compiled indirectly: every BPos<N,D> and ---
// --- BNeg<N,D> in this file has N: Pos, hence N::VAL >= 1, hence NUM != 0. ---
// --- There is no N with N::VAL == 0 to instantiate the claim against, ---
// --- which is the perimeter argument itself: the type that would carry a ---
// --- second zero has no way to be named.) ---

pub fn there_is_one_zero_bias() {
    same_type(
        core::marker::PhantomData::<BZero>,
        core::marker::PhantomData::<BZero>,
    );
}

// unused-import guard
const _NAT_REACHABLE: u64 = <P3 as Pos>::VAL;
const _NAT_REACHABLE_2: u64 = <P4 as Pos>::VAL;
