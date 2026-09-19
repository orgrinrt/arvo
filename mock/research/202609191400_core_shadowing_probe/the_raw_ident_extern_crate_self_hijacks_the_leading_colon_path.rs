// `extern crate self as core;` renames the crate root itself to `core`, so a
// leading-`::` path resolves through the shadow rather than around it: this is
// the one form the alias spelling's leading `::` does not resist, and the
// reason the second lint arm exists at all rather than the leading `::` being
// enough on its own. This arm spells the alias as the raw identifier
// `r#core`, which is ordinary text once a comment-and-literal strip runs on
// it, to show the raw-identifier spelling reaches exactly the same hazard as
// the plain one.
//
// Outcome: WORKS (as in: the hijack succeeds, which is the finding). Exit 0,
// and `MAX == 255` (this crate's own `usize::BITS` is 8, so the real answer
// would be `MAX == (1 << 8) - 1` too on a matching host; the width chosen here
// makes the exploited and the honest answer coincide by construction, so the
// second assertion below is the one that actually distinguishes them). Read
// with `the_bare_spelling_without_the_leading_colon_is_shadowed.rs`, the
// negative control demonstrating a construction this harness catches.
#![no_std]

extern crate self as r#core;

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

// The crate's real, unshadowed `usize::BITS` (the language primitive) is
// whatever the compiling host's pointer width is; the point of this probe is
// only that the leading-:: path now resolves through the extern-crate-self
// alias to this crate's own `primitive::usize::BITS`, which is 8, rather than
// staying rooted at the real `core`.
pub type CHECK = W<{ ::core::primitive::usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == 255,
    "the leading-:: path did not resolve through `extern crate self as r#core`"
);
