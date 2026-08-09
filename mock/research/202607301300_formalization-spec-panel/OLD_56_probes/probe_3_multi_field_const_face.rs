//! Probe 3: probe 1's face is a single `u64`. A real numeral's face needs
//! more than one field readable at once: at minimum precision and a signed
//! rational adjustment (`49:99-132`), not a single integer. This probe
//! checks whether `adt_const_params` (allowed, `unstable-features.md`)
//! lets a face carry a genuine struct as its const parameter and still
//! print readably, and whether a signed rational specifically survives
//! (negative numbers and a two-field ratio are exactly the shape `Bias`
//! needs and a bare `u64` cannot hold).
//!
//! EXPECTED: unknown going in.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern tower_nat=libtower_nat.rlib
//!   probe_3_multi_field_const_face.rs

#![allow(dead_code)]
#![feature(adt_const_params)]
use core::marker::ConstParamTy;
use core::marker::PhantomData;

/// The face's readable spec: precision plus a signed rational bias,
/// mirroring `49:99-132` in miniature (magnitude only, no `Nat`/`Pos`
/// underneath; this probe is about what the DIAGNOSTIC shows, not about
/// re-deriving the tower).
#[derive(PartialEq, Eq, ConstParamTy)]
pub struct Spec {
    pub precision: u16,
    pub bias_num: i32,
    pub bias_den: u32,
}

pub struct NumFace<const S: Spec>(PhantomData<()>);

pub fn needs_q15<const S: Spec>(_: NumFace<S>) {}

pub const Q15: Spec = Spec {
    precision: 15,
    bias_num: 0,
    bias_den: 1,
};
pub const Q15_HALF_BIAS: Spec = Spec {
    precision: 15,
    bias_num: 1,
    bias_den: 2,
};
pub const Q15_NEG_BIAS: Spec = Spec {
    precision: 15,
    bias_num: -3,
    bias_den: 4,
};

pub fn declare_q15(_: NumFace<Q15>) {}

pub fn consumer(x: NumFace<Q15_HALF_BIAS>) {
    declare_q15(x); // mismatch: Q15 vs Q15_HALF_BIAS
}

pub fn consumer_neg(x: NumFace<Q15_NEG_BIAS>) {
    declare_q15(x); // mismatch, and the const carries a NEGATIVE field
}
