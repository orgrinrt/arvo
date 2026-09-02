// A width-dependent carrier that IS expressible gate-free, because both array
// lengths are standalone const arguments and nothing is computed. Recorded so
// the irreducibility claim is stated precisely rather than too broadly.
#![no_std]
#[derive(Copy, Clone)]
pub struct Split<const I: usize, const F: usize> {
    hi: [u8; I],
    lo: [u8; F],
}

pub const S_13_3: usize = core::mem::size_of::<Split<13, 3>>();
pub const S_3_0: usize = core::mem::size_of::<Split<3, 0>>();
pub const S_200_100: usize = core::mem::size_of::<Split<200, 100>>();
const _: () = assert!(S_13_3 == 16); // 16 BYTES for a 16-BIT numeral
const _: () = assert!(S_3_0 == 3); // 3 bytes for a 3-bit numeral
const _: () = assert!(S_200_100 == 300); // 300 bytes for a 300-bit numeral

// and the machine type is not reachable from it: any body wanting u16 must
// case-split on I + F, which is the same const operation.
