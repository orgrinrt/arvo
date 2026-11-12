// Multiplication over a scaled integer, the pair that turns on the phase.
//
// At a phase of one half step the product of two members sits at
// (phase + a)(phase + b) * q - phase, which the step does not compute, so the
// obligation refuses it. At zero phase the same declaration is admitted. The
// two files differ in that one token and nothing else.

use arvo_format::multiplication::product_position;
use arvo_format::points::Biased;
use arvo_format::slots::Slot;

const _PRODUCT: () = {
    let _ = product_position::<Biased<8, 0, 1>>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
