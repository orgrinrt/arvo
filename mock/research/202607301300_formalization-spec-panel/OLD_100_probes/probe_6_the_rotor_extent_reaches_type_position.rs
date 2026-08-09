//! Probe 6. D10 flags the rotor's component count as a const-expression hazard
//! in type position. Under the capacity `91` section 1.26 settled, it is not a
//! hazard, and the derivation is the same structural recursion the design
//! already uses for `Dec`, `Cmp` and `Gcd`.
//!
//! D10 (`202607281220`, op): "A rotor's 1 + n(n-1)/2 components is a computed
//! extent, and putting that in type position is the const-expression hazard
//! `Capacity` exists to avoid." The stated fallback is "the hlist itself, since
//! a bivector basis is the set of 2-subsets of the axes and so is derivable by
//! a type-level fold rather than by arithmetic in type position". The prior-art
//! pass reports no library derives a basis that way (`202607281220`, addendum:
//! "No library was found that derives the bivector basis as a compile-time
//! type-level fold over an axis list").
//!
//! Two things this probe establishes. First, that the fold works, so the
//! absence in the literature is an absence rather than an impossibility.
//! Second, and this is the correction, that the extent it should derive is NOT
//! the bivector count: probe 4 shows a general rotor occupies the whole even
//! subalgebra, 2^(n-1) components, and exhibits a 4D rotor with a nonzero
//! grade-4 part that 1 + n(n-1)/2 slots cannot hold.
#![no_std]

use core::marker::PhantomData;

mod seal {
    pub trait Sealed {}
}

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
pub struct Z;
pub struct Pz<P>(PhantomData<P>);

impl seal::Sealed for H {}
impl<P: Pos> seal::Sealed for O<P> {}
impl<P: Pos> seal::Sealed for I<P> {}
impl seal::Sealed for Z {}
impl<P: Pos> seal::Sealed for Pz<P> {}

