//! A hygiene finding probe_4's clean compile pointed at. The operation
//! trait `BiasProduct<Rhs>` declares NO bound on `Rhs`
//! (`vu_bias_sealed_adj.rs`: `pub trait BiasProduct<Rhs> { type Out:
//! Bias; }`), so the same coherence door probe_4 opened admits a
//! downstream impl over a non-`Bias` right operand. EXPECTED: compiles
//! clean. This does NOT breach the uniqueness guarantee (the declared
//! `Out: Bias` bound means only genuine inhabitants come out, and the
//! evil impl fires only on the downstream's own nonsense query), but it
//! is the one shape in the tower where the reachability argument rests on
//! the OUTPUT bound rather than the input bounds, and the spec should
//! close it by writing `trait BiasProduct<Rhs: Bias>` so every parameter
//! of every public trait in the tower is carrier-bounded uniformly.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_4c_unbounded_operand_position.rs

#![allow(dead_code)]

use vu_core::bias::{BZero, BiasProduct};

pub struct NotABias;

impl BiasProduct<NotABias> for BZero {
    type Out = BZero;
}

// the nonsense query type-checks, and its answer is a genuine Bias
pub fn nonsense(_: core::marker::PhantomData<<BZero as BiasProduct<NotABias>>::Out>) {}
