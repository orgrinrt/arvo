//! Question: under arm three, can a consumer that names no gate, the only kind
//! of consumer the obligation admits, call the contract at a concrete `Num` in
//! a const item? This is the one constness arm three could add for such a
//! consumer that the inherent `const fn add` does not already give. If this is
//! refused, arm three adds nothing an obligation-compliant consumer can reach.

use arm_const::Sum;
use arvo_format::Adapt;
use arvo_format::overflow::Saturate;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Quiet};

pub type I8 = Int<8, Adapt<TowardZero, Saturate>, Quiet>;

pub const SEVEN: I8 = <I8 as Sum>::sum(I8::constant::<3>(), I8::constant::<4>());
