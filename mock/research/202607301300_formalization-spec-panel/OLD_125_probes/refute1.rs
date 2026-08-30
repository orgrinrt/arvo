#![no_std]
#![feature(min_generic_const_args)]
extern crate tower;
use tower::*;
pub struct Idx<const N: u16>;
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
    type Out = Z;
}
