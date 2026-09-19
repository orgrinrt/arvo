// The positive control: with no shadow declared anywhere in the file, the
// leading-:: spelling resolves and the crate builds. It checks no value. With
// nothing shadowing `core`, a read through `::core` and a read of the primitive
// `usize` name the same constant, so a comparison of the two could not fail,
// and `run.sh` runs this arm as one that builds and holds no check.
#![no_std]

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type CHECK = W<{ ::core::primitive::usize::BITS }>;
