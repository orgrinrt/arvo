// Arm C negative control. The same outside crate, with the one coordinate value
// a `u32`-shaped door cannot hold written as the door would have to hold it.
// This must FAIL, or arm C's clean build says nothing: an experiment whose
// every arm passes has no discriminating power.
#![no_std]

use door::{Exponent, Radix, Width};

pub const REFUSED: Width = Width::bits(-4);
pub const ALSO: Radix = Radix::of(3);
pub const FINE: Exponent = Exponent::of(-4);
