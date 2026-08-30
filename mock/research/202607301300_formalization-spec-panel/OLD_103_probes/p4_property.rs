//! Probe 4. D15's second half: properties as associated consts.
//!
//! The round names exactly one property with a consequence in shipped source,
//! the fresh-accumulator guarantee at `arvo-comb/src/greedy.rs:33`, and claims
//! carrying it as an associated const makes the guard unreachable. Three things
//! are checked here, because the design has since acquired a ladder that sorts
//! properties by what checks them.
//!
//! 1. Does the const actually delete the guard, in emitted code.
//! 2. Can this particular property be witnessed, in file 07's rung-1 sense of a
//!    const the evaluator can refuse.
//! 3. What a false assertion costs, since that decides which rung it sits on.
#![no_std]
#![feature(const_trait_impl)]
#![crate_type = "lib"]
#![allow(dead_code)]

use p1_arvo::Bool;
use p1_foundation::{Cons, Describes, Nil, Pred, TruthHolds};

type L2<A, B> = Cons<A, Cons<B, Nil>>;

/// The property, carried on the predicate type rather than on the value.
pub trait Property {
    /// A fresh accumulator accepts any item. Not computed from anything: this
    /// is a claim about the closure's behaviour at every input, and the closure
    /// is opaque to the type system.
    const FRESH_ALWAYS_ACCEPTS: bool = false;
}

/// A predicate that promises the guarantee.
pub struct Promising;
impl Property for Promising {
    const FRESH_ALWAYS_ACCEPTS: bool = true;
}

/// A predicate that does not.
pub struct Silent;
impl Property for Silent {}

#[inline(always)]
fn group<P: Property, F>(items: &[u32], f: &Pred<L2<u32, u32>, F>) -> u32
where
    F: Fn(&u32, &u32) -> Bool,
{
    let mut acc = 0u32;
    let mut open = false;
    let mut groups = 0u32;
    let mut i = 0usize;
    while i < items.len() {
        let it = items[i];
        if !open {
            // The guard the round quotes. Under the promise it is unreachable.
            if !P::FRESH_ALWAYS_ACCEPTS && !f(&0, &it).holds() {
                i += 1;
                continue;
            }
            open = true;
            acc = it;
            groups += 1;
            i += 1;
            continue;
        }
        if f(&acc, &it).holds() {
            acc += it;
            i += 1;
        } else {
            open = false;
        }
    }
    groups
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn group_promising(items: &[u32]) -> u32 {
    let p: Pred<L2<u32, u32>, _> = Pred::new(|a: &u32, b: &u32| Bool::new(*a >= *b));
    group::<Promising, _>(items, &p)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn group_silent(items: &[u32]) -> u32 {
    let p: Pred<L2<u32, u32>, _> = Pred::new(|a: &u32, b: &u32| Bool::new(*a >= *b));
    group::<Silent, _>(items, &p)
}

// --- 2. Can the property be witnessed at rung 1?
//
// A rung-1 marker is refusable by the const evaluator. To refuse this one the
// evaluator would have to decide `forall x. f(0, x) == TRUE` over an opaque
// closure. Uncommenting the block below is the attempt; it does not compile,
// and the error is the finding rather than an obstacle.
//
// pub const fn witness<F: Fn(&u32, &u32) -> Bool>(f: &F) -> bool {
//     let mut x = 0u32;
//     while x < u32::MAX { if !f(&0, &x).holds() { return false; } x += 1; }
//     true
// }

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
