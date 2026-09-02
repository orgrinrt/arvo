#![no_std]
#![feature(macro_metavar_expr_concat)]
#![allow(dead_code)]
extern crate arvo;
use arvo::*;

macro_rules! two_digit {
    ( [ $($a:tt)* ] [ $($b:tt)* ] ) => {
        $( $( pub type ${concat(W, $a, $b)} = w!($a $b); )* )*
    };
}
two_digit!([1 2 3] [0 1 2]);
const _: () = assert!(<W13 as Nat>::VAL == 13);
const _: () = assert!(<W32 as Nat>::VAL == 32);
