// p4b: what the type system can actually refuse.
#![no_std]
#![crate_type = "lib"]
#![feature(negative_impls)]

pub trait TruthAlgebra: Copy + PartialEq {
    const TRUE: Self;
    fn and(self, o: Self) -> Self;
}
// the exit, declared with NO default body
pub trait Branch: TruthAlgebra {
    fn is_true(self) -> bool;
}

#[derive(Clone, Copy, PartialEq)]
pub struct Mask2([bool; 2]);
impl TruthAlgebra for Mask2 {
    const TRUE: Self = Mask2([true, true]);
    fn and(self, o: Self) -> Self {
        Mask2([self.0[0] & o.0[0], self.0[1] & o.0[1]])
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
// E1: state the absence in the type system rather than leaving it to a rule.
impl !Branch for Mask2 {}

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Bit(bool);
impl TruthAlgebra for Bit {
    const TRUE: Self = Bit(true);
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
