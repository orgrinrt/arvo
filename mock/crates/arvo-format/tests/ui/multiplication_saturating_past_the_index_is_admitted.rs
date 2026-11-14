// The product over the unsigned 64-bit range under saturation, admitted, one of
// three that differ in the policy and nothing else.
//
// The top corner (2^64 - 1)^2 is past the slot index. The step holds its floor
// in four limbs, so `adapt` reads that floor under every policy, and no policy
// turns the obligation.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::Dither;
use arvo_format::multiplication::mul;
use arvo_format::overflow::Saturate;
use arvo_format::points::UFixed;
use arvo_format::rounding::TowardZero;
use arvo_format::slots::Slot;

const _PRODUCT: () = {
    let _ = mul::<Signature<UFixed<64, 0>, Adapt<TowardZero, Saturate>>>(
        Slot::ZERO,
        Slot::ZERO,
        Dither::UNUSED,
    );
};

fn main() {}
