#![no_std]
use facade::*;
pub fn work(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) -> UFixed<26, 6, Warm> {
    mul_q13_3(a, b)
}
