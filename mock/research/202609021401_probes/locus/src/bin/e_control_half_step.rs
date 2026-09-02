//! Arm E. The control for B and C: the identical shape at a denominator of two.
//!
//! If this builds and B does not, the refusal in B is the denominator and
//! nothing else about the declaration, the call, the crate or the toolchain.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{has_additive_identity, Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

struct HalfStep;

impl Format for HalfStep {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 2);
}

fn main() {
    let answer = has_additive_identity::<HalfStep>().get();
    assert!(!answer, "a half-step grid was said to carry an identity");
    println!("E: control builds, has_additive_identity={answer}");
}
