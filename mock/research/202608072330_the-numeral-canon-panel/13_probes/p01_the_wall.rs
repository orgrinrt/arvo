// P1. Establish the wall with no features at all: an expression over a const
// parameter in a const-argument slot.
#![no_std]
#![allow(dead_code)]

pub struct Store<const B: usize>([u8; B]);

// legal: a bare parameter is not an expression
pub struct PassThrough<const B: usize>([u8; B]);

// the wall: ceil8 of a generic parameter, in const-argument position
pub struct Derived<const N: usize>(Store<{ (N + 7) / 8 }>);
