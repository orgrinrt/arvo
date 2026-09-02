#![allow(dead_code)]
#![feature(min_specialization)]

pub struct Idx<const N: u16>;
pub trait Nat {
    const VAL: u64;
}
pub struct Z;
impl Nat for Z {
    const VAL: u64 = 0;
}

pub trait ToNat {
    type Out: Nat;
}
impl<const N: u16> ToNat for Idx<N> {
    default type Out = Z;
}
impl ToNat for Idx<0> {
    type Out = Z;
}
