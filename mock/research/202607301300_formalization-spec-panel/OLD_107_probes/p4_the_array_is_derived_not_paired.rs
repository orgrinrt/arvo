//! Probe 4. The construction the checkpoint asked for, built rather than
//! argued: the array grammar DERIVED from the numeral, with no length in type
//! position, no companion literal, no agreement fact, no construction door,
//! and no feature gate.
//!
//! The move. Probe 3 showed the refusal is about obtaining the array BY ITS
//! LENGTH. So do not obtain it by its length. The numeral is already a
//! structural recursion; recur the storage alongside it. `O<P>` is twice `P`,
//! so its storage is two of `P`'s storage. `I<P>` is twice `P` plus one, so
//! its storage is two of `P`'s storage and one element. Every array length
//! that appears is a LITERAL (`0`, `1`), which type position accepts.
//!
//! `repr(C)` is what makes this a layout claim rather than a shape claim: a
//! two-field `repr(C)` struct whose fields have equal size and alignment lays
//! them out at offsets 0 and size, with no padding, because a Rust type's size
//! is always a multiple of its alignment. By induction over the three
//! constructors, `Array<T>` is exactly `VAL` contiguous `T`, which is
//! `[T; VAL]`'s layout without `[T; VAL]`'s spelling.
//!
//! Claims:
//!   A. it compiles, zero gates, and the storage law
//!      `size_of::<C::Array<T>>() == C::VAL * size_of::<T>()` holds at every
//!      numeral in the sweep, const-asserted, for four element types with four
//!      different sizes and alignments.
//!   B. alignment is preserved exactly, so the storage is a legal `[T]`.
//!   C. the law survives a numeral PRODUCED by type-level arithmetic
//!      (probe 2's `Sum`), which is the operation probe 1 refuses outright.
//!      This is the case that decides the fork: it is available in neither
//!      column as previously stated.
//!   D. the falsifiable surface is the three constructors, not the instances.
//!      Exhaustive over the grammar: three impls, each checked.
//!   E. a generic slice projection, with the law re-asserted per
//!      monomorphisation in an inline const block, so the `unsafe` is
//!      discharged by a compile-time fact rather than by a comment.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::marker::PhantomData;

pub struct Z;
pub struct Pz<P>(PhantomData<P>);
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

