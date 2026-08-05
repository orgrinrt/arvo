//! Probe 1. Does D4's shape (an hlist of capacities) survive the capacity that
//! consolidation nine actually settled?
//!
//! D4 (`202607281220`, op) declares rank and per-axis extents as a type-level
//! list of capacities, with "the backing storage is the recursive composition of
//! each capacity's array":
//!
//!     Nil::Array<E>        = E
//!     Cons<H, T>::Array<E> = H::Array<T::Array<E>>
//!
//! That was written on 2026-07-28, when a capacity was `Dim<const N: usize>`
//! with `type Array<T> = [T; N]` and one const generic doing both jobs. Since
//! then `91` section 1.26 split the job in two: the capacity's VALUE is a direct
//! `Nat` instance, and its array grammar is "a paired, non-derived fact, forced
//! by the language" carried by `Slot<N, const K: usize>` with an agreement check
//! at the one construction door.
//!
//! So the question this probe asks, which no file has asked: does the recursive
//! Array composition still compose when the capacity's length is a paired
//! literal rather than a derived one, and does the agreement check compose with
//! it or have to be restated per axis?
//!
//! Vocabulary reused verbatim from `79_probes/probe_1_capacity_is_a_nat.rs`
//! (which is the file `91` section 1.26 rests on), so that nothing here is a
//! fresh model of already-settled material.
#![no_std]

use core::marker::PhantomData;

mod seal {
    pub trait Sealed {}
}

// ---------------------------------------------------------------------------
// The settled tower, reproduced without amendment from 79_probes/probe_1.
// ---------------------------------------------------------------------------

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

pub trait Capacity: Nat {
    type Array<T: Copy>: AsRef<[T]> + AsMut<[T]> + Copy;
    const SIZE: usize = Self::VAL;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
}

pub struct Slot<N, const K: usize>(PhantomData<N>);
impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> {
    const VAL: usize = N::VAL;
}
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T: Copy> = [T; K];
    fn filled<T: Copy>(v: T) -> [T; K] {
        [v; K]
    }
}

pub const fn agrees<N: Nat, const K: usize>() -> bool {
    N::VAL == K
}

// Three model extents, each a Nat paired with its language-forced literal.
pub type E3 = Slot<Pz<I<H>>, 3>; // 3
pub type E4 = Slot<Pz<O<O<H>>>, 4>; // 4
pub type E5 = Slot<Pz<I<O<H>>>, 5>; // 5

const _: () = {
    assert!(<E3 as Nat>::VAL == 3);
    assert!(<E4 as Nat>::VAL == 4);
    assert!(<E5 as Nat>::VAL == 5);
};

// ---------------------------------------------------------------------------
// CLAIM A. The shape itself: rank and per-axis extents, and NOTHING else.
// D43 (`202607292100`, op): "The shape abstraction supplies rank and extent and
// nothing else." This trait is that sentence written down. Note what is absent:
// no `Array`, no storage, no element type. That absence is the probe's subject.
// ---------------------------------------------------------------------------

pub struct Scalar;
pub struct Axis<Hd, Tl>(PhantomData<(Hd, Tl)>);

pub trait Shape {
    /// Number of axes. The list's length, a fold.
    const RANK: usize;
    /// Product of the extents. Value position throughout: no extent product
    /// ever reaches type position, which is the constraint `Capacity` exists
    /// to enforce (`202607281126`, quoted at `inherited` and at 91:796-802).
    const COUNT: usize;
    /// Extent of axis `i`, counted from the outermost. Total by returning 0
    /// past the rank, which is a placeholder this probe does not defend; the
    /// deliverable argues it should be a declaration-site refusal.
    fn extent(i: usize) -> usize;
}

impl Shape for Scalar {
    const RANK: usize = 0;
    const COUNT: usize = 1; // the empty product, and rank 0 is a scalar (D7)
    fn extent(_: usize) -> usize {
        0
    }
}

impl<Hd: Capacity, Tl: Shape> Shape for Axis<Hd, Tl> {
    const RANK: usize = 1 + Tl::RANK;
    const COUNT: usize = <Hd as Nat>::VAL * Tl::COUNT;
    fn extent(i: usize) -> usize {
        if i == 0 {
            <Hd as Nat>::VAL
        } else {
            Tl::extent(i - 1)
        }
    }
}

pub type Vec3 = Axis<E3, Scalar>;
pub type Rect34 = Axis<E3, Axis<E4, Scalar>>;
pub type Vol345 = Axis<E3, Axis<E4, Axis<E5, Scalar>>>;

const _: () = {
    // CLAIM A: rank is the length, count is the product, extents differ per
    // axis. The rectangle `Matrix<W, C>` could not express (`202607281126`)
    // is expressible, in const position, with no feature gate.
    assert!(<Scalar as Shape>::RANK == 0 && <Scalar as Shape>::COUNT == 1);
    assert!(<Vec3 as Shape>::RANK == 1 && <Vec3 as Shape>::COUNT == 3);
    assert!(<Rect34 as Shape>::RANK == 2 && <Rect34 as Shape>::COUNT == 12);
    assert!(<Vol345 as Shape>::RANK == 3 && <Vol345 as Shape>::COUNT == 60);
};

// ---------------------------------------------------------------------------
// CLAIM B. The storage composition is a SEPARATE trait over the shape, and
// D4's recursion survives the `Slot` split unchanged.
// ---------------------------------------------------------------------------

