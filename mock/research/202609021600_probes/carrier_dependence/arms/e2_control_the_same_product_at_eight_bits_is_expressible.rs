// ARM E2, the control, must BUILD.
//
// The same construction for operands arvo also admits, at a width where the
// product fits the carrier. If this failed too, E1 would be saying something
// about `Slot::at` rather than about the width.
use arvo_format::slots::Slot;

// 2^7 * 2^7 = 2^14, the exact product's slot for two operands at the top of
// `Signed<8>`.
const EXACT_PRODUCT: Slot = Slot::at(16384);

fn main() {
    assert_eq!(EXACT_PRODUCT.index(), 16384);
}
