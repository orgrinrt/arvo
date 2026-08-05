// p2: the same design with ONE change: the producer declaration binds on the
// EXIT-carrying part rather than on the algebra, which is the third clause of
// the shape as the checkpoint states it. Everything else is identical to p1.
// Question: does the two-lane instance survive that bound?
#![no_std]

pub trait TruthAlgebra: Copy {
    const TRUE: Self;
    const FALSE: Self;
    fn and(self, o: Self) -> Self;
    fn or(self, o: Self) -> Self;
    fn not(self) -> Self;
}
pub trait Branch: TruthAlgebra {
    fn is_true(self) -> bool;
}

#[derive(Clone, Copy)]
pub struct Bit(bool);
impl TruthAlgebra for Bit {
    const TRUE: Self = Bit(true);
    const FALSE: Self = Bit(false);
    fn and(self, o: Self) -> Self {
        Bit(self.0 & o.0)
    }
    fn or(self, o: Self) -> Self {
        Bit(self.0 | o.0)
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
    const TRUE: Self = Mask2([true, true]);
    const FALSE: Self = Mask2([false, false]);
    fn and(self, o: Self) -> Self {
        Mask2([self.0[0] & o.0[0], self.0[1] & o.0[1]])
    }
    fn or(self, o: Self) -> Self {
        Mask2([self.0[0] | o.0[0], self.0[1] | o.0[1]])
    }
    fn not(self) -> Self {
        Mask2([!self.0[0], !self.0[1]])
    }
}
impl Mask2 {
    pub fn all(self) -> bool {
        self.0[0] & self.0[1]
    }
    pub fn any(self) -> bool {
        self.0[0] | self.0[1]
    }
}

// THE ONE CHANGE: the declaration binds on the exit-carrying part.
pub trait Compare {
    type Truth: Branch;
    fn eq(self, o: Self) -> Self::Truth;
}

#[derive(Clone, Copy)]
pub struct Scalar(u32);
impl Compare for Scalar {
    type Truth = Bit;
    fn eq(self, o: Self) -> Bit {
        Bit(self.0 == o.0)
    }
}

#[derive(Clone, Copy)]
pub struct Vec2([u32; 2]);
impl Compare for Vec2 {
    type Truth = Mask2;
    fn eq(self, o: Self) -> Mask2 {
        Mask2([self.0[0] == o.0[0], self.0[1] == o.0[1]])
    }
}
