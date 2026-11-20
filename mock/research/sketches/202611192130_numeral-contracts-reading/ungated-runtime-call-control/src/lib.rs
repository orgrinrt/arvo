//! The control for `ungated-call-of-gated-generic`: the same consumer with no
//! gate, the same gated generic `const fn`, called outside a const context. It
//! builds and runs, so the refusal beside it is the const context and nothing
//! else.

use gated_const_eval::{I8, sum_in_const};

pub fn seven() -> I8 {
    sum_in_const(I8::constant::<3>(), I8::constant::<4>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_family_runs() {
        assert_eq!(seven(), I8::constant::<7>());
        assert_ne!(seven(), I8::constant::<6>());
    }
}
