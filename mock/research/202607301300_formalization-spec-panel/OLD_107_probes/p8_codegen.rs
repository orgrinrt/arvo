//! Probe 8. What the derived storage lowers to, against `[T; N]`, on the same
//! operations in the same binary.
//!
//! The pricing pillar buys compile time with runtime. Probe 7 established the
//! compile-time side. This one establishes there is nothing to buy it with: if
//! the derived storage lowered worse than `[T; N]`, the whole construction
//! would be dead regardless of how clean the type story is.
//!
//! Four operations, each written twice against two storages that are supposed
//! to be layout-identical: fill, indexed read, whole-array sum, and a
//! pass-by-value copy. Inspect with:
//!
//!   rustc --edition 2024 -O --crate-type=lib --emit=asm p8_codegen.rs -o /tmp/p8.s
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::marker::PhantomData;
use core::mem::MaybeUninit;

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Twin<A>(A, A);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TwinOne<A, T>(A, A, T);

// The constructor and the projection are written ONCE, on the trait, over the
// projected slice. Probe 7 measured what happens when they are written as
// structural recursion instead: 27x the compile time at K = 100, for identical
// semantics. Recur the type; never recur the code.
pub trait Capacity: Pos {
    type Array<T: Copy>: Copy;

    #[inline(always)]
    fn filled<T: Copy>(v: T) -> Self::Array<T> {
        const {
            assert!(
                core::mem::size_of::<Self::Array<T>>() == Self::VAL * core::mem::size_of::<T>()
            );
            assert!(core::mem::align_of::<Self::Array<T>>() == core::mem::align_of::<T>());
        }
        let mut out = MaybeUninit::<Self::Array<T>>::uninit();
        let p = out.as_mut_ptr() as *mut T;
        let mut i = 0usize;
        while i < Self::VAL {
            // SAFETY: the const block above establishes that the storage is
            // exactly `VAL` contiguous `T` at `T`'s alignment.
            unsafe {
                p.add(i).write(v);
            }
            i += 1;
        }
        // SAFETY: every one of the `VAL` slots was written above.
        unsafe { out.assume_init() }
    }

    #[inline(always)]
    fn slice<T: Copy>(a: &Self::Array<T>) -> &[T] {
        const {
            assert!(
                core::mem::size_of::<Self::Array<T>>() == Self::VAL * core::mem::size_of::<T>()
            );
        }
        // SAFETY: as above.
        unsafe { core::slice::from_raw_parts(a as *const Self::Array<T> as *const T, Self::VAL) }
    }
}
impl Capacity for H {
    type Array<T: Copy> = [T; 1];
}
impl<P: Capacity> Capacity for O<P> {
    type Array<T: Copy> = Twin<P::Array<T>>;
}
impl<P: Capacity> Capacity for I<P> {
    type Array<T: Copy> = TwinOne<P::Array<T>, T>;
}

pub type N13 = I<O<I<H>>>;
pub type Derived13 = <N13 as Capacity>::Array<u32>;
pub type Native13 = [u32; 13];

// 1. fill
#[unsafe(no_mangle)]
pub fn derived_fill(v: u32) -> Derived13 {
    <N13 as Capacity>::filled(v)
}
#[unsafe(no_mangle)]
pub fn native_fill(v: u32) -> Native13 {
    [v; 13]
}

// 2. indexed read at a runtime index
#[unsafe(no_mangle)]
pub fn derived_get(a: &Derived13, i: usize) -> u32 {
    <N13 as Capacity>::slice(a)[i]
}
#[unsafe(no_mangle)]
pub fn native_get(a: &Native13, i: usize) -> u32 {
    a[i]
}

// 3. whole-array sum
#[unsafe(no_mangle)]
pub fn derived_sum(a: &Derived13) -> u32 {
    let s = <N13 as Capacity>::slice(a);
    let mut t = 0u32;
    let mut i = 0;
    while i < s.len() {
        t = t.wrapping_add(s[i]);
        i += 1;
    }
    t
}
#[unsafe(no_mangle)]
pub fn native_sum(a: &Native13) -> u32 {
    let mut t = 0u32;
    let mut i = 0;
    while i < a.len() {
        t = t.wrapping_add(a[i]);
        i += 1;
    }
    t
}

// 4. by-value copy through a call boundary
#[unsafe(no_mangle)]
pub fn derived_copy(a: Derived13) -> Derived13 {
    a
}
#[unsafe(no_mangle)]
pub fn native_copy(a: Native13) -> Native13 {
    a
}

// The layout claim, asserted rather than assumed, so the comparison above is
// between two spellings of one layout and not between two different things.
const _: () = assert!(core::mem::size_of::<Derived13>() == core::mem::size_of::<Native13>());
const _: () = assert!(core::mem::align_of::<Derived13>() == core::mem::align_of::<Native13>());
