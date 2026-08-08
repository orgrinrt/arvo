// p1: can a fold use a widening operation?
//
// The panel's whole width algebra is a WIDENING binary operation: two operands
// of declared widths produce a result of a derived, larger width. Every
// algorithm in the layer above is a fold. This probe asks whether the two
// compose, and it asks in the only way that settles it: by compiling.
//
// Five arms.
//   A. widening op in a fixed-arity EXPRESSION             expect: compiles
//   B. widening op in a LOOP over a runtime-length slice    expect: refused
//   C. widening op folded over a STATIC-length hlist        expect: compiles
//   D. closed op in a loop over a runtime-length slice      expect: compiles
//   E. closed op into a separately named wider accumulator  expect: compiles
//
// Arm B is the finding. A, C, D and E are the negative controls that locate
// it: without them "widening does not work" would be too strong and the real
// boundary (a static trip count, or a closed operation) would be invisible.
//
// Build with the pinned toolchain:
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p1_fold_cannot_widen.rs
//   rustc ... --cfg arm_b1   (and arm_b2, arm_b3, arm_b4)
//
// FIRST-PASS DEFECT, kept because it is instructive. The first version of this
// file defined WAdd only for equal widths (Num<W> + Num<W>), which made arm C
// fail with a trait-bound error that said nothing about folds. That was the
// probe's shortcut, not the design's: a right fold produces mismatched widths
// at every step, so the derivation has to be heterogeneous. The type-level Max
// below is the repair. A probe whose setup cannot express the thing under test
// reports a fact about itself.

#![allow(dead_code, unused_variables)]

use core::marker::PhantomData;

// ---- a type-level width ladder, gate-free -------------------------------
// Deliberately not arvo's. The claim is about folds, not about arvo's
// encoding, so the encoding is the simplest thing that carries a width.

pub struct W0;
pub struct S<N>(PhantomData<N>);

pub type W1 = S<W0>;
pub type W2 = S<W1>;
pub type W3 = S<W2>;
pub type W4 = S<W3>;

// ---- type-level max, so the widening rule can be heterogeneous ----------

pub trait Max<Rhs> {
    type Out;
}
impl Max<W0> for W0 {
    type Out = W0;
}
impl<N> Max<S<N>> for W0 {
    type Out = S<N>;
}
impl<N> Max<W0> for S<N> {
    type Out = S<N>;
}
impl<A, B> Max<S<B>> for S<A>
where
    A: Max<B>,
{
    type Out = S<<A as Max<B>>::Out>;
}

// ---- a width-tagged datum ------------------------------------------------

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

// ---- the widening operation ----------------------------------------------
// Adding an A-bit and a B-bit value needs max(A,B)+1 bits. This is the panel's
// derivation surface in miniature: the result numeral is a total function of
// the operand numerals and is strictly wider than either.

pub trait WAdd<Rhs> {
    type Output;
    fn wadd(self, rhs: Rhs) -> Self::Output;
}

impl<A, B> WAdd<Num<B>> for Num<A>
where
    A: Max<B>,
{
    type Output = Num<S<<A as Max<B>>::Out>>;
    fn wadd(self, rhs: Num<B>) -> Self::Output {
        Num::new(self.0 + rhs.0)
    }
}

// ---- the closed operation ------------------------------------------------
// Same width in, same width out. What every algorithm crate's bound asks for.

pub trait CAdd {
    fn cadd(self, rhs: Self) -> Self;
}

impl<W> CAdd for Num<W> {
    fn cadd(self, rhs: Self) -> Self {
        Num::new(self.0.wrapping_add(rhs.0))
    }
}

// ==== ARM A: widening in a fixed-arity expression ==========================
// Compiles. The result type is named by composing the derivation twice, and
// the annotations are written out so that a wrong derivation fails the build.
// This is the arm the panel has been designing for.

pub fn arm_a(a: Num<W1>, b: Num<W1>, c: Num<W2>) -> Num<W4> {
    let ab: Num<W2> = a.wadd(b); // max(1,1)+1 = 2
    let abc: Num<W3> = ab.wadd(c); // max(2,2)+1 = 3
    let d: Num<W3> = Num::new(0);
    let out: Num<W4> = abc.wadd(d); // max(3,3)+1 = 4
    out
}

// ==== ARM B: widening in a loop over a runtime-length slice ================
// The claim. Each arm is a different formulation of the same attempt, so that
// "it was written badly" is not an available explanation.

