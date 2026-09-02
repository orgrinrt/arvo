//! Probe 3c: COMMITTED REFUSING, on purpose. The negative control for probe 3.
//!
//! Probe 3's five call sites each annotate a grade in their return type and
//! compile. That is only evidence if the annotation is CHECKED rather than
//! inferred, so this probe annotates the wrong one: `Precise` below interior
//! safety, claimed as `Folded<Faithful>`.
//!
//! Two halves. The first is the consumer understating the grade of their own
//! combinator, which is the direction file 37 requires to refuse. The second is
//! the consumer's downstream contract refusing a weakened value, which is file
//! 37's probe 4d in the projected form.
//!
//! EXPECTED: E0308 twice, naming the grades.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_3c_the_projection_is_checked.rs

#![allow(dead_code)]

#[path = "probe_3_the_grade_is_projected.rs"]
mod mechanism;

use mechanism::{needs_faithful_definedness, regroup_fold, Faithful, Folded, Refuse, Signed};
use tower::nat::{H, I};

/// Understating: the projection says `RefusalsTransferred`, the signature says
/// `Faithful`.
pub fn understated(xs: &[i32]) -> Folded<Faithful> {
    regroup_fold::<Refuse, Refuse, Signed, H, I<H>>(xs)
}

/// The caller contract: a definedness-faithful consumer handed the `Precise`
/// regrouping below interior safety.
pub fn caller_contract(xs: &[i32]) -> i32 {
    needs_faithful_definedness(mechanism::precise_below(xs))
}
