//! Probe 5b: the direct test. `tower_annotated.rs` is file 46's real,
//! ratified `Reduce` trait, unmodified except for one
//! `#[diagnostic::on_unimplemented]` attribute on its declaration. This
//! reproduces 48's probe 1 exactly (the fold-signature refactor that
//! spells interior safety as a reduced headroom ratio) against the
//! annotated copy, and the question is only whether the attribute changes
//! the printed diagnostic at all.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern
//!   tower_annotated=libtower_annotated.rlib
//!   probe_5b_on_unimplemented_on_the_real_reduce.rs

#![allow(dead_code)]

use tower_annotated::nat::{Pos, Ratio, Reduce};

pub fn regroup_fold_reduced_headroom<Hd, Am1>(_xs: &[i32])
where
    Hd: Pos,
    Am1: Pos,
    Ratio<Hd, Am1>: Reduce,
{
}
