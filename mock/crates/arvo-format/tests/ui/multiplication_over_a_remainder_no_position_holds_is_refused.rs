// Multiplication over a phase whose remainder no exact position holds, the pair
// that turns on the phase's denominator.
//
// The product of two members at a phase n / d over the quantum 2^-132 sits over
// the denominator d^2 2^132, and an exact step is formed in four 64-bit limbs.
// At d = 2^62 that is 2^256, past them, so the obligation does not admit the
// format yet. At d = 2^61 it is 2^254 and the same declaration is admitted. The
// two files differ in the phase's denominator and nothing else.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::multiplication::product_position;
use arvo_format::quantum;
use arvo_format::slots::{Signed, Slot};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Constant<-132>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 1 << 62);
}

const _AT: () = {
    let _ = product_position::<Probe>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
