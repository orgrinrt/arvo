#![no_std]
extern crate arvo;
use arvo::*;
const _: () = assert!(<w!(9 9) as Nat>::VAL > 0);
