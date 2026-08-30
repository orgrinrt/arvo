//! P14: gate-free type-level numerals. Canonical binary rep, addition by trait.
//! No feature gate, no -Z flag. Does canonicity hold, and does it reach a const?
#![no_std]
pub struct Z; // zero
pub struct B0<T>(T); // 2n
pub struct B1<T>(T); // 2n+1

pub trait Nat {
    const V: u16;
}
impl Nat for Z {
    const V: u16 = 0;
}
impl<T: Nat> Nat for B0<T> {
    const V: u16 = 2 * T::V;
}
impl<T: Nat> Nat for B1<T> {
    const V: u16 = 2 * T::V + 1;
}

// increment, then addition by recursion
pub trait Inc {
    type Out;
}
impl Inc for Z {
    type Out = B1<Z>;
}
impl<T> Inc for B0<T> {
    type Out = B1<T>;
}
impl<T: Inc> Inc for B1<T> {
    type Out = B0<T::Out>;
}

pub trait Add<R> {
    type Out;
}
impl<R> Add<R> for Z {
    type Out = R;
}
impl<T: Add<R>, R> Add<R> for B0<T> {
    type Out = B0<T::Out>;
}

pub fn value<N: Nat>() -> u16 {
    N::V
}
// 13 = B1<B0<B1<B1<Z>>>> reads 1101 little-endian -> 1+0+4+8 = 13
pub type N13 = B1<B0<B1<B1<Z>>>>;
pub type N3 = B1<B1<Z>>;
pub fn check() -> u16 {
    value::<N13>() + value::<N3>()
}
