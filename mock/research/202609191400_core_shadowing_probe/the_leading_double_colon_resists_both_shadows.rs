// With a `mod core` in scope, the leading-`::` path
// `::core::primitive::usize::BITS` still resolves to the real primitive rather
// than through the shadow. `usize::BITS` unqualified is the oracle: primitive
// type name resolution does not go through a path lookup on `core`, so it is
// not itself shadowed by the declaration below, and is what the crate's real
// pointer width reads as.
//
// `mod fake` is not used in this file; it stays only because `the_use_alias_
// does_not_reach_the_leading_colon_path.rs`, the sibling probe demonstrating
// the same resistance against a `use ... as core` alias, needs a second module
// to alias, and a `use ... as core` alongside this file's own `mod core` would
// collide on the name `core`, so that arm lives in its own file rather than
// here. `the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path.rs`
// is the third sibling, the one form the leading `::` does not resist, and
// `the_bare_spelling_without_the_leading_colon_is_shadowed.rs` is the negative
// control showing this harness can fail.
//
// Outcome: WORKS. Exit 0, and the `MAX == 255` assertion holds.
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

pub type CHECK = W<{ ::core::primitive::usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == (1i128 << usize::BITS) - 1,
    "the leading-:: path resolved through a shadow"
);
