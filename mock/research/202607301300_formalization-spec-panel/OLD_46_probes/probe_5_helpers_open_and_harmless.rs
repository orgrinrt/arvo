//! The helper traits (`Dbl`, `DblInc`, `AsPos`, `Cmp`, `NSub`, `Gcd`,
//! `ExactDivOdd`, `Strip2`, `Reduce`, ...) carry NO seal, deliberately,
//! and this probe states what that costs: nothing. A downstream crate CAN
//! implement any of them for its own local types (first half, expected
//! clean: the impls are orphan-legal and overlap nothing, since the
//! upstream impls are on the closed constructors, not blankets over
//! Self). What it CANNOT do is move any such type into the tower, because
//! every consuming position re-bounds its inputs on the sealed carriers
//! (second half, expected E0277). The guarantee is quantified over
//! `Pos`/`Nat`/`Adjustment`/`Bias` obligations, never over helper-trait
//! obligations, and this probe is that sentence, compiled.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern vu_core=libvu_core.rlib probe_5_helpers_open_and_harmless.rs

#![allow(dead_code)]

use vu_core::nat::{Dbl, Pos, Ratio, Reduce, H, Z};

pub struct LocalNat;

// open: a local type may implement a helper trait, with any answer at all
impl Dbl for LocalNat {
    type Out = Z;
}

// harmless: the local type cannot enter the tower anyway. The reduction
// chain's own bound (`N: Pos`) refuses it before Dbl's answer could ever
// be consulted. Forced through a const that reads the projection's VAL;
// the bare-alias form defers the check and proves nothing.
pub const ATTACK: u64 = <<Ratio<LocalNat, H> as Reduce>::N as Pos>::VAL;
