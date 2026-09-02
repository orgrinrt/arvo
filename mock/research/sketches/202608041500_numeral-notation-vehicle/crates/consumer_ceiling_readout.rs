//! FAILS test, on purpose: a literal past the 64-bit `Pos::VAL` readout
//! wall. Compile-fail is the expected, correct outcome; the check is what
//! the refusal SAYS, per `a-test-that-cannot-compile-is-the-finding.md`
//! ("a test that will not compile is the finding, not an obstacle").
//!
//! 123456789012345678901234567890 (30 digits, ~97 bits) sits between the
//! two walls: well under the 128-bit structural ceiling (nameable as a
//! type), well past the 64-bit readout ceiling (its `Pos::VAL` cannot be
//! read). The macro must say so using the actual decimal number, computed
//! host-side before any type token existed, never the encoding.

#[path = "tower.rs"]
mod tower;
use tower::*;

extern crate numeral_pm;
use numeral_pm::raw_bias;

type _T = raw_bias!(123456789012345678901234567890);

fn main() {}
