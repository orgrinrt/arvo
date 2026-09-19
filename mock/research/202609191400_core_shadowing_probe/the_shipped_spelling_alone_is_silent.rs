// The positive control: with no shadow declared anywhere in the file, the
// leading-:: spelling compiles and reads the real pointer width, exactly as the
// bare spelling did before any of this probe's shadows were introduced.
#![no_std]

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type CHECK = W<{ ::core::primitive::usize::BITS }>;

const _: () = assert!(<CHECK as F>::MAX == (1i128 << usize::BITS) - 1);
