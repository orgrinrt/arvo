// p3b: ARM C2, split out of p3 because it cannot share a file with it.
//
// The question p3 arms B1, B2 and C1 leave open: a consumer needs a const `K`
// for `[T; K]` storage and the derivation needs a type `C` for the inductive
// ceil(log2). Can the agreement between them be enforced at the definition
// site, so `Both<T, 4, C256>` is refused rather than merely wrong?
//
// The natural spelling is associated-const equality. Two things are measured
// here and both are results.
//
//   C2a  the bound written with no feature attribute at all
//        REQUIRED: REFUSE, and the diagnostic must name what it wants
//   C2b  the same with `#![feature(associated_const_equality)]`
//        REQUIRED: REFUSE. If it compiles, the bridge exists behind one gate
//        and the finding is "gated" rather than "unavailable".
//
// Why this is a separate file: the bound is gated at PARSE time, so writing it
// under a `#[cfg]` that is off still fails the build of every other item in
// the file. That is the reason `191`'s p1 and `35`'s p7 could each stay
// gate-free while the composition of the two cannot be tested inside either.

#![allow(dead_code)]
#![cfg_attr(with_feature, feature(associated_const_equality))]

use core::marker::PhantomData;

pub struct One;
pub struct Twice<N>(PhantomData<N>);
pub trait PosVal {
    const VAL: u64;
}
impl PosVal for One {
    const VAL: u64 = 1;
}
impl<N: PosVal> PosVal for Twice<N> {
    const VAL: u64 = 2 * N::VAL;
}

pub struct Both<T, const K: usize, C> {
    pub items: [T; K],
    pub cap: PhantomData<C>,
}

pub fn c2_consistent<T, const K: usize, C>(items: [T; K]) -> Both<T, K, C>
where
    C: PosVal<VAL = { K as u64 }>,
{
    Both {
        items,
        cap: PhantomData,
    }
}
