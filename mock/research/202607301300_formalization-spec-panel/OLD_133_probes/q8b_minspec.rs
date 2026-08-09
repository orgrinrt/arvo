#![no_std]
#![feature(min_specialization)]
pub trait Store {
    type T: Copy;
}
pub struct Rung<const N: usize>;
pub trait Narrow {}
impl<const N: usize> Store for Rung<N> {
    default type T = u64;
}
impl<const N: usize> Store for Rung<N>
where
    Rung<N>: Narrow,
{
    type T = u8;
}
