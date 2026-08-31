// Probe 4, seat 225, built in phase two. The packed-column probe file 222 names as
// missing: build a shared placement and measure whether a per-element footprint
// observation exists.
//
// A packed column of five 13-bit elements in a dense byte stream. Three arms:
//
//   arm A (measurement): the column's own footprint is const-observable (9 bytes,
//     ceil(65/8)), and an even per-element split of it (9/5 bytes) is not a whole
//     number, so no whole-byte per-element footprint value exists to return.
//
//   arm B (control, must work): the elements genuinely live there. Write five
//     distinct 13-bit values, read all five back exactly, over a sweep of
//     patterns including all-ones. If this fails the placement is fake and arm A
//     measured nothing.
//
//   arm C (must fail, negative control): the only size observation reachable
//     from an element access, size_of_val on the value get() returns, is asserted
//     equal to the element's placement share. It fails: it returns 2 bytes, the
//     size of the u16 extraction target, because get() returns a value and not a
//     place. The observation that exists reads the extraction target; the
//     placement footprint (13 bits) is returned by nothing.
//
// What this settles, for the fork 222 leaves open (an occupancy dimension row
// against a const size_of read): at shared occupancy the const read exists only
// for the column, not the element, so a per-element predicate cannot be built
// from size_of there. Whatever route the panel picks must carry the
// sole-against-shared condition somewhere; the const read alone does not.
//
// holds for: W = 13, elements = 5, container = dense bit stream (u8 backing),
// alignment in {aligned, straddling} (elements 1..4 straddle byte boundaries),
// signedness = unsigned, threads = 1, toolchain = the committed rustc in
// probe4_out.txt, edition = 2024.

use core::mem::{size_of, size_of_val};

const W: usize = 13;
const N: usize = 5;
const BYTES: usize = (W * N + 7) / 8; // 9

/// Five 13-bit elements packed contiguously into a dense bit stream.
struct Packed {
    bytes: [u8; BYTES],
}

impl Packed {
    fn new() -> Self {
        Packed { bytes: [0; BYTES] }
    }
    /// Read element `i` by value. There is no place to borrow: the element's
    /// bits straddle byte boundaries, so the accessor can only extract.
    fn get(&self, i: usize) -> u16 {
        let bit = i * W;
        let mut v: u32 = 0;
        for k in 0..3 {
            let idx = bit / 8 + k;
            if idx < BYTES {
                v |= (self.bytes[idx] as u32) << (8 * k);
            }
        }
        ((v >> (bit % 8)) & ((1 << W) - 1)) as u16
    }
    fn set(&mut self, i: usize, val: u16) {
        assert!(val < (1 << W) as u16);
        let bit = i * W;
        for k in 0..W {
            let b = bit + k;
            let mask = 1u8 << (b % 8);
            if (val >> k) & 1 == 1 {
                self.bytes[b / 8] |= mask;
            } else {
                self.bytes[b / 8] &= !mask;
            }
        }
    }
}

fn main() {
    println!("arm A: the column footprint is const-observable, the per-element one is not whole");
    println!("  size_of::<Packed>() = {} (ceil({} * {} / 8) = {BYTES})", size_of::<Packed>(), N, W);
    if size_of::<Packed>() != BYTES {
        println!("  INSTRUMENT BROKEN: the column does not have the dense footprint");
        std::process::exit(2);
    }
    println!("  per-element even split: {BYTES} / {N} bytes, remainder {}", BYTES % N);
    if BYTES % N == 0 {
        println!("  UNEXPECTED: split is whole; pick a width where it is not");
        std::process::exit(2);
    }

    println!();
    println!("arm B (control, must work): the elements genuinely live in the stream");
    let patterns: [[u16; N]; 4] = [
        [0, 1, 2, 3, 4],
        [8191, 0, 8191, 0, 8191],
        [5440, 7567, 1344, 3471, 13],
        [8191, 8191, 8191, 8191, 8191],
    ];
    for p in patterns {
        let mut col = Packed::new();
        for (i, v) in p.iter().enumerate() {
            col.set(i, *v);
        }
        for (i, v) in p.iter().enumerate() {
            if col.get(i) != *v {
                println!("  INSTRUMENT BROKEN: element {i} wrote {v}, read {}", col.get(i));
                std::process::exit(2);
            }
        }
    }
    println!("  all patterns roundtrip exactly, straddling elements included");

    println!();
    println!("arm C (negative control, MUST FAIL): the reachable size observation reads the");
    println!("  placement share, i.e. equals W bits");
    let col = Packed::new();
    let extracted = col.get(2); // element 2 straddles bytes 3..5
    let observed_bits = size_of_val(&extracted) * 8;
    if observed_bits == W {
        println!("  UNEXPECTED PASS: a per-element footprint observation exists after all");
        std::process::exit(2);
    }
    println!("  FAILED AS REQUIRED: size_of_val(&col.get(2)) * 8 = {observed_bits} bits, W = {W} bits.");
    println!("  the only size observation an element access reaches measures the extraction");
    println!("  target (u16), not the placement. the placement footprint is returned by");
    println!("  nothing: get() yields a value, and there is no place of the element's own to");
    println!("  observe, because its bits straddle byte boundaries inside the column.");
}
