// p3: what the exit costs where it is satisfied by identity.
#![no_std]
#![crate_type = "lib"]
pub trait TruthAlgebra: Copy {
    fn and(self, o: Self) -> Self;
}
pub trait Branch: TruthAlgebra {
    fn is_true(self) -> bool;
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bit(bool);
impl TruthAlgebra for Bit {
    #[inline(always)]
    fn and(self, o: Self) -> Self {
        Bit(self.0 & o.0)
    }
}
impl Branch for Bit {
    #[inline(always)]
    fn is_true(self) -> bool {
        self.0
    }
}

pub trait Compare {
    type Truth: TruthAlgebra;
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
pub fn c_generic<T: Compare + Copy>(a: T, b: T, c: T, d: T) -> u32
where
    T::Truth: Branch,
{
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
