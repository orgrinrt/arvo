// The rename of `user`, onto a crate that re-exports the real `core`'s
// `prelude` and none of the rest of `core`. The prelude import resolves, so
// the build reaches this file, and the leading-`::` path reads the renamed
// crate's `primitive::usize::BITS`, 8.
//
// Outcome: WORKS, meaning the hijack succeeds and `cargo check` exits 0, with
// both assertions holding.
#![no_std]

pub const W: u32 = ::core::primitive::usize::BITS;

const _: () = assert!(W == 8, "the manifest rename did not take over `::core`");
const _: () = assert!(
    W != usize::BITS,
    "the renamed answer is the real pointer width"
);
