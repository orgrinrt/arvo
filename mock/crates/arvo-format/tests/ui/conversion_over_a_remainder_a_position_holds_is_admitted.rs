// Conversion whose denominator passes the four limbs an exact step is formed
// in, the pair that turns on the target's exponent.
//
// From a quantum of `2^-128` onto one of `2^128` the denominator is `2^256`, one
// past the four 64-bit limbs, so the obligation holds it out. Onto `2^127` it is
// `2^255` and admitted. The two files differ in the target's exponent and
// nothing else.

use arvo_format::conversion::converted_position;
use arvo_format::points::UFixed;
use arvo_format::slots::Slot;

const _CONVERTED: () = {
    let _ = converted_position::<UFixed<8, -128>, UFixed<8, 127>>(Slot::ZERO);
};

fn main() {}
