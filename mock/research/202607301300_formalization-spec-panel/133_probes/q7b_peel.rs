#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
pub struct Z;
pub struct Su<T>(core::marker::PhantomData<T>);
pub struct Idx<const N: usize>;
pub trait ToNat {
    type N;
}
impl ToNat for Idx<0> {
    type N = Z;
}
impl<const N: usize> ToNat for Idx<N>
where
    Idx<{ N - 1 }>: ToNat,
{
    type N = Su<<Idx<{ N - 1 }> as ToNat>::N>;
}
