// A constant divisor at the zero member, the pair that turns on the constant.
//
// A divisor named by a constant is checked where the constant is named: at the
// zero member `phase + B` is zero, and the quotient by it is not a function of
// the signature, so the obligation refuses it. At one it is admitted. The two
// files differ in the constant and nothing else.

use arvo_format::division::{Divisor, quotient_position};
use arvo_format::points::Integer;
use arvo_format::slots::Slot;

const _QUOTIENT: () = {
    let _ = quotient_position::<Integer<8>>(Slot::ZERO, Divisor::constant::<0>());
};

fn main() {}
