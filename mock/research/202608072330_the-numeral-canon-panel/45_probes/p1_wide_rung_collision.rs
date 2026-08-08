// p1: does the (declared width, stride) PAIR determine the carrier, even setting Precise
// entirely aside?
//
// `16` section 6 (16_probes/p5_recovery_direction.rs) sweeps (W, strategy) for W = 1..=128
// and reports the pair (W, stride) determines carrier at "0 of 251" cells, absent Precise
// widening, and states the pair's irreducibility then "rests only on the const-to-type
// argument". That sweep never reaches the wide rung (W > 128), and its own `carrier_bits`
// function returns a bare bit count (u32), never a type. `16`'s OWN p7, in the same file,
// establishes that a bit count cannot carry alignment and that alignment "rides on the
// carrier" as a property of a TYPE. p5's model is blind to exactly the thing p7 diagnoses,
// by construction of what it measures, because it never re-derives carrier as a type.
//
// This probe checks whether that blindness is exercised once the sweep is widened past 128
// bits, using REAL carrier types (not bit counts) matching arvo's own documented shape:
// `WideBits<BYTES>` at align 1 (Warm/Cold/Precise, N > 128) and `AlignedWideBits16<BYTES>`
// at align 16 (Hot, N > 128), per the crate architecture notes.
//
//   rustc +nightly-2026-05-28 --edition 2021 -O p1_wide_rung_collision.rs -o bin/p1 && ./bin/p1
//
// Spike.

use std::mem::{align_of, size_of};

#[repr(C)]
#[derive(Copy, Clone)]
struct WideBits<const BYTES: usize>([u8; BYTES]);

#[repr(C, align(16))]
#[derive(Copy, Clone)]
struct AlignedWideBits16<const BYTES: usize>([u8; BYTES]);

// Warm/Cold/Precise (non-widening reading): byte count is ceil(W/8), no further padding.
const fn warm_bytes(w: u32) -> usize {
    ((w + 7) / 8) as usize
}

// Hot: byte count is Warm's byte count, further padded up to the next multiple of 16.
const fn hot_bytes(w: u32) -> usize {
    let b = warm_bytes(w);
    ((b + 15) / 16) * 16
}

fn main() {
    println!("sweeping W = 129..=768 (the wide rung, three 128-bit periods)");
    println!();

    let mut collisions = 0usize;
    let mut total = 0usize;
    let mut first_witness: Option<(u32, usize)> = None;

    for w in 129u32..=768 {
        total += 1;
        let wb = warm_bytes(w);
        let hb = hot_bytes(w);
        // A collision on the (W, stride) key means Hot's padding was a no-op: hb == wb.
        if hb == wb {
            collisions += 1;
            if first_witness.is_none() {
                first_witness = Some((w, wb));
            }
        }
    }

    println!("W values in 129..=768 where Hot's byte count equals Warm's (no padding needed): {collisions} of {total}");

    if let Some((w, bytes)) = first_witness {
        println!("first witness: W = {w}, byte count = {bytes} for both Hot and Warm");
        println!();
        println!("at this W, both declarations have IDENTICAL (W, stride):");
        println!("  W = {w}, stride = {} bits, for BOTH Hot and Warm", bytes * 8);
    }

    // Concrete witness, checked with const assertions so the compiler proves the sizes and
    // alignments rather than the runtime print statements merely asserting them.
    // W = 256: warm_bytes(256) = 32, hot_bytes(256) = 32 (32 is already a multiple of 16).
    const W: u32 = 256;
    const WB: usize = warm_bytes(W);
    const HB: usize = hot_bytes(W);
    const _: () = assert!(WB == 32);
    const _: () = assert!(HB == 32);
    const _: () = assert!(WB == HB); // same stride

    type WarmCarrier = WideBits<WB>;
    type HotCarrier = AlignedWideBits16<HB>;

    const _: () = assert!(size_of::<WarmCarrier>() == size_of::<HotCarrier>());
    const _: () = assert!(align_of::<WarmCarrier>() != align_of::<HotCarrier>());

    println!();
    println!("compiled, const-checked witness at W = {W}:");
    println!(
        "  Warm carrier: WideBits<{WB}>            size {:>3}  align {:>2}",
        size_of::<WarmCarrier>(),
        align_of::<WarmCarrier>()
    );
    println!(
        "  Hot  carrier: AlignedWideBits16<{HB}>    size {:>3}  align {:>2}",
        size_of::<HotCarrier>(),
        align_of::<HotCarrier>()
    );
    println!("  same declared width (256), same stride (256 bits), same byte count (32).");
    println!("  different carrier TYPE (different alignment).");
    println!();
    println!(
        "so the pair (W, stride) does NOT determine carrier, at {collisions} of {total} wide-rung"
    );
    println!(
        "widths in this sweep, with ZERO dependence on Precise and ZERO dependence on sign."
    );
    println!(
        "this collision arises purely from Hot's align-16 wide-rung padding coinciding with"
    );
    println!(
        "Warm's align-1 unpadded byte count at widths where the byte count already happens"
    );
    println!("to be a multiple of 16.");

    println!();
    println!("negative control: at a width where the byte counts genuinely differ, no collision.");
    // 15's own worked example: I=200, F=40, so W = I + F = 240. Warm/Cold give 30 bytes,
    // Hot pads to 32. 15:352 reports exactly these numbers.
    const W2: u32 = 240;
    const WB2: usize = warm_bytes(W2);
    const HB2: usize = hot_bytes(W2);
    const _: () = assert!(WB2 == 30);
    const _: () = assert!(HB2 == 32);
    const _: () = assert!(WB2 != HB2);
    println!(
        "  W = {W2}: Warm bytes = {WB2}, Hot bytes = {HB2}. Different strides, no collision at this W."
    );
}
