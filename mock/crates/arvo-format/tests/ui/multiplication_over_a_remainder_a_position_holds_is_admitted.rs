// Multiplication over a phase whose remainder no exact position holds, the pair
// that turns on the phase's denominator.
//
// The product of two members at a phase n / d over a quantum one sits over the
// denominator d^2, and an exact position carries its remainder over a positive
// signed 64-bit denominator. At d = 2^32 that is 2^64, past it, so the
// obligation does not admit the format yet. At d = 2^31 it is 2^62 and the same
// declaration is admitted. The two files differ in the phase's denominator and
// nothing else.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::multiplication::product_position;
use arvo_format::quantum;
use arvo_format::slots::{Signed, Slot};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 1 << 31);
}

const _AT: () = {
    let _ = product_position::<Probe>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
