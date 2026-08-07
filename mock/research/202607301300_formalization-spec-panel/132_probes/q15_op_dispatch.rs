//! Relocating the case split from the TYPE to the OPERATION BODY.
//! The carrier is opaque bytes; the body asks "which machine type is B?".
//! Gate-free attempt.
#![no_std]
#![crate_type = "lib"]
use core::marker::PhantomData;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bytes<const B: usize>([u8; B]);

pub trait Machine {
    type M: Copy;
    fn add(a: Self::M, b: Self::M) -> Self::M;
}
pub struct Rung<const B: usize>(PhantomData<[(); B]>);
impl Machine for Rung<1> {
    type M = u8;
    fn add(a: u8, b: u8) -> u8 {
        a.wrapping_add(b)
    }
}
impl Machine for Rung<2> {
    type M = u16;
    fn add(a: u16, b: u16) -> u16 {
        a.wrapping_add(b)
    }
}
impl Machine for Rung<4> {
    type M = u32;
    fn add(a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
    }
}
impl Machine for Rung<8> {
    type M = u64;
    fn add(a: u64, b: u64) -> u64 {
        a.wrapping_add(b)
    }
}

/// The op body wants the machine type for this B. Bound is standalone: `Rung<B>`.
impl<const B: usize> Bytes<B>
where
    Rung<B>: Machine,
{
    #[inline]
    pub fn add(self, o: Self) -> Self {
        // needs to reinterpret [u8; B] as <Rung<B> as Machine>::M
        let a: <Rung<B> as Machine>::M = unsafe { core::mem::transmute_copy(&self.0) };
        let b: <Rung<B> as Machine>::M = unsafe { core::mem::transmute_copy(&o.0) };
        let s = <Rung<B> as Machine>::add(a, b);
        Bytes(unsafe { core::mem::transmute_copy(&s) })
    }
}

#[unsafe(no_mangle)]
pub fn d_b2(a: Bytes<2>, b: Bytes<2>) -> Bytes<2> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn d_b8(a: Bytes<8>, b: Bytes<8>) -> Bytes<8> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn v_d_b2(a: &mut [Bytes<2>; 1024], b: &[Bytes<2>; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add(b[i]);
    }
}
