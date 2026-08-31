//! B2, must FAIL, and the failure is NOT the staging boundary.
//!
//! The length is a const generic, so the accumulator is perfectly stageable. What is
//! missing is an impl. If the row's sentence is read as a universal over a fold's
//! compile-time refusals, this arm falsifies it.
#![allow(dead_code)]

trait Addable {
    fn zero() -> Self;
    fn plus(self, other: Self) -> Self;
}

impl Addable for i64 {
    fn zero() -> Self {
        0
    }
    fn plus(self, other: Self) -> Self {
        self + other
    }
}

/// A perfectly staged fold: the length is a const generic.
fn fold<T: Addable + Copy, const N: usize>(xs: [T; N]) -> T {
    let mut acc = T::zero();
    for x in xs {
        acc = acc.plus(x);
    }
    acc
}

/// `String` has no `Addable` impl. Nothing about staging is involved.
fn main() {
    let v: [String; 2] = [String::new(), String::new()];
    let _ = fold(v);
}
