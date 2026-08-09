// p4: by which routes does an unnamed default exit arrive? Enumerated by
// introduction route, not by attacks anyone thought of.
#![no_std]
#![crate_type = "lib"]

// R1: the algebra carries equality, which every truth type will want.
// Then the exit has a canonical-LOOKING default that is `all` in disguise.
pub trait TruthAlgebraEq: Copy + PartialEq {
    const TRUE: Self;
    fn and(self, o: Self) -> Self;
}
pub trait BranchR1: TruthAlgebraEq {
    #[inline(always)]
    fn is_true(self) -> bool {
        self == Self::TRUE
    } // <-- silently ALL
}

#[derive(Clone, Copy, PartialEq)]
pub struct Mask2([bool; 2]);
impl TruthAlgebraEq for Mask2 {
    const TRUE: Self = Mask2([true, true]);
    fn and(self, o: Self) -> Self {
        Mask2([self.0[0] & o.0[0], self.0[1] & o.0[1]])
    }
}
impl BranchR1 for Mask2 {} // no method written. the exit arrived anyway.
pub fn r1(m: Mask2) -> bool {
    m.is_true()
}

// R2: a blanket impl over the algebra.
pub trait Branch2 {
    fn is_true2(self) -> bool;
}
impl<T: TruthAlgebraEq> Branch2 for T {
    #[inline(always)]
    fn is_true2(self) -> bool {
        self == T::TRUE
    }
}
pub fn r2(m: Mask2) -> bool {
    m.is_true2()
}

// R3: an inherent method with the exit's name, while the trait bound stays
// unsatisfied. Inherent methods win resolution.
pub trait Branch3 {
    fn is_true3(self) -> bool;
}
impl Mask2 {
    #[inline(always)]
    pub fn is_true3(self) -> bool {
        self.0[0] & self.0[1]
    }
}
pub fn r3(m: Mask2) -> bool {
    m.is_true3()
} // reads like the trait, is not

// R4: Deref, which this design already uses for predicate call syntax.
use core::ops::Deref;
#[derive(Clone, Copy)]
pub struct Mask2D([bool; 2]);
impl Deref for Mask2D {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0[0]
    } // silently lane 0
}
pub fn r4(m: Mask2D) -> bool {
    *m
}

// R5: an ordinary conversion.
impl From<Mask2> for bool {
    fn from(m: Mask2) -> bool {
        m.0[0] | m.0[1]
    }
}
pub fn r5(m: Mask2) -> bool {
    m.into()
} // silently ANY this time
