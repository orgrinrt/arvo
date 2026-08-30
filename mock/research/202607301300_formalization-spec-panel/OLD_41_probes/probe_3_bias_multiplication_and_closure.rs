//! Probe 3: `bias = B1 * B2` (`31:399-400`), lifted from integer to signed-
//! rational algebra, checked against file 39 probe 1's own witnesses.
//!
//! Magnitude and sign are computed separately (see `vu_bias.rs`'s header
//! for why: a single generic trait spanning both is exactly what probe 2
//! refuses). `BiasMagN`/`BiasMagD` are bare aliases (probe 2b's shape):
//! multiply numerator by numerator and denominator by denominator via
//! `PMul`, then `Reduce`. `BiasMulPP`/`PN`/`NN` pick the sign and wrap.
//!
//! CLAIM A is file 39 probe 1's own witness, run at the type level rather
//! than the value level: file 39 checked `bias = 1/2, bias = 5/2` closure
//! exhaustively over 17x17 operand pairs and found every product lands on
//! `adjustment * k + bias` with integer `k` (`39:148-151`); here the
//! product of the two biases themselves, `1/2 * 5/2 = 5/4`, is the type
//! this file compiles and asserts against.
//!
//! CLAIM B is an unreduced-magnitude witness: 2/3 times 3/4 multiplies to
//! 6/12 componentwise, and `BiasMagN`/`BiasMagD` must renormalise it to
//! 1/2, the same reduction file 34's probe 5b needed one layer down (now
//! `36_probes/probe_4`'s CLAIM D, compiling).
//!
//! CLAIM C checks the sign table against probe 6's own four-combination
//! result (`36_probes/probe_6`), now over rational rather than integer
//! magnitudes: positive times positive is positive, positive times
//! negative is negative, negative times negative is positive.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_3_bias_multiplication_and_closure.rs
//! Outcome: WORKS. rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]
#[path = "vu_bias.rs"]
mod bias;

use bias::nat::{Pos, H, I, O};
use bias::{Bias, BiasMagD, BiasMagN, BiasMulNN, BiasMulPN, BiasMulPP};

pub type P1 = H;
pub type P2 = O<H>;
pub type P3 = I<H>;
pub type P4 = O<O<H>>;
pub type P5 = I<O<H>>;

// --- CLAIM A: file 39 probe 1's witness, 1/2 * 5/2 = 5/4, at the type ---
// --- level, all three sign combinations. ---

type ProdPP = BiasMulPP<P1, P2, P5, P2>;
const _: () = assert!(<ProdPP as Bias>::NUM == 5);
const _: () = assert!(<ProdPP as Bias>::DEN == 4);

type ProdPN = BiasMulPN<P1, P2, P5, P2>;
const _: () = assert!(<ProdPN as Bias>::NUM == -5);
const _: () = assert!(<ProdPN as Bias>::DEN == 4);

type ProdNN = BiasMulNN<P1, P2, P5, P2>;
const _: () = assert!(<ProdNN as Bias>::NUM == 5);
const _: () = assert!(<ProdNN as Bias>::DEN == 4);

// --- CLAIM B: 2/3 * 3/4's raw componentwise product is 6/12; the ---
// --- magnitude aliases must renormalise it to 1/2. ---

type MagN = BiasMagN<P2, P3, P3, P4>;
type MagD = BiasMagD<P2, P3, P3, P4>;
const _: () = assert!(<MagN as Pos>::VAL == 1);
const _: () = assert!(<MagD as Pos>::VAL == 2);

type UnreducedProd = BiasMulPP<P2, P3, P3, P4>;
const _: () = assert!(<UnreducedProd as Bias>::NUM == 1);
const _: () = assert!(<UnreducedProd as Bias>::DEN == 2);

// --- CLAIM C: a second pair, to confirm the sign table is not an ---
// --- accident of the first (3/4 and 4/3 multiply to exactly 1/1, the ---
// --- identity case, where a bug in the reduction path would be easiest ---
// --- to miss). ---

type Identity = BiasMulPP<P3, P4, P4, P3>;
const _: () = assert!(<Identity as Bias>::NUM == 1);
const _: () = assert!(<Identity as Bias>::DEN == 1);

type IdentityNeg = BiasMulPN<P3, P4, P4, P3>;
const _: () = assert!(<IdentityNeg as Bias>::NUM == -1);
const _: () = assert!(<IdentityNeg as Bias>::DEN == 1);
