//! P12: container dispatch WITHOUT generic_const_args. The "carry and read,
//! never transform" discipline: the container is CARRIED alongside the width,
//! and a const assertion READS both to check the pairing. No gate, no -Z flag.
#![no_std]

pub trait Container: Copy + Default {
    const BITS: u16;
}
impl Container for u8 {
    const BITS: u16 = 8;
}
impl Container for u16 {
    const BITS: u16 = 16;
}
impl Container for u32 {
    const BITS: u16 = 32;
}
impl Container for u64 {
    const BITS: u16 = 64;
}
impl Container for u128 {
    const BITS: u16 = 128;
}

pub struct Fx<const W: u16, C: Container> {
    pub raw: C,
}

impl<const W: u16, C: Container> Fx<W, C> {
    // read both, never transform on the way into a type
    const FITS: () = assert!(W <= C::BITS, "width does not fit its container");
    pub fn new(raw: C) -> Self {
        let () = Self::FITS;
        Fx { raw }
    }
}

pub fn ok() {
    let _a = Fx::<3, u8>::new(0);
    let _b = Fx::<13, u16>::new(0);
    let _c = Fx::<47, u64>::new(0);
    let _d = Fx::<100, u128>::new(0);
}
pub fn bad() {
    let _e = Fx::<9, u8>::new(0);
} // 9 > 8, must be refused
