//! Seat 242. Does seat 237's proposed repair survive the case seat 242 found?
//!
//! `237_the_format_proposals_against_the_ratification_gate.md:464` proposes the
//! predicate `PHASE_NUM % PHASE_DEN == 0 && slot_in_range(0)` as the fix that
//! "would make `PHASE_DEN` load-bearing". Seat 242's `admission` probe measured
//! that `PHASE_DEN = 0` is admitted today. `%` by zero is not defined, so the
//! question is what the proposed predicate does at the value that motivated it.
//!
//! The cases that must fail, stated before the run:
//!   - the well-formed arm must evaluate and agree with the crate, or the
//!     harness is not evaluating the predicate at all;
//!   - the zero arm must do something OTHER than return a verdict, or there is
//!     nothing to report.
//!
//! Build: `rustc --edition 2024 -O proposed_phase_predicate.rs -o /tmp/pp`
//! With `--cfg zero_den` for the second arm.

const fn slot_in_range(slot: i64, min: i64, max: i64) -> bool {
    slot >= min && slot <= max
}

/// Seat 237's predicate, transcribed.
const fn proposed(phase_num: i64, phase_den: i64, min: i64, max: i64) -> bool {
    phase_num % phase_den == 0 && slot_in_range(0, min, max)
}

// The well-formed arm: an unbiased grid, which the crate ships four of.
const WELL_FORMED: bool = proposed(0, 1, -8, 7);

// The half-step arm, which is the case the coordinate exists for.
const HALF_STEP: bool = proposed(1, 2, -8, 7);

// The arm seat 242 measured admitted today. A const, so it is forced.
#[cfg(zero_den)]
const ZERO_DEN: bool = proposed(1, 0, -8, 7);

fn main() {
    assert!(WELL_FORMED, "unbiased grid should carry an additive identity");
    assert!(!HALF_STEP, "half-step grid should not carry one");
    println!("control: the predicate evaluates and separates the two shipped phases.");

    #[cfg(zero_den)]
    println!("ZERO_DEN evaluated to {ZERO_DEN}: the proposed predicate returned a verdict at PHASE_DEN = 0.");

    #[cfg(not(zero_den))]
    println!("zero arm not built; rerun with --cfg zero_den.");
}
