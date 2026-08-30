//! Attack class 4: no impl fabricated at all; instead, well-formed TYPES
//! that are not inhabitants are pushed at bounded positions. Three shapes:
//! an unreduced genuine pair as an `Adjustment` (six-twelfths), the same
//! as a `Bias` magnitude, and a padded constructor over a non-`Pos`
//! parameter. EXPECTED: all refused. The first two by E0271 (the
//! `Gcd<D, Out = H>` associated-type equality reports the gcd, which is
//! not `H`); the third by E0277 (`Evil: Pos` unsatisfied). These re-run
//! file 41's probe_1b class against the composed, fully sealed tower.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_3d_malformed_types_refused.rs

#![allow(dead_code)]

use vu_core::bias::{BPos, Bias};
use vu_core::nat::{Adjustment, Pos, Ratio, H, I, O};

type P6 = O<I<H>>; // 6
type P12 = O<O<I<H>>>; // 12

pub struct AdjPos<A: Adjustment>(core::marker::PhantomData<A>);
pub struct BiasPos<B: Bias>(core::marker::PhantomData<B>);
pub struct PosPos<P: Pos>(core::marker::PhantomData<P>);

pub struct Evil;

// forced through fn signatures: a bare type alias defers its bound
// checks (this probe's own first draft compiled clean for exactly that
// reason, recorded in OUTCOMES.md as the tautology it was).
pub fn unreduced_adjustment(_: AdjPos<Ratio<P6, P12>>) {} // E0271: gcd is 6, not H
pub fn unreduced_bias(_: BiasPos<BPos<P6, P12>>) {} // E0271, same route
pub fn padded_foreign(_: PosPos<O<Evil>>) {} // E0277: Evil is not Pos
