// Probe: can `min_generic_const_args` accept a projected associated CONST
// (with no further arithmetic wrapping it) as a const-generic ARGUMENT,
// where the arithmetic that PRODUCED that const happened inside an ordinary
// impl body rather than in type position? This is the load-bearing question
// for the typed exact product: does it let a function generic over two
// widths return a type parameterised by their SUM, without `generic_const_exprs`?
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct W<const N: u16>;

pub trait AddWidth<Rhs> {
    const OUT: u16;
}

impl<const A: u16, const B: u16> AddWidth<W<B>> for W<A> {
    const OUT: u16 = A + B; // ordinary const-eval, NOT type position
}

pub struct Fixed<const I: u16, const F: u16>;

// The function is GENERIC over I1, F1, I2, F2. Its return type's const
// generic arguments are bare projections `<W<X> as AddWidth<W<Y>>>::OUT`,
// no arithmetic wrapping them in the braces.
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
