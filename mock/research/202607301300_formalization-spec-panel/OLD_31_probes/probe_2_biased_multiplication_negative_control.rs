//! Probe 2: the negative control for `30_probes/probe_5`'s closure formula,
//! compiled rather than narrated.
//!
//! `30_probes/OUTCOMES.md` describes a negative control for the biased-
//! multiplication closure formula (drop the cross terms, use the naive
//! `A1*A2` as the product adjustment) without committing it as compiled
//! source. This file is that negative control, on the exact operand pair
//! `30_probes/probe_5` uses for its own primary witness
//! (`A1=4, B1=2, A2=6, B2=4`), plus the specific failing pair found by
//! exhaustive search rather than picked by hand.
//!
//! Every claim is a `const` assertion; compiling is the check.

#![no_std]

const A1: i64 = 4;
const B1: i64 = 2; // values 2, 6, 10, 14
const A2: i64 = 6;
const B2: i64 = 4; // values 4, 10, 16, 22

/// The WRONG adjustment: cross terms dropped, exactly as if biased
/// multiplication were treated the same as the unbiased case.
const NAIVE_ADJUSTMENT: i64 = A1 * A2; // 24
const BIAS: i64 = B1 * B2; // 8, unaffected by the bug being checked

/// Does every product land on the grid the naive (wrong) adjustment
/// describes?
const fn every_product_on_naive_grid(k1max: i64, k2max: i64) -> bool {
    let mut k1 = 0;
    while k1 <= k1max {
        let v1 = A1 * k1 + B1;
        let mut k2 = 0;
        while k2 <= k2max {
            let v2 = A2 * k2 + B2;
            let p = v1 * v2;
            if (p - BIAS) % NAIVE_ADJUSTMENT != 0 {
                return false;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

/// The bug is real over the same window `30_probes/probe_5` checks the
/// correct formula against.
const _: () = assert!(!every_product_on_naive_grid(3, 3));

/// The specific failing pair, found by exhaustive search rather than
/// selected: the first one, in `(k1, k2)` order, that the naive formula
/// gets wrong.
const fn first_failure(k1max: i64, k2max: i64) -> (i64, i64, i64, i64) {
    let mut k1 = 0;
    while k1 <= k1max {
        let v1 = A1 * k1 + B1;
        let mut k2 = 0;
        while k2 <= k2max {
            let v2 = A2 * k2 + B2;
            let p = v1 * v2;
            let rem = (p - BIAS) % NAIVE_ADJUSTMENT;
            if rem != 0 {
                return (k1, k2, p, rem);
            }
            k2 += 1;
        }
        k1 += 1;
    }
    (-1, -1, -1, -1)
}

const FIRST_FAILURE: (i64, i64, i64, i64) = first_failure(3, 3);

/// k1 = 0, k2 = 1: v1 = 2, v2 = 10, product = 20, and (20 - 8) mod 24 = 12,
/// not zero. Twenty is not on the naive 24-adjustment, 8-bias grid at all;
/// the true grid (adjustment 4, from `gcd(24, 16, 12)`, per probe 5) places
/// it correctly, since (20 - 8) mod 4 = 0.
const _: () = assert!(FIRST_FAILURE.0 == 0);
const _: () = assert!(FIRST_FAILURE.1 == 1);
const _: () = assert!(FIRST_FAILURE.2 == 20);
const _: () = assert!(FIRST_FAILURE.3 == 12);

/// Confirming the true formula (carried over from probe 5, restated here so
/// this file stands alone) does place the same value correctly.
const fn gcd(a: i64, b: i64) -> i64 {
    let mut x = if a < 0 { -a } else { a };
    let mut y = if b < 0 { -b } else { b };
    while y != 0 {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}
const TRUE_ADJUSTMENT: i64 = gcd(gcd(A1 * A2, A1 * B2), A2 * B1);
const _: () = assert!(TRUE_ADJUSTMENT == 4);
const _: () = assert!((20 - BIAS) % TRUE_ADJUSTMENT == 0);