pub trait Pos {
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

pub trait Nat {
    const VAL: usize;
}
impl Nat for Z {
    const VAL: usize = 0;
}
impl<P: Pos> Nat for Pz<P> {
    const VAL: usize = P::VAL;
}

// The two storage combinators. Both `repr(C)`, both homogeneous in alignment.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Twin<A>(A, A);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TwinOne<A, T>(A, A, T);

// CLAIM A and D. Three impls. That is the entire falsifiable surface: the
// array grammar can only be wrong in one of these three lines, and all three
// are checked below at every numeral in the sweep.
pub trait Capacity: Nat {
    type Array<T: Copy>: Copy;
}

pub trait PosCapacity: Pos {
    type PArray<T: Copy>: Copy;
}
impl PosCapacity for H {
    type PArray<T: Copy> = [T; 1]; // literal, not a computed length
}
impl<P: PosCapacity> PosCapacity for O<P> {
    type PArray<T: Copy> = Twin<P::PArray<T>>;
}
impl<P: PosCapacity> PosCapacity for I<P> {
    type PArray<T: Copy> = TwinOne<P::PArray<T>, T>;
}

impl Capacity for Z {
    type Array<T: Copy> = [T; 0]; // literal
}
impl<P: PosCapacity> Capacity for Pz<P> {
    type Array<T: Copy> = P::PArray<T>;
}

pub type N1 = H;
pub type N2 = O<H>;
pub type N3 = I<H>;
pub type N4 = O<O<H>>;
pub type N5 = I<O<H>>;
pub type N6 = O<I<H>>;
pub type N7 = I<I<H>>;
pub type N8 = O<O<O<H>>>;
pub type N9 = I<O<O<H>>>;
pub type N10 = O<I<O<H>>>;
pub type N11 = I<I<O<H>>>;
pub type N12 = O<O<I<H>>>;
pub type N13 = I<O<I<H>>>;
pub type N14 = O<I<I<H>>>;
pub type N15 = I<I<I<H>>>;
pub type N16 = O<O<O<O<H>>>>;
pub type N23 = I<I<I<O<H>>>>;
pub type N31 = I<I<I<I<H>>>>;
pub type N32 = O<O<O<O<O<H>>>>>;
pub type N47 = I<I<I<I<O<H>>>>>;
pub type N64 = O<O<O<O<O<O<H>>>>>>;
pub type N127 = I<I<I<I<I<I<H>>>>>>;
pub type N128 = O<O<O<O<O<O<O<H>>>>>>>;

// Every spelling asserted against its decimal value, so a mis-spelled numeral
// fails here rather than passing a law that is true of whatever number it is.
const _: () = assert!(<N1 as Pos>::VAL == 1);
const _: () = assert!(<N2 as Pos>::VAL == 2);
const _: () = assert!(<N3 as Pos>::VAL == 3);
const _: () = assert!(<N4 as Pos>::VAL == 4);
const _: () = assert!(<N5 as Pos>::VAL == 5);
const _: () = assert!(<N6 as Pos>::VAL == 6);
const _: () = assert!(<N7 as Pos>::VAL == 7);
const _: () = assert!(<N8 as Pos>::VAL == 8);
const _: () = assert!(<N9 as Pos>::VAL == 9);
const _: () = assert!(<N10 as Pos>::VAL == 10);
const _: () = assert!(<N11 as Pos>::VAL == 11);
const _: () = assert!(<N12 as Pos>::VAL == 12);
const _: () = assert!(<N13 as Pos>::VAL == 13);
const _: () = assert!(<N14 as Pos>::VAL == 14);
const _: () = assert!(<N15 as Pos>::VAL == 15);
const _: () = assert!(<N16 as Pos>::VAL == 16);
const _: () = assert!(<N23 as Pos>::VAL == 23);
const _: () = assert!(<N31 as Pos>::VAL == 31);
const _: () = assert!(<N32 as Pos>::VAL == 32);
const _: () = assert!(<N47 as Pos>::VAL == 47);
const _: () = assert!(<N64 as Pos>::VAL == 64);
const _: () = assert!(<N127 as Pos>::VAL == 127);
const _: () = assert!(<N128 as Pos>::VAL == 128);

// CLAIM A and B, at four element types. `u8` (size 1 align 1), `u16` (2, 2),
// `u32` (4, 4), and a nine-byte struct at align 1 whose size is not a power of
// two, which is where a padding assumption would show.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Odd9([u8; 9]);

macro_rules! law_for {
    ($n:ty, $t:ty) => {
        const _: () = assert!(
            core::mem::size_of::<<Pz<$n> as Capacity>::Array<$t>>()
                == <Pz<$n> as Nat>::VAL * core::mem::size_of::<$t>()
        );
        const _: () = assert!(
            core::mem::align_of::<<Pz<$n> as Capacity>::Array<$t>>() == core::mem::align_of::<$t>()
        );
    };
}
macro_rules! law {
    ($n:ty) => {
        law_for!($n, u8);
        law_for!($n, u16);
        law_for!($n, u32);
        law_for!($n, Odd9);
    };
}

law!(N1);
law!(N2);
law!(N3);
law!(N4);
law!(N5);
law!(N6);
law!(N7);
law!(N8);
law!(N9);
law!(N10);
law!(N11);
law!(N12);
law!(N13);
law!(N14);
law!(N15);
law!(N16);
law!(N23);
law!(N31);
law!(N32);
law!(N47);
law!(N64);
law!(N127);
law!(N128);

// The empty capacity, whose storage is a zero-length array and whose law is
// the degenerate instance rather than an exception written in.
const _: () = assert!(core::mem::size_of::<<Z as Capacity>::Array<u32>>() == 0);
const _: () = assert!(core::mem::align_of::<<Z as Capacity>::Array<u32>>() == 4);
const _: () = assert!(<Z as Nat>::VAL == 0);

// CLAIM C. A numeral produced by type-level arithmetic, then given storage.
// This is the operation probe 1 refuses under a const-parameter capacity, and
// the storage probe 3 refuses under a length-obtained array. Both halves, one
// expression.
pub trait Inc {
    type Out: Pos + PosCapacity;
}
impl Inc for H {
    type Out = O<H>;
}
impl<P: Pos + PosCapacity> Inc for O<P> {
    type Out = I<P>;
}
impl<P: Pos + PosCapacity + Inc> Inc for I<P> {
    type Out = O<<P as Inc>::Out>;
}
pub type Suc<A> = <A as Inc>::Out;

pub trait Add<R> {
    type Out: Pos + PosCapacity;
}
pub type Sum<A, B> = <A as Add<B>>::Out;
pub trait AddC<R> {
    type Out: Pos + PosCapacity;
}
pub type SumC<A, B> = <A as AddC<B>>::Out;

impl Add<H> for H {
    type Out = O<H>;
}
impl<Q: Pos + PosCapacity> Add<O<Q>> for H {
    type Out = I<Q>;
}
impl<Q: Pos + PosCapacity + Inc> Add<I<Q>> for H {
    type Out = O<Suc<Q>>;
}
impl<P: Pos + PosCapacity + Inc> Add<H> for O<P> {
    type Out = I<P>;
}
impl<P: Pos + PosCapacity + Add<Q>, Q: Pos + PosCapacity> Add<O<Q>> for O<P> {
    type Out = O<Sum<P, Q>>;
}
impl<P: Pos + PosCapacity + Add<Q>, Q: Pos + PosCapacity> Add<I<Q>> for O<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + PosCapacity + Inc> Add<H> for I<P> {
    type Out = O<Suc<P>>;
}
impl<P: Pos + PosCapacity + Add<Q>, Q: Pos + PosCapacity> Add<O<Q>> for I<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + PosCapacity + AddC<Q>, Q: Pos + PosCapacity> Add<I<Q>> for I<P> {
    type Out = O<SumC<P, Q>>;
}
impl AddC<H> for H {
    type Out = I<H>;
}
impl<Q: Pos + PosCapacity + Inc> AddC<O<Q>> for H {
    type Out = O<Suc<Q>>;
}
impl<Q: Pos + PosCapacity + Inc> AddC<I<Q>> for H {
    type Out = I<Suc<Q>>;
}
impl<P: Pos + PosCapacity + Inc> AddC<H> for O<P> {
    type Out = O<Suc<P>>;
}
impl<P: Pos + PosCapacity + Add<Q>, Q: Pos + PosCapacity> AddC<O<Q>> for O<P> {
    type Out = I<Sum<P, Q>>;
}
impl<P: Pos + PosCapacity + AddC<Q>, Q: Pos + PosCapacity> AddC<I<Q>> for O<P> {
    type Out = O<SumC<P, Q>>;
}
impl<P: Pos + PosCapacity + Inc> AddC<H> for I<P> {
    type Out = I<Suc<P>>;
}
impl<P: Pos + PosCapacity + AddC<Q>, Q: Pos + PosCapacity> AddC<O<Q>> for I<P> {
    type Out = O<SumC<P, Q>>;
}
impl<P: Pos + PosCapacity + AddC<Q>, Q: Pos + PosCapacity> AddC<I<Q>> for I<P> {
    type Out = I<SumC<P, Q>>;
}

// The concatenation of a capacity-5 domain and a capacity-7 domain, given
// storage. Nobody declared a twelve anywhere in this file.
pub type Cat57 = Pz<Sum<N5, N7>>;
const _: () = assert!(<Cat57 as Nat>::VAL == 12);
const _: () = assert!(core::mem::size_of::<<Cat57 as Capacity>::Array<u32>>() == 48);
const _: () = assert!(core::mem::align_of::<<Cat57 as Capacity>::Array<u32>>() == 4);

pub type Cat4747 = Pz<Sum<N47, N47>>;
const _: () = assert!(<Cat4747 as Nat>::VAL == 94);
const _: () = assert!(core::mem::size_of::<<Cat4747 as Capacity>::Array<Odd9>>() == 94 * 9);

// CLAIM E. A generic slice projection over any capacity. The `unsafe` is
// discharged by an inline const block that re-derives the law at every
// monomorphisation, so a wrong constructor in one of the three impls above
// fails the build at the use site rather than corrupting a read.
pub const fn as_slice<C: Capacity, T: Copy>(a: &C::Array<T>) -> &[T] {
    const {
        assert!(
            core::mem::size_of::<C::Array<T>>() == C::VAL * core::mem::size_of::<T>(),
            "storage law violated: the array grammar does not match the numeral"
        );
        assert!(core::mem::align_of::<C::Array<T>>() == core::mem::align_of::<T>());
    }
    // SAFETY: the const block above establishes, at this monomorphisation,
    // that the storage is exactly `C::VAL` contiguous `T` at `T`'s alignment.
    unsafe { core::slice::from_raw_parts(a as *const C::Array<T> as *const T, C::VAL) }
}

// CLAIM C, the generic form. Probe 9 claim B is the same signature under the
// paired shape and it cannot be written. Here it needs no literal, because the
// storage is a projection of the numeral rather than a restatement of it.
pub fn concat_storage<A, B, T>() -> <Pz<Sum<A, B>> as Capacity>::Array<T>
where
    A: Pos + PosCapacity + Add<B>,
    B: Pos + PosCapacity,
    T: Copy,
{
    unimplemented!()
}

pub const fn concat_count<A, B>() -> usize
where
    A: Pos + PosCapacity + Add<B>,
    B: Pos + PosCapacity,
{
    <Sum<A, B> as Pos>::VAL
}
const _: () = assert!(concat_count::<N5, N7>() == 12);
const _: () = assert!(concat_count::<N47, N47>() == 94);
