//! Probe 2: where the round-first amendment actually bites for division.
//!
//! `40:178-180` states the overflow band (exact results past the largest representable
//! but within half a quantum of it, where round-first and classify-first disagree) "is
//! empty for same-format addition and inhabited for multiplication, division,
//! mixed-format addition and every float operation." For multiplication that was
//! compiled (files 30/31). For division it was reasoned, and it is WRONG as stated:
//!
//! CLAIM A (compiled, exhaustive): for unsigned SAME-PRECISION division (operand and
//!   result numerals share one precision p, with ANY dyadic scales on all three), the
//!   band is EMPTY, for every p in 2..=8 and every scale combination. Membership
//!   reduces to: exist k1, k2 <= K = 2^p - 1 and m >= 0 with
//!       2*K*k2  <  k1 * 2^m  <=  (2*K + 1) * k2
//!   (m absorbs all three dyadic scales plus the half-quantum; m < 0 is impossible
//!   because the right endpoint is then below 2*K*k2). For m <= p+1 there is a clean
//!   algebraic proof: 2*K*k2 = k2*2^(p+1) - 2*k2 == -2*k2 (mod 2^m), so the residue is
//!   2^m - 2*k2, and a multiple of 2^m lands in the half-open window of width k2 only
//!   if the residue >= 2^m - k2, i.e. k2 <= 0. For m in p+2 ..= 2p+2 no such clean
//!   argument was found and the range is swept exhaustively instead. Contrast, same
//!   probe: same-precision MULTIPLICATION does inhabit the band (11/8 * 11/8 = 121/64
//!   lies in (15/8, 31/16] at p=4, F=3), so 40's sentence is right about
//!   multiplication and wrong about division: division patterns with ADDITION here,
//!   not with multiplication.
//!
//! CLAIM B (compiled): the band IS inhabited for division the moment operand and
//!   result precisions decouple, which the ratified design makes first-class (MATLAB's
//!   SpecifyPrecision quantises into a consumer-chosen third numeral, `40:580-583`).
//!   Witness, checked by exact integer arithmetic: 196/13 = 15.0769... with 8-bit
//!   integer operands, quantised into the p=4, F=0 numeral (Vmax = 15): the exact
//!   quotient lies strictly inside (15, 15.5), so round-first delivers 15 (in range,
//!   no overflow) while classify-first sees quotient > Vmax and declares overflow.
//!   The two orders genuinely diverge, on an input a real MATLAB-shaped composition
//!   produces. A second witness pins the tie point: 31/2 = 15.5 exactly; ties-to-even
//!   rounds to 16 (15 is odd), which IS past the range, so round-first also overflows
//!   there, by the tie rule rather than by classification, which is the amendment's
//!   own distinction (`40:170-177`) arriving at division.
//!
//! Consequence for the shape: the round-first, classify-second quantiser needs no
//! division-specific amendment (CLAIM B shows it doing exactly its job on quotients),
//! and for the same-precision family the two orders agree everywhere (CLAIM A), so no
//! implementation of that family can be distinguished by this test. The correction
//! owed to the consolidation is one word: division moves from the "inhabited" list to
//! a per-format-triple statement.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_2_the_overflow_band_for_division.rs --out-dir <dir>
//! Outcome: WORKS (all claims assert).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

// ---------------------------------------------------------------- CLAIM A

/// Does any same-precision division quotient land in the overflow band?
/// Band membership: 2*K*k2 < k1*2^m <= (2*K+1)*k2, k1, k2 in 1..=K, m in 0..=mmax.
const fn same_precision_band_inhabited(p: u32) -> bool {
    let k_max: u128 = (1u128 << p) - 1;
    let mmax: u32 = 2 * p + 2; // 2^m <= (2K+1)*K < 2^(2p+1) bounds any hit; +2 margin
    let mut k2: u128 = 1;
    while k2 <= k_max {
        let lo = 2 * k_max * k2; // exclusive
        let hi = (2 * k_max + 1) * k2; // inclusive
        let mut m: u32 = 0;
        while m <= mmax {
            let step = 1u128 << m;
            // smallest multiple of 2^m strictly above lo
            let first = (lo / step + 1) * step;
            if first <= hi && first / step <= k_max {
                return true;
            }
            m += 1;
        }
        k2 += 1;
    }
    false
}

