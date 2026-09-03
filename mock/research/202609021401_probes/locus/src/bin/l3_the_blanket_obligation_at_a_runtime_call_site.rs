//! Arm L3. The blanket obligation reached from a run-time call site, so the
//! comparison with `Format::ADMITTED` is on one axis at a time.
//!
//! L2 refuses from a const item, which is check-time evaluation, exactly as arm
//! C does. That says nothing about the verb axis. This arm puts the disarmed
//! declaration behind an ordinary run-time call, which is arm B's shape.
//!
//! Predicted: `cargo check` passes and `cargo build` refuses, matching arm B.
//! Which would mean the blanket form is no weaker than the defaulted const on
//! the verb axis and strictly stronger on the disarm axis, rather than trading
//! one for the other.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

trait Admits {
    const OK: ();
}

impl<F: Format> Admits for F {
    const OK: () = {
        assert!(
            F::PHASE.denotes().get(),
            "a phase denominator of zero names no position on the grid"
        );
    };
}

struct Disarmed;

impl Format for Disarmed {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const ADMITTED: () = ();
    const PHASE: Phase = Phase::of(1, 0);
}

fn forced<F: Format>() -> i64 {
    let () = <F as Admits>::OK;
    F::PHASE.denominator()
}

fn main() {
    println!("L3: {}", forced::<Disarmed>());
}
