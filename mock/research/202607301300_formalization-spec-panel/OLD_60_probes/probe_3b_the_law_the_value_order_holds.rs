//! Probe 3b. The identical law from probe 3a, against the value-level
//! reading from probe 2. Same assertion text, same cohort, only the
//! comparator changes.
//!
//! POSITIVE CONTROL. Expected: WORKS. Compiles clean; the const assertion
//! evaluates to `()` rather than panicking.

use core::cmp::Ordering;

const fn is_nan_bits(bits: u32) -> bool {
    (bits & 0x7f80_0000) == 0x7f80_0000 && (bits & 0x007f_ffff) != 0
}

const CANON_NAN_BITS: u32 = 0x7fc0_0000;

const fn canonicalize_f32_bits(bits: u32) -> u32 {
    if is_nan_bits(bits) {
        CANON_NAN_BITS
    } else if bits == 0x8000_0000 {
        0
    } else {
        bits
    }
}

const fn total_cmp_bits(left: i32, right: i32) -> Ordering {
    let mut left = left;
    let mut right = right;
    left ^= (((left >> 31) as u32) >> 1) as i32;
    right ^= (((right >> 31) as u32) >> 1) as i32;
    if left < right {
        Ordering::Less
    } else if left > right {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

const fn value_total_cmp_f32(a: f32, b: f32) -> Ordering {
    total_cmp_bits(
        canonicalize_f32_bits(a.to_bits()) as i32,
        canonicalize_f32_bits(b.to_bits()) as i32,
    )
}

/// LAW: identical wording to probe 3a. Only the order under test changed.
const LAW_HOLDS_FOR_VALUE_ORDER: () = {
    let ord = value_total_cmp_f32(-0.0f32, 0.0f32);
    assert!(
        matches!(ord, Ordering::Equal),
        "the value order does not respect the value-equality of -0.0 and 0.0"
    );
};

fn main() {
    let _ = LAW_HOLDS_FOR_VALUE_ORDER;
    println!("probe_3b WORKS: the identical law holds under the value-level reading");
}
