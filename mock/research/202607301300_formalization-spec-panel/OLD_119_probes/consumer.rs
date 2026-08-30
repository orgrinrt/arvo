#![allow(dead_code)]
extern crate carrier;
use carrier::{Idx, Nat, NatIndex, Z};
// A downstream crate tries to add the one width it needs.
impl NatIndex for Idx<13> {
    type Out = Z;
}
