// The rename of `user`, onto a crate that does not re-export the real `core`.
// A `no_std` crate's prelude is imported through the name `core`, so with
// `core` renamed to a crate that has no `prelude` module, the build stops at
// that import before any path of this file is read.
//
// Outcome: FAILS TO COMPILE, on the prelude import, which `run.sh` checks for.
#![no_std]

pub const W: u32 = ::core::primitive::usize::BITS;
