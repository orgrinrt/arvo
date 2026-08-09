//! Probe 2c: route (b) of the grade-carrier attack, the supertrait itself.
//!
//! Split from probe 2b because E0603 is a resolution error and aborts before
//! any trait check reports; see probe 2b's header.
//!
//! EXPECTED: FAILS, E0603, module `sealed` is private.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib
//!   --extern grade_lib=libgrade_lib.rlib probe_2c_the_seal_is_unnameable.rs

#![allow(dead_code)]

pub struct TotallyFineGrade;

impl grade_lib::sealed::GradeSealed for TotallyFineGrade {}
