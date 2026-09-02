//! P1: where exactly does the wide add survive? Five bodies over the SAME
//! eight-byte payload, from "no machine type anywhere" to "the ladder names one".
//! No features, no flags.
#![no_std]
#![crate_type = "lib"]

use core::marker::PhantomData;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bytes<const B: usize>([u8; B]);

// --- A. q9's ripple carry, reproduced verbatim as the baseline ---------------
impl<const B: usize> Bytes<B> {
    #[inline]
    pub fn add_ripple(self, o: Self) -> Self {
        let mut out = [0u8; B];
        let mut carry = 0u8;
        let mut i = 0;
        while i < B {
            let (s, c1) = self.0[i].overflowing_add(o.0[i]);
            let (s, c2) = s.overflowing_add(carry);
            out[i] = s;
            carry = (c1 as u8) | (c2 as u8);
            i += 1;
        }
        Bytes(out)
    }

    // --- B. the same, written with the proper carrying_add primitive ---------
    #[inline]
    pub fn add_carrying(self, o: Self) -> Self {
        let mut out = [0u8; B];
        let mut carry = false;
        let mut i = 0;
        while i < B {
            let (s, c) = self.0[i].carrying_add(o.0[i], carry);
            out[i] = s;
            carry = c;
            i += 1;
        }
        Bytes(out)
    }

    // --- C. carry-free: add each byte independently, no chain ---------------
    // Semantically WRONG as a wide add, included only to isolate whether the
    // chain is what blocks vectorisation or whether the byte layout is.
    #[inline]
    pub fn add_nocarry(self, o: Self) -> Self {
        let mut out = [0u8; B];
        let mut i = 0;
        while i < B {
            out[i] = self.0[i].wrapping_add(o.0[i]);
            i += 1;
        }
        Bytes(out)
    }
}

// --- D. limb-generic ripple: does a WIDER limb type fix it on its own? ------
pub trait Limb: Copy {
    const ZERO: Self;
    fn carrying(self, o: Self, c: bool) -> (Self, bool);
}
impl Limb for u8 {
    const ZERO: u8 = 0;
    #[inline]
    fn carrying(self, o: u8, c: bool) -> (u8, bool) {
        self.carrying_add(o, c)
    }
}
impl Limb for u16 {
    const ZERO: u16 = 0;
    #[inline]
    fn carrying(self, o: u16, c: bool) -> (u16, bool) {
        self.carrying_add(o, c)
    }
}
impl Limb for u64 {
    const ZERO: u64 = 0;
    #[inline]
    fn carrying(self, o: u64, c: bool) -> (u64, bool) {
        self.carrying_add(o, c)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Limbs<L: Limb, const N: usize>([L; N]);

impl<L: Limb, const N: usize> Limbs<L, N> {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        let mut out = [L::ZERO; N];
        let mut carry = false;
        let mut i = 0;
        while i < N {
            let (s, c) = self.0[i].carrying(o.0[i], carry);
            out[i] = s;
            carry = c;
            i += 1;
        }
        Limbs(out)
    }
}

// --- E. the ladder: rung names the machine type -----------------------------
pub struct Rung<const B: usize>(PhantomData<[(); B]>);
pub trait Machine {
    type M: Copy;
    fn add(a: Self::M, b: Self::M) -> Self::M;
}
impl Machine for Rung<1> {
    type M = u8;
    #[inline]
    fn add(a: u8, b: u8) -> u8 {
        a.wrapping_add(b)
    }
}
impl Machine for Rung<2> {
    type M = u16;
    #[inline]
    fn add(a: u16, b: u16) -> u16 {
        a.wrapping_add(b)
    }
}
impl Machine for Rung<4> {
    type M = u32;
    #[inline]
    fn add(a: u32, b: u32) -> u32 {
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

impl<const B: usize> Bytes<B>
where
    Rung<B>: Machine,
{
    #[inline]
    pub fn add_ladder(self, o: Self) -> Self {
        // size_of::<Rung<B>::M>() == B holds by construction of the impls.
        unsafe {
            let a: <Rung<B> as Machine>::M = core::mem::transmute_copy(&self.0);
            let b: <Rung<B> as Machine>::M = core::mem::transmute_copy(&o.0);
            let s = <Rung<B> as Machine>::add(a, b);
            Bytes(core::mem::transmute_copy(&s))
        }
    }
}

// --- natives, the bar ---------------------------------------------------------
#[unsafe(no_mangle)]
pub fn nat_64(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub fn nat_16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}

// --- scalar sites -------------------------------------------------------------
#[unsafe(no_mangle)]
pub fn a_ripple_b8(a: Bytes<8>, b: Bytes<8>) -> Bytes<8> {
    a.add_ripple(b)
}
#[unsafe(no_mangle)]
pub fn b_carrying_b8(a: Bytes<8>, b: Bytes<8>) -> Bytes<8> {
    a.add_carrying(b)
}
#[unsafe(no_mangle)]
pub fn c_nocarry_b8(a: Bytes<8>, b: Bytes<8>) -> Bytes<8> {
    a.add_nocarry(b)
}
#[unsafe(no_mangle)]
pub fn d_limb_u8x8(a: Limbs<u8, 8>, b: Limbs<u8, 8>) -> Limbs<u8, 8> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn d_limb_u64x1(a: Limbs<u64, 1>, b: Limbs<u64, 1>) -> Limbs<u64, 1> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn d_limb_u16x4(a: Limbs<u16, 4>, b: Limbs<u16, 4>) -> Limbs<u16, 4> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn e_ladder_b8(a: Bytes<8>, b: Bytes<8>) -> Bytes<8> {
    a.add_ladder(b)
}
#[unsafe(no_mangle)]
pub fn e_ladder_b2(a: Bytes<2>, b: Bytes<2>) -> Bytes<2> {
    a.add_ladder(b)
}

// --- vector sites: 1024 elements of a 2-byte numeral ---------------------------
#[unsafe(no_mangle)]
pub fn v_nat_16(a: &mut [u16; 1024], b: &[u16; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].wrapping_add(b[i]);
    }
}
#[unsafe(no_mangle)]
pub fn v_ripple_b2(a: &mut [Bytes<2>; 1024], b: &[Bytes<2>; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add_ripple(b[i]);
    }
}
#[unsafe(no_mangle)]
pub fn v_carrying_b2(a: &mut [Bytes<2>; 1024], b: &[Bytes<2>; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add_carrying(b[i]);
    }
}
#[unsafe(no_mangle)]
pub fn v_ladder_b2(a: &mut [Bytes<2>; 1024], b: &[Bytes<2>; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add_ladder(b[i]);
    }
}
