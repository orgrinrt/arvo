// Probe 2: the mutation perimeter gap, compiled and run. A minimal model of a
// Lowering carrier: 13 live field bits in a 16-bit repr(transparent) container, 3
// padding bits statement P/C require canonical (here: zero). Demonstrates that a
// raw mutable door into the padding zone leaves every value-keyed observation
// correct while the raw byte image (and a raw-byte digest standing in for
// arvo-hash's own always-optimal-internals fast path) silently decorrelates.
#![allow(dead_code)]

const FIELDS_MASK: u16 = 0x1FFF; // low 13 bits
const PADDING_MASK: u16 = !FIELDS_MASK; // high 3 bits

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Carrier(u16);

// embed: D -> Carrier, a pure one-argument function of the datum. Zero-pads,
// per the forced-canonicalisation argument (73:139-191): every call on the
// same datum produces a bit-identical carrier.
const fn embed(datum: u16) -> Carrier {
    Carrier(datum & FIELDS_MASK)
}

// The canonical, value-keyed door: masks padding on every read. This is the
// ONLY thing statement C's "only door" sentence actually governs.
const fn canonical_read(c: &Carrier) -> u16 {
    c.0 & FIELDS_MASK
}

// A raw-byte digest standing in for arvo-hash's own documented fast path
// ("what a hash of a Number<N, S> consumes is the digest law, not the hash's
// own business", 78:684): reads the carrier's actual bytes, not the masked
// value. This is a datum/carrier-keyed observation, legitimately below the
// canonical door per file 73 section 3 ("the one fact that legitimately does
// depend on carrier identity is the byte image itself").
const fn raw_digest(c: &Carrier) -> u16 {
    // FNV-1a-shaped single-word mix would do the same job; a raw read is
    // sufficient to demonstrate the dependency on carrier identity.
    c.0
}

fn main() {
    let value: u16 = 5000; // an arbitrary in-range fields value
    let mut c = embed(value);

    // Statement C holds immediately after construction: padding is exactly
    // canonical-zero.
    assert_eq!(c.0 & PADDING_MASK, 0, "statement C holds at birth");
    let byte_image_at_birth = raw_digest(&c);

    // A raw mutable door into the container, exactly the shape repr(transparent)
    // makes reachable regardless of any shipped API (73:172-184's own argument,
    // applied to writes rather than reads). No unsafe transmute is even needed
    // here because the model's carrier has no validity-range invariant of its
    // own (an ordinary u16 has none); this is deliberately the WEAKEST possible
    // form of the attack, present even where the niche mechanism from Q1 is
    // absent entirely.
    let raw: &mut u16 = &mut c.0;
    *raw |= PADDING_MASK; // dirty every padding bit; touch no field bit

    // Every value-keyed observation stays correct. This is what makes the
    // defect silent: nothing a consumer would ordinarily check catches it.
    assert_eq!(canonical_read(&c), value, "value-keyed read is unaffected");

    // But the byte image, and any raw-byte-keyed digest, have decorrelated
    // from what a fresh, canonical construction of the SAME value produces.
    let fresh = embed(value);
    let byte_image_now = raw_digest(&c);
    let byte_image_fresh = raw_digest(&fresh);

    assert_ne!(
        byte_image_now, byte_image_fresh,
        "the mutated value's byte image no longer matches a fresh construction \
         of the identical logical value: statement C is falsified post-birth"
    );
    assert_eq!(
        byte_image_at_birth, byte_image_fresh,
        "sanity: the fresh construction matches what this value looked like \
         before the mutation, confirming the divergence is the mutation's doing"
    );

    println!(
        "value-keyed read: {} (correct) | byte image at birth: {:#06x} | \
         byte image after padding-only mutation: {:#06x} | fresh construction: {:#06x}",
        canonical_read(&c),
        byte_image_at_birth,
        byte_image_now,
        byte_image_fresh
    );
}
