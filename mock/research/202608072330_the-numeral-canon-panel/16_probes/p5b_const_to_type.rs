// p5b: THIS FILE DOES NOT COMPILE, ON PURPOSE. The diagnostic is the result.
//
// Checks ONE thing: whether a site holding the extent as a CONST can recover the carrier as a
// TYPE by writing the arithmetic it already knows. If it cannot, then "the carrier is
// recoverable from the extent" is true of the arithmetic and false of the type system, and a
// derivation that emitted only the extent would push the const-to-type problem to every use
// site instead of solving it once.
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p5b_const_to_type.rs
//
// Expected: refusal. Committed with its diagnostic in p5b_const_to_type.err.
//
// Spike. No feature gates are enabled here deliberately: the whole question is what is
// reachable WITHOUT the forbidden ones.

#![no_std]

const fn native(w: u32) -> u32 {
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else {
        64
    }
}

// Attempt 1: name the recovered carrier directly in return position.
// The arithmetic is right there and const-evaluable. The type system still cannot use it.
pub fn carrier_of<const W: u32>() -> [u8; native(W) as usize / 8] {
    [0u8; native(W) as usize / 8]
}

// Attempt 2: the same thing through an alias, in case the indirection helps.
pub type CarrierBytes<const W: u32> = [u8; native(W) as usize / 8];

// Attempt 3: a where-clause carrying the derived quantity, which is the shape an extent-only
// derivation would force at every use site.
pub fn store<const W: u32>(_v: u32) -> u32
where
    [(); native(W) as usize / 8]:,
{
    0
}
