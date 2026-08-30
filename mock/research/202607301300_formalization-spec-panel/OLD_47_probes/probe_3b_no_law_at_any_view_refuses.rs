//! Probe 3b: COMMITTED REFUSING, on purpose.
//!
//! The clamping composition has no associativity law at any view. In file 37's
//! const form this is a const-eval panic carrying a message
//! (`37_probes/probe_4b`). Here the arm is the ABSENCE of a `FoldGrade` impl,
//! so it is an unsatisfied trait bound with a designed
//! `#[diagnostic::on_unimplemented]`, reported at the call site during type
//! checking rather than during const evaluation.
//!
//! EXPECTED: E0277, carrying probe 3's own four lines of explanation.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_3b_no_law_at_any_view_refuses.rs
//!
//! Verbatim diagnostic in OUTCOMES.md.

#![allow(dead_code)]

#[path = "probe_3_the_grade_is_projected.rs"]
mod mechanism;

use mechanism::{regroup_fold, Clamp, Signed};
use tower::nat::{H, I};

pub fn saturating_fold(xs: &[i32]) -> i32 {
    let f = regroup_fold::<Clamp, Clamp, Signed, H, I<H>>(xs);
    f.0
}
