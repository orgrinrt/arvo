//! E2b: min_generic_const_args. Arithmetic in VALUE position (an associated
//! const body, unrestricted), with only a PATH appearing in type position.
//! This is op's `Capacity` rule read literally: carried and read, never
//! transformed on the way in.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct W<const N: u16>;

pub trait Add2<const I: u16, const F: u16> {
    const SUM: u16;
}
pub struct Adder;
impl<const I: u16, const F: u16> Add2<I, F> for Adder {
    // value position: ordinary const arithmetic, no restriction whatsoever
    const SUM: u16 = I + F;
}

// type position: a PATH to an associated const, not an expression
pub type PrecisionOf<const I: u16, const F: u16> = W<{ <Adder as Add2<I, F>>::SUM }>;

fn takes16(_: W<16>) {}
fn probe() {
    takes16(PrecisionOf::<13, 3> {});
    takes16(PrecisionOf::<8, 8> {});
}
