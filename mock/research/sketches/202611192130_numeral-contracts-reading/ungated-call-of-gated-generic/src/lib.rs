//! Question: under arm three, the families arvo writes against the contracts
//! are generic `const fn`s with `[const]` bounds, written inside arvo where the
//! gate is allowed (`gated-const-eval` stands in for one). Can a consumer that
//! names no gate evaluate such a function at a concrete `Num` in a const item?
//! If yes, arm three delivers build-time evaluation through the contracts to
//! exactly the consumers the obligation describes, which is the argument for
//! it that the shape does not make.

use gated_const_eval::{I8, sum_in_const};

pub const SEVEN: I8 = sum_in_const(I8::constant::<3>(), I8::constant::<4>());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_family_evaluated_at_build_time() {
        assert_eq!(SEVEN, I8::constant::<7>());
        assert_ne!(SEVEN, I8::constant::<6>());
    }
}
