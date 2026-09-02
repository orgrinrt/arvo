//! Probe 1. Capacity maps DIRECTLY onto the shared Nat carrier for its VALUE;
//! the array grammar stays a paired, non-derived companion fact.
//!
//! This extends file 76's b2 (`Slot<N, const K: usize>`) to test the specific
//! claim this dispatch derives: `Capacity` is not a wrapper or a copy of `Nat`,
//! it IS a `Nat` (a direct subtrait, same seal, same ordering, same arithmetic),
//! and the four uses the dispatch names (array length, index-bound check,
//! iteration terminator, arity) are all satisfied by that one Nat-typed count
//! with no second encoding anywhere.
#![no_std]

use core::marker::PhantomData;

mod seal {
    pub trait Sealed {}
}

// The sealed value-unique vocabulary, unchanged from `68:549-556` / file 76's b2.
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

/// The shared bottom carrier. One authority on what a number is.
pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

/// `Capacity` is a direct subtrait of `Nat`, not a wrapper and not a second
/// encoding. Every type implementing `Nat` (through the one sealed tower) is
/// eligible; the blanket impl below makes the mapping total rather than a
/// per-instance choice, matching D7's own "each domain aliases the cell to
/// its own semantics" pattern (`inherited:570`).
///
/// The `Array` GAT is the one part of `Capacity` that is NOT derived from the
/// Nat value: it is a paired, declaration-site fact, because the language's
/// own array-length grammar `[T; K]` forces a bare `usize` literal in that
/// position and no const expression over a type parameter reaches it under
/// the permitted feature set (file 76, probes a1 through a3b). This impl
/// block states that split in one place: `SIZE` reads straight through to
/// the Nat's own `VAL` (direct), `Array` is declared per concrete `Slot`
/// (paired).
pub trait Capacity: Nat {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const SIZE: usize = Self::VAL; // direct: no capacity-specific arithmetic exists
}

pub struct Slot<N, const K: usize>(PhantomData<N>);
impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> {
    const VAL: usize = N::VAL;
}
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T> = [T; K];
}
pub const fn agrees<N: Nat, const K: usize>() -> bool {
    N::VAL == K
}
impl<N: Nat, const K: usize> Slot<N, K> {
    pub fn build<T: Copy>(v: T) -> [T; K] {
        const {
            assert!(
                agrees::<N, K>(),
                "capacity's declared length disagrees with its value"
            )
        };
        [v; K]
    }
}

pub type C13 = Slot<Pz<I<O<I<H>>>>, 13>;
pub type C0 = Slot<Z, 0>;

const _: () = {
    // SIZE reads straight through the Nat: no second arithmetic exists for it.
    assert!(<C13 as Capacity>::SIZE == 13);
    assert!(<C13 as Capacity>::SIZE == <C13 as Nat>::VAL);
    assert!(<C0 as Capacity>::SIZE == 0);
};

// --- The four uses the dispatch names, all against the one Nat-typed count. ---

/// Use 1: array length. `[T; K]`, the paired grammar fact, built from the
/// Nat-agreeing `Slot`.
pub fn array_length_use() -> [u32; 13] {
    Slot::<Pz<I<O<I<H>>>>, 13>::build(0u32)
}

/// Use 2: index-bound check. A membership test against the SAME `SIZE`,
/// no separate width or bound type.
pub const fn in_bounds<C: Capacity>(i: usize) -> bool {
    i < C::SIZE
}

/// Use 3: iteration terminator. A while-loop bound against the SAME `SIZE`.
pub const fn count_live<C: Capacity>(mut i: usize) -> usize {
    let mut n = 0;
    while i < C::SIZE {
        n += 1;
        i += 1;
    }
    n
}

/// Use 4: arity. A fixed-arity carrier's own "how many positions" fact is the
/// SAME Nat-typed count, not a fourth vocabulary. `Arity2` below stands in for
/// a two-place carrier (the shape an hlist-of-fixed-length or a fold's `n`
/// would use); its arity is `<C as Nat>::VAL`, read the identical way a
/// capacity's element count is read.
pub struct FixedArity<C>(PhantomData<C>);
impl<C: Nat> FixedArity<C> {
    pub const ARITY: usize = C::VAL;
}
pub type Arity2 = FixedArity<Pz<O<H>>>;

const _: () = {
    assert!(in_bounds::<C13>(12));
    assert!(!in_bounds::<C13>(13));
    assert!(!in_bounds::<C0>(0));
    assert!(count_live::<C13>(0) == 13);
    assert!(count_live::<C0>(0) == 0);
    assert!(Arity2::ARITY == 2);
};

fn main() {}
