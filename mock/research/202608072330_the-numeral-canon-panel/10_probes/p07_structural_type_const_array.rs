//! P07. The direction that IS open: type -> const. Can a structural nat compute
//! its value as a `type const` recursively, and can that value then be an array
//! length? `133:410-417` reported `[u8; <B as Nat>::V]` refused; that was
//! without `min_generic_const_args`. This retests it with `type const`.
#![no_std]
#![crate_type = "lib"]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub struct Term;
pub struct D0<T>(PhantomData<T>);
pub struct D1<T>(PhantomData<T>);

pub trait Nat {
    type const V: usize;
}
impl Nat for Term {
    type const V: usize = 0;
}
impl<T: Nat> Nat for D0<T> {
    type const V: usize = const { 2 * <T as Nat>::V };
}
impl<T: Nat> Nat for D1<T> {
    type const V: usize = const { 2 * <T as Nat>::V + 1 };
}

pub type N13 = D1<D0<D1<D1<Term>>>>;

// the thing `133` said could not be written
pub struct Payload<T: Nat>(pub [u8; <T as Nat>::V]);

const _: () = assert!(core::mem::size_of::<Payload<N13>>() == 13);
