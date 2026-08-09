#![no_std]
extern crate arvo;
use arvo::*;
const _: () = assert!(<w!(1 3) as Nat>::VAL > 0);
