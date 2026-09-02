#![no_std]
#![allow(dead_code)]
extern crate lawlib;
use lawlib::{UFixed, Warm};
pub fn consumer(a: UFixed<13, 3, u16, Warm>) -> UFixed<13, 3, u16, Warm> {
    lawlib::square_wrong(a)
}
