#![no_std]
pub trait Store {
    type T: Copy;
}
pub struct Rung<const N: usize>;
pub trait Small {}
pub trait Large {}
impl<const N: usize> Store for Rung<N>
where
    Rung<N>: Small,
{
    type T = u8;
}
impl<const N: usize> Store for Rung<N>
where
    Rung<N>: Large,
{
    type T = u64;
}
