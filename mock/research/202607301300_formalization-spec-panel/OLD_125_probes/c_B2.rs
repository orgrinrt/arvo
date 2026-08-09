#![no_std]
#![allow(dead_code)]
extern crate arvo;
use arvo::*;
// prove the macro expansion is the same type as the alias, not merely parsed
fn a(x: UFixed<13, 3, Warm>) -> ufixed!(13, 3, Warm) {
    x
}
fn b(x: ufixed!(13, 3, Warm)) -> UFixedT<W13, W3, Warm> {
    x
}
fn c(x: UFixedT<W13, W3, Warm>) -> w_ufixed!([1 3], [3]) {
    x
}
