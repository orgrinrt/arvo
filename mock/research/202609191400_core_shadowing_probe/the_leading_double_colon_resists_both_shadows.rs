// The control for the two sibling probes: with both a `mod core` and a
// `use ... as core` in scope, the leading-`::` path `::core::primitive::usize::BITS`
// still resolves to the real primitive rather than through either shadow.
// `usize::BITS` unqualified is the oracle: primitive type name resolution does
// not go through a path lookup on `core`, so it is not itself shadowed by either
// declaration below, and is what the crate's real pointer width reads as.
#![no_std]

mod core {
    pub mod primitive {
        pub struct usize;
        impl usize {
            pub const BITS: u32 = 8;
        }
    }
}

mod fake {
    pub mod primitive {
        pub struct usize;
        impl usize {
            pub const BITS: u32 = 9;
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

pub type CHECK = W<{ ::core::primitive::usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == (1i128 << usize::BITS) - 1,
    "the leading-:: path resolved through a shadow"
);
