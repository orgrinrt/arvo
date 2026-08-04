// Probe 1: the biased-niche lowering (file 84 section 5.4) against statement 0's
// hardened quantifier (file 80 section 1).
//
// File 80's spec sentence: "statement 0 quantifies over every bit pattern of
// Encoding::Fields' width; an encoding whose decode is partial on that set does
// not satisfy Crosses, and partiality is expressed by shrinking the fields, not
// by a domain side-condition."
//
// File 84's construction: store the datum biased by one in core::num::NonZero,
// spending one pattern (zero) so Option<T> is the same width as T.
//
// Hypothesis: the two are inconsistent as written. The biased carrier's decode
// is partial on exactly one pattern of the 16-bit fields width, and no field
// shrink expresses a 65535-member domain, because every width-shaped domain has
// a power-of-two size. If both facts compile, the collision is real and one of
// the two one-pass sentences needs amending before either hardens.
//
// Compile-only, const-position assertions throughout, zero feature gates.
#![no_std]

use core::mem::size_of;
use core::num::NonZeroU16;

/// The model: a bounded unsigned numeral with 16-bit fields and a
/// 65535-member datum set (one pattern spent), exactly file 84's probe_6 shape.
const DOMAIN: u32 = 65_535;

/// encode: datum -> carrier, biased by one. Total on 0..=65534.
const fn encode(d: u16) -> NonZeroU16 {
    // d + 1 never wraps: d <= 65534.
    match NonZeroU16::new(d + 1) {
        Some(c) => c,
        None => unreachable!(),
    }
}

/// decode: carrier -> datum. Total on the carrier TYPE's inhabitants.
const fn decode(c: NonZeroU16) -> u16 {
    c.get() - 1
}

// ---------------------------------------------------------------------------
// Fact 1: the layout dividend is real (file 84's claim, reconfirmed here so
// the collision cannot be dismissed by disputing the construction's value).
const _: () = assert!(size_of::<NonZeroU16>() == 2);
const _: () = assert!(size_of::<Option<NonZeroU16>>() == 2);
const _: () = assert!(size_of::<Option<u16>>() == 4);

// ---------------------------------------------------------------------------
// Fact 2: no field shrink expresses this domain. A domain expressed by a field
// width w has exactly 2^w members; 65535 is not a power of two, and no
// w in 0..=16 hits it. Checked over every width, not asserted from arithmetic.
const _: () = {
    let mut w = 0u32;
    while w <= 16 {
        assert!((1u32 << w) != DOMAIN);
        w += 1;
    }
};

// ---------------------------------------------------------------------------
// Fact 3: the decode is partial on exactly one bit pattern of the fields'
// width, and the exclusion is compiler-enforced rather than a side-condition:
// the pattern is not an inhabitant of the carrier type at all. NonZeroU16
// carries a declared validity range; the constructor refuses zero, and a
// transmuted zero is undefined behaviour by the type's own declaration, not by
// a module invariant the tower promises to maintain.
const _: () = assert!(NonZeroU16::new(0).is_none());

// ---------------------------------------------------------------------------
// Fact 4: on the carrier type's actual inhabitants, the crossing is total and
// exact: round trip over the whole 65535-member domain, every member, no
// sample. Statement 0's INTENT (no reachable datum decodes outside the value
// set) holds; only its quantifier domain ("every bit pattern of the fields'
// width") is what the construction violates.
const _: () = {
    let mut d: u32 = 0;
    while d < DOMAIN {
        let c = encode(d as u16);
        assert!(decode(c) as u32 == d);
        d += 1;
    }
};

// ---------------------------------------------------------------------------
// Fact 5: the bias is monotone over the whole domain, so raw carrier order
// agrees with datum order and every datum-keyed order fact survives the
// re-encoding. This holds for the additive bias at an initial-segment domain;
// it is NOT a property of niche-spending in general (an interior spare
// pattern, e.g. E4M3's NaN slots, admits no additive monotone bias onto the
// nonzero patterns; see the deliverable, section 1.1).
const _: () = {
    let mut d: u32 = 0;
    while d + 1 < DOMAIN {
        assert!(encode(d as u16).get() < encode((d + 1) as u16).get());
        d += 1;
    }
};
