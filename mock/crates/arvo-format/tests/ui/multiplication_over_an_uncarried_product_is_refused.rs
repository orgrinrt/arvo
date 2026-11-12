// Multiplication over the unsigned range a slot index cannot carry the product
// of, the pair that turns on the width.
//
// The largest product of two members of the unsigned 64-bit range is just under
// 2^128, past the slot index, so the obligation refuses it. One bit narrower the
// largest product is under 2^126 and the same declaration is admitted. The two
// files differ in that one token and nothing else.

use arvo_format::multiplication::product_position;
use arvo_format::points::UFixed;
use arvo_format::slots::Slot;

const _PRODUCT: () = {
    let _ = product_position::<UFixed<64, 0>>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
