//! Probe 4. Push the fork through hilavitkutin's actual real consumer.
//!
//! `arvo-spectral/src/partition.rs:59` and `:156,181` (read fresh) classify
//! every Fiedler component by exactly one line:
//!
//!   if let Ordering::Greater = fs[i].total_cmp(zero) { class 0 } else { class 1 }
//!
//! reproduced verbatim as `classify` below. hilavitkutin's `spectral_partition`
//! step (hilavitkutin `mock/design_rounds/202605300120`) consumes exactly this
//! classification to seed `FiberGrouping`; the consumer's own doc comment
//! (arvo-spectral/src/fiedler.rs) says only the sign pattern matters, so
//! `class[i]` is the ENTIRE information hilavitkutin extracts from node i's
//! Fiedler component before the number itself is discarded.
//!
//! A Fiedler component can be NaN in practice (division by a near-zero
//! pivot inside power iteration on a near-degenerate operator; arvo-spectral
//! has no `Specials` handling yet, per 58 section 1.16's "float model"
//! being unbuilt design, not shipped code). Claim under test: under the
//! shipped datum order, which NaN happened to come out of the arithmetic
//! decides a node's class. Under the value order, it does not, because
//! every NaN maps to one canonical position.
//!
//! POSITIVE CONTROL for the instability half; the second half is the
//! comparison that would decide it the other way.

use core::cmp::Ordering;

const fn total_cmp_f32(a: f32, b: f32) -> Ordering {
    let mut left = a.to_bits() as i32;
    let mut right = b.to_bits() as i32;
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
const fn value_total_cmp_f32(a: f32, b: f32) -> Ordering {
    let l = canonicalize_f32_bits(a.to_bits()) as i32;
    let r = canonicalize_f32_bits(b.to_bits()) as i32;
    total_cmp_f32(f32::from_bits(l as u32), f32::from_bits(r as u32))
}

/// arvo-spectral/src/partition.rs:59, verbatim shape: class 0 iff strictly
/// greater than zero under the order `cmp` is instantiated with.
fn classify(x: f32, cmp: fn(f32, f32) -> Ordering) -> u8 {
    if let Ordering::Greater = cmp(x, 0.0f32) {
        0
    } else {
        1
    }
}

fn main() {
    // Two NaN payloads a real power-iteration division can plausibly
    // produce: one with the sign bit clear, one with it set, differing
    // only in which operand of a 0.0/0.0 or inf-inf produced the quiet
    // NaN. Both are equally "not a value" under the crossing contract;
    // neither should carry classification information.
    let nan_pos_sign = f32::from_bits(0x7fc0_0001); // sign bit 0
    let nan_neg_sign = f32::from_bits(0xffc0_0001); // sign bit 1, same payload otherwise

    let datum_pos = classify(nan_pos_sign, total_cmp_f32);
    let datum_neg = classify(nan_neg_sign, total_cmp_f32);
    let value_pos = classify(nan_pos_sign, value_total_cmp_f32);
    let value_neg = classify(nan_neg_sign, value_total_cmp_f32);

    println!("datum order:  class(nan, sign=0) = {datum_pos}, class(nan, sign=1) = {datum_neg}");
    println!("value order:  class(nan, sign=0) = {value_pos}, class(nan, sign=1) = {value_neg}");

    assert_ne!(
        datum_pos, datum_neg,
        "expected the datum order to classify the two NaNs differently"
    );
    assert_eq!(
        value_pos, value_neg,
        "expected the value order to classify both NaNs the same way"
    );
    println!(
        "probe_4 WORKS: under the shipped datum order, spectral_bisection's class assignment for a \
         degenerate (NaN) Fiedler component depends on the NaN's sign bit alone. Under the value \
         order it does not."
    );
}
