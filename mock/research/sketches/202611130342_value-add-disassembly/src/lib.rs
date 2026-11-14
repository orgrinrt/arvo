//! `add` at 13 bits in a 16-bit carrier, signed and unsigned, under both
//! overflow policies, and at the full width of 64 bits beside the host's
//! `wrapping_add` and `saturating_add`.
//!
//! Each probe takes and returns the value type, so what is read is `add` and
//! nothing that builds its operands. `#[unsafe(no_mangle)]` keeps each probe in
//! the emitted assembly under its plain name.
//!
//! The probes are under the `Quiet` check, and two of them are repeated under
//! `Abort`, the default: the check is a debug build's, so in this profile the
//! two are expected to be the same instructions.
//!
//! Read with `cargo rustc --release --lib -- --emit asm`. An ad hoc reading of
//! one build, `release` profile, x86_64 with the default target features:
//! nothing here is a measurement.

#![no_std]

use arvo_format::Adapt;
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Abort, Int, Quiet, Uint};

type I13W = Int<13, Adapt<TowardZero, Wrap>, Quiet>;
type I13S = Int<13, Adapt<TowardZero, Saturate>, Quiet>;
type U13W = Uint<13, Adapt<TowardZero, Wrap>, Quiet>;
type U13S = Uint<13, Adapt<TowardZero, Saturate>, Quiet>;
type U64W = Uint<64, Adapt<TowardZero, Wrap>, Quiet>;
type I64S = Int<64, Adapt<TowardZero, Saturate>, Quiet>;
type I13SAbort = Int<13, Adapt<TowardZero, Saturate>, Abort>;
type U64WAbort = Uint<64, Adapt<TowardZero, Wrap>, Abort>;

#[unsafe(no_mangle)]
pub fn i13_wrapping(a: I13W, b: I13W) -> I13W {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn i13_saturating(a: I13S, b: I13S) -> I13S {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn i13_saturating_under_abort(a: I13SAbort, b: I13SAbort) -> I13SAbort {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn u13_wrapping(a: U13W, b: U13W) -> U13W {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn u13_saturating(a: U13S, b: U13S) -> U13S {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn u64_wrapping(a: U64W, b: U64W) -> U64W {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn u64_wrapping_under_abort(a: U64WAbort, b: U64WAbort) -> U64WAbort {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn host_u64_wrapping_add(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}

#[unsafe(no_mangle)]
pub fn i64_saturating(a: I64S, b: I64S) -> I64S {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn host_i64_saturating_add(a: i64, b: i64) -> i64 {
    a.saturating_add(b)
}