const _CLAIM_A_P2: () = assert!(!same_precision_band_inhabited(2));
const _CLAIM_A_P3: () = assert!(!same_precision_band_inhabited(3));
const _CLAIM_A_P4: () = assert!(!same_precision_band_inhabited(4));
const _CLAIM_A_P5: () = assert!(!same_precision_band_inhabited(5));
const _CLAIM_A_P6: () = assert!(!same_precision_band_inhabited(6));
const _CLAIM_A_P7: () = assert!(!same_precision_band_inhabited(7));
const _CLAIM_A_P8: () = assert!(!same_precision_band_inhabited(8));

/// The multiplication contrast: same-precision products DO inhabit the band.
/// p=4, F=3: values k/8, Vmax = 15/8, band (15/8, 31/16]. Product k1*k2/64 is in the
/// band iff 120 < k1*k2 <= 124. 11*11 = 121.
const fn same_precision_mul_band_inhabited(p: u32, f: u32) -> bool {
    let k_max: u128 = (1u128 << p) - 1;
    // product value = k1*k2 / 2^(2f); band ((K/2^f), (2K+1)/2^(f+1)]
    // membership: 2*K*2^f < k1*k2*2 <= (2K+1)*2^f  ... scale both sides by 2^(f+1):
    // k1*k2/2^(2f) > K/2^f      <=> k1*k2 > K*2^f
    // k1*k2/2^(2f) <= (2K+1)/2^(f+1) <=> 2*k1*k2 <= (2K+1)*2^(f-1)*2 ... do it directly:
    let mut k1: u128 = 1;
    while k1 <= k_max {
        let mut k2: u128 = 1;
        while k2 <= k_max {
            let prod2 = 2 * k1 * k2; // value*2^(2f+1)
            let lo2 = 2 * k_max * (1u128 << f); // Vmax*2^(2f+1) = K*2^f * 2
            let hi2 = (2 * k_max + 1) * (1u128 << f); // (Vmax + q/2)*2^(2f+1)
            if prod2 > lo2 && prod2 <= hi2 {
                return true;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    false
}

const _CLAIM_A_MUL_CONTRAST: () = assert!(same_precision_mul_band_inhabited(4, 3));
// And the specific witness named in the header: 121/64 in (120/64, 124/64].
const _CLAIM_A_MUL_WITNESS: () = assert!(120 < 11 * 11 && 11 * 11 <= 124);

// ---------------------------------------------------------------- CLAIM B

// Witness 1: 196/13 strictly inside (15, 15.5). Exact integer arithmetic:
//   196/13 > 15    <=>  196 > 195
//   196/13 < 31/2  <=>  392 < 403
const _CLAIM_B_INTERIOR: () = assert!(196 > 15 * 13 && 2 * 196 < 31 * 13);

/// Round-first at the interior witness: RNE of 196/13 onto the integer grid.
/// q0 = 15, rem = 1, 2*rem = 2 < 13, so rounds DOWN to 15: in range, no overflow.
const fn rne_div(n: u128, d: u128) -> u128 {
    let q0 = n / d;
    let r = n % d;
    if 2 * r > d || (2 * r == d && q0 % 2 == 1) {
        q0 + 1
    } else {
        q0
    }
}

const _CLAIM_B_ROUND_FIRST_IN_RANGE: () = assert!(rne_div(196, 13) == 15 && 15 <= 15);
// Classify-first sees exact quotient > Vmax (196 > 15*13) and would declare overflow:
// the two orders diverge on this input. (The divergence is the assertion pair above:
// exact > Vmax while rounded <= Vmax.)

// Witness 2: the tie. 31/2 = 15.5, q0 = 15 (odd), 2*rem == d, ties-to-even goes UP to
// 16 > Vmax = 15: round-first overflows here, via the tie rule on the unbounded grid,
// exactly the mechanism files 30/31 pinned for multiplication.
const _CLAIM_B_TIE: () = assert!(rne_div(31, 2) == 16 && 16 > 15);
