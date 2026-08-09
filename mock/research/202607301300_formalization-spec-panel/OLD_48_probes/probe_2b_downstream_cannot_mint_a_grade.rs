//! Probe 2b: the downstream attack on the grade carrier, the same two routes
//! file 41 ran against `Bias` (`41:189-206`).
//!
//! File 47 left `Grade` unsealed and argued (correctly, as far as it went)
//! that `Folded`'s private field alone closes the perimeter. That argument is
//! distributed: it rests on no constructor existing anywhere, which is the
//! exact shape file 46's checklist replaces with a local, per-trait property
//! (`46:section 1`, and its 6.3 finding is the same point one token large).
//! With the seal from probe 2, the argument is local again.
//!
//! Route (a): a downstream type implementing `Grade` directly, refused at
//! the seal. Route (b), naming the seal module itself, is probe 2c, split
//! into its own file because its E0603 is a resolution error that aborts the
//! build before route (a)'s trait check would report, and a probe whose
//! second refusal is shadowed by its first is asserting one thing while
//! looking like two.
//!
//! EXPECTED: FAILS, E0277 on the private supertrait.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern grade_lib=libgrade_lib.rlib probe_2b_downstream_cannot_mint_a_grade.rs

#![allow(dead_code)]

use grade_lib::Grade;

pub struct TotallyFineGrade;

// the direct impl.
impl Grade for TotallyFineGrade {
    const BITS: u8 = 0; // claims faithfulness, checked by nobody
}
