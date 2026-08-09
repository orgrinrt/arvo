#![no_std]
pub trait Store {
    type T: Copy;
}
pub struct Rung<const N: usize>;
pub trait Small {}
pub trait Large {}
impl Small for Rung<3> {}
impl Large for Rung<40> {}
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
const _: () = {
    let _x: <Rung<3> as Store>::T = 0u8;
};
const _: () = {
    let _y: <Rung<40> as Store>::T = 0u64;
};
pub fn unclassified(_: <Rung<7> as Store>::T) {} // no Small, no Large impl for 7
