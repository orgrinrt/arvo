//! P10: nested GCA, where-clauses, and chained normalization.
#![no_std]
#![feature(min_generic_const_args, generic_const_args)]
#![allow(incomplete_features)]
pub struct W<const N: u16>;
pub trait Add2<const I: u16, const F: u16> {
    type const SUM: u16;
}
pub struct Adder;
impl<const I: u16, const F: u16> Add2<I, F> for Adder {
    type const SUM: u16 = const { I + F };
}

pub trait Dbl<const N: u16> {
    type const OUT: u16;
}
pub struct D;
impl<const N: u16> Dbl<N> for D {
    type const OUT: u16 = const { N * 2 };
}

// NESTED: feed one GCA result into another
pub type Nested<const I: u16, const F: u16> =
    W<{ <D as Dbl<{ <Adder as Add2<I, F>>::SUM }>>::OUT }>;

pub fn t32(_: W<32>) {}
pub fn probe() {
    t32(Nested::<13, 3> {});
}
