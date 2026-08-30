// p6: files 105 and 106 both say Bool gets the exit "by a blanket" and neither
// names the blanket's bound. A blanket over the algebra supplies the exit to
// masks too, which kills the never-a-default clause in the same breath.
// Candidate repair: index the algebra by its lane count and bound the blanket
// on one lane, so the exit is DERIVED from arity rather than chosen.
#![no_std]
#![crate_type = "lib"]

pub trait TruthAlgebra<const W: usize>: Copy {
    const TRUE: Self;
    fn and(self, o: Self) -> Self;
    fn or(self, o: Self) -> Self;
    fn not(self) -> Self;
    fn lane(self, i: usize) -> bool; // the W coordinate projections
}
pub trait Branch {
    fn is_true(self) -> bool;
}

// the exit exists exactly at one lane, and is the unique projection there
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
    const TRUE: Self = Bit(true);
    #[inline(always)]
    fn and(self, o: Self) -> Self {
        Bit(self.0 & o.0)
    }
    #[inline(always)]
    fn or(self, o: Self) -> Self {
        Bit(self.0 | o.0)
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
impl TruthAlgebra<2> for Mask2 {
    const TRUE: Self = Mask2([true, true]);
    #[inline(always)]
    fn and(self, o: Self) -> Self {
        Mask2([self.0[0] & o.0[0], self.0[1] & o.0[1]])
    }
    #[inline(always)]
    fn or(self, o: Self) -> Self {
        Mask2([self.0[0] | o.0[0], self.0[1] | o.0[1]])
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
    #[inline(always)]
    pub fn all(self) -> bool {
        self.0[0] & self.0[1]
    }
    #[inline(always)]
    pub fn any(self) -> bool {
        self.0[0] | self.0[1]
    }
}

pub fn bit_branches(b: Bit) -> u8 {
    if b.is_true() {
        1
    } else {
        0
    }
}
pub fn mask_named(m: Mask2) -> u8 {
    if m.any() {
        1
    } else {
        0
    }
}
impl Branch for Bit {
    fn is_true(self) -> bool {
        !self.0
    }
} // a hand-written exit at one lane
