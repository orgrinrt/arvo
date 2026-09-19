// The rename of `user`, onto a crate holding none of the real `core`, only an
// empty `prelude::rust_2024` and its own `primitive`. The prelude is empty, so
// `assert!` is not in scope here; each check is an array length instead, which
// needs no macro and fails to compile as a length mismatch when its condition
// is false.
//
// Outcome: WORKS, meaning the hijack succeeds and `cargo check` exits 0, with
// both checks holding.
#![no_std]

pub const W: u32 = ::core::primitive::usize::BITS;

const _: [(); 1] = [(); (W == 8) as usize];
const _: [(); 1] = [(); (W != usize::BITS) as usize];
