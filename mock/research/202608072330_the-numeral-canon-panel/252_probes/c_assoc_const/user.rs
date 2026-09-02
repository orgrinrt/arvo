// Arm C, the outside crate. No feature attribute, and no machine type on any
// line that declares a coordinate. A negative exponent and a radix of three,
// so the values are the ones a `u32`-only door could not carry.
#![no_std]

use door::{smallest_step, Declared, Exponent, Radix, Width};

pub struct Ternary;

impl Declared for Ternary {
    const WIDTH: Width = Width::bits(13);
    const RADIX: Radix = Radix::of(3);
    const EXPONENT: Exponent = Exponent::of(-4);
}

/// Forced at check time. If the door needed a gate the consumer does not have,
/// this is where it would say so.
pub const STEP: i32 = smallest_step::<Ternary>();
