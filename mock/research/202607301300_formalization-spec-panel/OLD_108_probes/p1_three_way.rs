// p1: the algebra / exit / select three-way, non-const form.
// Question: can a producer declaration bind on the ALGEBRA alone and still
// admit both a one-lane and a two-lane truth type, while a control-flow
// consumer binds additionally on the EXIT and refuses the two-lane one?
#![no_std]

// ---- the Boolean-algebra core ----
pub trait TruthAlgebra: Copy {
    const TRUE: Self;
    const FALSE: Self;
    fn and(self, o: Self) -> Self;
    fn or(self, o: Self) -> Self;
    fn not(self) -> Self;
}

// ---- the exit, separately declared ----
pub trait Branch: TruthAlgebra {
    fn is_true(self) -> bool;
}

// ---- one-lane truth: satisfies the exit by identity ----
#[derive(Clone, Copy)]
pub struct Bit(bool);
impl TruthAlgebra for Bit {
    const TRUE: Self = Bit(true);
    const FALSE: Self = Bit(false);
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
}
impl Branch for Bit {
    #[inline(always)]
    fn is_true(self) -> bool {
        self.0
    }
}

// ---- two-lane truth: the direct product. Algebra only, no exit. ----
#[derive(Clone, Copy)]
pub struct Mask2([bool; 2]);
impl TruthAlgebra for Mask2 {
    const TRUE: Self = Mask2([true, true]);
    const FALSE: Self = Mask2([false, false]);
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
}
// the two reductions, INHERENT, named, neither one a trait impl
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

// ---- the producer declaration: bound on the ALGEBRA ----
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

#[derive(Clone, Copy)]
pub struct Vec2([u32; 2]);
impl Compare for Vec2 {
    type Truth = Mask2;
    #[inline(always)]
    fn eq(self, o: Self) -> Mask2 {
        Mask2([self.0[0] == o.0[0], self.0[1] == o.0[1]])
    }
}

// ---- a control-flow consumer: bound additionally on the EXIT ----
pub fn pick<T: Compare + Copy>(a: T, b: T) -> u8
where
    T::Truth: Branch,
{
    if a.eq(b).is_true() {
        1
    } else {
        0
    }
}

// ---- combining happens in the algebra, before any exit ----
pub fn both_eq<T: Compare + Copy>(a: T, b: T, c: T, d: T) -> T::Truth {
    a.eq(b).and(c.eq(d))
}

pub fn use_scalar(a: Scalar, b: Scalar) -> u8 {
    pick(a, b)
}
pub fn combine_vec(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> Mask2 {
    both_eq(a, b, c, d)
}
// the named reduction is the caller's own word, at the caller's site
pub fn vec_any_eq(a: Vec2, b: Vec2) -> bool {
    a.eq(b).any()
}
pub fn vec_all_eq(a: Vec2, b: Vec2) -> bool {
    a.eq(b).all()
}
