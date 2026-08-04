//! Probe 2: the ungoverned padding region under Dense lives BETWEEN the stored
//! width and the container width, at a preset where statement P is vacuous.
//!
//! Model: Hot fixed 13-bit. Fields extent = stored width = 13 (minimum), so
//! statement P quantifies over zero bits and is vacuously satisfied. The Dense
//! container is a u16, so three physical bits exist at [13, 16) that neither
//! statement 0 nor statement P reaches.
//!
//! This probe enumerates the WHOLE matrix (8192 data times 7 nonzero padding
//! patterns, 57,344 same-value pairs), asserting:
//!   1. a compare keyed on the raw container misorders every single pair;
//!   2. one witness where raw order inverts value order outright;
//!   3. the canonical projection (the only door) reports Equal on all pairs;
//!   4. the container clause discharges by purity: the tower-side constructor
//!      is a one-argument pure function, two calls are bit-identical, and the
//!      committed padding is observable through a transmute with no API.
//!
//! Counts are asserted so an empty loop cannot pass.
//!
//! This is file 80 probe_3's finding (9-in-u16, [W_F, W_C) with W_F = W_S)
//! re-instantiated at the preset-table's own width to pin WHERE the region
//! sits in the three-level picture: it is [stored, container), not
//! [fields, stored).

const EXTENT: usize = 13; // fields extent = stored width (Hot: minimum)
const CONTAINER: usize = 16; // dense dispatch: u16
const MASK: u16 = (1u16 << EXTENT) - 1;

/// the canonical projection: the only door for value-keyed observation
fn canonical(c: u16) -> u16 {
    c & MASK
}

/// the tower-side container constructor: pure function of the datum,
/// canonical padding (the forced zero-padding of file 73, applied at the
/// second map)
fn embed_container(datum: u16) -> u16 {
    debug_assert!(datum <= MASK);
    datum // padding bits [13,16) are zero by construction
}

fn main() {
    let pad_patterns: u32 = (1 << (CONTAINER - EXTENT)) - 1; // 7 nonzero
    let data: u32 = 1 << EXTENT; // 8192

    // 1 + 3: the whole matrix, no sample
    let mut pairs_checked: u64 = 0;
    let mut raw_misordered: u64 = 0;
    let mut canonical_equal: u64 = 0;
    for d in 0..data {
        let clean = embed_container(d as u16);
        for p in 1..=pad_patterns {
            let dirty = clean | ((p as u16) << EXTENT);
            pairs_checked += 1;
            // same datum, same value; raw container compare must NOT be
            // consulted, and here is what it says if it is:
            if clean.cmp(&dirty) != core::cmp::Ordering::Equal {
                raw_misordered += 1;
            }
            if canonical(clean) == canonical(dirty) {
                canonical_equal += 1;
            }
        }
    }
    assert_eq!(pairs_checked, 57_344, "matrix size");
    assert_eq!(raw_misordered, pairs_checked, "every pair misorders raw");
    assert_eq!(
        canonical_equal, pairs_checked,
        "every pair equal canonically"
    );

    // 2: raw order inverts value order outright: a dirty zero above the
    // largest clean datum
    let dirty_zero = 0u16 | (1u16 << EXTENT); // value 0, padding bit set
    let clean_max = MASK; // value 8191
    assert!(dirty_zero > clean_max, "raw order inverts value order");
    assert!(canonical(dirty_zero) < canonical(clean_max));

    // 4: purity + perimeter. Two constructions of the same datum are
    // bit-identical, and the committed padding is observable with no API.
    for d in [0u16, 1, 0x0AAA, MASK] {
        let a = embed_container(d);
        let b = embed_container(d);
        assert_eq!(a, b, "constructor is pure");
        let bytes: [u8; 2] = a.to_ne_bytes();
        let via_transmute: [u8; 2] = unsafe { core::mem::transmute(a) };
        assert_eq!(bytes, via_transmute);
        // the padding region is zero in the committed image
        assert_eq!(a >> EXTENT, 0, "container padding is canonical");
    }

    println!(
        "OK: {} pairs, raw misorders all, canonical door equal on all, \
         constructor pure, padding canonical",
        pairs_checked
    );
}
