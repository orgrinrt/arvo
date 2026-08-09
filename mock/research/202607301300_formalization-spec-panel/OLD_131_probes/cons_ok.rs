// A DOWNSTREAM consumer: no #![feature], no -Z flag.
#![no_std]
use arvocore::*;
pub fn work(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) {
    let _p: UFixed<26, 6, Warm> = mul(a, b);
    let _s: UFixed<14, 3, Warm> = add(a, b);
}
