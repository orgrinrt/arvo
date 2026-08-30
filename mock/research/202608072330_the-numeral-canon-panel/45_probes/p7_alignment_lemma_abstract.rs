// p7: the alignment-collision mechanism, stated and checked with ZERO reference to arvo's
// pre-panel architecture, Hot, Warm, or the number 16.
//
// 46 attacked 45_probes/p1 on the grounds that its carrier types (`WideBits`,
// `AlignedWideBits16`) are borrowed from the pre-panel crate architecture, which `15`
// itself (15:418-429, 15:553-556) states is an unresolved, replaceable assumption, not
// settled design. 46 also stated, correctly, that the underlying MECHANISM is general and
// does not need arvo's specific numbers: "pick any two alignments a1 != a2 and any byte
// count n that is a multiple of both, and the same shape appears."
//
// This probe checks that claim directly, disentangled from arvo entirely: three
// independent alignment pairs, none matching arvo's 1-and-16, all showing the identical
// shape (same size, different type, because alignment is a property of the type). This is
// the part of the finding that survives attack one: a general, unconditional fact about
// any type system with alignment as a first-class property (true of C's ABI, Rust's ABI,
// and any hardware-aware layout, not specific to this design). What does NOT survive is
// the claim that arvo's four strategies actually instantiate two such alignments today;
// that remains open, per 15's own flag, and this probe does not attempt to settle it.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p7_alignment_lemma_abstract.rs -o bin/p7 && ./bin/p7
//
// Spike.

use std::mem::{align_of, size_of};

// Pair 1: align 4 vs align 8, byte count 8 (a multiple of both). Arbitrary, chosen only to
// be different from arvo's 1-and-16.
#[repr(C, align(4))]
#[derive(Copy, Clone)]
struct A4([u8; 8]);

#[repr(C, align(8))]
#[derive(Copy, Clone)]
struct A8([u8; 8]);

// Pair 2: align 2 vs align 32, byte count 32.
#[repr(C, align(2))]
#[derive(Copy, Clone)]
struct A2([u8; 32]);

#[repr(C, align(32))]
#[derive(Copy, Clone)]
struct A32([u8; 32]);

// Pair 3: align 1 vs align 4, byte count 4. The smallest possible instance of the shape.
#[repr(C, align(1))]
#[derive(Copy, Clone)]
struct A1([u8; 4]);

#[repr(C, align(4))]
#[derive(Copy, Clone)]
struct A4Small([u8; 4]);

fn main() {
    println!("the abstract lemma: for any two distinct alignments a1 != a2, and any byte");
    println!("count n that is a common multiple of both, a size-n type at align a1 and a");
    println!("size-n type at align a2 have equal size and unequal alignment, hence are");
    println!("different types. checked at three pairs, none matching arvo's 1-and-16.");
    println!();

    const _: () = assert!(size_of::<A4>() == size_of::<A8>());
    const _: () = assert!(align_of::<A4>() != align_of::<A8>());
    println!(
        "pair 1: align({},{}) size {:>2} vs {:>2}  align {:>2} vs {:>2}",
        4,
        8,
        size_of::<A4>(),
        size_of::<A8>(),
        align_of::<A4>(),
        align_of::<A8>()
    );

    const _: () = assert!(size_of::<A2>() == size_of::<A32>());
    const _: () = assert!(align_of::<A2>() != align_of::<A32>());
    println!(
        "pair 2: align({},{}) size {:>2} vs {:>2}  align {:>2} vs {:>2}",
        2,
        32,
        size_of::<A2>(),
        size_of::<A32>(),
        align_of::<A2>(),
        align_of::<A32>()
    );

    const _: () = assert!(size_of::<A1>() == size_of::<A4Small>());
    const _: () = assert!(align_of::<A1>() != align_of::<A4Small>());
    println!(
        "pair 3: align({},{}) size {:>2} vs {:>2}  align {:>2} vs {:>2}",
        1,
        4,
        size_of::<A1>(),
        size_of::<A4Small>(),
        align_of::<A1>(),
        align_of::<A4Small>()
    );

    println!();
    println!("all three pairs compile and const-check: equal size, unequal alignment, at");
    println!("byte counts and alignments that share nothing with arvo's WideBits<BYTES> /");
    println!("AlignedWideBits16<BYTES> shape. the mechanism is general.");
    println!();
    println!("what this probe does NOT establish: that arvo's four strategies actually pick");
    println!("two different alignments anywhere. that is a design choice, not a fact about");
    println!("type systems, and per 15:418-429 and 15:553-556 it is stated as an open,");
    println!("replaceable assumption in this panel's own source, not as settled design.");
}
