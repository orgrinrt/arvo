//! c01c: the same width added against ARVO's marker instead of a local one.
//! This is what a consumer would write if the marker parameter were not there.
#![no_std]
#![crate_type = "lib"]
extern crate arvo_min;
use arvo_min::*;

pub type T7 = D1<D1<D1<Term>>>;
impl ToNat<Arvo> for Idx<7> {
    type N = T7;
}
