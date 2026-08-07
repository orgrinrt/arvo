#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
// Direction (b): unary type-level nat back to a usize const, by recursion.
pub struct Z;
pub struct Su<T>(core::marker::PhantomData<T>);
pub trait Nat {
    type const V: usize;
}
impl Nat for Z {
    type const V: usize = 0;
}
impl<T: Nat> Nat for Su<T> {
    type const V: usize = const { 1 + <T as Nat>::V };
}
