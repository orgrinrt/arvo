//! Probe 2. Once capacities compose into a shape, section 1.26's "one
//! construction door" is not one door, and the repair is a declaration-site
//! refusal rather than a wider assert.
//!
//! `91:796-802` states the array grammar is a paired non-derived fact "checked
//! to agree in an inline const block at the one construction door". Probe 1
//! showed, compiled, that a rank-3 shape whose middle axis declares a `Nat` of
//! 4 against a literal of 7 has `COUNT == 12` and `size_of(Store) == 21`, both
//! computable, neither an error. This probe locates why and fixes it.
//!
//! The reason: the door section 1.26 means is `Slot`'s own INHERENT method
//! (`79_probes/probe_1_capacity_is_a_nat.rs`, `impl<N, const K> Slot<N, K> { fn
//! build }`), and the recursive composition D4 declares never calls it. It
//! calls the TRAIT method, `Capacity::filled`, which carries no check because
//! at rank 1 nothing needed it.
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

// ---------------------------------------------------------------------------
// CLAIM A. The leak, reproduced at its smallest. `Capacity::filled` is the
// method the composition calls, and it has no check to bypass.
// ---------------------------------------------------------------------------

pub mod leaking {
    use super::*;

    pub trait Capacity: Nat {
        type Array<T: Copy>: AsRef<[T]> + AsMut<[T]> + Copy;
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
            [v; K] // no assert: at rank 1 the inherent door was the only door
        }
    }

    /// The inherent door section 1.26 names. Correct, and unreachable from a
    /// composition, because the composition is written against the trait.
    impl<N: Nat, const K: usize> Slot<N, K> {
        pub fn build<T: Copy>(v: T) -> [T; K] {
            const { assert!(N::VAL == K, "capacity's length disagrees with its value") };
            [v; K]
        }
    }

    pub type Lying = Slot<Pz<O<O<H>>>, 7>; // Nat says 4, the language says 7

    /// Compiles and runs. Seven wide. The declared count is four.
    pub fn through_the_trait() -> [u8; 7] {
        <Lying as Capacity>::filled(0)
    }

    // Through the inherent door the same type is refused:
    // pub fn through_the_door() -> [u8; 7] { Lying::build(0) }   // const-eval error
}

const _: () = {
    // CLAIM A: the two routes to the identical array disagree about whether the
    // type is legal, and the recursive composition takes the permissive one.
    assert!(core::mem::size_of::<[u8; 7]>() == 7);
    assert!(<leaking::Lying as Nat>::VAL == 4);
};

// ---------------------------------------------------------------------------
// CLAIM B. The repair, in the shape the design already uses for the width
// levels: a declaration-site refusal, not an assert at a door. `91:560-561`
// establishes the precedent, `W_F <= W_S <= W_C` failing with `E0080` at
// declaration in the coverage shape.
// ---------------------------------------------------------------------------

pub mod sealed_door {
    use super::*;

    pub trait Capacity: Nat {
        type Array<T: Copy>: AsRef<[T]> + AsMut<[T]> + Copy;
        /// The agreement, lifted from an inherent method onto the trait, so it
        /// is a fact about the TYPE rather than about one call. Every route to
        /// the capacity, trait or inherent, direct or through a composition,
        /// reads the same const.
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
        // Evaluated when the impl's consts are needed, which every use needs.
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

    pub type Honest = Slot<Pz<O<O<H>>>, 4>;

    pub fn ok() -> [u8; 4] {
        <Honest as Capacity>::filled(0)
    }

    // The lying instantiation now fails through the TRAIT route as well:
    // pub type Lying = Slot<Pz<O<O<H>>>, 7>;
    // pub fn bad() -> [u8; 7] { <Lying as Capacity>::filled(0) }   // E0080

    // ---- and it composes: the shape inherits the refusal for free ----

    pub struct Scalar;
    pub struct Axis<Hd, Tl>(PhantomData<(Hd, Tl)>);

    pub trait Shape {
        const RANK: usize;
        const COUNT: usize;
    }
    impl Shape for Scalar {
        const RANK: usize = 0;
        const COUNT: usize = 1;
    }
    impl<Hd: Capacity, Tl: Shape> Shape for Axis<Hd, Tl> {
        const RANK: usize = 1 + Tl::RANK;
        const COUNT: usize = <Hd as Nat>::VAL * Tl::COUNT;
    }

    pub trait Dense: Shape {
        type Store<E: Copy>: Copy;
        fn build<E: Copy>(v: E) -> Self::Store<E>;
    }
    impl Dense for Scalar {
        type Store<E: Copy> = E;
        fn build<E: Copy>(v: E) -> E {
            v
        }
    }
    impl<Hd: Capacity, Tl: Dense> Dense for Axis<Hd, Tl> {
        type Store<E: Copy> = <Hd as Capacity>::Array<Tl::Store<E>>;
        fn build<E: Copy>(v: E) -> Self::Store<E> {
            Hd::filled(Tl::build(v))
        }
    }

    pub type E3 = Slot<Pz<I<H>>, 3>;
    pub type E5 = Slot<Pz<I<O<H>>>, 5>;
    pub type Vol = Axis<E3, Axis<Honest, Axis<E5, Scalar>>>;

    pub fn build_vol() -> <Vol as Dense>::Store<u8> {
        Vol::build(0)
    }

    const _: () = {
        assert!(<Vol as Shape>::RANK == 3);
        assert!(<Vol as Shape>::COUNT == 60);
        assert!(core::mem::size_of::<<Vol as Dense>::Store<u8>>() == 60);
        // Storage size and COUNT now cannot diverge, because a capacity whose
        // two facts disagree does not have a usable impl at all.
        assert!(core::mem::size_of::<<Vol as Dense>::Store<u8>>() == <Vol as Shape>::COUNT);
    };
}

pub fn exercise() -> (usize, usize) {
    (
        leaking::through_the_trait().len(),
        sealed_door::build_vol().as_ref().len(),
    )
}
