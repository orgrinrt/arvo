// `use fake as core;` in the same crate as a bare `core::primitive::usize::BITS`
// path shadows the crate root the same way a `mod core` does. Checked at the
// host, same reasoning as the sibling probe.
#![no_std]

mod fake {
    pub mod primitive {
        pub struct usize;
        impl usize {
            pub const BITS: u32 = 8;
        }
    }
}

use fake as core;

pub struct W<const B: u32>;

pub trait F {
    const MAX: i128;
}

impl<const B: u32> F for W<B> {
    const MAX: i128 = (1i128 << B) - 1;
}

pub type CHECK = W<{ core::primitive::usize::BITS }>;

const _: () = assert!(<CHECK as F>::MAX == 255, "the alias shadow did not win");
