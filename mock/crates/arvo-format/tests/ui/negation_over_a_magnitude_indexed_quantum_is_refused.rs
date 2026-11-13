// Negation over a format whose quantum moves with the magnitude, the pair that
// pins `negation_position` forcing the addition obligation.
//
// Under a magnitude-indexed quantum, which is what a floating point format is,
// addition's obligation does not admit the format, and negation is one of the
// steps that force it. Under a constant quantum at the same exponent the same
// declaration is admitted. The two files differ in the quantum the format names
// and nothing else.

use arvo_format::addition::negation_position;
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum;
use arvo_format::slots::{Signed, Slot};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Indexed<-3, 4>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::ZERO;
}

const _AT: () = {
    let _ = negation_position::<Probe>(Slot::ZERO);
};

fn main() {}
