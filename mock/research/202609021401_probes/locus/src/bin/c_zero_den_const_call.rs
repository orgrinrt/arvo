//! Arm C. The same declaration reached from a const item instead of a run-time
//! call.
//!
//! The design says the totality of the predicates exists for "the check-time
//! evaluation an obligation cannot reach" (`DESIGN.md.tmpl:740`). A const item
//! is check-time evaluation, and `has_additive_identity` opens by forcing
//! `<F as Format>::ADMITTED`, so the question is whether the obligation reaches
//! it after all.
//!
//! Predicted, if the design's sentence is right: `cargo check` succeeds and
//! prints `false`, because the obligation cannot reach a const item.
//! Predicted, if forcing the const in the body does reach: `cargo check` fails.
//!
//! Either answer is a result. The arm is written so both are visible.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{has_additive_identity, Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;
use arvo_format::width::Bool;

struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 0);
}

const ANSWER: Bool = has_additive_identity::<NoDenominator>();

fn main() {
    println!("C: {}", ANSWER.get());
}
