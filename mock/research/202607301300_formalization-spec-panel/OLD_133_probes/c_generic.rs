#![no_std]
extern crate acore;
use acore::{mul, UFixed, Warm};
pub fn arith(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) -> UFixed<26, 6, Warm> {
    mul(a, b)
}