pub trait Dense: Shape {
    type Store<E: Copy>: Copy;
    fn build<E: Copy>(v: E) -> Self::Store<E>;
    fn fold<E: Copy>(s: &Self::Store<E>, acc: &mut usize, f: fn(&mut usize, E));
}

impl Dense for Scalar {
    type Store<E: Copy> = E; // D4: Nil::Array<E> = E
    fn build<E: Copy>(v: E) -> E {
        v
    }
    fn fold<E: Copy>(s: &E, acc: &mut usize, f: fn(&mut usize, E)) {
        f(acc, *s)
    }
}

impl<Hd: Capacity, Tl: Dense> Dense for Axis<Hd, Tl> {
    // D4: Cons<H, T>::Array<E> = H::Array<T::Array<E>>, verbatim.
    type Store<E: Copy> = <Hd as Capacity>::Array<Tl::Store<E>>;
    fn build<E: Copy>(v: E) -> Self::Store<E> {
        Hd::filled(Tl::build(v))
    }
    fn fold<E: Copy>(s: &Self::Store<E>, acc: &mut usize, f: fn(&mut usize, E)) {
        for inner in s.as_ref() {
            Tl::fold(inner, acc, f);
        }
    }
}

// CLAIM B, checked by construction: these type aliases only exist if the
// recursive composition resolved through `Slot`'s paired literal at every axis.
pub type Store3 = <Vec3 as Dense>::Store<u32>; // [u32; 3]
pub type Store34 = <Rect34 as Dense>::Store<u32>; // [[u32; 4]; 3]
pub type Store345 = <Vol345 as Dense>::Store<u32>; // [[[u32; 5]; 4]; 3]

const _: () = {
    assert!(core::mem::size_of::<Store3>() == 3 * 4);
    assert!(core::mem::size_of::<Store34>() == 12 * 4);
    assert!(core::mem::size_of::<Store345>() == 60 * 4);
    // The storage size and the shape's own COUNT agree. They are computed by
    // two independent routes (the language's layout of a nested array, and a
    // fold over the Nat values), so agreeing is content rather than tautology.
    assert!(core::mem::size_of::<Store345>() == <Vol345 as Shape>::COUNT * 4);
};

// ---------------------------------------------------------------------------
// CLAIM C. A function generic over RANK. `202607281127` names this "the
// concrete missing piece ... with no worked example anywhere in arvo", and D4
// names it "the one thing actually missing". One body, every rank.
// ---------------------------------------------------------------------------

pub fn total<S: Dense>(v: u32) -> usize {
    let s = S::build(v);
    let mut acc = 0usize;
    S::fold(&s, &mut acc, |a, e| *a += e as usize);
    acc
}

/// A second rank-generic body that reads only the SHAPE, not the storage:
/// the row-major stride of axis `i`. Nothing per-rank, nothing per-domain.
pub fn stride<S: Shape>(i: usize) -> usize {
    let mut s = 1usize;
    let mut k = i + 1;
    while k < S::RANK {
        s *= S::extent(k);
        k += 1;
    }
    s
}

// ---------------------------------------------------------------------------
// CLAIM D. The agreement check does NOT have to be restated per axis. It is a
// property of one `Slot`, and a shape-level check is a fold over the axes that
// each capacity already discharges. Stated in const position so it is a
// declaration-site refusal rather than a runtime test.
// ---------------------------------------------------------------------------

pub trait Agreeing: Shape {
    const AGREES: bool;
}
impl Agreeing for Scalar {
    const AGREES: bool = true;
}
impl<N: Nat, const K: usize, Tl: Agreeing> Agreeing for Axis<Slot<N, K>, Tl> {
    const AGREES: bool = agrees::<N, K>() && Tl::AGREES;
}

const _: () = {
    assert!(<Vol345 as Agreeing>::AGREES);
};

/// A shape whose middle axis declares a `Nat` of 4 and a literal of 7. The
/// const block below is commented out because it is the REFUSAL being
/// demonstrated; uncommenting it is the compile-fail half of this claim.
pub type Lying = Axis<E3, Axis<Slot<Pz<O<O<H>>>, 7>, Scalar>>;
// const _: () = { assert!(<Lying as Agreeing>::AGREES) };  // fails: 4 != 7

const _: () = {
    // The lie is detectable in const position, at the shape level, from one
    // fold. Nothing per-axis was written to catch it.
    assert!(!<Lying as Agreeing>::AGREES);
    // ... and note what is NOT caught: the storage still typechecks and is
    // seven wide, because the literal is what the language reads.
    assert!(core::mem::size_of::<<Lying as Dense>::Store<u8>>() == 21);
    assert!(<Lying as Shape>::COUNT == 12);
};

// ---------------------------------------------------------------------------
// Exercise, so nothing above is dead and the trait solver actually runs it.
// ---------------------------------------------------------------------------

pub fn exercise() -> (usize, usize, usize, usize, usize, usize) {
    (
        total::<Vec3>(2),    // 3 * 2 = 6
        total::<Rect34>(2),  // 12 * 2 = 24
        total::<Vol345>(2),  // 60 * 2 = 120
        stride::<Vol345>(0), // 4 * 5 = 20
        stride::<Vol345>(1), // 5
        stride::<Vol345>(2), // 1
    )
}
