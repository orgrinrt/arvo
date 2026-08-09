#![no_std]
extern crate ladder;
extern crate natmac;
use ladder::*;
use natmac::nat;
pub type W1 = <nat!(1152921504606846976) as Container>::C; // 2^60
pub type W2 = <nat!(4294967295) as Container>::C; // 2^32 - 1
pub type W3 = <nat!(1) as Container>::C;
