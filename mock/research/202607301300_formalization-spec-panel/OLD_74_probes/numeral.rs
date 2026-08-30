//! probe 1, crate `numeral`: the numeral contract as a SECOND independent
//! consumer of the same sealed vocabulary. Precision is a Nat drawn from the
//! identical carrier the capacity crate aliases.
#![no_std]
use carrier::{Nat, Pz, H, I, O};

pub trait Numeral {
    type Precision: Nat;
}
pub struct Binary13;
impl Numeral for Binary13 {
    type Precision = Pz<I<O<I<H>>>>; // p = 13, the same type Cap13 names
}
