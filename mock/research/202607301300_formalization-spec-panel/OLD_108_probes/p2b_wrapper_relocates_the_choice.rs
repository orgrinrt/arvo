// p2b: the repair a reader reaches for when p2 fails. If the mask cannot
// satisfy the exit, wrap it: the producer names All<Mask2> as its Truth.
// Question: what happens to the choice between all and any?
#![no_std]
pub trait TruthAlgebra: Copy {
    fn and(self, o: Self) -> Self;
    fn not(self) -> Self;
}
pub trait Branch: TruthAlgebra {
    fn is_true(self) -> bool;
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

// All<M> and Any<M> carry the SAME algebra and differ only in the exit.
#[derive(Clone, Copy)]
pub struct All<M>(M);
#[derive(Clone, Copy)]
pub struct Any<M>(M);
impl<M: TruthAlgebra> TruthAlgebra for All<M> {
    fn and(self, o: Self) -> Self {
        All(self.0.and(o.0))
    }
    fn not(self) -> Self {
        All(self.0.not())
    }
}
impl<M: TruthAlgebra> TruthAlgebra for Any<M> {
    fn and(self, o: Self) -> Self {
        Any(self.0.and(o.0))
    }
    fn not(self) -> Self {
        Any(self.0.not())
    }
}
impl Branch for All<Mask2> {
    fn is_true(self) -> bool {
        (self.0).0[0] & (self.0).0[1]
    }
}
impl Branch for Any<Mask2> {
    fn is_true(self) -> bool {
        (self.0).0[0] | (self.0).0[1]
    }
}

pub trait Compare {
    type Truth: Branch;
    fn eq(self, o: Self) -> Self::Truth;
}

#[derive(Clone, Copy)]
pub struct Vec2([u32; 2]);
// The impl must pick ONE. There is exactly one associated type per (trait, Self).
impl Compare for Vec2 {
    type Truth = All<Mask2>;
    fn eq(self, o: Self) -> All<Mask2> {
        All(Mask2([self.0[0] == o.0[0], self.0[1] == o.0[1]]))
    }
}

// A caller who wants "any lane equal" has no route. This is the whole test:
pub fn caller_wants_any(a: Vec2, b: Vec2) -> bool {
    // the only truth this producer can hand back is All-flavoured
    a.eq(b).is_true() // silently means ALL, and the call site says nothing
}
