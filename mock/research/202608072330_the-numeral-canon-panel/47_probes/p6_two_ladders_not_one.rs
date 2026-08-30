// p6: what 16's "the access width is not a third output" verdict actually costs, computed
// exhaustively rather than argued.
//
// 16 section 4 (16:173-198) dismisses the packed access width because it is a closed-form
// function of W. p3/p3b established that the closed form does not reach a TYPE from a
// const-carried width, and does reach one from a type-carried width via a ladder. This file asks
// the next question: is that the SAME ladder the native carrier already uses, or a second one?
//
// If the two rung partitions of 1..=128 coincide, the access type is free: it is another
// associated item on the ladder that already exists. If they cross, a design needs either two
// ladders or one ladder over their common refinement, and neither is free.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p6_two_ladders_not_one.rs -o bin/p6 && ./bin/p6
//
// Definitions, taken from the panel rather than invented here:
//   native bytes  = smallest power-of-two byte count holding W bits           (16 section 2)
//   access bytes  = floor((W+6)/8)+1, rounded up to a power of two            (16 section 4,
//                   checked there against an exhaustive phase scan, 0 mismatches)
//
// No #![feature] gate.

#![no_std]
extern crate std;
use std::{println, vec::Vec};

const fn pow2_ceil(n: u32) -> u32 {
    let mut p = 1u32;
    while p < n {
        p *= 2;
    }
    p
}

/// smallest native container, in bytes, holding W bits
const fn native_bytes(w: u32) -> u32 {
    pow2_ceil(w.div_ceil(8))
}

/// 16's closed form: max byte span of a W-bit field at unknown phase, rounded to a load width
const fn access_bytes(w: u32) -> u32 {
    pow2_ceil((w + 6) / 8 + 1)
}

fn jumps(f: fn(u32) -> u32, lo: u32, hi: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for w in (lo + 1)..=hi {
        if f(w) != f(w - 1) {
            v.push(w);
        }
    }
    v
}

fn main() {
    const LO: u32 = 1;
    const HI: u32 = 128;

    let nj = jumps(native_bytes, LO, HI);
    let aj = jumps(access_bytes, LO, HI);

    println!("widths {}..={}", LO, HI);
    println!("native rung jumps at W = {:?}", nj);
    println!("access rung jumps at W = {:?}", aj);

    let shared: Vec<u32> = nj.iter().copied().filter(|w| aj.contains(w)).collect();
    println!("jump points shared by both partitions: {:?}", shared);

    let mut all: Vec<u32> = nj.iter().chain(aj.iter()).copied().collect();
    all.sort_unstable();
    all.dedup();
    println!(
        "classes: native {}, access {}, common refinement {}",
        nj.len() + 1,
        aj.len() + 1,
        all.len() + 1
    );

    let mut disagree = 0u32;
    for w in LO..=HI {
        if native_bytes(w) != access_bytes(w) {
            disagree += 1;
        }
    }
    println!(
        "widths where the native carrier is the WRONG load type: {} of {}",
        disagree,
        HI - LO + 1
    );

    println!();
    println!("first sixteen widths, both rungs:");
    println!("  W   native  access");
    for w in LO..=16 {
        println!("  {:<3} {:<7} {}", w, native_bytes(w), access_bytes(w));
    }

    println!();
    if shared.is_empty() {
        println!("the two partitions share NO jump point in this range, so neither refines the");
        println!("other. one width ladder cannot key both. a design needs two ladders, or one");
        println!("over the common refinement, which is strictly larger than either.");
    } else {
        println!("the partitions share jump points; check whether one refines the other.");
    }
    println!();
    println!("16's verdict that the access width is not a third OUTPUT stands: it is a function");
    println!("of W. what it costs is a second rung partition, which 16 did not name and which is");
    println!("not free, because the ladder is the part of the derivation the design has already");
    println!("refused to enumerate.");
}
