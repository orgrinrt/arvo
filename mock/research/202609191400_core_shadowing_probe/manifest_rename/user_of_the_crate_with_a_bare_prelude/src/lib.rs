// The rename of `user`, onto a crate holding a `prelude` module with nothing
// in it at all, no `rust_2024` submodule. Isolates whether the edition's
// prelude import needs the exact `rust_2024` path or is satisfied by any
// `prelude` module existing.
#![no_std]

pub const W: u32 = ::core::primitive::usize::BITS;
