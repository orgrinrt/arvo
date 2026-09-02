//! Probe 3: the exact division subfamily, carried by machinery that already exists.
//!
//! File 28 named division by a power of the radix as an exact subfamily worth offering
//! (`28:329-336`), and the dispatch asks whether "the design's own machinery (a
//! numeral's exponent, its radix, its normal form) should carry it without new
//! mechanism." Tested here, and the answer is stronger than the claim: under the
//! ratified rational adjustment, the exact subfamily is not division by a radix power,
//! it is division by ANY fixed nonzero representable constant, and the numeral-level
//! map for it is the ALREADY-BUILT rational multiplication of files 41/42 with the
//! constant's components swapped. Nothing new is built in this file; every alias below
//! is `PMul` + `Reduce` from `vu_bias_sealed.rs` / `vu_nat_sealed.rs` (the sealed
//! tower, file 42's fix), applied at concrete types, which per files 41/42 is the one
//! shape the toolchain accepts for `Reduce` composition anyway.
//!
//! The mechanism: dividing every value of a numeral by a constant c = cn/cd maps
//! value k*A1 + B1 to k*(A1/c) + (B1/c). So the result numeral has
//!   adjustment A1 * (cd/cn), bias B1 * (cd/cn), same index set, same precision,
//! and the operation on indices is the identity: k passes through untouched. That is
//! what "exact" means here, and it is why the operation costs nothing at runtime for
//! the dyadic case (an exponent reinterpretation) and nothing but the type-level
//! reduction otherwise.
//!
//! Two notes the review's own record makes load-bearing:
//!
//! 1. Reciprocal-multiply is the ILLEGITIMATE form at the value level (two roundings
//!    against one; my own file 24, `24:403-413`) and the EXACT form at the numeral
//!    level: type-level rational arithmetic has no rounding to double, so
//!    "divide-by-c = multiply-by-(cd/cn)" is not the arcp liberty here, it is an
//!    identity of rationals. The same move that is a licensed liberty one level down
//!    is a theorem one level up.
//!
//! 2. Totality is by construction, not by check: the constant's numerator position is
//!    `Pos`-bounded, and no `Pos` constructor produces zero (`41:132-141`, the same
//!    no-constructor induction as the whole tower), so divide-by-zero is unspellable
//!    for this subfamily. With file 38's correction (`40:279-287`), IS_EXACT together
//!    with totality trivialises the grade monoid: the exact subfamily gets every law
//!    at the finest view, by construction, with no quantiser and no cause.
//!
//! Witnesses (all cross-checked against Python fractions.Fraction before spelling,
//! recorded in OUTCOMES.md):
//!   A: 3/4 divided by 4      -> 3/16   (the radix-power case: denominator stays dyadic,
//!                                       the lowering is an exponent shift)
//!   B: 3/4 divided by 3/2    -> 1/2    (raw 6/12; the reduction renormalises)
//!   C: 2/3 divided by 2/3    -> 1/1    (self-division; the unit)
//!   D: bias 1/2 divided by 3/2 -> 1/3  (the bias divides through the same alias)
//!
//! Build: rustc --edition 2021 --crate-type lib probe_3_exact_division_by_a_constant.rs --out-dir <dir>
//! Outcome: WORKS (all four witnesses, every component asserted).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

#[path = "vu_bias_sealed.rs"]
pub mod bias;

use bias::nat::{Pos, Ratio, Reduce};
use bias::nat::{H, I, O};
use bias::PMul;

// Small positives in the value-unique encoding: H=1, O<P>=2P, I<P>=2P+1.
type P1 = H;
type P2 = O<H>;
type P3 = I<H>;
type P4 = O<O<H>>;
type P16 = O<O<O<O<H>>>>;

/// The raw componentwise quotient numerator and denominator: A1 * (cd/cn).
/// Identical shape to file 41's `BiasMagN`/`BiasMagD` with the constant inverted.
type DivAdjN<A1n, A1d, Cn, Cd> =
    <Ratio<<A1n as PMul<Cd>>::Out, <A1d as PMul<Cn>>::Out> as Reduce>::N;
type DivAdjD<A1n, A1d, Cn, Cd> =
    <Ratio<<A1n as PMul<Cd>>::Out, <A1d as PMul<Cn>>::Out> as Reduce>::D;

// Witness A: adjustment 3/4, constant 4/1 -> 3/16. The denominator stays a power of
// two: this is the exponent-shift case, `Implicit<E, A, B>` lowering to E - 2.
const _A_N: () = assert!(<DivAdjN<P3, P4, P4, P1> as Pos>::VAL == 3);
const _A_D: () = assert!(<DivAdjD<P3, P4, P4, P1> as Pos>::VAL == 16);
// And the value-unique encoding says 3/16 is ONE type; assert against the literal.
const _A_TY: fn(DivAdjN<P3, P4, P4, P1>) -> P3 = |x| x;
const _A_TY_D: fn(DivAdjD<P3, P4, P4, P1>) -> P16 = |x| x;

// Witness B: adjustment 3/4, constant 3/2 -> raw 6/12 -> reduced 1/2.
const _B_N: () = assert!(<DivAdjN<P3, P4, P3, P2> as Pos>::VAL == 1);
const _B_D: () = assert!(<DivAdjD<P3, P4, P3, P2> as Pos>::VAL == 2);

// Witness C: adjustment 2/3, constant 2/3 -> 1/1.
const _C_N: () = assert!(<DivAdjN<P2, P3, P2, P3> as Pos>::VAL == 1);
const _C_D: () = assert!(<DivAdjD<P2, P3, P2, P3> as Pos>::VAL == 1);

// Witness D: bias 1/2 divided by the same constant 3/2 -> 1/3, through the SAME
// alias, because the map is the same map: the bias is one more rational the constant
// divides. (Sign untouched: a negative constant flips SignDomain, an identity-axis
// consequence this probe does not model; noted in the deliverable.)
const _D_N: () = assert!(<DivAdjN<P1, P2, P3, P2> as Pos>::VAL == 1);
const _D_D: () = assert!(<DivAdjD<P1, P2, P3, P2> as Pos>::VAL == 3);
