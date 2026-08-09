//! Probe 3. Probe 2 showed `min_generic_const_args` gives `type const` and a
//! path in array-length position, and refuses the INDUCTIVE STEP that computes
//! a type-level numeral's value (`P::VAL + 1`, needing `generic_const_args`).
//!
//! So the question sharpens. Is the pairing forced by the language, or forced by
//! the decision to make a capacity a type-level numeral whose value is computed
//! by recursion?
//!
//! Claims:
//!   A. a capacity whose value is a CONST PARAMETER, not an inductive numeral,
//!      needs no pair: `[T; <Self as Nat>::VAL]` resolves from `const N`.
//!   B. under A the disagreement is unnameable, at every rank, by construction.
//!   C. what A costs: the tower's inductive machinery is not inherited, so the
//!      ordering/Cmp/Gcd `79:91-110` buys by `Capacity: Nat` has to come from
//!      somewhere else.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]
#![feature(min_generic_const_args)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

pub trait Nat {
    type const VAL: usize;
}

pub trait Capacity: Nat {
    type Array<T: Copy>: Copy;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
}

// CLAIM A. One parameter. The array length and the numeric value are the same
// const, read twice, so there is no second name.
pub struct Dim<const N: usize>;

impl<const N: usize> Nat for Dim<N> {
    type const VAL: usize = N; // a PATH, not an expression
}

impl<const N: usize> Capacity for Dim<N> {
    type Array<T: Copy> = [T; <Self as Nat>::VAL];
    fn filled<T: Copy>(v: T) -> Self::Array<T> {
        [v; N]
    }
}

// CLAIM B. Composition at rank 3, with no agreement check anywhere in this
// module, and no way to write a disagreeing axis.
pub struct Scalar;
pub struct Axis<Hd, Tl>(PhantomData<(Hd, Tl)>);

pub trait Shape {
    const RANK: usize;
    const COUNT: usize;
    type Store<T: Copy>: Copy;
}
impl Shape for Scalar {
    const RANK: usize = 0;
    const COUNT: usize = 1;
    type Store<T: Copy> = T;
}
impl<Hd: Capacity, Tl: Shape> Shape for Axis<Hd, Tl> {
    const RANK: usize = 1 + Tl::RANK;
    const COUNT: usize = <Hd as Nat>::VAL * Tl::COUNT;
    type Store<T: Copy> = <Hd as Capacity>::Array<Tl::Store<T>>;
}

// File 100's own rank-3 shape, middle axis 4 rather than 3, so the shape is
// not accidentally cubic and a transposed count would be caught.
pub type Rank3 = Axis<Dim<3>, Axis<Dim<4>, Axis<Dim<5>, Scalar>>>;

pub const R3_RANK: usize = <Rank3 as Shape>::RANK;
pub const R3_COUNT: usize = <Rank3 as Shape>::COUNT;
pub const R3_SIZE: usize = core::mem::size_of::<<Rank3 as Shape>::Store<u8>>();

const _: () = assert!(R3_RANK == 3);
const _: () = assert!(R3_COUNT == 60);
// The agreement, holding through the TRAIT route, with no `AGREES`, no inline
// const block, and no construction door anywhere in the program.
const _: () = assert!(R3_COUNT == R3_SIZE);

// And the honest scope check file 100 named: a bare const READ, no construction.
pub const BARE_READ: usize = <Axis<Dim<7>, Scalar> as Shape>::COUNT;
const _: () = assert!(BARE_READ == 7);
const _: () = assert!(core::mem::size_of::<<Axis<Dim<7>, Scalar> as Shape>::Store<u8>>() == 7);
