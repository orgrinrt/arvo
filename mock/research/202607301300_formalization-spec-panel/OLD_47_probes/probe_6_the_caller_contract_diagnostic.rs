//! Probe 6: can the caller-contract mismatch name its own remedy?
//!
//! Probe 3c's caller contract refuses with a bare E0308: `expected
//! Folded<Faithful>, found Folded<RefusalsTransferred>`. That is correct and
//! it tells the consumer nothing about what to do, because E0308 carries no
//! customisation surface. The two remedies (widen the accumulator until the
//! fold is interior-safe, or take the sequential combinator) are exactly the
//! knowledge the design has and the message does not.
//!
//! CLAIM. Stating the caller's contract as a BOUND rather than an exact type
//! moves the refusal from E0308 to E0277, which does have
//! `#[diagnostic::on_unimplemented]`, so the remedy ships attached to the
//! error. The cost is one extra generic parameter in the consumer's own
//! signature.
//!
//! EXPECTED: the `ok` half compiles, the `refused` half fails with E0277
//! carrying the two remedies. COMMITTED REFUSING in its second half.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_6_the_caller_contract_diagnostic.rs
//!
//! Verbatim diagnostic in OUTCOMES.md.

#![allow(dead_code)]

#[path = "probe_3_the_grade_is_projected.rs"]
mod mechanism;

use mechanism::{Faithful, Folded, Grade};

/// The contract a consumer states when its own correctness depends on the
/// fold's definedness matching the sequential one. Implemented for exactly one
/// grade, so it is the same constraint as naming `Faithful` outright.
#[diagnostic::on_unimplemented(
    message = "this fold's definedness does not match the sequential fold's",
    label = "published grade `{Self}`",
    note = "this combinator may refuse where a sequential fold returned, or return where it refused",
    note = "to get a faithful fold: widen the accumulator numeral until the fold is interior-safe, or call `fold_sequential`, which does not regroup and pays for it"
)]
pub trait Definite: Grade {}
impl Definite for Faithful {}

/// The consumer's own signature. One extra parameter against `Folded<Faithful>`.
pub fn alarm_threshold<G: Definite>(f: Folded<G>) -> i32 {
    f.0
}

pub fn ok(xs: &[i32]) -> i32 {
    alarm_threshold(mechanism::precise_safe(xs))
}

pub fn refused(xs: &[i32]) -> i32 {
    alarm_threshold(mechanism::precise_below(xs))
}
