// `p1` arms G1-G3 established that a const assertion refuses only where the
// constant is forced: `PlanC::<8, 300>` can be returned, aliased and held in a
// field, and nothing complains. So the const-assertion route is a landmine
// rather than a refusal, and the one spelling that would refuse at the
// definition site needs `generic_const_exprs`, which is forbidden here.
//
// This probe attacks that wall rather than reporting it. Can "the capacity fits
// the id width" be a *definition-site* refusal, gate-free, on the pinned
// toolchain?
//
// THE MOVE. Const comparison of two const parameters is what rustc refuses.
// Trait resolution over types is not. So carry the capacity as a **type** in
// binary rather than as a `usize` const, and make "fits in W bits" a structural
// relation the trait solver decides. No const arithmetic appears anywhere.
//
// Binary naturals, most-significant-bit last, so `Fits` recurses on structure:
//   `E`            zero, the empty bit string
//   `B0<T>`, `B1<T>`   T with a 0 or 1 bit appended
// and `W` is spent one bit per recursion step.
//
// ARMS, and three of the five must be refused or the relation is vacuous:
//   H1 capacity 200, id width 8   MUST COMPILE   (200 needs 8 bits)
//   H2 capacity 300, id width 8   MUST BE REFUSED
//   H3 capacity 300, id width 9   MUST COMPILE   (300 needs 9 bits)
//   H4 H2's bad type merely named in a field, never constructed
//                                 MUST BE REFUSED  <- this is the whole point,
//                                 the case `p1` arms G1-G3 failed
//   H5 H2's bad type behind a type alias
//                                 MUST BE REFUSED  <- same, the other spelling
//
// CONTROL. H1 and H3 are the positive controls: if the relation refused
// everything the H2/H4/H5 refusals would mean nothing. H3 specifically must
// compile at one more bit of width than H2, so the relation is measuring the
// width rather than refusing large capacities generally.
#![allow(dead_code)]

use core::marker::PhantomData;

// --- type-level binary naturals ---------------------------------------------

pub struct E; // empty, value 0
pub struct B0<T>(PhantomData<T>);
pub struct B1<T>(PhantomData<T>);

// --- peano widths, small enough that the depth is not a concern -------------

pub struct Z;
pub struct S<T>(PhantomData<T>);

// --- the relation: N is representable in W bits -----------------------------
//
// Sealed so a consumer cannot assert it about a shape that does not have it,
// which is the failure `a_law_stated_as_an_author_written_marker_is_checked_by
// _nothing` measured on a different subject.
mod sealed {
    pub trait Sealed {}
}

pub trait FitsIn<W>: sealed::Sealed {}

// Zero fits in any width, including none.
impl sealed::Sealed for E {}
impl<W> FitsIn<W> for E {}

// A bit costs one width. There is deliberately no impl at `W = Z` for a
// non-empty string, and that missing impl is the refusal.
impl<T> sealed::Sealed for B0<T> {}
impl<T> sealed::Sealed for B1<T> {}
impl<T: FitsIn<W>, W> FitsIn<S<W>> for B0<T> {}
impl<T: FitsIn<W>, W> FitsIn<S<W>> for B1<T> {}

// --- widths and capacities, spelled out --------------------------------------

pub type W8 = S<S<S<S<S<S<S<S<Z>>>>>>>>;
pub type W9 = S<W8>;

// 200 = 0b11001000, lsb first: 0,0,0,1,0,0,1,1  -> eight bits
pub type C200 = B0<B0<B0<B1<B0<B0<B1<B1<E>>>>>>>>;
// 300 = 0b100101100, lsb first: 0,0,1,1,0,1,0,0,1 -> nine bits
pub type C300 = B0<B0<B1<B1<B0<B1<B0<B0<B1<E>>>>>>>>>;

// --- the gated structure. The bound is on the *definition*, so naming the type
// at all is what triggers resolution.
pub struct Plan<N: FitsIn<W>, W>(PhantomData<(N, W)>);

// --- arms --------------------------------------------------------------------

#[cfg(arm_h1)]
pub type H1 = Plan<C200, W8>;

#[cfg(arm_h2)]
pub fn h2() -> Plan<C300, W8> {
    Plan(PhantomData)
}

#[cfg(arm_h3)]
pub type H3 = Plan<C300, W9>;

#[cfg(arm_h4)]
pub struct HoldsBad {
    inner: Plan<C300, W8>,
}

#[cfg(arm_h5)]
pub type H5 = Plan<C300, W8>;

fn main() {
    println!("compiled");
}

// H5 compiled. Rust does not check trait bounds written on a `type` alias at
// all: that is the `type_alias_bounds` behaviour, general to aliases, and not a
// property of this relation. The question that decides whether it is a real
// hole or a deferral is whether the alias can be *used* without the refusal
// arriving. H6 and H7 ask it.
#[cfg(arm_h6)]
pub type H5Alias = Plan<C300, W8>;
#[cfg(arm_h6)]
pub struct UsesAlias {
    inner: H5Alias,
}

#[cfg(arm_h7)]
pub type H7Alias = Plan<C300, W8>;
#[cfg(arm_h7)]
pub fn h7() -> H7Alias {
    Plan(PhantomData)
}
