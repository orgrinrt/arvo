//! Probe 1. Is the shipped `TotalOrd::total_cmp` for floats datum-level or
//! value-level?
//!
//! `total_cmp_f32` below is a verbatim copy of the shipped mechanism at
//! `arvo/src/traits/total_ord.rs:29-41` (read fresh for this dispatch), the
//! const-callable reimplementation of `f32::total_cmp` (the XOR-mask trick).
//! I did not invent it; I reproduce it here because a probe cannot depend on
//! the `arvo` crate directly without pulling in the whole strategy/storage
//! tower for one function, and because the point under test is the shipped
//! ALGORITHM, not the wrapper type it is attached to (`FastFloat`/
//! `StrictFloat` are `repr(transparent)` over the bare float, so operating on
//! the bare float is operating on the identical bit pattern the wrapper
//! carries).
//!
//! Claim under test: this order is datum-level (distinguishes -0.0 from
//! +0.0, which the value coordinates say are the same value: the crossing
//! contract's own "unrepurposed signed zero" cohort, 58:182-186).
//!
//! POSITIVE CONTROL. Expected: WORKS, and the printed evidence shows -0.0
//! strictly less than +0.0, and two differently-payloaded NaNs strictly
//! ordered against each other.

use core::cmp::Ordering;

// Verbatim copy of arvo/src/traits/total_ord.rs:29-41.
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

const NAN_A: f32 = f32::from_bits(0x7fc0_0001); // quiet NaN, payload bit 0 set
const NAN_B: f32 = f32::from_bits(0x7fc0_0002); // quiet NaN, payload bit 1 set

const ZERO_ORDER: Ordering = total_cmp_f32(-0.0f32, 0.0f32);
const NAN_ORDER: Ordering = total_cmp_f32(NAN_A, NAN_B);

// COMPILED PROOF, not a runtime observation: if the shipped mechanism were
// value-level it would place -0.0 and +0.0 at the same position (Equal).
// It does not; this line only compiles because it is not Equal.
const _ZERO_IS_STRICT: () = assert!(!matches!(ZERO_ORDER, Ordering::Equal));
// Likewise the two NaN payloads are placed at different positions.
const _NAN_IS_STRICT: () = assert!(!matches!(NAN_ORDER, Ordering::Equal));

fn main() {
    println!("total_cmp_f32(-0.0, 0.0)  = {ZERO_ORDER:?} (shipped: NOT Equal)");
    println!("total_cmp_f32(nan1, nan2) = {NAN_ORDER:?} (shipped: NOT Equal)");
    assert!(matches!(ZERO_ORDER, Ordering::Less));
    println!("probe_1 WORKS: shipped total_cmp is datum-level (distinguishes -0.0 from 0.0 and NaN payloads)");
}
