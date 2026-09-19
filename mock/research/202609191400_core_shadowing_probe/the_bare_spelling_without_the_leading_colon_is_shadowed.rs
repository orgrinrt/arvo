// The negative control: every other arm in this directory that is expected to
// build does, and a harness that only ever sees exit 0 has not shown it can
// fail. This arm reads the bare, unqualified spelling
// `core::primitive::usize::BITS`, with no leading `::`, beside a `mod core`
// shadow, and asserts what a leading-`::` read would give, the host's pointer
// width. The bare spelling resolves through the shadow instead and reads 8, so
// the assertion fails.
//
// Outcome: FAILS TO COMPILE, by construction: the `assert!` inside a `const _`
// panics at compile time with the message below, which `run.sh` checks for.
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

// No leading `::`: this resolves through the `mod core` shadow above, to `8`,
// not through the real crate `core`.
pub type CHECK = W<{ core::primitive::usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == (1i128 << usize::BITS) - 1,
    "the bare spelling read the `mod core` shadow's 8 rather than the host's pointer width"
);
