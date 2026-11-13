//! Arm B from a crate with no `#![feature]` line: the five consumer cases.
//!
//! Case (3) bounds the signature by `Sole` alone.

#![no_std]

use arm_b::Num;

common::consumer_cases!(Num, [common::Sole]);

/// The accumulator with its term count in its type, used in a `const` item from
/// a crate with no `#![feature]` line: four members of `Integer<8>` under
/// `Saturate`, at the range edge, adapted once.
pub mod counted {
    use arm_b::{Counted, Num, Zero};
    use arvo_format::overflow::Saturate;
    use arvo_format::points::Integer;
    use arvo_format::rounding::TowardZero;
    use arvo_format::{Adapt, Signature};

    /// The signature the count is taken in.
    pub type S8 = Signature<Integer<8>, Adapt<TowardZero, Saturate>>;

    /// `127 + 1 - 1 - 127`, exactly, then adapted: zero, where adapting at each
    /// step gives `-1`.
    pub const EDGE: Num<S8> = Counted::<S8, Zero>::ZERO
        .push(Num::from_raw(127))
        .push(Num::from_raw(1))
        .push(Num::from_raw(-1))
        .push(Num::from_raw(-127))
        .narrow();

    #[cfg(test)]
    mod tests {
        #[test]
        fn the_counted_sum_is_exact_in_a_const_item() {
            assert_eq!(super::EDGE.raw(), 0);
        }
    }
}
