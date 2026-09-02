//! Can the const and non-const capacity traits stay separate, both implemented
//! on the same type, with the non-const bodies delegating to the const ones?
//!
//! The claim under test: no supertrait is needed, so `Capacity::Array<T>` keeps
//! its unbounded `T` and `from_fn` survives, while the duplicated bodies go.
#![feature(const_trait_impl)]
#![allow(incomplete_features, dead_code)]

pub struct Cap(pub usize);

pub const trait ConstCapacity {
    type Array<T: Copy>: Copy;
    const CAP: Cap;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
    fn get<T: Copy>(a: &Self::Array<T>, i: usize) -> T;
}

pub trait Capacity: ConstCapacity {
    // Unbounded T, and the AsRef bound the const half cannot call.
    type Array<T>: AsRef<[T]> + AsMut<[T]>;
    const CAP: Cap;
    fn filled<T: Copy>(v: T) -> <Self as Capacity>::Array<T>;
    fn from_fn<T, F: FnMut(usize) -> T>(f: F) -> <Self as Capacity>::Array<T>;
}

pub struct Dim<const N: usize>;

const impl<const N: usize> ConstCapacity for Dim<N> {
    type Array<T: Copy> = [T; N];
    const CAP: Cap = Cap(N);
    fn filled<T: Copy>(v: T) -> [T; N] {
        [v; N]
    }
    fn get<T: Copy>(a: &[T; N], i: usize) -> T {
        a[i]
    }
}

impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];
    const CAP: Cap = Cap(N);
    // The delegation under test: a const fn is callable from a runtime body,
    // and the concrete type implements both traits, so no supertrait relation
    // is required to name the other one.
    fn filled<T: Copy>(v: T) -> [T; N] {
        <Self as ConstCapacity>::filled(v)
    }
    // Stays here, and is the reason the runtime half exists at all: a closure
    // is not const-callable and T is not bounded Copy.
    fn from_fn<T, F: FnMut(usize) -> T>(mut f: F) -> [T; N] {
        core::array::from_fn(|i| f(i))
    }
}

// A non-Copy element, to prove the runtime half really is unbounded in T.
struct NotCopy(u32);
pub fn builds_non_copy() -> <Dim<4> as Capacity>::Array<NotCopy> {
    <Dim<4> as Capacity>::from_fn(|i| NotCopy(i as u32))
}

// The const half, in a const context.
pub const CONST_BUILT: [u8; 4] = <Dim<4> as ConstCapacity>::filled(7u8);

// Runtime slice access via the AsRef bound the const half cannot carry.
pub fn slice_access<C: Capacity>(a: &<C as Capacity>::Array<u8>) -> usize {
    a.as_ref().len()
}

// With the supertrait in place, does a generic caller reach the const half
// without naming ConstCapacity itself?
pub fn generic_one_bound<C: Capacity>() -> <C as ConstCapacity>::Array<u8> {
    <C as ConstCapacity>::filled(0u8)
}
