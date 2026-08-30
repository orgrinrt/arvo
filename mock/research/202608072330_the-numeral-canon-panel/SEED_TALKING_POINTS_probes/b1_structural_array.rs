//! B1. The attempt A1 through A3 say nobody has an obvious route to: derive
//! the backing array structurally from the binary encoding, so no const
//! arithmetic ever sits in type position.
//!
//! This is the same move the design already made once, at file 65's probe 4,
//! where the container bucket stopped being a const fn and became three
//! type-level functions. Applied to storage: `O<P>` doubles its child's
//! array, `I<P>` doubles and adds one slot, and `repr(C)` makes the nest
//! layout-identical to `[T; N]`.
//!
//! What it costs is stated rather than hidden: recovering `&[T]` from the nest
//! is a pointer cast, so the carrier every consumer pays for acquires one
//! `unsafe` at the bottom of the design. The layout asserts below are what
//! discharge it, and they are checked per instantiation, at compile time.
#![no_std]

use core::marker::PhantomData;
use core::mem::{align_of, size_of};

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

/// Two of a thing, laid out end to end.
#[repr(C)]
pub struct Twice<A>(A, A);
/// Two of a thing and one more slot.
#[repr(C)]
pub struct TwicePlus<A, T>(A, A, T);

pub trait Pos: seal::Sealed {
    const VAL: usize;
    /// The backing storage for `VAL` slots of `T`, built structurally.
    type Store<T>;
}
impl Pos for H {
    const VAL: usize = 1;
    type Store<T> = [T; 1];
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
    type Store<T> = Twice<P::Store<T>>;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
    type Store<T> = TwicePlus<P::Store<T>, T>;
}

pub trait Nat: seal::Sealed {
    const VAL: usize;
    type Store<T>;
}
impl Nat for Z {
    const VAL: usize = 0;
    type Store<T> = [T; 0];
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
    type Store<T> = P::Store<T>;
}

/// The semantic alias, one blanket impl over the shared carrier.
///
/// `as_slice` is where the structural nest becomes the flat view every
/// consumer of a capacity actually wants. The cast is sound exactly when the
/// nest is layout-identical to `[T; VAL]`, which `layout_holds` asserts at
/// compile time for the instantiation in hand.
pub trait Capacity: Nat {
    const CAP: usize = <Self as Nat>::VAL;

    fn as_slice<T>(s: &<Self as Nat>::Store<T>) -> &[T] {
        // The precondition is discharged HERE, in the path of the only door,
        // rather than in a macro somewhere that lists the capacities someone
        // remembered. An inline const block is evaluated per monomorphisation,
        // so a capacity nobody thought to assert still cannot reach the cast
        // without the check running.
        const {
            assert!(
                layout_holds::<Self, T>(),
                "capacity store is not layout-identical to a flat array"
            )
        };
        // SAFETY: `Store<T>` is built only from `[T; 0]`, `[T; 1]`, and
        // `repr(C)` structs whose every field is itself such a store. Each
        // field therefore has alignment `align_of::<T>()` and a size that is a
        // multiple of `size_of::<T>()`, so `repr(C)` inserts no padding and the
        // whole nest is `VAL` values of `T` laid end to end. The two const
        // assertion above checks exactly that, for this instantiation, and a
        // failure is a build error at the use site rather than a wrong slice at
        // run time.
        unsafe { core::slice::from_raw_parts(s as *const _ as *const T, <Self as Nat>::VAL) }
    }
}

impl<N: Nat> Capacity for N {}

/// The layout precondition, stated as a checkable proposition rather than as
/// a comment. Instantiating this const for a `(C, T)` pair is what discharges
/// the cast for that pair.
pub const fn layout_holds<C: Capacity + ?Sized, T>() -> bool {
    size_of::<<C as Nat>::Store<T>>() == <C as Nat>::VAL * size_of::<T>()
        && (align_of::<<C as Nat>::Store<T>>() == align_of::<T>() || <C as Nat>::VAL == 0)
}

pub type N0 = Z;
pub type N1 = Pz<H>;
pub type N5 = Pz<I<O<H>>>;
pub type N7 = Pz<I<I<H>>>;
pub type N13 = Pz<I<O<I<H>>>>;
pub type N28 = Pz<O<O<I<I<H>>>>>;
pub type N64 = Pz<O<O<O<O<O<O<H>>>>>>>;
pub type N4096 = Pz<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>;

// The law over every capacity this file names, at three element types with
// different sizes and alignments, so the assertion is not sampled at one.
macro_rules! layout_law {
    ($($c:ty => $n:expr),* $(,)?) => {$(
        const _: () = {
            assert!(<$c as Nat>::VAL == $n);
            assert!(layout_holds::<$c, u8>());
            assert!(layout_holds::<$c, u32>());
            assert!(layout_holds::<$c, u128>());
        };
    )*};
}
layout_law! {
    N0 => 0, N1 => 1, N5 => 5, N7 => 7, N13 => 13, N28 => 28, N64 => 64, N4096 => 4096,
}

/// A capacity-generic consumer, the shape the shipped capacity foundation
/// exists to make expressible: build nothing, walk the storage, no const
/// expression in type position anywhere.
pub fn sum_generic<C: Capacity>(s: &<C as Nat>::Store<u32>) -> u32 {
    let mut acc = 0;
    let mut i = 0;
    let v = C::as_slice::<u32>(s);
    while i < v.len() {
        acc += v[i];
        i += 1;
    }
    acc
}
