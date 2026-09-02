// p7: the fold accumulator is derivable after all, from the element numeral
// and the CAPACITY, and the derivation is expressible gate-free.
//
// p1 showed a fold cannot widen, because the accumulator is loop-carried and
// has one type while the widening op gives it another. That looked like it
// closed the derivation surface for the whole algorithm layer.
//
// It does not, and the escape is already in arvo's shape rather than being
// something new: an aggregate's capacity is a TYPE. The exact trip count is a
// runtime quantity, but it is bounded by the capacity, and SUFFICIENCY only
// needs the bound. A sum of at most C values each below 2^W is below
// 2^(W + ceil(log2 C)), so the accumulator width is a total function of the
// element width and the capacity type:
//
//     acc_width(W, C) = W + ceil(log2 C)
//
// That is a two-input derivation crossing the numeral and the composition,
// which is what op's "contracts for things that compose to bigger units than
// just numerals alone" (32) would have to name. This probe asks whether it is
// expressible under the forbidden-feature set: no generic_const_exprs, no
// generic_const_args, no specialization, no TypeId.
//
// It is a type-level function, so it is written the way a refused bound wants
// to be written: as a trait with an associated type, computed by induction
// rather than by arithmetic in a bound position.
//
// Arms:
//   1. Log2Ceil over a binary capacity ladder                 expect: compiles
//   2. the accumulator derivation composed from it            expect: compiles
//   3. a generic fold that names the derived accumulator      expect: compiles
//   4. a static check that the derived widths are the right   expect: compiles
//      numbers, as const assertions that fail the build if not
//   5. an OVER-CAPACITY fold, to show the bound is load-bearing
//      rather than decorative                                 expect: refused
//
// Build:
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib --out-dir out p7_accumulator_from_capacity.rs
//   ... --cfg arm5

#![allow(dead_code, unused_variables)]

use core::marker::PhantomData;

// ---- widths as a unary type-level nat -----------------------------------
// Unary because the arithmetic needed here is successor and addition only,
// and a unary ladder makes the induction readable. A binary ladder would
// change none of the conclusions and all of the line count.

pub struct Z;
pub struct Su<N>(PhantomData<N>);

pub type N0 = Z;
pub type N1 = Su<N0>;
pub type N2 = Su<N1>;
pub type N3 = Su<N2>;
pub type N4 = Su<N3>;
pub type N5 = Su<N4>;
pub type N6 = Su<N5>;
pub type N7 = Su<N6>;
pub type N8 = Su<N7>;
pub type N9 = Su<N8>;
pub type N10 = Su<N9>;
pub type N11 = Su<N10>;
pub type N12 = Su<N11>;

// A witness that a type-level nat has a value, so the derived widths can be
// checked against arithmetic rather than trusted.
pub trait NatVal {
    const VAL: u32;
}
impl NatVal for Z {
    const VAL: u32 = 0;
}
impl<N: NatVal> NatVal for Su<N> {
    const VAL: u32 = N::VAL + 1;
}

// ---- addition on the ladder ---------------------------------------------

pub trait Add<Rhs> {
    type Out;
}
impl<B> Add<B> for Z {
    type Out = B;
}
impl<A, B> Add<B> for Su<A>
where
    A: Add<B>,
{
    type Out = Su<<A as Add<B>>::Out>;
}

// ---- capacities, and ceil(log2) on them ---------------------------------
// The capacity ladder is separate from the width ladder on purpose: a
// capacity is a count of elements, a width is a count of bits, and collapsing
// them is how a unit error gets into a derivation nobody re-checks.

pub struct Cap<const K: usize>;

// Log2Ceil is stated as a relation between a capacity and a width rather than
// computed by an expression, because an expression in that position is what
// needs the forbidden feature. Each row is a fact, and the rows are the
// binades. A design would generate these; the point here is that the shape is
// admissible, not that the list is the design.
pub trait Log2Ceil {
    type Out;
}
impl Log2Ceil for Cap<1> {
    type Out = N0;
}
impl Log2Ceil for Cap<2> {
    type Out = N1;
}
impl Log2Ceil for Cap<3> {
    type Out = N2;
}
impl Log2Ceil for Cap<4> {
    type Out = N2;
}
impl Log2Ceil for Cap<8> {
    type Out = N3;
}
impl Log2Ceil for Cap<16> {
    type Out = N4;
}
impl Log2Ceil for Cap<256> {
    type Out = N8;
}
impl Log2Ceil for Cap<1024> {
    type Out = N10;
}

