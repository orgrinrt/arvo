//! The control, must build: the twin beside it with the gate named, so what
//! refuses there is the missing gate and nothing else.

#![feature(const_trait_impl)]

use arm_const::{Sum, Zero};

pub const fn sum_in_const<T: [const] Sum + Zero + Copy>(a: T, b: T) -> T {
    T::ZERO.sum(a).sum(b)
}
