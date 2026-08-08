//! p13. p12 found something nobody had looked at: a type ALIAS does not check
//! its bounds, so an undeclared width written at the alias-definition site is
//! SILENT there. This file finds out where the error does land, and what it
//! says when it gets there.
//!
//! The alias is declared at the top of the file. The use is at the bottom, in a
//! different item, which is the honest shape: in real code the two are in
//! different files.
//!
//! THIS FILE IS EXPECTED TO FAIL TO COMPILE. Captured in out/p13.log.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p13.meta p13_where_the_door_error_lands.rs 2> out/p13.log
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
