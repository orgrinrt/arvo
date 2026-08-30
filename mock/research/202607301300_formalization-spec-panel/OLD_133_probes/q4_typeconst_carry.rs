#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]
pub trait Tagged {
    type const B: usize;
}
pub struct Rung<const N: usize>;
// pure carry: the RHS is the parameter itself, nothing computed
impl<const N: usize> Tagged for Rung<N> {
    type const B: usize = N;
}
pub trait Store {
    type T: Copy;
}
impl<const N: usize> Store for Rung<N> {
    type T = [u8; <Rung<N> as Tagged>::B];
}
