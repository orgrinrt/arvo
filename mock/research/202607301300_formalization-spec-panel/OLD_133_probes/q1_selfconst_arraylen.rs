// q1: an inherent associated const path used as an array length, inside an
// associated type body. No gates. The question is whether a PATH (rather than
// an operation) escapes the rule, since the arithmetic sits in the const's body.
#![no_std]

pub trait Store {
    type T: Copy;
}
pub struct Rung<const I: usize, const F: usize>;

impl<const I: usize, const F: usize> Rung<I, F> {
    pub const B: usize = (I + F).div_ceil(8); // value position: computes freely
}

impl<const I: usize, const F: usize> Store for Rung<I, F> {
    type T = [u8; Self::B]; // type position: a path, not an op
}
