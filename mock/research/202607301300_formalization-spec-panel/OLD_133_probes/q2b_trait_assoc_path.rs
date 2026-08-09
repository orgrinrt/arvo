#![no_std]
pub trait Tagged {
    const B: usize;
}
pub trait Store {
    type T: Copy;
}
pub struct Rung<const I: usize, const F: usize>;

impl<const I: usize, const F: usize> Tagged for Rung<I, F> {
    const B: usize = (I + F).div_ceil(8);
}
impl<const I: usize, const F: usize> Store for Rung<I, F> {
    type T = [u8; <Rung<I, F> as Tagged>::B];
}
