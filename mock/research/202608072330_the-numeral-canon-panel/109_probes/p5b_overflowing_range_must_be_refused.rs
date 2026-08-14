//! P5, question 4, as its own file because the wanted outcome is a BUILD
//! FAILURE and a build failure cannot share a binary with tests that pass.
//!
//! Per I15 an invalid must be caught at compile time. Here the invalid is a
//! carried range whose sum cannot fit the container: 0..=200 plus 0..=100 is
//! 0..=300, and the container holds 0..=255. Nothing about this is visible in
//! the values; it is a fact about the types.
//!
//! Expected: rustc refuses. If it compiles, the mechanism defers the failure
//! to runtime, which is exactly what I15 forbids, and the whole approach is
//! disqualified.
//!
//! Build: rustc --edition 2021 -O p5b_overflowing_range_must_be_refused.rs
#![allow(dead_code)]
use core::marker::PhantomData;

type Store = u8;
const STORE_MAX: i32 = 255;

trait Range {
    const LO: i32;
    const HI: i32;
}
struct Lit<const L: i32, const H: i32>;
impl<const L: i32, const H: i32> Range for Lit<L, H> {
    const LO: i32 = L;
    const HI: i32 = H;
}
struct RSum<A, B>(PhantomData<(A, B)>);
impl<A: Range, B: Range> Range for RSum<A, B> {
    const LO: i32 = A::LO + B::LO;
    const HI: i32 = A::HI + B::HI;
}

struct Ranged<R: Range>(Store, PhantomData<R>);
impl<R: Range> Ranged<R> {
    const FITS: () = assert!(
        R::LO >= 0 && R::HI <= STORE_MAX,
        "the carried range does not fit the container"
    );
}

fn add<A: Range, B: Range>(a: Ranged<A>, b: Ranged<B>) -> Ranged<RSum<A, B>> {
    let () = Ranged::<RSum<A, B>>::FITS;
    Ranged(a.0 + b.0, PhantomData)
}

fn main() {
    // 0..=200 and 0..=100. The sum reaches 300; the container stops at 255.
    let a: Ranged<Lit<0, 200>> = Ranged(200, PhantomData);
    let b: Ranged<Lit<0, 100>> = Ranged(100, PhantomData);
    let c = add(a, b);
    println!("if this line runs, question 4 FAILED: {}", c.0);
}
