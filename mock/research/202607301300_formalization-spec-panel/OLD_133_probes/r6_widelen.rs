#![no_std]
pub struct Z;
pub struct S<T>(core::marker::PhantomData<T>);
pub trait Nat {
    const V: usize;
}
impl Nat for Z {
    const V: usize = 0;
}
impl<T: Nat> Nat for S<T> {
    const V: usize = 1 + T::V;
} // value position: fine
pub struct Wide<B: Nat>([u8; <B as Nat>::V]); // type position: the wall
