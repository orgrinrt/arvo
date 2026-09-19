// With a `mod core` in scope, the leading-`::` path
// `::core::primitive::usize::BITS` still resolves to the real primitive rather
// than through the shadow. `usize::BITS` unqualified is the oracle: primitive
// type name resolution does not go through a path lookup on `core`, so it is
// not itself shadowed by the declaration below, and is what the crate's real
// pointer width reads as.
//
// This file is the `mod core` half of the two shadows the name promises. The
// `use ... as core` half is `the_use_alias_does_not_reach_the_leading_colon_path.rs`,
// in a file of its own because a `use ... as core` beside this file's
// `mod core` would collide on the name `core`.
// `the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path.rs` is the
// form the leading `::` does not resist, and
// `the_bare_spelling_without_the_leading_colon_is_shadowed.rs` is the negative
// control showing this harness can fail.
//
// Outcome: WORKS. Exit 0, and the assertion that the path reads the host's
// pointer width holds.
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
