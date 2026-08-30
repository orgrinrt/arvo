//! B2. The second constructive answer, and the cheaper one: keep the count on
//! the shared carrier and put the array grammar on the lowering side.
//!
//! The design already draws exactly this line once, between `Numeral` (what
//! the number is) and `Lowering` (how a carrier holds it), and the layer-keying
//! rule says a fact is keyed on the coarsest layer whose identity its truth
//! depends on. A count's arithmetic, ordering and laws depend on the value, so
//! they key on the shared `Nat`. The array grammar `[T; K]` depends on nothing
//! but the carrier, and `K` is a language-level array-length const, which is
//! the one place the grammar forces a bare literal.
//!
//! So the unification op ratified holds where it was aimed (one authority on
//! the number, `Capacity` keeping its name) without asking the carrier to
//! answer a question that belongs to the storage.
//!
//! What must be checked, and is: the two sides cannot disagree. A `Slot` whose
//! declared length is not its carrier's value fails to build.
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

/// The shared bottom carrier. One authority on what a number is: the tower's
/// precisions and exponents, and the capacity domain's counts, are this.
pub trait Nat: seal::Sealed {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

/// `Capacity`, the semantic alias op kept, over the shared carrier.
///
/// Its `Array` is a lowering-side fact and so is not derived from `N`. It is
/// declared alongside, and the declaration is checked against the value.
pub trait Capacity: Nat {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
}

/// The lowering-side door: a count paired with the array grammar for it.
///
/// `K` is the language's own array length. Nothing computes it from `N` in
/// type position, which is why this needs no feature gate; the macro below
/// emits both halves already agreeing, and the const assertion refuses any
/// pair that does not.
pub struct Slot<N, const K: usize>(PhantomData<N>);

impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}
impl<N: Nat, const K: usize> Nat for Slot<N, K> {
    const VAL: usize = N::VAL;
}
impl<N: Nat, const K: usize> Capacity for Slot<N, K> {
    type Array<T> = [T; K];
}

/// The agreement obligation, discharged per instantiation at the only door
/// rather than in a list someone maintains.
pub const fn agrees<N: Nat, const K: usize>() -> bool {
    N::VAL == K
}

pub trait CheckedCapacity: Capacity {
    fn build<T: Copy>(v: T) -> Self::Array<T>;
}
impl<N: Nat, const K: usize> CheckedCapacity for Slot<N, K> {
    fn build<T: Copy>(v: T) -> [T; K] {
        const {
            assert!(
                agrees::<N, K>(),
                "capacity's declared length disagrees with its value"
            )
        };
        [v; K]
    }
}

/// The introduction route. Host-staged: the reduced binary encoding and the
/// literal length are both emitted at expansion time, so neither is computed
/// by the type checker at any use site.
#[macro_export]
macro_rules! cap {
    (0) => { $crate::Slot<$crate::Z, 0> };
    (1) => { $crate::Slot<$crate::Pz<$crate::H>, 1> };
    (5) => { $crate::Slot<$crate::Pz<$crate::I<$crate::O<$crate::H>>>, 5> };
    (13) => { $crate::Slot<$crate::Pz<$crate::I<$crate::O<$crate::I<$crate::H>>>>, 13> };
}

pub type C13 = Slot<Pz<I<O<I<H>>>>, 13>;

const _: () = {
    assert!(<C13 as Nat>::VAL == 13);
    assert!(agrees::<Pz<I<O<I<H>>>>, 13>());
};

/// The consumer shape: generic over the capacity, builds and walks, and the
/// count and the storage agree by construction.
pub fn fill_and_sum<C: CheckedCapacity>(live: usize) -> u32 {
    let mut a = C::build(0u32);
    let s: &mut [u32] = a.as_mut();
    let mut i = 0;
    while i < live && i < s.len() {
        s[i] = (i as u32) + 1;
        i += 1;
    }
    let r: &[u32] = a.as_ref();
    let mut acc = 0;
    let mut j = 0;
    while j < r.len() {
        acc += r[j];
        j += 1;
    }
    acc
}

pub fn use_it() -> u32 {
    fill_and_sum::<C13>(13)
}
