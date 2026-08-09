//! Attack class 1: implement each sealed trait directly on a local type.
//! Four impls, one per guarantee-carrying trait. EXPECTED: every one
//! refused (E0277, unsatisfied private-supertrait bound), including
//! `Adjustment`, which probe 1b shows was NOT refused before this file's
//! fix.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_3_direct_impls_refused.rs

#![allow(dead_code)]

use vu_core::bias::Bias;
use vu_core::nat::{Adjustment, Nat, Pos};

pub struct EvilPos;
pub struct EvilNat;
pub struct EvilAdj;
pub struct EvilBias;

impl Pos for EvilPos {
    const VAL: u64 = 4;
}

impl Nat for EvilNat {
    const VAL: u64 = 4;
}

impl Adjustment for EvilAdj {
    const NUM: u64 = 6;
    const DEN: u64 = 12;
}

impl Bias for EvilBias {
    const NUM: i64 = 1;
    const DEN: u64 = 2;
}
