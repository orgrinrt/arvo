// A `no_std` crate whose `Cargo.toml` renames a dependency to the key `core`.
// The source carries none of the three forms the lint refuses, and the
// leading-`::` path the platform-width aliases use reads the renamed crate's
// `primitive::usize::BITS`, 8, rather than the pointer width, which no Rust host
// has at 8 bits.
//
// Outcome: WORKS, meaning the hijack succeeds and `cargo check` exits 0.
#![no_std]

pub const W: u32 = ::core::primitive::usize::BITS;

const _: () = assert!(W == 8, "the manifest rename did not take over `::core`");
const _: () = assert!(
    W != usize::BITS,
    "the renamed answer is the real pointer width"
);
