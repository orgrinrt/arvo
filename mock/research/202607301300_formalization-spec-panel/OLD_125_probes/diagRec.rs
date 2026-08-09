#![no_std]
extern crate arvo_rec as arvo;
use arvo::*;
pub fn ok(_x: UFixed<13, 3, Warm>) {}
pub fn wide(_x: UFixedAt<w!(1 0 0 0), w!(3), Warm>) {}
pub fn oops(_x: UFixed<1000, 3, Warm>) {}
