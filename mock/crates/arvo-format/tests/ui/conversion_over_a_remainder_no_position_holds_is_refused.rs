// Conversion whose remainder's denominator passes what an exact position
// carries, the pair that turns on the target's exponent.
//
// From a quantum of `2^-64` onto one of `2^64` the denominator is `2^128`, one
// past the unsigned 128 bits a remainder is carried over, so the obligation
// holds it out. Onto `2^63` it is `2^127` and admitted. The two files differ in
// the target's exponent and nothing else.

use arvo_format::conversion::converted_position;
use arvo_format::points::UFixed;
use arvo_format::slots::Slot;

const _CONVERTED: () = {
    let _ = converted_position::<UFixed<8, -64>, UFixed<8, 64>>(Slot::ZERO);
};

fn main() {}
