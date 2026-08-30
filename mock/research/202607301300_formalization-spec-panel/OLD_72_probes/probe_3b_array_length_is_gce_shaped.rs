// Probe 3b: EXPECTED FAIL. The tempting spelling, an array sized by a const
// computed from the generic numeral's parameters, is the forbidden
// generic_const_exprs shape. This file exists to pin that the buffer length is
// a genuine spine-rule firing site rather than a style preference: the const
// route refuses under the permitted feature set.

#![no_std]

pub trait Numeral {
    const RADIX: u32;
    const PRECISION: u32;
}

pub const fn short_budget(radix: u32, precision: u32) -> usize {
    let mut pow: u128 = 1;
    let mut i = 0;
    while i < precision {
        pow *= radix as u128;
        i += 1;
    }
    let mut d: usize = 1;
    let mut ten: u128 = 1;
    while ten <= pow {
        ten *= 10;
        d += 1;
    }
    d + 6
}

pub fn print_short<N: Numeral>() -> [u8; short_budget(N::RADIX, N::PRECISION)] {
    [0; short_budget(N::RADIX, N::PRECISION)]
}
