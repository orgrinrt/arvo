// Division over a format whose quantum moves with the magnitude, the pair that
// turns on the quantum family.
//
// Under a magnitude-indexed quantum the applied map has no magnitude coordinate
// to land the quotient at, so the obligation holds it out. Under a constant
// quantum at the same exponent the same declaration is admitted. The two files
// differ in the quantum the format names and nothing else.

use arvo_format::ambient::BinaryRationals;
use arvo_format::division::{Divisor, quotient_position};
use arvo_format::format::{Format, Phase};
use arvo_format::quantum;
use arvo_format::slots::{Signed, Slot};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Constant<-3>;
    type Slots = Signed<4>;

    const PHASE: Phase = Phase::ZERO;
}

const _QUOTIENT: () = {
    let _ = quotient_position::<Probe>(Slot::ZERO, Divisor::constant::<1>());
};

fn main() {}
