//! Probe 2. Building the value-level reading: is it constructible at all,
//! under the same no-forbidden-feature constraints as the shipped one?
//!
//! Canonicalise-then-compare: fold the -0.0/+0.0 cohort and every NaN
//! payload to one representative bit pattern before running the identical
//! datum comparator from probe 1. This is the crossing contract's own move
//! (58:163-169, "no law may read past the canonical quotient") applied to
//! the comparator itself rather than argued about it.
//!
//! Claim under test: this reading exists, costs nothing exotic, and is
//! `const fn` (so it is a real candidate trait method on a `pub const
//! trait`, not a runtime-only compromise).
//!
//! POSITIVE CONTROL. Expected: WORKS. Both cohort members compare Equal.

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

// The datum comparator from probe 1, unchanged: the only new thing is what
// bit pattern it is handed.
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

const NAN_A: f32 = f32::from_bits(0x7fc0_0001);
const NAN_B: f32 = f32::from_bits(0x7fc0_0002);

const ZERO_ORDER: Ordering = value_total_cmp_f32(-0.0f32, 0.0f32);
const NAN_ORDER: Ordering = value_total_cmp_f32(NAN_A, NAN_B);

// COMPILED PROOF: the value-level reading places the cohort members at the
// same position. This line only compiles because both actually are Equal.
const _ZERO_IS_QUOTIENTED: () = assert!(matches!(ZERO_ORDER, Ordering::Equal));
const _NAN_IS_QUOTIENTED: () = assert!(matches!(NAN_ORDER, Ordering::Equal));

// Sanity: the value order still separates genuinely distinct values, so the
// quotient did not collapse everything.
const ONE_VS_TWO: Ordering = value_total_cmp_f32(1.0f32, 2.0f32);
const _STILL_A_REAL_ORDER: () = assert!(matches!(ONE_VS_TWO, Ordering::Less));

fn main() {
    println!("value_total_cmp_f32(-0.0, 0.0)  = {ZERO_ORDER:?} (value-level: Equal)");
    println!("value_total_cmp_f32(nan1, nan2) = {NAN_ORDER:?} (value-level: Equal)");
    println!("value_total_cmp_f32(1.0, 2.0)   = {ONE_VS_TWO:?} (still a real order)");
    println!("probe_2 WORKS: the value-level reading compiles, const, no forbidden feature");
}
