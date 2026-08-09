//! Probe 1. The cost column of the const-parameter capacity, stated as a
//! compile rather than as a worry.
//!
//! `106:555-568` prices the const column and names its one real cost: "the
//! numeral tower needs type-level arithmetic producing types, and a const
//! parameter does not participate in it." That is asserted, not compiled, in
//! every file that states it. This probe compiles it at the smallest
//! capacity-producing operation the design actually has a consumer for.
//!
//! The consumer is not hypothetical. `102:904-913` records the bivector extent
//! `1 + n(n-1)/2` being derived by structural recursion from a rank, and the
//! shape chapter (`102:799-805`) composes per-axis extents. Both are operations
//! whose ARGUMENT is a capacity and whose RESULT is a capacity, which is
//! exactly the position a const parameter cannot occupy.
//!
//! Claims:
//!   A. reading a const-parameter capacity's value: WORKS (this is p3 of 106).
//!   B. producing a new capacity from two others: REFUSED, forbidden feature.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
//! Outside the tree rustup resolves to stable and `type const` is a parse
//! error, which is a different refusal about a different thing.
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

pub trait Nat {
    type const VAL: usize;
}

pub trait Capacity: Nat {
    type Array<T: Copy>: Copy;
}

pub struct Dim<const N: usize>;

impl<const N: usize> Nat for Dim<N> {
    type const VAL: usize = N; // a PATH: this is what 106's p3 established
}

impl<const N: usize> Capacity for Dim<N> {
    type Array<T: Copy> = [T; <Self as Nat>::VAL];
}

// CLAIM A. Reading works. Reproduced in one line so the refusal below cannot
// be mistaken for the whole construction failing.
pub const READ_13: usize = <Dim<13> as Nat>::VAL;
const _: () = assert!(READ_13 == 13);
const _: () = assert!(core::mem::size_of::<<Dim<13> as Capacity>::Array<u8>>() == 13);

// CLAIM B. A capacity-producing operation: concatenating two index domains of
// capacity A and B yields one of capacity A + B. The result must be a TYPE,
// because downstream code names it (`<Cat<A, B> as Capacity>::Array<T>`), and
// a type is what a const parameter cannot be computed into.
pub trait Concat<Rhs> {
    type Out: Capacity;
}

impl<const A: usize, const B: usize> Concat<Dim<B>> for Dim<A> {
    type Out = Dim<{ A + B }>;
}

pub type Cat<L, R> = <L as Concat<R>>::Out;
pub const CAT_3_4: usize = <Cat<Dim<3>, Dim<4>> as Nat>::VAL;
const _: () = assert!(CAT_3_4 == 7);
