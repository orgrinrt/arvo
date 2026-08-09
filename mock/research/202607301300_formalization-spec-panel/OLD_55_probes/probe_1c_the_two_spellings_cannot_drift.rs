//! Probe 1c: the negative control on probe 1's CLAIM D.
//!
//! `DimBoth<const N: usize, P: Pos>` carries the capacity twice, once as the
//! array-length const the language demands and once as the `Pos` the
//! interior-safety obligation demands. Two spellings of one quantity is exactly
//! the decorrelation risk the review names elsewhere (`48_probes/
//! probe_2_grade_algebra_lib.rs:64-66`, keeping `BITS` "so the two encodings are
//! provably the same object rather than two statements that will decorrelate").
//!
//! The question this probe answers is whether the forced const assertion
//! actually fires, or whether it is a comment wearing an `assert!`.
//!
//! CLAIM. `DimBoth<63, P64>` is refused at monomorphisation, with the assertion
//! named. So the second spelling costs one line and cannot lie.
//!
//! EXPECTED: FAILS, one const-eval error. Verbatim in OUTCOMES.md.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_1c_the_two_spellings_cannot_drift.rs

#![allow(dead_code)]

use core::marker::PhantomData;
use tower::nat::{Nat, Pos, Pz, H, O};

pub trait CapacityWithNat {
    type Array<T>;
    type Dim: Pos;
    const CAP: usize;
}

pub struct DimBoth<const N: usize, P>(PhantomData<P>);

impl<const N: usize, P: Pos> CapacityWithNat for DimBoth<N, P> {
    type Array<T> = [T; N];
    type Dim = P;
    const CAP: usize = N;
}

pub trait DimAgrees: CapacityWithNat {
    const AGREES: ();
    fn witness() {
        let () = Self::AGREES;
    }
}

impl<const N: usize, P: Pos> DimAgrees for DimBoth<N, P> {
    const AGREES: () = assert!(N as u64 == <Pz<P> as Nat>::VAL);
}

type P64 = O<O<O<O<O<O<H>>>>>>; // 64

/// The honest one. Compiles.
pub fn good() {
    <DimBoth<64, P64> as DimAgrees>::witness();
}

/// The drifted one: sixty-three slots labelled sixty-four.
pub fn drifted() {
    <DimBoth<63, P64> as DimAgrees>::witness();
}
