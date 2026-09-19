// The negative control C4 of review146f asked for: every sibling arm in this
// directory exits 0, which is what a probe answering "the leading `::` holds"
// looks like, but is indistinguishable from a probe that checks nothing. This
// arm shows the harness can fail. It reads the BARE, unqualified spelling
// `core::primitive::usize::BITS`, with no leading `::`, in the presence of a
// `mod core` shadow, and asserts the outcome a leading-`::` read would give,
// which the bare spelling does not give: it resolves through the shadow
// instead.
//
// Outcome: FAILS TO COMPILE, by construction: `assert!` inside a `const _`
// panics at compile time, which is the failure this arm exists to produce.
// `cargo build` (or `rustc --edition 2024 --crate-type lib`) on this file
// alone exits non-zero with "evaluation of constant value failed" pointing at
// the message below.
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
    "the bare spelling read the real pointer width rather than the shadow, \
     which is not what a bare `core` path does in the presence of a `mod \
     core` declared in the same crate"
);