// B1: the obvious loop. The accumulator is loop-carried, so it has exactly one
// type, and the widening op gives it a different one on every iteration.
#[cfg(arm_b1)]
pub fn arm_b1<W>(xs: &[Num<W>]) -> Num<W>
where
    W: Max<W>,
{
    let mut acc: Num<W> = Num::new(0);
    for x in xs {
        acc = acc.wadd(*x);
    }
    acc
}

// B2: name the output honestly, as the once-widened type.
#[cfg(arm_b2)]
pub fn arm_b2<W>(xs: &[Num<W>]) -> <Num<W> as WAdd<Num<W>>>::Output
where
    W: Max<W>,
{
    let mut acc: Num<W> = Num::new(0);
    for x in xs {
        acc = acc.wadd(*x);
    }
    acc
}

// B3: give up on naming it and let inference try.
#[cfg(arm_b3)]
pub fn arm_b3<W>(xs: &[Num<W>]) -> impl Sized
where
    W: Max<W>,
{
    let mut acc = Num::<W>::new(0);
    for x in xs {
        acc = acc.wadd(*x);
    }
    acc
}

// B4: recursion instead of a loop, in case the loop is what refuses.
#[cfg(arm_b4)]
pub fn arm_b4<W>(xs: &[Num<W>]) -> Num<W>
where
    W: Max<W>,
{
    match xs.split_first() {
        None => Num::new(0),
        Some((h, t)) => (*h).wadd(arm_b4(t)),
    }
}

// ==== ARM C: widening folded over a static-length hlist ===================
// Compiles. The trip count is a compile-time fact, so the result type is a
// compile-time fact, and the derivation composes exactly as in arm A. This is
// the boundary: it is the runtime trip count that refuses, not the widening.

pub struct Nil;
pub struct Cons<H, T>(pub H, pub T);

pub trait WFold {
    type Output;
    fn wfold(self) -> Self::Output;
}

impl<W> WFold for Cons<Num<W>, Nil> {
    type Output = Num<W>;
    fn wfold(self) -> Num<W> {
        self.0
    }
}

impl<W, H, T> WFold for Cons<Num<W>, Cons<H, T>>
where
    Cons<H, T>: WFold,
    Num<W>: WAdd<<Cons<H, T> as WFold>::Output>,
{
    type Output = <Num<W> as WAdd<<Cons<H, T> as WFold>::Output>>::Output;
    fn wfold(self) -> Self::Output {
        self.0.wadd(self.1.wfold())
    }
}

// Three W1 values, right-folded: W1 + (W1 + W1) = W1 + W2 = W3. The return
// type is written out, so a wrong derivation fails the build rather than
// passing quietly.
pub fn arm_c() -> Num<W3> {
    let l = Cons(
        Num::<W1>::new(1),
        Cons(Num::<W1>::new(2), Cons(Num::<W1>::new(3), Nil)),
    );
    l.wfold()
}

// Four values, to show the width tracks the trip count rather than being
// pinned by the impl: W1 + (W1 + (W1 + W1)) = W1 + (W1 + W2) = W1 + W3 = W4.
pub fn arm_c4() -> Num<W4> {
    let l = Cons(
        Num::<W1>::new(1),
        Cons(
            Num::<W1>::new(2),
            Cons(Num::<W1>::new(3), Cons(Num::<W1>::new(4), Nil)),
        ),
    );
    l.wfold()
}

// ==== ARM D: closed op in a loop over a runtime-length slice ==============
// Compiles. This is the shape every algorithm crate's bound already asks for.

pub fn arm_d<W>(xs: &[Num<W>]) -> Num<W> {
    let mut acc: Num<W> = Num::new(0);
    for x in xs {
        acc = acc.cadd(*x);
    }
    acc
}

// ==== ARM E: the escape ====================================================
// A closed op INTO a separately named, wider accumulator. Compiles, and needs
// two types plus a conversion between them rather than one derivation. The
// accumulator width is chosen by whoever writes this signature, not derived
// from the element width.

pub trait WidenInto<A> {
    fn widen_into(self) -> A;
}

impl WidenInto<Num<W4>> for Num<W1> {
    fn widen_into(self) -> Num<W4> {
        Num::new(self.0)
    }
}

pub fn arm_e(xs: &[Num<W1>]) -> Num<W4> {
    let mut acc: Num<W4> = Num::new(0);
    for x in xs {
        let widened: Num<W4> = (*x).widen_into();
        acc = acc.cadd(widened);
    }
    acc
}
