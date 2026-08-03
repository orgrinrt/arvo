#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct W<const N: u16>;

pub trait AddWidth<Rhs> {
    type const OUT: u16;
}

impl<const A: u16, const B: u16> AddWidth<W<B>> for W<A> {
    type const OUT: u16 = const { A + B };
}

pub struct Fixed<const I: u16, const F: u16>;

fn mul_full<const I1: u16, const F1: u16, const I2: u16, const F2: u16>(
    _a: Fixed<I1, F1>,
    _b: Fixed<I2, F2>,
) -> Fixed<{ <W<I1> as AddWidth<W<I2>>>::OUT }, { <W<F1> as AddWidth<W<F2>>>::OUT }> {
    Fixed
}

fn main() {
    let a = Fixed::<3, 5>;
    let b = Fixed::<7, 2>;
    let _c: Fixed<10, 7> = mul_full(a, b);
}
