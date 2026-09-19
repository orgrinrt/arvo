// The shape the deleted per-target arms had: a `#[test]` function behind the
// same `cfg`, asserting at run time. Checked with `--test` at the same target,
// it exits zero over the literal alias, because a check never runs a test
// body. This is the half of the old sentence that held, and only for this shape.
#![no_std]

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type USize = W<64>;

#[cfg(all(test, target_pointer_width = "32"))]
mod tests {
    use super::{F, USize};

    #[test]
    fn the_alias_is_the_32_bit_point() {
        assert_eq!(<USize as F>::MAX, u32::MAX as i128);
    }
}
