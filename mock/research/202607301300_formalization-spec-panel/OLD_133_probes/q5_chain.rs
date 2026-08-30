#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
pub trait A {
    type const V: usize;
}
pub trait B {
    type const V: usize;
}
pub struct P<const N: usize>;
pub struct Q<const N: usize>;
impl<const N: usize> A for P<N> {
    type const V: usize = N;
}
// chained: the RHS is a projection whose operand mentions a generic const param
impl<const N: usize> B for Q<N> {
    type const V: usize = <P<N> as A>::V;
}
pub trait Store {
    type T: Copy;
}
impl<const N: usize> Store for Q<N> {
    type T = [u8; <Q<N> as B>::V];
}
