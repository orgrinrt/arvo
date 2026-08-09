//! Probe 1. The `AGREES` defect exists because a capacity carries two names for
//! one number. Does it have to?
//!
//! File 100 found that `Slot<N: Nat, const K: usize>` with
//! `type Array<T> = [T; K]` can be declared with `N::VAL != K`, and that the
//! agreement check lives at one of two legal routes. File 105 reads that as an
//! instance of "a fact checked at one route among several".
//!
//! Before accepting that reading I want to know whether the fact needs checking
//! at all. A fact that is checked is a fact that could have been false. The pair
//! `(N, K)` is what makes it possible to be false. If the array length can be
//! taken from `N` directly, there is no pair, no disagreement, no check, and no
//! route question, because the illegal state is unrepresentable rather than
//! refused.
//!
//! The obstacle is supposed to be `generic_const_exprs`, which is FORBIDDEN in
//! this workspace (`unstable-features.md`, forbidden table). So the question is
//! narrow and compilable: does an associated-const path in array-length
//! position, `[T; <N as Nat>::VAL]`, need that feature?
//!
//! Claims:
//!   A. the pair, reproduced: `N::VAL != K` is declarable and the two disagree.
//!   B. the pair dissolved: `[T; <N as Nat>::VAL]` in an associated type.
//!   C. under B, the disagreement is not merely refused, it is unnameable.
#![no_std]
#![forbid(incomplete_features)]

use core::marker::PhantomData;

mod seal {
    pub trait Sealed {}
}

pub struct Z;
pub struct S<P>(PhantomData<P>);
impl seal::Sealed for Z {}
impl<P: Nat> seal::Sealed for S<P> {}

pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Nat> Nat for S<P> {
    const VAL: usize = P::VAL + 1;
}

pub type N3 = S<S<S<Z>>>;
pub type N7 = S<S<S<S<S<S<S<Z>>>>>>>;

// ---------------------------------------------------------------------------
// CLAIM A. The pair as shipped in file 100's model: two names, one number,
// nothing forcing them to agree.
// ---------------------------------------------------------------------------
pub mod paired {
    use super::*;

    pub trait Capacity: Nat {
        type Array<T: Copy>: Copy;
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

    // The lying capacity: Nat says 3, literal says 7. Both const-evaluable.
    pub type Lying = Slot<N3, 7>;
    pub const LYING_NAT: usize = <Lying as Nat>::VAL;
    pub const LYING_LEN: usize = core::mem::size_of::<<Lying as Capacity>::Array<u8>>();
}

// ---------------------------------------------------------------------------
// CLAIM B. The pair dissolved. The array length is READ FROM the Nat, so there
// is no second name to disagree with the first.
// ---------------------------------------------------------------------------
pub mod unpaired {
    use super::*;

    pub trait Capacity: Nat {
        type Array<T: Copy>: Copy;
        fn filled<T: Copy>(v: T) -> Self::Array<T>;
    }

    pub struct Slot<N>(PhantomData<N>);
    impl<N: Nat> seal::Sealed for Slot<N> {}
    impl<N: Nat> Nat for Slot<N> {
        const VAL: usize = N::VAL;
    }
    impl<N: Nat> Capacity for Slot<N> {
        // The load-bearing line of this probe. No `const K`, no expression, an
        // associated-const PATH in array-length position.
        type Array<T: Copy> = [T; <N as Nat>::VAL];
        fn filled<T: Copy>(v: T) -> Self::Array<T> {
            [v; <N as Nat>::VAL]
        }
    }

    pub type Honest3 = Slot<N3>;
    pub type Honest7 = Slot<N7>;
    pub const H3_NAT: usize = <Honest3 as Nat>::VAL;
    pub const H3_LEN: usize = core::mem::size_of::<<Honest3 as Capacity>::Array<u8>>();
    pub const H7_NAT: usize = <Honest7 as Nat>::VAL;
    pub const H7_LEN: usize = core::mem::size_of::<<Honest7 as Capacity>::Array<u8>>();

    // CLAIM C. There is no way to write down a disagreeing instantiation. The
    // type constructor takes one argument. `Slot<N3>` has length 3 and value 3
    // because they are the same const, read twice.
    const _: () = assert!(H3_NAT == H3_LEN);
    const _: () = assert!(H7_NAT == H7_LEN);
}

// ---------------------------------------------------------------------------
// Composition, because the defect only appeared once capacities composed.
// ---------------------------------------------------------------------------
pub mod composed {
    use super::unpaired::*;
    use super::*;

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

    // File 100's own rank-3 shape, with the middle axis the one that lied.
    pub type Rank3 = Axis<Slot<N3>, Axis<Slot<N3>, Axis<Slot<N3>, Scalar>>>;
    pub const R3_COUNT: usize = <Rank3 as Shape>::COUNT;
    pub const R3_SIZE: usize = core::mem::size_of::<<Rank3 as Shape>::Store<u8>>();

    // The agreement holds by construction at every rank, through the trait
    // route, with no check anywhere in this module.
    const _: () = assert!(R3_COUNT == R3_SIZE);
}

fn main() {
    // Claim A: the pair disagrees, exactly as file 100 reported.
    assert_eq!(paired::LYING_NAT, 3);
    assert_eq!(paired::LYING_LEN, 7);
    assert_ne!(paired::LYING_NAT, paired::LYING_LEN);

    // Claim B/C: no pair, no disagreement, no check.
    assert_eq!(unpaired::H3_NAT, 3);
    assert_eq!(unpaired::H3_LEN, 3);
    assert_eq!(unpaired::H7_NAT, 7);
    assert_eq!(unpaired::H7_LEN, 7);

    assert_eq!(composed::R3_COUNT, 27);
    assert_eq!(composed::R3_SIZE, 27);

    let _ = unpaired::Honest7::filled(0u8);
}
