#![allow(dead_code)]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub struct Idx<const N: u16>;
pub trait Nat {
    const VAL: u64;
}
pub struct Z;
pub struct Dbl<T: Nat>(core::marker::PhantomData<T>);
pub struct DblOne<T: Nat>(core::marker::PhantomData<T>);
impl Nat for Z {
    const VAL: u64 = 0;
}
impl<T: Nat> Nat for Dbl<T> {
    const VAL: u64 = 2 * T::VAL;
}
impl<T: Nat> Nat for DblOne<T> {
    const VAL: u64 = 2 * T::VAL + 1;
}

// The escape without a table: recurse on the const by halving it.
pub trait ToNat {
    type Out: Nat;
}
impl ToNat for Idx<0> {
    type Out = Z;
}
impl<const N: u16> ToNat for Idx<N>
where
    Idx<{ N / 2 }>: ToNat,
{
    type Out = Dbl<<Idx<{ N / 2 }> as ToNat>::Out>;
}
