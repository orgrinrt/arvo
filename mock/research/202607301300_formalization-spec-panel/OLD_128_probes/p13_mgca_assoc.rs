//! P13: the literal "carry and read" shape. Arithmetic in VALUE position
//! (an ordinary associated const, unrestricted). Only a PATH in type position.
//! min_generic_const_args ONLY, which the rule already ALLOWS.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
pub struct W<const N: u16>;
pub trait Add2<const I: u16, const F: u16> {
    const SUM: u16;
}
pub struct Adder;
impl<const I: u16, const F: u16> Add2<I, F> for Adder {
    const SUM: u16 = I + F; // value position, ordinary assoc const
}
pub type PrecisionOf<const I: u16, const F: u16> = W<{ <Adder as Add2<I, F>>::SUM }>;
pub fn takes16(_: W<16>) {}
pub fn probe() {
    takes16(PrecisionOf::<13, 3> {});
    takes16(PrecisionOf::<8, 8> {});
}
