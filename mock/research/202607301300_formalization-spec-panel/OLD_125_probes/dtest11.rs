#![no_std]
extern crate arvo;
use arvo::*;
const _: () = assert!(<w!(6 5 5 3 5) as Nat>::VAL == 65535);
