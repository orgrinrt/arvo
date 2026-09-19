// The control for the mechanism itself: a bare `mod usize` shadows the bare
// `usize::BITS` path, which is why the aliases are spelled through
// `core::primitive::usize::BITS` rather than `usize::BITS`. Kept beside the arms
// about `core` itself so the directory shows the mechanism one level down still
// holds.
//
// Outcome: WORKS. Exit 0, and the shadow's 8 is what the path reads.
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
