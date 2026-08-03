//! The adversary, a genuinely separate downstream crate against
//! `tower_as_42_left_it`. This is file 41's probe_4b attack, unchanged in
//! spirit: implement `Adjustment` DIRECTLY on a local type, fabricating
//! the consts. No `Pos`, no `Ratio`, no `Gcd`, so the `Pos`/`Nat` seal
//! (file 42's fix) is never touched.
//!
//! EXPECTED: COMPILES CLEAN, which is the defect. A foreign `Adjustment`
//! carrying NUM = 6, DEN = 12 (the unreduced six-twelfths, a second
//! spelling of the value `Reduced<H, O<H>>` already names as 1/2) reaches
//! any `A: Adjustment`-bounded position, e.g. the identity contract's
//! `Implicit<E, A: Adjustment, B: Bias>` (`40:70`), defeating both
//! validity (unreduced) and value-uniqueness (two types, one value).
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern tower_as_42_left_it=libtower_as_42_left_it.rlib
//!   probe_1b_foreign_adjustment_still_lands.rs

#![allow(dead_code)]

use tower_as_42_left_it::nat::Adjustment;

/// The fabricated adjustment. Six-twelfths, never reduced, never a
/// rational at the type level at all: two integers typed in by hand.
pub struct EvilAdjustment;

impl Adjustment for EvilAdjustment {
    const NUM: u64 = 6;
    const DEN: u64 = 12;
}

/// A stand-in for any Adjustment-bounded position in the design
/// (the identity contract's `Implicit<E, A: Adjustment, B: Bias>` is the
/// shipped-shape example).
pub struct Quantum<A: Adjustment>(core::marker::PhantomData<A>);

// forced through a fn signature, which IS well-formedness-checked
// (a bare type alias is not; its bounds defer, a lesson probe_3d in this
// directory learned the measured way).
pub fn landed(_: Quantum<EvilAdjustment>) {}

// And the value-uniqueness defeat, stated as consts: two types, same
// denoted quantum (1/2 after reduction), different NUM/DEN, both
// admitted wherever `A: Adjustment` is the bound.
const _: () = assert!(EvilAdjustment::NUM == 6 && EvilAdjustment::DEN == 12);
