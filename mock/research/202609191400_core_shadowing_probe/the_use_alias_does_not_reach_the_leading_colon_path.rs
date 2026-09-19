// A `use ... as core` alias, unlike `extern crate self as core`, only adds a
// competing name to this file's scope; it does not rename the crate root, so
// the leading-`::` path `::core::primitive::usize::BITS` resolves through the
// real crate root regardless. `usize::BITS` unqualified is the oracle, as in
// the sibling probe: primitive-type name resolution does not go through a
// path lookup on `core`, so it is not itself shadowed by the alias below.
//
// Outcome: WORKS. Exit 0, the `MAX == real width` assertion holds, confirming
// the leading `::` resists a `use ... as core` alias exactly as it resists a
// `mod core`.
#![no_std]

mod fake {
    pub mod primitive {
        pub struct usize;
        impl usize {
            pub const BITS: u32 = 9;
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

pub type CHECK = W<{ ::core::primitive::usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == (1i128 << usize::BITS) - 1,
    "the leading-:: path resolved through the use-alias shadow"
);
