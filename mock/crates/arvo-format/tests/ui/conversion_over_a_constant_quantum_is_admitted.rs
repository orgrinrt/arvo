// Conversion from a format whose quantum moves with the magnitude, the pair
// that turns on the quantum family.
//
// Under a magnitude-indexed quantum the applied map has no magnitude coordinate
// to read the member from, so the obligation holds it out. Under a constant
// quantum at the same exponent the same declaration is admitted. The two files
// differ in the quantum the source names and nothing else.

use arvo_format::ambient::BinaryRationals;
use arvo_format::conversion::converted_position;
use arvo_format::format::{Format, Phase};
use arvo_format::points::Integer;
use arvo_format::quantum;
use arvo_format::slots::{Signed, Slot};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Constant<-3>;
    type Slots = Signed<4>;

    const PHASE: Phase = Phase::ZERO;
}

const _CONVERTED: () = {
    let _ = converted_position::<Probe, Integer<8>>(Slot::ZERO);
};

fn main() {}
