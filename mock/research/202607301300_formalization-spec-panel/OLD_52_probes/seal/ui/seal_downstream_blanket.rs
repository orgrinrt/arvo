//! Attack class 6: the last introduction route in the enumeration, a
//! downstream BLANKET impl of a sealed trait over a type parameter
//! (`impl<T: Local> Adjustment for T`), which would mint inhabitants
//! wholesale if admitted. EXPECTED: E0210, the uncovered-type-parameter
//! orphan rule, refused before the seal is even consulted. Recorded so
//! the introduction-route enumeration is exhaustive by error class
//! (E0277 seal, E0603 privacy, E0117 orphan, E0210 uncovered parameter),
//! not merely by the attacks someone thought of.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_3f_downstream_blanket_refused.rs

#![allow(dead_code)]

use vu_core::nat::Adjustment;

pub trait LocalLicence {
    const N: u64;
    const D: u64;
}

impl<T: LocalLicence> Adjustment for T {
    const NUM: u64 = T::N;
    const DEN: u64 = T::D;
}
