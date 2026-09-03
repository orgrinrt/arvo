//! Arm D. The verdict form, which does not force the obligation.
//!
//! `is_admissible_format` (`format.rs:241`) reads `F::PHASE.denotes()` and never
//! touches `ADMITTED`, so this is the one call site at which a zero denominator
//! is meant to be observable rather than refused.
//!
//! Predicted: check, build and run all succeed, printing `false`.
//!
//! What must fail for it to mean anything: arm E's control declaration must
//! print `true` through the same function, or the verdict answers `false`
//! regardless of what it is handed.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase, is_admissible_format};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 0);
}

struct HalfStep;

impl Format for HalfStep {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 2);
}

fn main() {
    let bad = is_admissible_format::<NoDenominator>().get();
    let good = is_admissible_format::<HalfStep>().get();
    assert!(!bad, "the verdict admitted a zero denominator");
    assert!(good, "the verdict refused the control");
    println!("D: zero-denominator verdict={bad} control verdict={good}");
}
