// The control: the same assertion over an alias that reads `usize::BITS`.
// It has to exit zero at the same target, or the first arm's failure says
// nothing about the literal.
#![no_std]

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type USize = W<{ usize::BITS }>;

#[cfg(target_pointer_width = "32")]
const _: () = assert!(
    <USize as F>::MAX == u32::MAX as i128,
    "the alias is not the 32-bit point"
);
