// Written to be flagged by the premise pass and the spelling pass. This is the
// assertion `the_shipped_spelling_alone_is_silent.rs` held: with no shadow in
// the file, both sides read the real width, so it cannot fail. There is no
// fake width to set, and dropping the leading `::` still reads the real
// `core`.
#![no_std]

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type CHECK = W<{ ::core::primitive::usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == (1i128 << usize::BITS) - 1,
    "the leading-:: spelling did not read the host's pointer width"
);
