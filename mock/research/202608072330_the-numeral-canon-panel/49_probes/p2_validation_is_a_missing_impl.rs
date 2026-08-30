// Probe 2: does "validates" fall out of the trait mechanism for free, as a
// missing-impl compile error, with no separate runtime or const-eval check
// needed?
//
// Expected outcome: FAILS TO COMPILE, and the diagnostic names a missing
// trait bound rather than a panicking assertion. That failure is the
// result this probe is checking for; do not "fix" it.
//
// Compile with:
//   rustc +nightly-2026-05-28 --edition 2024 --crate-type lib \
//     p2_validation_is_a_missing_impl.rs

#![no_std]
#![allow(dead_code)]

include!("_shared_schema.rs");

// no impl exists for Cold at N = 999 (unsigned). asking for its Storage
// should be refused by the trait solver, not accepted and then panic at
// some later const-eval or runtime step.
type Bogus = <Cold as NumeralFacts<Unsigned, 999>>::Storage;

pub fn force_resolution() -> Bogus {
    unreachable!()
}
