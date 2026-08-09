//! Probe 2. What the fork costs in emitted instructions.
//!
//! Three spellings of one loop, monomorphised across the real crate boundary
//! built in probe 1. If they lower identically, the truth-contract fork is free
//! at runtime and is a compile-surface question only. If they do not, the
//! difference is the price of branch B and op should see it as a number.
#![no_std]
#![feature(const_trait_impl)]
#![crate_type = "lib"]

use p1_arvo::Bool;
use p1_consumer::{greedy_a, greedy_b1, greedy_b2};
use p1_foundation::{Cons, Nil, Pred};

type L2<A, B> = Cons<A, Cons<B, Nil>>;

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn run_a(items: &[u32]) -> u32 {
    let p: Pred<L2<u32, u32>, _> = Pred::new(|a: &u32, b: &u32| Bool::new(*a >= *b));
    greedy_a(items, &p)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn run_b1(items: &[u32]) -> u32 {
    let p: Pred<L2<u32, u32>, _> = Pred::new(|a: &u32, b: &u32| Bool::new(*a >= *b));
    greedy_b1(items, &p)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn run_b2(items: &[u32]) -> u32 {
    let p: Pred<L2<u32, u32>, _> = Pred::new(|a: &u32, b: &u32| Bool::new(*a >= *b));
    greedy_b2(items, &p)
}

/// The floor: the same loop with no predicate abstraction at all, so the
/// mechanism's total cost is visible rather than only the fork's.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn run_bare(items: &[u32]) -> u32 {
    let mut acc = 0u32;
    let mut groups = 0u32;
    for it in items {
        if acc >= *it {
            acc += *it;
        } else {
            groups += 1;
            acc = *it;
        }
    }
    groups
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
