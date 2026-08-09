// p7: does the split survive the design's own const-callable idiom?
// pub const trait, const impl, [const] bounds, blanket bounded on one lane.
#![no_std]
#![crate_type = "lib"]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

pub const trait TruthAlgebra<const W: usize>: Copy {
    const TRUE: Self;
    fn and(self, o: Self) -> Self;
    fn not(self) -> Self;
    fn lane(self, i: usize) -> bool;
}
pub const trait Branch {
    fn is_true(self) -> bool;
}

const impl<T: [const] TruthAlgebra<1>> Branch for T {
    #[inline(always)]
    fn is_true(self) -> bool {
        self.lane(0)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bit(bool);
const impl TruthAlgebra<1> for Bit {
    const TRUE: Self = Bit(true);
    #[inline(always)]
    fn and(self, o: Self) -> Self {
        Bit(self.0 & o.0)
    }
    #[inline(always)]
    fn not(self) -> Self {
        Bit(!self.0)
    }
    #[inline(always)]
    fn lane(self, _i: usize) -> bool {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct Mask2([bool; 2]);
const impl TruthAlgebra<2> for Mask2 {
    const TRUE: Self = Mask2([true, true]);
    #[inline(always)]
    fn and(self, o: Self) -> Self {
        Mask2([self.0[0] & o.0[0], self.0[1] & o.0[1]])
    }
    #[inline(always)]
    fn not(self) -> Self {
        Mask2([!self.0[0], !self.0[1]])
    }
    #[inline(always)]
    fn lane(self, i: usize) -> bool {
        self.0[i]
    }
}
impl Mask2 {
    pub const fn any(self) -> bool {
        self.0[0] | self.0[1]
    }
}

// a producer declaration bound on the ALGEBRA, const-callable, any lane count
pub const trait Compare<const W: usize>: Copy {
    type Truth: [const] TruthAlgebra<W>;
    fn eq(self, o: Self) -> Self::Truth;
}
#[derive(Clone, Copy)]
pub struct Scalar(u32);
const impl Compare<1> for Scalar {
    type Truth = Bit;
    #[inline(always)]
    fn eq(self, o: Self) -> Bit {
        Bit(self.0 == o.0)
    }
}
#[derive(Clone, Copy)]
pub struct Vec2([u32; 2]);
const impl Compare<2> for Vec2 {
    type Truth = Mask2;
    #[inline(always)]
    fn eq(self, o: Self) -> Mask2 {
        Mask2([self.0[0] == o.0[0], self.0[1] == o.0[1]])
    }
}

// the exit used in a CONST position, at one lane, through the blanket
pub const EQ_AT_COMPILE_TIME: bool = Scalar(7).eq(Scalar(7)).is_true();
pub const NE_AT_COMPILE_TIME: bool = Scalar(7).eq(Scalar(9)).is_true();
// the two-lane instance exists and reduces only by its own named word
pub const VEC_ANY: bool = Vec2([1, 2]).eq(Vec2([1, 5])).any();
const _: () = assert!(EQ_AT_COMPILE_TIME && !NE_AT_COMPILE_TIME && VEC_ANY);
