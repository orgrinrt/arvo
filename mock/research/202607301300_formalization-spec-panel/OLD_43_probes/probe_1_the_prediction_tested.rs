//! Probe 1: the consolidation's division prediction, tested rather than read.
//!
//! The claim under test, `26:678-681`, carried verbatim into `40:651-655`: division's
//! interior-exactness bound "is predicted (not measured) to have no finite accumulator
//! solution at all, since the exact quotient is generically not representable at any
//! width." Nothing ever measured it. This probe measures it, and finds the prediction
//! is true in the coordinates it was written in and FALSE in the coordinates the review
//! has since ratified, because the identity contract's adjustment became a rational
//! (file 28's closure fix, built by files 36/41/42) after the prediction was recorded,
//! and nobody re-ran the prediction across the coordinate change.
//!
//! CLAIM A (the prediction's true half, dyadic coordinates): no radix-2 quantum at any
//!   finite width represents the quotient 1/3. Checked: 2^F mod 3 is never 0 for
//!   F in 0..=1000 (the residue cycles 1, 2, 1, 2 and never touches 0). Under the OLD
//!   dyadic-only adjustment, the prediction stands: the quotient set of any two dyadic
//!   numerals contains 1/3, and no dyadic grid at any width contains 1/3.
//!
//! CLAIM B (the prediction's false half, ratified coordinates): under the rational
//!   adjustment the accumulator EXISTS. For zero-bias operands with quanta A1, A2 and
//!   index bound K on the divisor, the numeral with adjustment (A1/A2) * (1/lcm(1..K))
//!   contains every quotient exactly, because every divisor index k2 <= K divides
//!   lcm(1..K). Checked exhaustively at p=3 (K=7, lcm=420) and p=4 (K=15, lcm=360360):
//!   (k1 * L) % k2 == 0 for every pair, both widths.
//!
//! CLAIM C (minimality): lcm(1..K) is not one convenient choice among many; it is the
//!   least possible denominator. Every d <= K appears as a lowest-terms quotient
//!   denominator (1/d = 1/d), so any common quantum's denominator is divisible by every
//!   d <= K, hence by their lcm. Checked exhaustively at p=3: no b < 420 is divisible
//!   by every d in 1..=7.
//!
//! CLAIM D (the sharpened force of the prediction): the accumulator's precision grows
//!   as Theta(2^p) BITS, against 2p for multiplication's exact product and
//!   p + ceil(log2(n-1)) for an addition fold's interior safety. Computed exactly, in
//!   const eval, and asserted against values computed independently in Python
//!   (math.lcm) before this file was written:
//!
//!     operand precision p     2    3    4    5    6    7    8
//!     accumulator bits        5   12   23   51   95  190  370
//!
//!   (bits of K * lcm(1..K), K = 2^p - 1). p <= 6 fits u128 and is computed directly;
//!   p = 7, 8 exceed u128 (183- and 362-bit lcms) and are computed with a fixed-size
//!   [u64; 8] const bignum built from prime powers, cross-checked against the same
//!   Python values. So the accumulator exists, and at eight fractional bits it is
//!   already 370 bits wide; at p = 16 it would be on the order of 2^16 * log2(e)
//!   ~ 94500 bits. Division is therefore not "the operation with no accumulator"; it
//!   is the operation whose accumulator is exponentially wide, which is a THIRD growth
//!   class beside addition (logarithmic in arity) and multiplication (linear).
//!
//! Build: rustc --edition 2021 --crate-type lib probe_1_the_prediction_tested.rs --out-dir <dir>
//! Outcome: WORKS (all claims assert; the compile succeeding IS the measurement).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

// ---------------------------------------------------------------- CLAIM A

/// 2^F mod 3, computed by residue iteration so no width limit applies.
const fn pow2_mod3_never_zero(up_to: u32) -> bool {
    let mut r: u32 = 1; // 2^0 mod 3
    let mut f = 0;
    while f <= up_to {
        if r == 0 {
            return false;
        }
        r = (r * 2) % 3;
        f += 1;
    }
    true
}

const _CLAIM_A: () = assert!(pow2_mod3_never_zero(1000));

// ---------------------------------------------------------------- CLAIM B

const fn lcm_1_to(k: u128) -> u128 {
    let mut l: u128 = 1;
    let mut d: u128 = 2;
    while d <= k {
        // l = lcm(l, d) = l * d / gcd(l, d)
        let mut a = l;
        let mut b = d;
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        l = (l / a) * d;
        d += 1;
    }
    l
}

