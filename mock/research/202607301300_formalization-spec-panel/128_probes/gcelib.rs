#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
pub struct A<const N: usize>(pub [u8; N]);
pub fn widen<const N: usize>(_: A<N>) -> A<{ N + 1 }> {
    A([0; N + 1])
}
