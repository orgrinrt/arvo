#![no_std]
extern crate arvo;
use arvo::*;
const _: () = assert!(<w!(1 2 3 4 5 6) as Nat>::VAL == 123456);
