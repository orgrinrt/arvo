//! Probe 5. The attempt to put D15's property on file 07's rung 1.
//!
//! Rung 1 is "computed and witnessed": a const the evaluator can refuse. The
//! fresh-accumulator guarantee quantifies over every input of an opaque
//! closure, so the witness would have to be an exhaustive const-position call
//! of a user closure. This file is expected NOT to compile; the error is the
//! result.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
use p1_arvo::Bool;
use p1_foundation::TruthHolds;

pub const fn witness<F: Fn(&u32, &u32) -> Bool>(f: &F) -> bool {
    let mut x = 0u32;
    loop {
        if x == u32::MAX {
            return true;
        }
        if !f(&0, &x).holds() {
            return false;
        }
        x += 1;
    }
}
