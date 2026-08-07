#![allow(dead_code)]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct Idx<const N: u16>;
pub trait Nat {
    const VAL: u64;
}
pub struct Z;
pub struct Dbl<T: Nat>(core::marker::PhantomData<T>);
impl Nat for Z {
    const VAL: u64 = 0;
}
impl<T: Nat> Nat for Dbl<T> {
    const VAL: u64 = 2 * T::VAL;
}

// Route 2: hide the arithmetic behind a `type const` path, which min_gca
// admits in const-argument position, and recurse on the path.
pub trait Halve {
    type const HALF: u16;
}
impl<const N: u16> Halve for Idx<N> {
    type const HALF: u16 = N / 2;
}

pub trait ToNat {
    type Out: Nat;
}
impl<const N: u16> ToNat for Idx<N>
where
    Idx<{ <Idx<N> as Halve>::HALF }>: ToNat,
{
    type Out = Dbl<<Idx<{ <Idx<N> as Halve>::HALF }> as ToNat>::Out>;
}
