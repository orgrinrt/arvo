#![no_std]
#![crate_type = "lib"]
pub trait TruthAlgebra<const W: usize>: Copy {
    fn and(self, o: Self) -> Self;
    fn lane(self, i: usize) -> bool;
}
pub trait Branch {
    fn is_true(self) -> bool;
}
impl<T: TruthAlgebra<1>> Branch for T {
    #[inline(always)]
    fn is_true(self) -> bool {
        self.lane(0)
    }
}
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bit(bool);
impl TruthAlgebra<1> for Bit {
    #[inline(always)]
    fn and(self, o: Self) -> Self {
        Bit(self.0 & o.0)
    }
    #[inline(always)]
    fn lane(self, _i: usize) -> bool {
        self.0
    }
}
pub trait Compare: Copy {
    type Truth: TruthAlgebra<1>;
    fn eq(self, o: Self) -> Self::Truth;
}
#[derive(Clone, Copy)]
pub struct Scalar(u32);
impl Compare for Scalar {
    type Truth = Bit;
    #[inline(always)]
    fn eq(self, o: Self) -> Bit {
        Bit(self.0 == o.0)
    }
}

#[no_mangle]
pub fn a_raw(x: u32, y: u32, p: u32, q: u32) -> u32 {
    if (x == y) & (p == q) {
        17
    } else {
        31
    }
}
#[no_mangle]
pub fn b_concrete(x: u32, y: u32, p: u32, q: u32) -> u32 {
    let t = Bit(x == y).and(Bit(p == q));
    if t.is_true() {
        17
    } else {
        31
    }
}
pub fn c_generic<T: Compare>(a: T, b: T, c: T, d: T) -> u32 {
    let t = a.eq(b).and(c.eq(d));
    if t.is_true() {
        17
    } else {
        31
    }
}
#[no_mangle]
pub fn c_generic_at_scalar(x: u32, y: u32, p: u32, q: u32) -> u32 {
    c_generic(Scalar(x), Scalar(y), Scalar(p), Scalar(q))
}
