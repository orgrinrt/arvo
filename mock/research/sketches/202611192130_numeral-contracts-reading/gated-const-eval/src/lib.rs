//! The positive control the first sketch's `const-const-bound-with-gate` twin
//! did not reach: that twin only declares the generic `const fn`, and nothing
//! ever evaluates it at `Num`. Here a gated consumer evaluates it in a const
//! item, both through the generic function and at the concrete type, and the
//! test reads the evaluated value back.

#![feature(const_trait_impl)]

use arm_const::{Sum, Zero};
use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Quiet};

pub type I8 = Int<8, Adapt<TowardZero, Saturate>, Quiet>;

pub const fn sum_in_const<T: [const] Sum + Zero + Copy>(a: T, b: T) -> T {
    T::ZERO.sum(a).sum(b)
}

pub const GENERIC: I8 = sum_in_const(I8::constant::<100>(), I8::constant::<100>());
pub const CONCRETE: I8 = <I8 as Sum>::sum(I8::constant::<3>(), I8::constant::<4>());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_generic_const_sum_evaluates_and_saturates() {
        assert_eq!(GENERIC, I8::constant::<127>());
        assert_ne!(GENERIC, I8::constant::<-56>());
    }

    #[test]
    fn the_concrete_const_sum_evaluates() {
        assert_eq!(CONCRETE, I8::constant::<7>());
    }
}
