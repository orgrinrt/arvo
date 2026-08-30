//! c01b: a downstream crate adding a width. Local marker, foreign trait,
//! foreign carrier. The question is whether coherence admits this.
#![no_std]
#![crate_type = "lib"]
extern crate arvo_min;
use arvo_min::*;

pub struct Mine; // local type
pub type T7 = D1<D1<D1<Term>>>; // 7 = 0b111
impl ToNat<Mine> for Idx<7> {
    type N = T7;
} // foreign trait, foreign carrier, LOCAL param
impl ToNat<Mine> for Idx<13> {
    type N = T13;
} // the consumer must re-declare arvo's widths too

pub type MyU7 = Fixed<7, 0, Mine>;
