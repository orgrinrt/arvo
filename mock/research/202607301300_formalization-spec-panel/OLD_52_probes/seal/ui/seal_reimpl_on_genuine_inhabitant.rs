//! Attack class 5: do not add a new inhabitant; RE-implement a sealed
//! trait (or a helper whose answers the tower consumes) on a GENUINE
//! inhabitant, changing its answer. Three shapes: re-implement
//! `Adjustment` for `Ratio<H, H>` with lying consts; re-implement `Gcd`
//! for a concrete genuine pair; re-implement `Pos` for `H` with a lying
//! `VAL`. EXPECTED: all refused by E0117 (orphan rule: only traits
//! defined in the current crate can be implemented for arbitrary types)
//! or E0119 (conflicting implementation), before any values could be
//! corrupted.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_3e_reimpl_on_genuine_inhabitant.rs

#![allow(dead_code)]

use vu_core::nat::{Adjustment, Gcd, Pos, Ratio, H, I, O};

impl Adjustment for Ratio<H, H> {
    const NUM: u64 = 7;
    const DEN: u64 = 3;
}

impl Gcd<O<H>> for I<H> {
    type Out = I<H>; // lie: gcd(3, 2) reported as 3
}

impl Pos for H {
    const VAL: u64 = 99;
}
