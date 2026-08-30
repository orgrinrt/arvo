#![no_std]
extern crate d14_lib;
use d14_lib::*;
pub type UNat<const N: u32> = <Idx<N> as ToNat>::N;
pub fn ok(_: UNat<13>) {}
pub fn bad(_: UNat<14>) {}
