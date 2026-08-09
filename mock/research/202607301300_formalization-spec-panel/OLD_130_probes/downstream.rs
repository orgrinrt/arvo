#![no_std]
#![allow(dead_code)]
extern crate arvolib;
use arvolib::{UFixed, Warm};
pub fn consumer(a: UFixed<13, 3, u16, Warm>) -> UFixed<13, 3, u16, Warm> {
    arvolib::square_wrong(a)
}
