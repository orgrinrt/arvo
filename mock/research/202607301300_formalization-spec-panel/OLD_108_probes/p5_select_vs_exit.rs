// p5: what a consumer that "branches" actually needs at more than one lane.
#![no_std]
#![crate_type = "lib"]
pub trait TruthAlgebra: Copy {
    fn and(self, o: Self) -> Self;
    fn not(self) -> Self;
}
pub trait Branch: TruthAlgebra {
    fn is_true(self) -> bool;
}

// select is keyed on the PAIR (datum, its truth), not on the truth alone.
pub trait Select: Copy {
    type Truth: TruthAlgebra;
    fn select(t: Self::Truth, a: Self, b: Self) -> Self;
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(transparent)]
pub struct Bit(bool);
impl TruthAlgebra for Bit {
    fn and(self, o: Self) -> Self {
        Bit(self.0 & o.0)
    }
    fn not(self) -> Self {
        Bit(!self.0)
    }
}
impl Branch for Bit {
    fn is_true(self) -> bool {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct Mask2([bool; 2]);
impl TruthAlgebra for Mask2 {
    fn and(self, o: Self) -> Self {
        Mask2([self.0[0] & o.0[0], self.0[1] & o.0[1]])
    }
    fn not(self) -> Self {
        Mask2([!self.0[0], !self.0[1]])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scalar(pub u32);
impl Select for Scalar {
    type Truth = Bit;
    #[inline(always)]
    fn select(t: Bit, a: Self, b: Self) -> Self {
        if t.0 {
            a
        } else {
            b
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2(pub [u32; 2]);
impl Select for Vec2 {
    type Truth = Mask2;
    #[inline(always)]
    fn select(t: Mask2, a: Self, b: Self) -> Self {
        Vec2([
            if t.0[0] { a.0[0] } else { b.0[0] },
            if t.0[1] { a.0[1] } else { b.0[1] },
        ])
    }
}

pub trait Compare: Copy {
    type Truth: TruthAlgebra;
    fn lt(self, o: Self) -> Self::Truth;
}
impl Compare for Scalar {
    type Truth = Bit;
    #[inline(always)]
    fn lt(self, o: Self) -> Bit {
        Bit(self.0 < o.0)
    }
}
impl Compare for Vec2 {
    type Truth = Mask2;
    #[inline(always)]
    fn lt(self, o: Self) -> Mask2 {
        Mask2([self.0[0] < o.0[0], self.0[1] < o.0[1]])
    }
}

// max, written once, correct at every lane count, needing NO exit.
pub fn max<T>(a: T, b: T) -> T
where
    T: Compare + Select<Truth = <T as Compare>::Truth>,
{
    T::select(a.lt(b), b, a)
}
#[no_mangle]
pub fn max_scalar(a: u32, b: u32) -> u32 {
    max(Scalar(a), Scalar(b)).0
}
#[no_mangle]
pub fn max_vec(a: [u32; 2], b: [u32; 2]) -> [u32; 2] {
    max(Vec2(a), Vec2(b)).0
}
#[no_mangle]
pub fn max_raw(a: u32, b: u32) -> u32 {
    if a < b {
        b
    } else {
        a
    }
}
