// q2: same as q1 but the path names the type explicitly rather than `Self`,
// which removes the "generic Self in anonymous constant" complaint and asks
// the underlying question directly.
#![no_std]

pub trait Store {
    type T: Copy;
}
pub struct Rung<const I: usize, const F: usize>;

impl<const I: usize, const F: usize> Rung<I, F> {
    pub const B: usize = (I + F).div_ceil(8);
}

impl<const I: usize, const F: usize> Store for Rung<I, F> {
    type T = [u8; Rung::<I, F>::B];
}
