#![no_std]
use core::marker::PhantomData;
pub struct Warm;
pub struct Unsigned;
// step B: rung -> machine type, gate-free, literal-keyed
pub struct Rung<const B: usize>;
pub trait Machine {
    type M: Copy;
    fn add(a: Self::M, b: Self::M) -> Self::M;
}
impl Machine for Rung<2> {
    type M = u16;
    #[inline]
    fn add(a: u16, b: u16) -> u16 {
        a.wrapping_add(b)
    }
}
impl Machine for Rung<8> {
    type M = u64;
    #[inline]
    fn add(a: u64, b: u64) -> u64 {
        a.wrapping_add(b)
    }
}
// the numeral: ONE real field, everything else a ZST marker
#[repr(transparent)]
pub struct Fixed<const I: u32, const F: u32, const B: usize, G, S>
where
    Rung<B>: Machine,
{
    raw: <Rung<B> as Machine>::M,
    _m: PhantomData<(G, S)>,
}

impl<const I: u32, const F: u32, const B: usize, G, S> Fixed<I, F, B, G, S>
where
    Rung<B>: Machine,
{
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Self {
            raw: <Rung<B> as Machine>::add(self.raw, o.raw),
            _m: PhantomData,
        }
    }
}
impl<const I: u32, const F: u32, const B: usize, G, S> Clone for Fixed<I, F, B, G, S>
where
    Rung<B>: Machine,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<const I: u32, const F: u32, const B: usize, G, S> Copy for Fixed<I, F, B, G, S> where
    Rung<B>: Machine
{
}
pub type Q13_3 = Fixed<13, 3, 2, Unsigned, Warm>;
pub type Q40_24 = Fixed<40, 24, 8, Unsigned, Warm>;

// layout erasure, asserted at compile time
const _: () = assert!(core::mem::size_of::<Q13_3>() == core::mem::size_of::<u16>());
const _: () = assert!(core::mem::align_of::<Q13_3>() == core::mem::align_of::<u16>());
const _: () = assert!(core::mem::size_of::<Q40_24>() == core::mem::size_of::<u64>());

#[unsafe(no_mangle)]
pub fn arvo16(a: Q13_3, b: Q13_3) -> Q13_3 {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn native16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub fn arvo64(a: Q40_24, b: Q40_24) -> Q40_24 {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn native64(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub fn arvo_vec(x: &mut [Q13_3; 1024], y: &[Q13_3; 1024]) {
    for i in 0..1024 {
        x[i] = x[i].add(y[i]);
    }
}
#[unsafe(no_mangle)]
pub fn native_vec(x: &mut [u16; 1024], y: &[u16; 1024]) {
    for i in 0..1024 {
        x[i] = x[i].wrapping_add(y[i]);
    }
}
