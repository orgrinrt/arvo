// The product over the unsigned 64-bit range under saturation, refused, the pair
// that turns on the overflow policy.
//
// The top corner (2^64 - 1)^2 is past the slot index. Wrapping reads that
// floor modulo twice the range's count, which the step forms, so the wrapping
// product is admitted; under saturation the product is not yet admitted. The two
// files differ in the policy and nothing else.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::Dither;
use arvo_format::multiplication::mul;
use arvo_format::overflow::Wrap;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::slots::Slot;

const _PRODUCT: () = {
    let _ = mul::<Signature<UFixed<64, 0>, Adapt<TowardZero, Wrap>>>(
        Slot::ZERO,
        Slot::ZERO,
        Dither::UNUSED,
    );
};

fn main() {}
