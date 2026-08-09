//! The honest opaque carrier: arithmetic written ONCE, generically, over [u8; B].
//! No ladder, no case split, B carried and never transformed. Zero features.
#![no_std]
#![crate_type = "lib"]

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bytes<const B: usize>([u8; B]);

impl<const B: usize> Bytes<B> {
    /// Ripple-carry add over the limbs. The only body the design would write.
    #[inline]
    pub fn add(self, o: Self) -> Self {
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
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Native16(u16);
impl Native16 {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Native16(self.0.wrapping_add(o.0))
    }
}
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Native64(u64);
impl Native64 {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Native64(self.0.wrapping_add(o.0))
    }
}

#[unsafe(no_mangle)]
pub fn limb_b2(a: Bytes<2>, b: Bytes<2>) -> Bytes<2> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn limb_b8(a: Bytes<8>, b: Bytes<8>) -> Bytes<8> {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn nat_16(a: Native16, b: Native16) -> Native16 {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn nat_64(a: Native64, b: Native64) -> Native64 {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn v_limb_b2(a: &mut [Bytes<2>; 1024], b: &[Bytes<2>; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add(b[i]);
    }
}
#[unsafe(no_mangle)]
pub fn v_nat_16(a: &mut [Native16; 1024], b: &[Native16; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add(b[i]);
    }
}
