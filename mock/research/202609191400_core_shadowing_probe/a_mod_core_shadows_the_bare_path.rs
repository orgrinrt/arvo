// A `mod core` declared in the same crate as a bare `core::primitive::usize::BITS`
// path takes precedence over the crate root there, so the path resolves through
// the shadow rather than through the real `core` crate. Checked at the host
// (this is a name-resolution question, not a foreign-target one).
#![no_std]

mod core {
    pub mod primitive {
        pub struct usize;
        impl usize {
            pub const BITS: u32 = 8;
        }
    }
}

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type CHECK = W<{ core::primitive::usize::BITS }>;

// Fails to type-check as `MAX == 255` only if the bare path resolved to the real
// 64-bit `core::primitive::usize::BITS` instead of the shadow.
const _: () = assert!(<CHECK as F>::MAX == 255, "the shadow did not win");
