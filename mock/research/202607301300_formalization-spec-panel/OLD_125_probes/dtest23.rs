#![no_std]
extern crate arvo;
use arvo::*;
const _: () = assert!(<w!(1 0 0 0 0) as Nat>::VAL == 10000);
