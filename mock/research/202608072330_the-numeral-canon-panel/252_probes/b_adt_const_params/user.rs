// Arm B, the consumer. No feature attribute. It names the declaration whose
// const parameter is an ADT.
#![no_std]

use door::{Signed, Width};

pub type Thirteen = Signed<{ Width::bits(13) }>;

pub const DECLARED: u32 = Thirteen::DECLARED;
