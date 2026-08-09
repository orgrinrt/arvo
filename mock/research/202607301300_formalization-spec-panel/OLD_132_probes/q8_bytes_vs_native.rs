//! Does a byte-array carrier recover native codegen? Scalar and vector.
#![no_std]
#![crate_type = "lib"]

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Native(u16);

/// The opaque carrier: bytes, align 1, width carried as the only const.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bytes<const B: usize>([u8; B]);

impl Native {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        Native(self.0.wrapping_add(o.0))
    }
}
impl Bytes<2> {
    #[inline]
    pub fn add(self, o: Self) -> Self {
        let a = u16::from_le_bytes(self.0);
        let b = u16::from_le_bytes(o.0);
        Bytes(a.wrapping_add(b).to_le_bytes())
    }
}

#[unsafe(no_mangle)]
pub fn s_native(a: Native, b: Native) -> Native {
    a.add(b)
}
#[unsafe(no_mangle)]
pub fn s_bytes(a: Bytes<2>, b: Bytes<2>) -> Bytes<2> {
    a.add(b)
}

#[unsafe(no_mangle)]
pub fn v_native(a: &mut [Native; 1024], b: &[Native; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add(b[i]);
    }
}
#[unsafe(no_mangle)]
pub fn v_bytes(a: &mut [Bytes<2>; 1024], b: &[Bytes<2>; 1024]) {
    for i in 0..1024 {
        a[i] = a[i].add(b[i]);
    }
}
