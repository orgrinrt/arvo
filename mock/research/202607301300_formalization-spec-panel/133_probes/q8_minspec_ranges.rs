#![no_std]
#![feature(min_specialization)]
#![allow(incomplete_features)]
pub trait Store {
    type T: Copy;
    fn zero() -> Self::T;
}
pub struct Rung<const N: usize>;
impl<const N: usize> Store for Rung<N> {
    default type T = u64;
    default fn zero() -> u64 {
        0
    }
}
// the narrow rung, wanted for N <= 8: not structurally more specific, only range-specific
impl<const N: usize> Store for Rung<N>
where
    Rung<N>: Narrow,
{
    type T = u8;
    fn zero() -> u8 {
        0
    }
}
pub trait Narrow {}
