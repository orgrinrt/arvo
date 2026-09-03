//! Arm B. A zero denominator declared on a `Format`, reached from a run-time
//! call site.
//!
//! This is the design's central claim about the obligation, at
//! `arvo-format/DESIGN.md.tmpl:683` and `:737`: an obligation is a const
//! evaluated at monomorphisation, so `cargo build` refuses and `cargo check`
//! does not.
//!
//! Predicted: `cargo check --bin b_zero_den_runtime_call` succeeds,
//! `cargo build --bin b_zero_den_runtime_call` fails on `Format::ADMITTED`.
//!
//! Arm E is the control: the identical shape at a denominator of two must do
//! both, or the refusal here is about something other than the denominator.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase, has_additive_identity};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 0);
}

fn main() {
    println!("B: {}", has_additive_identity::<NoDenominator>().get());
}
