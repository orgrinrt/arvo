// Probe B. Spelling one from 110:2757-2759: the two contracts take the numeral,
// Policy<N> and Lowering<N>, "exactly as Crosses<N: Numeral> already does".
// Question: does this over-key, i.e. does it admit two numerals of the SAME kind
// disagreeing about what Warm means?
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
extern crate base;
use base::*;

pub const trait Policy<N: Numeral> {
    type Quantisation: Quantisation;
}
pub const trait Lowering<N: Numeral> {
    type StoredWidth: StoredWidth;
    type Layout: StorageLayout;
    type Door: LoweringDoor;
}

pub struct Warm;

// U13F3 and U14F2 are both Implicit numerals. Same kind. One table row applies
// to both: StoredWidth = doubled (110:2674).
const impl Lowering<U13F3> for Warm {
    type StoredWidth = DoubleLogical;
    type Layout = Dense;
    type Door = Inert;
}

// The same preset, the same number kind, the opposite row. Nothing relates them.
const impl Lowering<U14F2> for Warm {
    type StoredWidth = Minimum; // contradicts the ratified fixed-point row
    type Layout = Bitpacked; // and the Layout cell too
    type Door = Inert;
}
