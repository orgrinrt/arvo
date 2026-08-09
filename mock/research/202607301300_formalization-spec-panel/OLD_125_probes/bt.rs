#![no_std]
extern crate arvo;
use arvo::*;
const _: () = assert!(<b!(1 1 0 1) as Nat>::VAL == 13);
const _: () = assert!(<b!(0) as Nat>::VAL == 0);
const _: () = assert!(<b!(1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1) as Nat>::VAL == 65535);
const _: () = assert!(<Sum<b!(1 0 1 0 0 0), b!(1 1 1 1 0)> as Nat>::VAL == 70);
