//! Must fail: under arm one, a consumer writes a generic `const fn` through the
//! plain contract. A plain trait's method is not callable in a const context, so
//! the build is refused whatever gate the consumer names.

use arm_plain::{Sum, Zero};

pub const fn sum_in_const<T: Sum + Zero + Copy>(a: T, b: T) -> T {
    T::ZERO.sum(a).sum(b)
}
