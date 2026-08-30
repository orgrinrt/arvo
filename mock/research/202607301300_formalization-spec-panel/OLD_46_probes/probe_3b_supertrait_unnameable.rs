//! Attack class 2: implement the private supertrait itself, the route a
//! seal's guarantee is quantified over being unavailable. EXPECTED:
//! refused, E0603 (module `sealed` is private) or E0433, at the path,
//! before any trait solving happens.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_3b_supertrait_unnameable.rs

#![allow(dead_code)]

pub struct Evil;

impl vu_core::nat::sealed::PosSealed for Evil {}
impl vu_core::nat::sealed::AdjustmentSealed for Evil {}
impl vu_core::bias::bias_sealed::BiasSealed for Evil {}
