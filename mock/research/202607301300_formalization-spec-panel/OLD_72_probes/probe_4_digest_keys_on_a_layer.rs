// Probe 4: a digest is a one-way image, and it must factor through the layer of
// the equality it is paired with. Two compiled demonstrations of the failure the
// layer-keying rule names, at a layer the review has not yet run the rule over.
//
// (a) carrier padding. A 13-bit datum in a u16 carrier: two carriers agreeing on
//     all 13 datum bits and differing in the 3 padding bits are the same datum.
//     A digest over the carrier bytes separates them: equal keys, unequal hashes,
//     which breaks the consistency law (k1 == k2 implies h(k1) == h(k2)) of every
//     hash container. The digest must consume the datum through the projection
//     that masks the padding, and that projection must be the only door.
//
// (b) datum classes. A NanOnly-style model where several payloads carry the one
//     NaN value: a datum-keyed digest separates data that value equality (or a
//     value-canonicalising equality) identifies. Whichever equality the container
//     uses, the digest must factor through the same layer's canonicalising
//     projection: datum-equality pairs with the datum digest, value-equality
//     with the value digest, and mixing the two breaks the consistency law in
//     one direction or the other.

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

// (a) padding
const DATUM_MASK: u16 = 0x1FFF; // 13 live bits

fn datum_eq(a: u16, b: u16) -> bool {
    (a & DATUM_MASK) == (b & DATUM_MASK)
}

fn digest_carrier(c: u16) -> u64 {
    fnv1a(&c.to_le_bytes())
}

fn digest_datum(c: u16) -> u64 {
    fnv1a(&(c & DATUM_MASK).to_le_bytes()) // canonicalising projection first
}

// (b) datum classes: model datum = (is_nan, payload); one NaN value, many data
#[derive(Copy, Clone)]
struct D {
    nan: bool,
    bits: u16,
}

fn value_eq(a: D, b: D) -> bool {
    if a.nan && b.nan {
        return true; // one NaN value, per the value layer
    }
    if a.nan != b.nan {
        return false;
    }
    a.bits == b.bits
}

fn digest_datum_d(d: D) -> u64 {
    let mut buf = [0u8; 3];
    buf[0] = d.nan as u8;
    buf[1..3].copy_from_slice(&d.bits.to_le_bytes());
    fnv1a(&buf)
}

fn digest_value_d(d: D) -> u64 {
    // canonicalising projection: collapse the NaN class to one representative
    let canon = if d.nan { D { nan: true, bits: 0 } } else { d };
    digest_datum_d(canon)
}

fn main() {
    // (a) equal data, padding differs
    let a: u16 = 0x1A5C;
    let b: u16 = a | 0xE000;
    assert!(datum_eq(a, b), "same 13 datum bits");
    assert_ne!(
        digest_carrier(a),
        digest_carrier(b),
        "carrier digest separates equal data: the violation is real, not hypothetical"
    );
    assert_eq!(
        digest_datum(a),
        digest_datum(b),
        "projecting first restores the consistency law"
    );

    // (b) one value, two data
    let n1 = D { nan: true, bits: 1 };
    let n2 = D { nan: true, bits: 2 };
    assert!(value_eq(n1, n2), "one NaN value at the value layer");
    assert_ne!(
        digest_datum_d(n1),
        digest_datum_d(n2),
        "datum digest separates value-equal keys: pairing it with value equality breaks lookup"
    );
    assert_eq!(
        digest_value_d(n1),
        digest_value_d(n2),
        "value digest factors through the class collapse and is consistent with value equality"
    );

    println!(
        "padding: carrier digests {:#x} vs {:#x} on one datum; \
         datum-projected digests agree at {:#x}. \
         nan class: datum digests {:#x} vs {:#x} on one value; \
         value digests agree at {:#x}.",
        digest_carrier(a),
        digest_carrier(b),
        digest_datum(a),
        digest_datum_d(n1),
        digest_datum_d(n2),
        digest_value_d(n1)
    );
}
