// p6: can a GENERIC algorithm crate name its accumulator?
//
// p1 established that a fold cannot use a widening operation, so an algorithm
// that needs headroom has to name a separate, wider accumulator type. At a
// concrete call site that is easy: the consumer writes the type. Inside a
// generic algorithm crate there is nobody to write it, because the element
// type is a parameter and the crate never sees a width.
//
// This matters for the width-surface question in OPTIONS.md Q9, where
// arrangement D is "declare the output width explicitly; check it is wide
// enough by a free type-level comparison". `13` names the tier-one generic
// surface as the thing D does not address and as the first thing it would
// attack next. This is that attack, from the algorithm layer's side.
//
// Five arms, each a different way for a generic routine to obtain an
// accumulator:
//
//   A1  associated type on a trait the numeral implements     expect: compiles
//   A2  extra generic parameter, bound by a trait RELATION     expect: compiles
//   A3  const-generic widths, comparison in a where clause     expect: refused
//   A4  const-generic widths, comparison as a post-mono const  expect: compiles,
//       assert in the body                                     with the error
//                                                              moved to use
//   A5c A4 instantiated too narrow (cfg a5c)                  expect: refused
//   A5r A2 instantiated too narrow (cfg a5r)                  expect: refused
//       at instantiation, not at definition. Split into two cfgs because a
//       type-check error in one aborts before the other, a post-monomorphisation
//       assert, can fire at all.
//
// A3 is the const-generic spelling of D's check. A2 is the trait spelling of
// the same intent. Which of them compiles at a generic definition site is the
// finding, and it is not a matter of taste.
//
// Build:
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib --out-dir out p6_generic_accumulator.rs
//   ... --cfg a3    ... --cfg a5

#![allow(dead_code, unused_variables)]

use core::marker::PhantomData;

// ---- type-level width ladder --------------------------------------------

pub struct W0;
pub struct S<N>(PhantomData<N>);
pub type W1 = S<W0>;
pub type W2 = S<W1>;
pub type W4 = S<S<W2>>;
pub type W8 = S<S<S<S<W4>>>>;

pub struct Num<W>(pub u64, PhantomData<W>);
impl<W> Num<W> {
    pub const fn new(v: u64) -> Self {
        Num(v, PhantomData)
    }
}
impl<W> Clone for Num<W> {
    fn clone(&self) -> Self {
        Num(self.0, PhantomData)
    }
}
impl<W> Copy for Num<W> {}

pub trait CAdd {
    fn cadd(self, rhs: Self) -> Self;
    fn zero() -> Self;
}
impl<W> CAdd for Num<W> {
    fn cadd(self, rhs: Self) -> Self {
        Num::new(self.0.wrapping_add(rhs.0))
    }
    fn zero() -> Self {
        Num::new(0)
    }
}

// ==== A1: the accumulator is an associated type ==========================
// The numeral declares what it accumulates into. A generic routine names
// `T::Acc` and never sees a width. Compiles, and the accumulator's width is a
// function of the element's width chosen by whoever writes the impl.

pub trait Accumulates: Sized {
    type Acc: CAdd + Copy;
    fn lift(self) -> Self::Acc;
}

impl Accumulates for Num<W1> {
    type Acc = Num<W8>;
    fn lift(self) -> Num<W8> {
        Num::new(self.0)
    }
}
impl Accumulates for Num<W2> {
    type Acc = Num<W8>;
    fn lift(self) -> Num<W8> {
        Num::new(self.0)
    }
}

pub fn a1_sum<T: Accumulates + Copy>(xs: &[T]) -> T::Acc {
    let mut acc = <T::Acc as CAdd>::zero();
    for &x in xs {
        acc = acc.cadd(x.lift());
    }
    acc
}

// ==== A2: the accumulator is a generic parameter, bound by a relation ====
// The caller picks it; a trait relation proves it is wide enough. The relation
// is a pair of impls, not an arithmetic comparison, which is the shape a
// refused bound wants.

pub trait AtLeast<Rhs> {}
impl<A> AtLeast<W0> for A {}
impl<A, B> AtLeast<S<B>> for S<A> where A: AtLeast<B> {}

pub trait LiftInto<A> {
    fn lift_into(self) -> A;
}
impl<WA, WB> LiftInto<Num<WB>> for Num<WA>
where
    WB: AtLeast<WA>,
{
    fn lift_into(self) -> Num<WB> {
        Num::new(self.0)
    }
}

pub fn a2_sum<WE, WA>(xs: &[Num<WE>]) -> Num<WA>
where
    WA: AtLeast<WE>,
{
    let mut acc: Num<WA> = Num::new(0);
    for &x in xs {
        let w: Num<WA> = x.lift_into();
        acc = acc.cadd(w);
    }
    acc
}

// The relation is checked: a narrower accumulator than the element must not
// satisfy it. Instantiating this is the negative control, under cfg a5 below.
pub fn a2_ok() -> Num<W8> {
    let xs = [Num::<W2>::new(1), Num::<W2>::new(2)];
    a2_sum::<W2, W8>(&xs)
}

// ==== A3: const-generic widths, comparison in a where clause =============
// The const-generic spelling of arrangement D's "check it is wide enough by a
// free type-level comparison", at a GENERIC definition site where the widths
// are parameters rather than literals.

#[cfg(a3)]
pub fn a3_sum<const WE: u32, const WA: u32>(xs: &[u64]) -> u64
where
    [(); (WA >= WE) as usize - 1]:,
{
    let mut acc = 0u64;
    for &x in xs {
        acc = acc.wrapping_add(x);
    }
    acc
}

// ==== A4: the same check as a post-monomorphisation const assertion ======
// Compiles at the definition site. The check still happens, but it happens at
// instantiation, so the definition carries no evidence that it is checked and
// the diagnostic arrives somewhere else.

pub fn a4_sum<const WE: u32, const WA: u32>(xs: &[u64]) -> u64 {
    const {
        assert!(WA >= WE, "accumulator narrower than element width");
    }
    let mut acc = 0u64;
    for &x in xs {
        acc = acc.wrapping_add(x);
    }
    acc
}

pub fn a4_ok(xs: &[u64]) -> u64 {
    a4_sum::<8, 32>(xs)
}

// ==== A5: A4 instantiated too narrow, and A2 instantiated too narrow =====
// Both must be refused. If either is accepted the check is decorative.

#[cfg(a5c)]
pub fn a5_bad_const(xs: &[u64]) -> u64 {
    a4_sum::<32, 8>(xs)
}

#[cfg(a5r)]
pub fn a5_bad_relation() -> Num<W1> {
    let xs = [Num::<W4>::new(1)];
    a2_sum::<W4, W1>(&xs)
}
