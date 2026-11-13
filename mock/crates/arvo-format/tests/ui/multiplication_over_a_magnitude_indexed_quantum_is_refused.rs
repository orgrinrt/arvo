// Multiplication over a format whose quantum moves with the magnitude, the pair
// that turns on the quantum family.
//
// Under a magnitude-indexed quantum a slot names a different value at each
// magnitude, and the applied map has no magnitude coordinate to land the
// product at, so the obligation refuses it. Under a constant quantum at the
// same exponent the same declaration is admitted. The two files differ in the
// quantum the format names and nothing else.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::multiplication::product_position;
use arvo_format::quantum;
use arvo_format::slots::{Signed, Slot};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Indexed<-3, 4>;
    type Slots = Signed<4>;

    const PHASE: Phase = Phase::ZERO;
}

const _PRODUCT: () = {
    let _ = product_position::<Probe>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
