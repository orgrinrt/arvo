//! Arm L. The obligation moved off the trait's defaulted const and onto a
//! blanket impl the implementor cannot write.
//!
//! Arm I disarms `Format::ADMITTED` with one line, because a defaulted
//! associated const is the implementor's to override, and the implementor is the
//! party the obligation constrains. A blanket impl over `F: Format` is not: a
//! second impl is a coherence error, so there is no line the implementor can add.
//!
//! Predicted: forcing `<Disarmed as Admits>::OK` refuses even though `Disarmed`
//! writes `const ADMITTED: () = ();`.
//!
//! The control is `HalfStep`, which must force `OK` and build, or `Admits`
//! refuses everything and the refusal below carries nothing.

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
    const ADMITTED: () = ();
}

struct HalfStep;

impl Format for HalfStep {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 2);
}

const fn forced<F: Format>() -> i64 {
    let () = <F as Admits>::OK;
    F::PHASE.denominator()
}

// The refusing half of arm L, as its own bin.
const REFUSED: i64 = forced::<Disarmed>();

fn main() {
    // The control: the well-formed declaration forces `OK` and builds.
    println!("L2: {}", REFUSED);
    println!("L2 control: forced::<HalfStep>() = {}", forced::<HalfStep>());
    // And the disarm still works against the trait's own const, which is the
    // thing being replaced.
    let () = <Disarmed as Format>::ADMITTED;
    println!("L: Format::ADMITTED on the disarmed impl is still a no-op");
}
