//! Probe A2: the bound refuses a numeral whose finest system sits above
//! the bound. EXPECTED TO FAIL TO COMPILE; the E0277 is the result.
//! Self-contained restatement of probe A's minimal surface (an
//! `include!` of a file with an inner doc comment does not parse).
//!
//! Compile with: rustc --edition 2021 --crate-type lib
//! Expected: `Dyadic: ContainedIn<Zint>` is not satisfied.

#![no_std]

pub struct Nat;
pub struct Zint;
pub struct Dyadic;

pub trait ContainedIn<Upper> {}
impl ContainedIn<Nat> for Nat {}
impl ContainedIn<Zint> for Nat {}
impl ContainedIn<Dyadic> for Nat {}
impl ContainedIn<Zint> for Zint {}
impl ContainedIn<Dyadic> for Zint {}
impl ContainedIn<Dyadic> for Dyadic {}

pub trait Numeral {
    type System;
}
pub trait Inhabits<S> {}
impl<N: Numeral, S> Inhabits<S> for N where N::System: ContainedIn<S> {}

pub struct ModelU5_3; // UFixed<5, 3, S>: finest system Z[1/2]
impl Numeral for ModelU5_3 {
    type System = Dyadic;
}

pub fn needs_integers<N: Inhabits<Zint>>() {}

pub fn check_refusal() {
    // ModelU5_3's finest system is Dyadic; Dyadic is not contained in Z.
    needs_integers::<ModelU5_3>();
}
