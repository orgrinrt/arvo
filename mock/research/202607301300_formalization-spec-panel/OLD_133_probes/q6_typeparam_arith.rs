#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
pub trait Nat {
    type const V: usize;
}
pub struct Sum<X, Y>(core::marker::PhantomData<(X, Y)>);
// no const generic parameters in scope here; only TYPE parameters X and Y
impl<X: Nat, Y: Nat> Nat for Sum<X, Y> {
    type const V: usize = const { <X as Nat>::V + <Y as Nat>::V };
}
pub struct N13;
pub struct N3;
impl Nat for N13 {
    type const V: usize = 13;
}
impl Nat for N3 {
    type const V: usize = 3;
}
pub trait Store {
    type T: Copy;
}
pub struct S<X>(core::marker::PhantomData<X>);
impl<X: Nat> Store for S<X> {
    type T = [u8; <X as Nat>::V];
}
pub fn probe(_: <S<Sum<N13, N3>> as Store>::T) {}
