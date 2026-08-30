//! p05. What the consumer READS. The bar is a writing bar; this is the other
//! half and nobody has measured it.
//!
//! One deliberate mistake, made four ways: return a 26.6 numeral where a 13.3
//! was declared. Under C0 the numeral is keyed on consts and rustc has consts
//! to print. Under C1/C2/C4 it is keyed on nats and rustc has a digit tower to
//! print. `10` found rustc printing digit towers at the LAW site; this asks
//! whether the same thing happens at the most ordinary error a consumer makes.
//!
//! THIS FILE IS EXPECTED TO FAIL TO COMPILE. Four E0308s are the result.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p05.meta p05_diag_mismatch.rs 2> out/p05.log
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");
include!("surfaces.rs");

pub fn mistake_c0(x: c0::Product) -> c0::Coord {
    x
}
pub fn mistake_c1(x: c1::Product) -> c1::Coord {
    x
}
pub fn mistake_c2(x: c2::Product) -> c2::Coord {
    x
}
pub fn mistake_c4(x: c4::Product) -> c4::Coord {
    x
}
