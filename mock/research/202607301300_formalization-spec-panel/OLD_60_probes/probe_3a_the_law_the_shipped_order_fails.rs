//! Probe 3a. The decisive one: write the law a value-level total order is
//! FOR, against the shipped datum-level mechanism, and let the compile
//! decide rather than argue about it.
//!
//! The law: "a value-respecting order places two data that denote the same
//! value at the same position." This is not an exotic requirement; it is
//! the minimum a total order has to satisfy to be usable in the design's
//! own sense of a law (58:271-278, "a claim over a numeral's VALUE set").
//! An order that fails it cannot be the thing 58:547-551 calls "a
//! value-level TotalOrd... usable by laws", whatever it is named.
//!
//! Stated as a `const` assertion (`assert!` panicking in a const context is
//! a compile error, E0080), over the cohort probe 1 already exhibited:
//! -0.0 and +0.0, the value-unique encoding's own "unrepurposed signed
//! zero" cohort (58:182-186), same value, two data.
//!
//! NEGATIVE CONTROL. Expected: FAILS TO COMPILE, E0080, "evaluation of
//! `LAW_HOLDS_FOR_SHIPPED_ORDER` failed... the evaluated program panicked".

use core::cmp::Ordering;

// Verbatim shipped mechanism, arvo/src/traits/total_ord.rs:29-41.
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

/// LAW: two data that denote the same value compare Equal.
/// -0.0 and 0.0 are the design's own textbook cohort: `decode(-0.0) ==
/// decode(0.0) == 0` (58:171-177, statement 1, decode is total and value-
/// preserving), so a value-respecting order must place them at the same
/// position.
const LAW_HOLDS_FOR_SHIPPED_ORDER: () = {
    let ord = total_cmp_f32(-0.0f32, 0.0f32);
    assert!(
        matches!(ord, Ordering::Equal),
        "the shipped order does not respect the value-equality of -0.0 and 0.0"
    );
};

fn main() {
    // Force the const to be evaluated even if nothing else references it.
    let _ = LAW_HOLDS_FOR_SHIPPED_ORDER;
    println!(
        "probe_3a: unreachable if this compiled, the law should have refused the shipped order"
    );
}
