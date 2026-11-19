//! Must fail: under arm three, a consumer with no feature line writes a
//! `[const]` bound. The bound syntax is the gate's, so the build is refused
//! until the consumer names `const_trait_impl` itself.

use arm_const::{Sum, Zero};

pub const fn sum_in_const<T: [const] Sum + Zero + Copy>(a: T, b: T) -> T {
    T::ZERO.sum(a).sum(b)
}
