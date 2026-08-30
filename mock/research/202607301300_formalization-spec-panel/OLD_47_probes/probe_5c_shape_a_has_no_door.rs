//! Probe 5c: COMMITTED REFUSING, on purpose. Under shape A the consumer who
//! cannot widen has no program to write.
//!
//! `probe_3_the_grade_is_projected.rs` is shape A: one combinator, the grade
//! projected. A consumer whose contract is `Folded<Faithful>` and whose
//! accumulator is not interior-safe reaches for the sequential fold and finds
//! the name does not resolve, because shape A does not ship one.
//!
//! This is what "only the definedness-faithful form" and "the published-grade
//! form as well" actually cost, in the consumer's editor. Under shape A the
//! remedy is to change the storage numeral, which is often correct and
//! sometimes not available.
//!
//! EXPECTED: E0425, the name does not exist.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   probe_5c_shape_a_has_no_door.rs
//!
//! Verbatim diagnostic in OUTCOMES.md.

#![allow(dead_code)]

#[path = "probe_3_the_grade_is_projected.rs"]
mod mechanism;

use mechanism::{Faithful, Folded, Refuse, Signed};
use tower::nat::{H, O};

type Arity = O<O<O<O<H>>>>;
type NarrowHeadroom = O<H>;

pub fn alarm_threshold(f: Folded<Faithful>) -> i32 {
    f.0
}

pub fn cannot_widen(xs: &[i32]) -> i32 {
    alarm_threshold(mechanism::fold_sequential::<
        Refuse,
        Refuse,
        Signed,
        NarrowHeadroom,
        Arity,
    >(xs))
}