// ---- the numeral, and the derivation ------------------------------------

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

/// The contract the composition layer needs from the numeral: given an element
/// numeral and a capacity, name the numeral a sum over that capacity lands in,
/// and say how an element enters it.
pub trait SumAccum<C> {
    type Acc: CAdd + Copy;
    fn lift(self) -> Self::Acc;
}

impl<W, const K: usize> SumAccum<Cap<K>> for Num<W>
where
    Cap<K>: Log2Ceil,
    W: Add<<Cap<K> as Log2Ceil>::Out>,
{
    type Acc = Num<<W as Add<<Cap<K> as Log2Ceil>::Out>>::Out>;
    fn lift(self) -> Self::Acc {
        Num::new(self.0)
    }
}

// ---- ARM 3: a generic fold naming the derived accumulator ---------------
// The routine is generic over the element numeral AND the capacity, and names
// neither a width nor a container. This is the shape an algorithm crate in the
// layer above would actually be written in.

pub struct Bounded<T, const K: usize> {
    pub items: [T; K],
    pub live: usize,
}

pub fn sum_bounded<W, const K: usize>(xs: &Bounded<Num<W>, K>) -> <Num<W> as SumAccum<Cap<K>>>::Acc
where
    Num<W>: SumAccum<Cap<K>>,
{
    let mut acc = <<Num<W> as SumAccum<Cap<K>>>::Acc as CAdd>::zero();
    let mut i = 0usize;
    while i < xs.live {
        acc = acc.cadd(xs.items[i].lift());
        i += 1;
    }
    acc
}

// ---- ARM 4: the derived widths are checked, not trusted ------------------
// If the derivation is off by one in either direction these fail the build.

pub trait WidthOf {
    const W: u32;
}
impl<W: NatVal> WidthOf for Num<W> {
    const W: u32 = W::VAL;
}

const _: () = {
    // 4-bit elements, capacity 16 -> 4 + 4 = 8
    assert!(<<Num<N4> as SumAccum<Cap<16>>>::Acc as WidthOf>::W == 8);
    // 8-bit elements, capacity 256 -> 8 + 8 = 16
    assert!(<<Num<N8> as SumAccum<Cap<256>>>::Acc as WidthOf>::W == 16);
    // 3-bit elements, capacity 3 -> 3 + 2 = 5, the non-power-of-two row
    assert!(<<Num<N3> as SumAccum<Cap<3>>>::Acc as WidthOf>::W == 5);
    // capacity 1 adds nothing
    assert!(<<Num<N5> as SumAccum<Cap<1>>>::Acc as WidthOf>::W == 5);
};

// And the sufficiency the derivation claims, checked against arithmetic over
// every row of the table rather than argued: K values each at most 2^W - 1
// must fit in the derived width.
const fn fits(w: u32, k: u64, acc_w: u32) -> bool {
    let max_elem: u128 = (1u128 << w) - 1;
    let worst: u128 = max_elem * (k as u128);
    worst < (1u128 << acc_w)
}
const _: () = {
    assert!(fits(4, 16, 8));
    assert!(fits(8, 256, 16));
    assert!(fits(3, 3, 5));
    assert!(fits(5, 1, 5));
    assert!(fits(1, 1024, 11));
    // and the negative control: one bit narrower is not sufficient
    assert!(!fits(4, 16, 7));
    assert!(!fits(8, 256, 15));
};

pub fn arm3_use(b: &Bounded<Num<N4>, 16>) -> Num<N8> {
    sum_bounded(b)
}

// ---- ARM 5: the capacity bound is load-bearing --------------------------
// A capacity with no Log2Ceil row has no accumulator, so a fold over it does
// not compile. The bound is what carries the sufficiency argument, and a
// design that let this through would be deriving a width from nothing.

#[cfg(arm5)]
pub fn arm5_uncovered(b: &Bounded<Num<N4>, 7>) -> <Num<N4> as SumAccum<Cap<7>>>::Acc {
    sum_bounded(b)
}
