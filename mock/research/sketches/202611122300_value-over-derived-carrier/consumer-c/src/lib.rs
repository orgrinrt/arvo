//! Arm C from a crate with no `#![feature]` line: the five consumer cases.
//!
//! Case (3) bounds the signature by `Sole` and names the const trait as an
//! ordinary bound on the carrier.

#![no_std]

use arm_c::Num;

#[cfg(not(feature = "const-bound"))]
common::consumer_cases!(Num, [common::Sole<Repr: arm_c::Carrier>]);

// The bound the always-const variant asks for, written in a crate with no
// feature line.
#[cfg(feature = "const-bound")]
common::consumer_cases!(Num, [common::Sole<Repr: const arm_c::Carrier>]);
