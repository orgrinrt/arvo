//! Probe 2c: probe 2b's two committed refusals.
//!
//! SECTION 1 is shape 1: multiply the trip count and the step arity as consts.
//! The compiler names the forbidden feature.
//!
//! SECTION 2 is the actionability negative control. `spectral_bisection` reads
//! only signs and is correct at `EventsTransferred`; a consumer that reads
//! MAGNITUDES is not, and its bound is what stops it. Without this refusal, the
//! published grade would be decoration.
//!
//! EXPECTED: FAILS. Errors verbatim in OUTCOMES.md.
//!
//! Compiled as: rustc --edition 2021 --crate-type lib --extern tower=libtower.rlib
//!   --extern grade_lib=libgrade_lib.rlib --extern p2b=libprobe_2b_the_arity_of_an_unbounded_loop.rlib
//!   probe_2c_the_refusals_behind_the_fixpoint.rs

#![allow(dead_code)]

use p2b::{fiedler_hot, needs_faithful};
use tower::nat::Pos;

// SECTION 1. The arity of `trips` iterations of a `step`-wide body.
pub fn iterate_const_arity<Hd: Pos, const TRIPS: usize, const STEP: usize>()
where
    Hd: p2b::InteriorSafety<{ TRIPS * STEP }>,
{
}

// SECTION 2. The magnitude consumer against a wrapping solver.
pub fn magnitude_on_hot(t: usize) -> u8 {
    needs_faithful(fiedler_hot(t))
}

// SECTION 3. And the same consumer against a `Precise` solver, which also
// refuses, for the other reason: definedness may differ.
pub fn magnitude_on_precise(t: usize) -> u8 {
    needs_faithful(p2b::fiedler_precise(t))
}
