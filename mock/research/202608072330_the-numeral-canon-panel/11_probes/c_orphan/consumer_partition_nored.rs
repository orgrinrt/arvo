//! c01d: what a marker costs. Two independent libraries each extend the bridge
//! with one width they need. Question: do their numerals compose?
#![no_std]
#![crate_type = "lib"]
extern crate arvo_min;
use arvo_min::*;

pub struct LibA;
pub struct LibB;
pub type T7 = D1<D1<D1<Term>>>;

// LibA needs width 7 and also uses arvo's 13.
impl ToNat<LibA> for Idx<7> {
    type N = T7;
}

// LibB needs nothing extra. It uses arvo's 13, against arvo's marker.
// (declared here for the probe; in reality it is arvo's own row)

pub type FromA = Fixed<13, 0, LibA>;
pub type FromArvo = Fixed<13, 0, Arvo>;

// the same width, the same nat, two markers. Does one flow into the other?
pub fn compose(x: FromA) -> FromArvo {
    x
}
