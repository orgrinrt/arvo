// An alias written against a literal 64 where it should read the pointer width,
// and a const assertion at a 32-bit target that the alias is the 32-bit point.
//
// Checked at `i686-unknown-linux-gnu` with `--emit=metadata`, which is what a
// `cargo check` at that target runs. If checking only type-checked the
// assertion, this would exit zero.
#![no_std]

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type USize = W<64>;

#[cfg(target_pointer_width = "32")]
const _: () = assert!(
    <USize as F>::MAX == u32::MAX as i128,
    "the alias is not the 32-bit point"
);