/// Every quotient k1/k2 (k1 in 0..=K numerator indices, k2 in 1..=K divisor indices)
/// lands exactly on the grid with denominator L: (k1 * L) % k2 == 0 for all pairs.
const fn every_quotient_on_grid(kmax: u128, l: u128) -> bool {
    let mut k1: u128 = 0;
    while k1 <= kmax {
        let mut k2: u128 = 1;
        while k2 <= kmax {
            if (k1 * l) % k2 != 0 {
                return false;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

const L3: u128 = lcm_1_to(7);
const L4: u128 = lcm_1_to(15);
const _L3_VALUE: () = assert!(L3 == 420); // cross-check vs Python math.lcm
const _L4_VALUE: () = assert!(L4 == 360360); // cross-check vs Python math.lcm
const _CLAIM_B_P3: () = assert!(every_quotient_on_grid(7, L3));
const _CLAIM_B_P4: () = assert!(every_quotient_on_grid(15, L4));

// ---------------------------------------------------------------- CLAIM C

/// No b < 420 is divisible by every d in 1..=7 (lcm minimality at p = 3).
const fn no_smaller_common_denominator(kmax: u128, l: u128) -> bool {
    let mut b: u128 = 1;
    while b < l {
        let mut d: u128 = 1;
        let mut all = true;
        while d <= kmax {
            if b % d != 0 {
                all = false;
                break;
            }
            d += 1;
        }
        if all {
            return false;
        }
        b += 1;
    }
    true
}

const _CLAIM_C: () = assert!(no_smaller_common_denominator(7, L3));

// ---------------------------------------------------------------- CLAIM D

const fn bits_u128(x: u128) -> u32 {
    128 - x.leading_zeros()
}

// p <= 6: direct in u128. Accumulator precision = bits of K * lcm(1..K).
const _ACC_P2: () = assert!(bits_u128(3 * lcm_1_to(3)) == 5);
const _ACC_P3: () = assert!(bits_u128(7 * lcm_1_to(7)) == 12);
const _ACC_P4: () = assert!(bits_u128(15 * lcm_1_to(15)) == 23);
const _ACC_P5: () = assert!(bits_u128(31 * lcm_1_to(31)) == 51);
const _ACC_P6: () = assert!(bits_u128(63 * lcm_1_to(63)) == 95);

// The multiplicative contrast at the same widths: mulnum's exact product is 2p bits.
const _MUL_P6: () = assert!(bits_u128(63u128 * 63) == 12); // 2p, not 95

// p = 7, 8: lcm(1..K) exceeds u128 (183 and 362 bits). Fixed-size bignum, 8 limbs of
// u64 (512 bits), built as the product of prime powers p^floor(log_p K), which is the
// definition of lcm(1..K).

const LIMBS: usize = 8;

const fn bn_one() -> [u64; LIMBS] {
    let mut v = [0u64; LIMBS];
    v[0] = 1;
    v
}

const fn bn_mul_u64(a: [u64; LIMBS], m: u64) -> [u64; LIMBS] {
    let mut out = [0u64; LIMBS];
    let mut carry: u128 = 0;
    let mut i = 0;
    while i < LIMBS {
        let t = (a[i] as u128) * (m as u128) + carry;
        out[i] = t as u64;
        carry = t >> 64;
        i += 1;
    }
    assert!(carry == 0); // overflow of the fixed size would falsify the bit counts
    out
}

const fn bn_bits(a: [u64; LIMBS]) -> u32 {
    let mut i = LIMBS;
    while i > 0 {
        i -= 1;
        if a[i] != 0 {
            return (i as u32) * 64 + (64 - a[i].leading_zeros());
        }
    }
    0
}

const fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

/// lcm(1..=k) as the product over primes p <= k of p^floor(log_p k).
const fn bn_lcm_1_to(k: u64) -> [u64; LIMBS] {
    let mut acc = bn_one();
    let mut p = 2;
    while p <= k {
        if is_prime(p) {
            // multiply in the largest power of p not exceeding k
            let mut pw = p;
            while pw <= k / p {
                pw *= p;
            }
            acc = bn_mul_u64(acc, pw);
        }
        p += 1;
    }
    acc
}

// Internal consistency: the bignum agrees with the direct u128 computation where both exist.
const _BN_CROSS_P6: () = assert!(bn_bits(bn_mul_u64(bn_lcm_1_to(63), 63)) == 95);

const _ACC_P7: () = assert!(bn_bits(bn_mul_u64(bn_lcm_1_to(127), 127)) == 190);
const _ACC_P8: () = assert!(bn_bits(bn_mul_u64(bn_lcm_1_to(255), 255)) == 370);

// The lcm bit lengths alone, cross-checked against Python (183 and 362 bits).
const _LCM_P7: () = assert!(bn_bits(bn_lcm_1_to(127)) == 183);
const _LCM_P8: () = assert!(bn_bits(bn_lcm_1_to(255)) == 362);
