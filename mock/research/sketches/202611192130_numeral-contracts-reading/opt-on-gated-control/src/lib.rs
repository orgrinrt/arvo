//! The control the first sketch's claim 3 ("builds when one does") rests on
//! and never committed: arm two with its feature on, a gated consumer writing
//! a `[const]` bound and evaluating it at `Num` in a const item.

#![feature(const_trait_impl)]

use arm_opt::{Sum, Zero};
use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Quiet};

pub type I8 = Int<8, Adapt<TowardZero, Saturate>, Quiet>;

pub const fn sum_in_const<T: [const] Sum + Zero + Copy>(a: T, b: T) -> T {
    T::ZERO.sum(a).sum(b)
}

pub const GENERIC: I8 = sum_in_const(I8::constant::<3>(), I8::constant::<4>());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_generic_const_sum_evaluates() {
        assert_eq!(GENERIC, I8::constant::<7>());
    }
}
