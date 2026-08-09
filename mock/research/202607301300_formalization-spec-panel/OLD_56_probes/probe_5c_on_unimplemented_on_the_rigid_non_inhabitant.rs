//! Probe 5c: file 46 section 6.2 records a SEPARATE E0275 residual from
//! the one probe 5/5b test: the composition wall fires with the same
//! "overflow evaluating the requirement" text on a CONCRETE, rigid
//! non-inhabitant (`LocalNat`, a real type with no `Pos` impl at all), not
//! only on a fully abstract type parameter. `49:868-871` calls this
//! "worse, anonymous" because a consumer would reasonably expect a
//! concrete wrong type to produce a clear "not implemented" refusal
//! rather than the generic recursion-limit message. This is
//! `46_probes/probe_5`'s own attack, reproduced against the annotated
//! `Reduce`, to check whether the attribute reaches THIS shape of the
//! residual specifically, distinct from probe 5b's abstract-parameter
//! case.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern
//!   tower_annotated=libtower_annotated.rlib
//!   probe_5c_on_unimplemented_on_the_rigid_non_inhabitant.rs

#![allow(dead_code)]

use tower_annotated::nat::{Dbl, Pos, Ratio, Reduce, Z};

pub struct LocalNat;

impl Dbl for LocalNat {
    type Out = Z;
}

pub const ATTACK: u64 = <<Ratio<LocalNat, tower_annotated::nat::H> as Reduce>::N as Pos>::VAL;
