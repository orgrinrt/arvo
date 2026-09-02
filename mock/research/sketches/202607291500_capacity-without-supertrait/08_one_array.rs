//! The simple shape: ONE associated array, declared once. The ambiguity in the
//! other probes exists only because both traits declared an `Array`.
//!
//! ConstCapacity owns it, with the bounds the const half needs plus the AsRef
//! the runtime half wants (declaring AsRef is fine; only calling it is
//! non-const). Capacity extends it with the one method that cannot be const.
#![feature(const_trait_impl)]
#![allow(incomplete_features, dead_code)]

pub struct Cap(pub usize);

pub const trait ConstCapacity {
    type Array<T: Copy>: Copy + AsRef<[T]> + AsMut<[T]>;
    const CAP: Cap;
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
    fn get<T: Copy>(a: &Self::Array<T>, i: usize) -> T;
}

pub trait Capacity: ConstCapacity {
    // No second `Array`. That is the whole fix.
    fn from_fn<T: Copy, F: FnMut(usize) -> T>(f: F) -> Self::Array<T>;
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
    fn from_fn<T: Copy, F: FnMut(usize) -> T>(mut f: F) -> [T; N] {
        core::array::from_fn(|i| f(i))
    }
}

// Unqualified `C::Array`, one bound. This is what failed in every other shape.
pub fn slice_access<C: ConstCapacity>(a: &C::Array<u8>) -> usize {
    a.as_ref().len()
}
pub fn build<C: Capacity>() -> C::Array<u8> {
    C::from_fn(|i| i as u8)
}
pub fn both<C: Capacity>(a: &C::Array<u8>) -> usize {
    a.as_ref().len()
}
pub const CONST_BUILT: [u8; 4] = <Dim<4> as ConstCapacity>::filled(7u8);
