//! Attack class 3: file 42's own headline attack, replayed against this
//! file's tower as a regression check that adding the `Adjustment` seal
//! loosened nothing. A foreign `Pos` with a lying `Gcd` (unconditional
//! `Out = H`, no Stein step run), fed to `Adjustment`'s and `Bias`'s
//! blanket impls. EXPECTED: refused at the shared root, `impl Pos for
//! Fabricated`, E0277 on the private supertrait, exactly as
//! `42_probes/probe_3b` records.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_3c_fabricated_pos_replay.rs

#![allow(dead_code)]

use vu_core::bias::{BPos, Bias};
use vu_core::nat::{Adjustment, Gcd, Pos, Ratio, H, O};

pub struct Fabricated;

impl Pos for Fabricated {
    const VAL: u64 = 4;
}

// the lying gcd: claims coprimality with everything, computes nothing
impl<Rhs> Gcd<Rhs> for Fabricated {
    type Out = H;
}

type D4 = O<O<H>>;

pub struct AdjPos<A: Adjustment>(core::marker::PhantomData<A>);
pub struct BiasPos<B: Bias>(core::marker::PhantomData<B>);

// both routes 42 compiled, replayed; fn-forced, since a bare type alias
// defers its bound checks
pub fn attack_adjustment(_: AdjPos<Ratio<Fabricated, D4>>) {}
pub fn attack_bias(_: BiasPos<BPos<Fabricated, D4>>) {}
