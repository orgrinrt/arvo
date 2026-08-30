#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
pub struct A<const N: usize>([u8; N]);
pub fn f<const N: usize>(_: A<{ N + 1 }>) {}
