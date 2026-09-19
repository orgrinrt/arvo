// `extern crate self as core;` at the crate root binds this crate's own root to
// the name `core`, so `::core` (an absolute path rooted at the crate graph) now
// names this crate rather than the real one, and the leading `::` on
// `::core::primitive::usize::BITS` no longer saves it. Compiled with an explicit
// `--crate-name` other than `core`, since `extern crate self` needs the crate to
// have a name distinct from what it aliases.
#![no_std]

extern crate self as core;

pub mod primitive {
    pub struct usize;
    impl usize {
        pub const BITS: u32 = 8;
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
    <CHECK as F>::MAX == 255,
    "the leading-:: path was not hijacked by extern crate self as core"
);