pub trait Pos: seal::Sealed {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

pub struct Scalar;
pub struct Axis<Hd, Tl>(PhantomData<(Hd, Tl)>);

pub trait Shape {
    const RANK: usize;
}
impl Shape for Scalar {
    const RANK: usize = 0;
}
impl<Hd, Tl: Shape> Shape for Axis<Hd, Tl> {
    const RANK: usize = 1 + Tl::RANK;
}

// ---------------------------------------------------------------------------
// CLAIM A. `2^rank` in TYPE position, by structural recursion on the list's own
// constructors. Not one arithmetic expression appears in a type; each step is
// impl selection, which is the family `91:811-814` names (`Dec`/`PosPred`
// recursing "structurally on the constructor shape ... the identical family as
// `VAL`, `Cmp`, and `Gcd`").
// ---------------------------------------------------------------------------

pub trait TwoPow {
    /// `2^RANK`, as a member of the sealed tower, not as a usize.
    type P: Pos;
}
impl TwoPow for Scalar {
    type P = H; // 2^0 = 1
}
impl<Hd, Tl: TwoPow> TwoPow for Axis<Hd, Tl> {
    type P = O<Tl::P>; // doubling is one constructor, not a multiply
}

/// The even subalgebra of Cl(n) has dimension 2^(n-1) for n >= 1, and 1 for
/// n = 0. Reading the TAIL's `TwoPow` gives 2^(n-1) directly, with no
/// subtraction anywhere: the list's own shape does the decrement.
pub trait RotorExtent {
    type P: Pos;
}
impl RotorExtent for Scalar {
    type P = H; // Cl(0): the scalars, dimension 1
}
impl<Hd, Tl: TwoPow> RotorExtent for Axis<Hd, Tl> {
    type P = Tl::P; // 2^(n-1)
}

pub type R2 = Axis<(), Axis<(), Scalar>>;
pub type R3 = Axis<(), Axis<(), Axis<(), Scalar>>>;
pub type R4 = Axis<(), Axis<(), Axis<(), Axis<(), Scalar>>>>;
pub type R5 = Axis<(), Axis<(), Axis<(), Axis<(), Axis<(), Scalar>>>>>;
pub type R8 =
    Axis<(), Axis<(), Axis<(), Axis<(), Axis<(), Axis<(), Axis<(), Axis<(), Scalar>>>>>>>>;

const _: () = {
    assert!(<R2 as Shape>::RANK == 2 && <<R2 as RotorExtent>::P as Pos>::VAL == 2);
    assert!(<R3 as Shape>::RANK == 3 && <<R3 as RotorExtent>::P as Pos>::VAL == 4);
    assert!(<R4 as Shape>::RANK == 4 && <<R4 as RotorExtent>::P as Pos>::VAL == 8);
    assert!(<R5 as Shape>::RANK == 5 && <<R5 as RotorExtent>::P as Pos>::VAL == 16);
    assert!(<R8 as Shape>::RANK == 8 && <<R8 as RotorExtent>::P as Pos>::VAL == 128);
};

// ---------------------------------------------------------------------------
// CLAIM B. The count D10 states, 1 + n(n-1)/2, derived the same way, so the two
// can be compared inside one compilation rather than by argument. Also
// structural: `Tri` folds the triangular number, `Bivec` adds one.
// ---------------------------------------------------------------------------

pub trait Rank {
    const N: usize;
}
impl Rank for Scalar {
    const N: usize = 0;
}
impl<Hd, Tl: Rank> Rank for Axis<Hd, Tl> {
    const N: usize = 1 + Tl::N;
}

pub trait D10Count {
    /// The count D10 states. In VALUE position, because unlike `2^n` it is not
    /// one constructor per step and the tower has no multiplication that is
    /// pure impl selection.
    const C: usize;
}
impl<S: Rank> D10Count for S {
    const C: usize = 1 + S::N * (S::N - 1) / 2;
}

const _: () = {
    // The two agree at rank 2 and 3 and separate from rank 4 on. This is the
    // separation the review's own requirement asks for: a model checked only
    // at rank 3, which is the rank everyone reaches for, cannot tell the two
    // counts apart at all.
    assert!(<R2 as D10Count>::C == 2 && <<R2 as RotorExtent>::P as Pos>::VAL == 2);
    assert!(<R3 as D10Count>::C == 4 && <<R3 as RotorExtent>::P as Pos>::VAL == 4);
    assert!(<R4 as D10Count>::C == 7 && <<R4 as RotorExtent>::P as Pos>::VAL == 8);
    assert!(<R5 as D10Count>::C == 11 && <<R5 as RotorExtent>::P as Pos>::VAL == 16);
    assert!(<R8 as D10Count>::C == 29 && <<R8 as RotorExtent>::P as Pos>::VAL == 128);
};

// ---------------------------------------------------------------------------
// CLAIM C. The derived extent pairs with a literal exactly as section 1.26
// requires, so the rotor's storage is an ordinary rank-1 shape and the
// agreement check catches a wrong literal. Nothing rotor-specific is added to
// the capacity machinery.
// ---------------------------------------------------------------------------

pub trait Capacity: Nat {
    type Array<T: Copy>: AsRef<[T]> + AsMut<[T]> + Copy;
    const AGREES: bool;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
}

pub struct Slot<N, const K: usize>(PhantomData<N>);
impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> {
    const VAL: usize = N::VAL;
}
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T: Copy> = [T; K];
    const AGREES: bool = {
        assert!(
            N::VAL == K,
            "capacity's declared length disagrees with its value"
        );
        true
    };
    fn filled<T: Copy>(v: T) -> [T; K] {
        const { assert!(<Self as Capacity>::AGREES) };
        [v; K]
    }
}

/// A rotor over the axis list `S`, stored in `K` slots. `K` is the one
/// language-forced literal, and the `Nat` beside it is DERIVED from `S`, so a
/// wrong `K` is a compile error rather than a silent miscount.
pub type RotorSlot<S, const K: usize> = Slot<Pz<<S as RotorExtent>::P>, K>;

pub fn rotor3() -> [i32; 4] {
    <RotorSlot<R3, 4> as Capacity>::filled(0)
}
pub fn rotor4() -> [i32; 8] {
    <RotorSlot<R4, 8> as Capacity>::filled(0)
}

// The wrong literal, which is exactly D10's count at rank 4, is refused:
// pub fn rotor4_d10() -> [i32; 7] { <RotorSlot<R4, 7> as Capacity>::filled(0) }  // E0080

const _: () = {
    assert!(<RotorSlot<R3, 4> as Nat>::VAL == 4);
    assert!(<RotorSlot<R4, 8> as Nat>::VAL == 8);
};

pub fn exercise() -> (usize, usize) {
    (rotor3().len(), rotor4().len())
}
