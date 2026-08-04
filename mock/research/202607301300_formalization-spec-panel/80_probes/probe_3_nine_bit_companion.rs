// Probe 3, file 80. The nine-bit companion model: the u16 container class.
//
// File 67 established container class as a transfer coordinate and priced the
// companion ("an additional model at nine bits costs 2^18 pairs where eight costs
// 2^16", 67:406-407); files 68 and 78 carry it as owed and unbuilt. This builds it.
//
// Everything the review's eight-bit models checked about order and injectivity ran
// in the u8 class, where the logical width EQUALS the container width, so two things
// were structurally unexercised until now:
//   (1) the u16 class itself (the threshold is one width per class; nine bits is the
//       cheapest member), and
//   (2) carrier padding. At logical width == container width the padding is empty and
//       every padding-law claim (statement P, file 73) is vacuously true. At nine
//       bits in a u16 the carrier has seven padding bits, so this companion is the
//       first model at which statement P has any content at all.
//
// Checks, every one exhaustive over its stated matrix:
//   (a) container-class witnesses: the dispatched container is u16 (size 2), and
//       logical-width wrap differs from container wrap (masking is load-bearing),
//       where at eight bits the two coincide (the vacuity fact, asserted).
//   (b) unsigned 9-bit order: over all 512 x 512 canonical pairs, the datum compare
//       equals the value compare; injectivity (512 distinct values) holds.
//   (c) signed 9-bit two's complement: over all 512 x 512 pairs, bit-pattern-derived
//       order via sign extension equals value order; exactly one pattern denotes
//       zero (the injectivity witness, same shape as 64_probes/probe_3).
//   (d) the padding hazard, first expressible at this class: over the WHOLE matrix of
//       same-value-different-padding pairs (512 canonical data x 127 nonzero padding
//       patterns), a compare keyed on the raw u16 carrier misorders every one of
//       them, while the canonical (value-keyed) compare is Equal. Plus a cross
//       witness where raw order and value order invert outright.
//
// Build: rustc --edition 2021 -O; run. Assertions, not printout, except a summary.

const W: u32 = 9; // logical width
const MASK: u16 = (1u16 << W) - 1; // 0x01FF

fn value_u(d: u16) -> u16 {
    d & MASK
}

/// Sign-extend a canonical 9-bit two's-complement datum to i32.
fn value_s(d: u16) -> i32 {
    let v = (d & MASK) as i32;
    if v & (1 << (W - 1)) != 0 {
        v - (1 << W)
    } else {
        v
    }
}

fn main() {
    // (a) container-class witnesses.
    assert_eq!(core::mem::size_of::<u16>(), 2);
    // logical wrap != container wrap at nine bits...
    let top: u16 = MASK; // 511, the largest 9-bit value
    assert_eq!((top.wrapping_add(1)) & MASK, 0); // logical wrap to 0
    assert_ne!(top.wrapping_add(1), 0); // the container did NOT wrap (0x200)
                                        // ...where at eight bits in a u8 the two coincide (the u8-class vacuity fact):
    let top8: u8 = 0xFF;
    assert_eq!(top8.wrapping_add(1), 0); // container wrap IS logical wrap

    // (b) unsigned: datum order == value order, exhaustive 2^18 pairs; injectivity.
    for a in 0u16..(1 << W) {
        for b in 0u16..(1 << W) {
            assert_eq!(a.cmp(&b), value_u(a).cmp(&value_u(b)));
            if a != b {
                assert_ne!(value_u(a), value_u(b), "unsigned injectivity");
            }
        }
    }

    // (c) signed two's complement at nine bits: order derived from the datum (flip
    // the sign bit, compare as unsigned: the standard order-preserving bijection)
    // equals the value order, exhaustive 2^18 pairs; exactly one zero pattern.
    let flip = |d: u16| (d & MASK) ^ (1 << (W - 1));
    let mut zeros = 0u32;
    for a in 0u16..(1 << W) {
        if value_s(a) == 0 {
            zeros += 1;
        }
        for b in 0u16..(1 << W) {
            assert_eq!(flip(a).cmp(&flip(b)), value_s(a).cmp(&value_s(b)));
            if a != b {
                assert_ne!(value_s(a), value_s(b), "signed injectivity");
            }
        }
    }
    assert_eq!(zeros, 1, "exactly one datum denotes zero (no cohort)");

    // (d) the padding hazard: the whole same-value-different-padding matrix.
    let mut misordered: u32 = 0;
    for d in 0u16..(1 << W) {
        for pad in 1u16..(1 << (16 - W)) {
            let dirty = d | (pad << W);
            // same value...
            assert_eq!(value_u(dirty), value_u(d));
            // ...but a raw-carrier compare misorders the pair, every time:
            assert_eq!(dirty.cmp(&d), core::cmp::Ordering::Greater);
            misordered += 1;
        }
    }
    assert_eq!(misordered, 512 * 127);
    // and raw order can invert value order outright:
    let a = 0u16 | (1 << W); // value 0, one padding bit set -> raw 512
    let b = 1u16; // value 1, clean
    assert!(value_u(a) < value_u(b));
    assert!(
        a > b,
        "raw carrier order inverts value order under dirty padding"
    );

    println!(
        "OK. u16 class exercised at 9 bits: 2 x 2^18 order matrices, {} padding pairs \
         all misordered raw / equal canonical.",
        misordered
    );
}
