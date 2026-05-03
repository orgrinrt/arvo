//! Sketch 02 consumer: invokes ConstHash::hash_const through method
//! dispatch from a downstream crate.
//!
//! Reproduces the cross-crate scenario that failed for `.hash()` in #316.
//! Two scoped questions:
//!
//! Q1. Does `<Fnv1a<N> as ConstHash<N, Hot, Unsigned>>::hash_const(...)`
//!     resolve cross-crate when the trait-solver chain runs:
//!     ConstHash -> NarrowFromU64 -> BitsContainerFor -> Project.
//!
//! Q2. Does the chain resolve in a `const` context as well as runtime?
//!
//! Outcome target: WORKS for both Q1 and Q2.

#![feature(const_trait_impl)]
#![allow(incomplete_features, dead_code)]

use sketch_provider::{ConstHash, Fnv1a, Hot, Unsigned};

const HASH_7: u8 = {
    // Const-context call: forces the trait-solver to evaluate the chain
    // at compile time. The Round 1 substrate moved this category of work
    // forward, so the chain has to navigate it.
    let bits = <Fnv1a<7> as ConstHash<7, Hot, Unsigned>>::hash_const(b"hello");
    bits.raw()
};

const HASH_64: u64 = {
    let bits = <Fnv1a<64> as ConstHash<64, Hot, Unsigned>>::hash_const(b"hello");
    bits.raw()
};

fn main() {
    // Runtime call: trait dispatch at runtime should also work.
    let runtime_7 = <Fnv1a<7> as ConstHash<7, Hot, Unsigned>>::hash_const(b"hello").raw();
    let runtime_32 = <Fnv1a<32> as ConstHash<32, Hot, Unsigned>>::hash_const(b"hello").raw();

    println!("const N=7: {HASH_7:#x}");
    println!("const N=64: {HASH_64:#x}");
    println!("runtime N=7: {runtime_7:#x}");
    println!("runtime N=32: {runtime_32:#x}");

    assert_eq!(HASH_7, runtime_7);

    println!("CROSS-CRATE CHAIN: ConstHash -> NarrowFromU64 -> BitsContainerFor -> Project resolves at both compile and runtime.");
}
