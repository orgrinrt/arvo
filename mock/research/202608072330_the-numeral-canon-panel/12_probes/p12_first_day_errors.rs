//! p12. The first-day errors, at the alias site.
//!
//! Three mistakes a tier-two consumer makes in their first hour, in each
//! candidate surface. What they READ is the measurement.
//!
//!   (1) a width the program has not declared        `UInt<7>`
//!   (2) a nat name that does not exist              `UInt<N7>`
//!   (3) the two coordinates swapped                 `UFixed<3, 13, Hot>`
//!
//! (3) is the one nothing catches and it is included so that the comparison is
//! not flattering: no candidate turns a swapped pair into an error, because
//! 3.13 is a legal numeral. It shows up only as a width mismatch later, or not
//! at all.
//!
//! THIS FILE IS EXPECTED TO FAIL TO COMPILE. Captured in out/p12.log.
//!
//! rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
//!       --emit=metadata -o out/p12.meta p12_first_day_errors.rs 2> out/p12.log
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

include!("ladder.rs");
include!("surfaces.rs");

// (1) a width the door does not have, under C0 (const keyed, const door)
pub type Undeclared_C0 = c0::UInt<7>;

// (1) the same, under C4 (nat keyed, const door)
pub type Undeclared_C4 = c4::UInt<7>;

// (2) a nat name that does not exist, under C2 (nat keyed, name layer)
pub type Undeclared_C2 = c2::UInt<c2::N7>;
