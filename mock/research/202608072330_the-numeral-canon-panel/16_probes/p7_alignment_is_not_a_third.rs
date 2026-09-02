// p7: is alignment a third output? `15` section 5 raises it and does not resolve it.
//
// Checks ONE thing: whether two declarations can share a carrier-size and a stride while needing
// different alignments, and if so whether the CARRIER TYPE already distinguishes them. If the
// carrier distinguishes them, alignment is a property of output 1 rather than a third output.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p7_alignment_is_not_a_third.rs -o bin/p7 && ./bin/p7
//
// The adversarial case is the wide rung, where two byte payloads of the SAME size can have
// different alignment: the shipped design has WideBits at align 1 and AlignedWideBits16 at
// align 16 (per arvo's own crate instructions). Modelled here rather than imported.
//
// Spike.

use std::mem::{align_of, size_of};

// Two wide payloads of identical size and different alignment.
#[repr(C)]
#[derive(Copy, Clone)]
struct WideBits32([u8; 32]);

#[repr(C, align(16))]
#[derive(Copy, Clone)]
struct AlignedWide32([u8; 32]);

// The adversarial pair: same size, same stride, different alignment.
const _: () = assert!(size_of::<WideBits32>() == size_of::<AlignedWide32>());
const _: () = assert!(align_of::<WideBits32>() != align_of::<AlignedWide32>());

fn main() {
    println!("(a) can two declarations share size and stride but need different alignment?");
    println!(
        "    WideBits32   size {:>3}  align {:>3}   stride would be {} bits",
        size_of::<WideBits32>(),
        align_of::<WideBits32>(),
        size_of::<WideBits32>() * 8
    );
    println!(
        "    AlignedWide32 size {:>2}  align {:>3}   stride would be {} bits",
        size_of::<AlignedWide32>(),
        align_of::<AlignedWide32>(),
        size_of::<AlignedWide32>() * 8
    );
    println!("    yes. so alignment is not recoverable from the stride.");

    println!();
    println!("(b) does the CARRIER already distinguish them?");
    println!("    align_of is a property of the type, so a site holding the carrier type reads");
    println!(
        "    {} and {} directly. it does not need a third component.",
        align_of::<WideBits32>(),
        align_of::<AlignedWide32>()
    );

    println!();
    println!("(c) the one case where no carrier holds the run: a packed column's base.");
    // For a packed run the elements do not each sit in a carrier; the run is a byte buffer and
    // its base alignment is a free choice. Check whether any alignment is REQUIRED.
    const W: u32 = 13;
    let mut worst_span = 0usize;
    for phase in 0..8usize {
        let span = (phase + W as usize - 1) / 8 + 1;
        if span > worst_span {
            worst_span = span;
        }
    }
    println!("    a {W}-bit field at unknown phase touches at most {worst_span} bytes.");
    println!("    every byte offset is reachable, so no base alignment makes any element aligned;");
    println!("    align 1 is sufficient and nothing higher is required for correctness.");
    println!("    what a higher base alignment buys is a load-crossing question, and that is");
    println!("    UNPRICED: no bench harness run in this panel bears on it.");

    println!();
    println!("verdict: alignment rides on the carrier for the unpacked case and is a free choice");
    println!("for the packed base. it is not a third output. it IS a reason the carrier has to be");
    println!("a type rather than a width, because a width cannot carry an alignment.");
}
