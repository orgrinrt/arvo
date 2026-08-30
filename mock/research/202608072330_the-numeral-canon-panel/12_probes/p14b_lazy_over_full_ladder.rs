//! p14b. The library-side cost of `lazy_type_alias`, captured rather than
//! estimated. This is p13 verbatim with the feature gate added, so every error
//! it produces is an alias inside the ladder or the surfaces that now wants a
//! bound written on it. Sixteen of them, recorded in out/p14_full.log.
//!
//! `lazy_type_alias` is NOT on the workspace's vetted feature list. This file
//! measures what adopting it would cost and makes no admissibility argument.
//!
//! THIS FILE IS EXPECTED TO FAIL TO COMPILE.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p14b.meta p14b_lazy_over_full_ladder.rs 2> out/p14_full.log
#![feature(lazy_type_alias)]
#![allow(incomplete_features)]
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");
include!("surfaces.rs");

// --- line 24. The consumer writes this and the compiler says nothing. ---------
pub type PacketTag = c4::UInt<7>;

// forty lines of unrelated code, which is the point
pub struct Header {
    pub _pad: u8,
}
pub struct Body {
    pub _pad: u8,
}
pub fn unrelated_one(h: Header) -> Header {
    h
}
pub fn unrelated_two(b: Body) -> Body {
    b
}

// --- and here, much later, is where they are told. ---------------------------
pub fn read_tag(t: PacketTag) -> PacketTag {
    t
}
