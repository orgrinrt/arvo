#![no_std]
use gcecore::*;
pub fn work(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) {
    let _p: UFixed<26, 6, Warm> = mul(a, b);
}
