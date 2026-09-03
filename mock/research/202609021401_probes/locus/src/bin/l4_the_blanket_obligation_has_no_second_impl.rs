//! Arm L4. The negative control on the disarm route itself.
//!
//! Arm I's disarm works because a defaulted associated const is the
//! implementor's to write. The blanket form's claim is that there is no line the
//! implementor can add. That is a claim about coherence and it is checkable
//! rather than argued: writing the second impl must be a hard error.
//!
//! Predicted: `cargo check` refuses with E0119, conflicting implementations.
//!
//! Without this arm, "cannot be disarmed" is an assertion about a language rule
//! quoted from memory.

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

    const PHASE: Phase = Phase::of(1, 0);
}

// The line an implementor would have to be able to write for the disarm to work.
impl Admits for Disarmed {
    const OK: () = ();
}

fn main() {
    let () = <Disarmed as Admits>::OK;
    println!("L4: the second impl was accepted, so the form is disarmable");
}
