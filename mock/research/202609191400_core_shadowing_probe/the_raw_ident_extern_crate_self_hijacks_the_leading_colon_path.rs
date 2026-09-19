// `extern crate self as r#core;` renames the crate root itself to `core`, so a
// leading-`::` path resolves through the alias rather than around it: this is
// the one form the alias spelling's leading `::` does not resist, and the
// reason the lint's second arm refuses `extern crate ... as core` at all. The
// alias is spelled as the raw identifier `r#core` here, to show the raw
// spelling reaches exactly what the plain one in
// `extern_crate_self_as_core_hijacks_the_absolute_path.rs` reaches.
//
// Two assertions. The first says `::core::primitive::usize::BITS` reads this
// crate's own `primitive::usize::BITS`, 8. The second says that answer is not
// the host's pointer width, read through the primitive type `usize`, whose name
// resolution does not go through `core`; no Rust host has an 8-bit pointer, so
// the second holds wherever the first does and tells the hijacked answer from
// the honest one on every host.
//
// Outcome: WORKS, meaning the hijack succeeds and the build exits 0.
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

pub type CHECK = W<{ ::core::primitive::usize::BITS }>;

const _: () = assert!(
    <CHECK as F>::MAX == 255,
    "the leading-:: path did not resolve through `extern crate self as r#core`"
);

const _: () = assert!(
    ::core::primitive::usize::BITS != usize::BITS,
    "the leading-:: path read the host's pointer width, so nothing was hijacked"
);
