//! Probe 1, crate 3 of 3. The consumer (`arvo-comb`'s position).
//!
//! `greedy_group` is the shipped call site D15's own topic quotes. Three
//! spellings of the same function, so the fork can be priced in what a consumer
//! writes rather than in what a diagram says.
//!
//! A: concrete. The output is `arvo-platform`'s `Bool`, so this crate depends on
//!    the platform crate, which is branch A of the fork.
//! B1: generic over the truth contract, exiting through `holds() -> bool`.
//! B2: generic over the truth contract, exiting through `select`, so `bool`
//!    appears nowhere in this crate at all.
//!
//! The count that matters is bounds restated per level of the call chain, since
//! Rust has no implied bounds and file 07 named that as the recurring cost.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]

use p1_arvo::Bool;
use p1_foundation::{Cons, Describes, Nil, Pred, Truth, TruthHolds, TruthSelect};

type L2<A, B> = Cons<A, Cons<B, Nil>>;

// ---------------------------------------------------------------- branch A
// Bounds on the outer function: 1 (Describes). Concrete output type.

pub fn greedy_a<F>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> Bool,
{
    let mut acc = 0u32;
    let mut groups = 0u32;
    for it in items {
        if f(&acc, it)._0_marker() {
            acc += *it;
        } else {
            groups += 1;
            acc = *it;
        }
    }
    groups
}

// mid and inner levels of a call chain, to count restatements
pub fn mid_a<F>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> Bool,
{
    greedy_a(items, f)
}

pub fn outer_a<F>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> Bool,
{
    mid_a(items, f)
}

// ---------------------------------------------------------------- branch B1
// Bounds on the outer function: 2 (the Fn output, plus the Truth exit).

pub fn greedy_b1<F, B>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> B,
    B: TruthHolds,
{
    let mut acc = 0u32;
    let mut groups = 0u32;
    for it in items {
        if f(&acc, it).holds() {
            acc += *it;
        } else {
            groups += 1;
            acc = *it;
        }
    }
    groups
}

pub fn mid_b1<F, B>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> B,
    B: TruthHolds,
{
    greedy_b1(items, f)
}

pub fn outer_b1<F, B>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> B,
    B: TruthHolds,
{
    mid_b1(items, f)
}

// ---------------------------------------------------------------- branch B2
// `bool` appears nowhere below.

pub fn greedy_b2<F, B>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> B,
    B: TruthSelect,
{
    let mut acc = 0u32;
    let mut groups = 0u32;
    for it in items {
        let (na, ng) = f(&acc, it).select(|| (acc + *it, groups), || (*it, groups + 1));
        acc = na;
        groups = ng;
    }
    groups
}

// ---------------------------------------------------------------- the seal
// The typestate is enforced at construction, per the sketch's own finding that
// the unenforced version compiled and would have shipped a lying parameter.
pub fn make_ok() -> Pred<L2<u32, u32>, impl Fn(&u32, &u32) -> Bool> {
    Pred::new(|a: &u32, b: &u32| Bool::new(*a >= *b))
}

// A declared arity of two against a closure of one must not compile. The
// negative control lives in p1_negative.rs so this crate stays green.

// Helper so branch A can read its own concrete type without a public field.
trait Marker {
    fn _0_marker(self) -> bool;
}
impl Marker for Bool {
    #[inline(always)]
    fn _0_marker(self) -> bool {
        <Bool as TruthHolds>::holds(self)
    }
}
