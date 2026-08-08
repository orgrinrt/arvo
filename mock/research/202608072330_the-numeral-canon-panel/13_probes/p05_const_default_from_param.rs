// P5. If a const parameter's DEFAULT may reference an earlier parameter, the
// consumer writes one number and the machinery receives the derived one, with
// no table and no expression at any use site. Highest-value cheap probe.
#![no_std]
#![allow(dead_code)]

pub struct Store<const B: usize>([u8; B]);

pub struct Numeral<const BITS: usize, const BYTES: usize = { (BITS + 7) / 8 }>(Store<BYTES>);

pub type UInt<const N: usize> = Numeral<N>;
