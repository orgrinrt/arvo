//! Must fail: under arm two with its feature off, a consumer that names the gate
//! and writes a `[const]` bound. The trait is plain on this path, so a consumer
//! leaning on its constness breaks when no crate in the build turns the feature
//! on: the constness of the contract depends on the rest of the build.

#![feature(const_trait_impl)]

use arm_opt::{Sum, Zero};

pub const fn sum_in_const<T: [const] Sum + Zero + Copy>(a: T, b: T) -> T {
    T::ZERO.sum(a).sum(b)
}
