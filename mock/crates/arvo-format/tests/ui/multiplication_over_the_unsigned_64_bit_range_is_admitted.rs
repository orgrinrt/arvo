// The exact product over the unsigned 64-bit range, admitted.
//
// The top corner (2^64 - 1)^2 is past the slot index, and the step holds its
// floor in four limbs, so the position is exact there and the obligation reads
// nothing of the index. The refusal beside it is the product whose remainder no
// position holds.

use arvo_format::multiplication::product_position;
use arvo_format::points::UFixed;
use arvo_format::slots::Slot;

const _PRODUCT: () = {
    let _ = product_position::<UFixed<64, 0>>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
