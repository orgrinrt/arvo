// The control for the mechanism itself, not for this probe's new finding: a
// bare `mod usize` shadows the bare `usize::BITS` path, which is the hole the
// fourth review's respelling to `core::primitive::usize::BITS` closed. Kept here
// so the probe directory shows the original mechanism still holds, beside the
// two new ones (`core` itself, and `extern crate self`) this probe adds.
#![no_std]

mod usize {
    pub const BITS: u32 = 8;
}

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type CHECK = W<{ usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == 255,
    "the bare usize shadow did not win"
);
