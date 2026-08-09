//! A1. The ratified unification, written the obvious way.
//!
//! `74b` adopts one sealed bottom carrier shared by the tower's naturals and
//! the capacity domain, with `Capacity` kept as a semantic alias over it. The
//! capacity domain's whole job is to name the backing array for a count, so
//! the alias owes a GAT `type Array<T>` that is a real contiguous `[T; N]`.
//!
//! Written directly: the count is the sealed inductive natural, and the array
//! length is that natural's value.
//!
//! The question is whether that sentence has any expression under the
//! permitted feature set. If it does not, the unification's cost is not a
//! number and the bench does not begin.
#![no_std]
#![feature(adt_const_params)]

use core::marker::PhantomData;

mod seal {
    pub trait Sealed {}
}

// The sealed value-unique vocabulary, `68:549-556`.
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl seal::Sealed for H {}
impl<P: Pos> seal::Sealed for O<P> {}
impl<P: Pos> seal::Sealed for I<P> {}
impl seal::Sealed for Z {}
impl<P: Pos> seal::Sealed for Pz<P> {}

pub trait Pos: seal::Sealed {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

// The semantic alias op kept: `Capacity` by name, shared carrier underneath,
// one blanket impl so every natural is a capacity and no second encoding
// exists. This is the shape `74b` adopted, stated as code.
pub trait Capacity {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const CAP: usize;
}

impl<N: Nat> Capacity for N {
    type Array<T> = [T; <N as Nat>::VAL];
    const CAP: usize = <N as Nat>::VAL;
}

pub type C8 = Pz<O<O<O<H>>>>;

pub fn walk<C: Capacity>(a: &C::Array<u32>) -> u32 {
    let mut s = 0;
    for x in a.as_ref() {
        s += *x;
    }
    s
}
