//! Probe 4: the finite exact carrier for general division is a pair, and the design's
//! own machinery already sizes both halves.
//!
//! Probe 1 establishes that the single-numeral exact carrier for general division
//! exists only at exponential width. This probe establishes the alternative: the exact
//! content of a/b at FINITE, linear width is the Euclidean pair (q, r) with
//! a = q*b + r, 0 <= r < b, and correct rounding to any result grid is a function of a
//! remainder against that grid, never of the (unrepresentable) exact quotient. This is
//! the relocation question the dispatch names, answered: the multiplicative move
//! (make the exact intermediate a typed object, push rounding into the narrowing)
//! transfers to division only after the exact intermediate changes KIND, from one
//! numeral to a quotient-remainder pair.
//!
//! Everything below is exhaustive at a model width, mixed quanta chosen deliberately:
//! dividend on the 1/4 grid (A1 = 1/4, indices 0..=15), divisor on the 1/3 grid
//! (A2 = 1/3, indices 1..=15), so the remainder grid is forced to be the gcd quantum
//! gcd(1/4, 1/3) = 1/12, the same four-monomial gcd family the MAC accumulator already
//! uses (`40:345-350`, here with zero biases so it collapses to gcd(A1, A2)). All
//! arithmetic is exact integer arithmetic in 1/12 units.
//!
//! CLAIM A (the pair is exact and finite): for every pair, q = floor(a/b) is an
//!   integer, r = a - q*b lies on the 1/12 grid by construction (it is an integer in
//!   1/12 units), and 0 <= r < b. Asserted exhaustively.
//!
//! CLAIM B (both halves' numerals are computed by existing identity machinery):
//!   the quotient's index bound is floor(maxV(N1) / minposV(N2)) = floor((15/4)*3)
//!   = 11, a formula over the identity axes (index bound and adjustment ratio), and
//!   the exhaustive maximum observed equals it exactly. The remainder's numeral is the
//!   zero-bias gcd-quantum numeral with index bound < maxidx(N2) * (A2/gcd) = 15*4
//!   = 60; the observed maximum remainder index is asserted < 60. Both bounds are
//!   type-level computable with the Ratio/Reduce machinery probes 3 and 42/6 already
//!   exercise; nothing new.
//!
//! CLAIM C (rounding is a function of the pair): the correctly rounded quotient onto
//!   the result grid 1/4 (RNE), computed from the remainder of the SCALED integer
//!   division (the way every hardware divider and every libm does it), agrees with an
//!   argmin-by-cross-multiplication oracle written from the definition of rounding
//!   (scan candidates, compare |v - n*q| by cross-multiplication, break ties to even),
//!   for every pair. The two computations share no code path; the oracle never divides.
//!
//! CLAIM D (double-rounding control): rounding the exact quotient to the 1/12 grid
//!   first and then to the 1/4 grid does NOT always agree with rounding once (the
//!   two-step path diverges on at least one input). This pins, for division, the same
//!   fact file 24's probe 03 pinned for multiplication: the single quantisation in
//!   `quantize(exact quotient)` is load-bearing, not a stylistic choice, so a shipped
//!   divider that rounds an intermediate must use a width/order combination proven
//!   equivalent, not assumed.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_4_the_euclidean_pair.rs --out-dir <dir>
//! Outcome: WORKS (claims A-C assert exhaustively; claim D's divergence witness found
//! and asserted).
//! rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

// Units: 1/12. Dividend a = 3*k1 (k1 in 0..=15, value k1/4). Divisor b = 4*k2
// (k2 in 1..=15, value k2/3). Result grid 1/4 = 3 units.

const fn floor_div(a: i128, b: i128) -> i128 {
    // both nonnegative here
    a / b
}

/// CLAIM A + B: Euclidean pair exactness, plus the observed bounds.
const fn euclid_exhaustive() -> (i128, i128) {
    let mut max_q: i128 = 0;
    let mut max_r: i128 = 0;
    let mut k1: i128 = 0;
    while k1 <= 15 {
        let mut k2: i128 = 1;
        while k2 <= 15 {
            let a = 3 * k1; // 1/12 units
            let b = 4 * k2; // 1/12 units
            let q = floor_div(a, b); // dimensionless integer
            let r = a - q * b; // 1/12 units: exact by construction
            assert!(r >= 0 && r < b); // Euclid
            assert!(q * b + r == a); // the defining law, exactly
            if q > max_q {
                max_q = q;
            }
            if r > max_r {
                max_r = r;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    (max_q, max_r)
}

const EUCLID: (i128, i128) = euclid_exhaustive();
// Quotient bound from the identity axes: floor(maxV1 / minposV2) = floor((15/4)/(1/3)) = 11.
const _CLAIM_B_Q: () = assert!(EUCLID.0 == 11);
// Remainder bound: r < b <= maxV2 = 15/3 = 5 = 60 units.
const _CLAIM_B_R: () = assert!(EUCLID.1 < 60);

/// RNE of the rational (num/den) onto the grid with quantum 1/g (result index n:
/// value n/g), computed the divider way: scale, divide, compare twice the remainder.
const fn rne_scaled(num: i128, den: i128, g: i128) -> i128 {
    let t = num * g;
    let q0 = t / den;
    let r = t % den;
    if 2 * r > den || (2 * r == den && q0 % 2 == 1) {
        q0 + 1
    } else {
        q0
    }
}

/// The definition-shaped oracle: scan result indices, pick the nearest by
/// cross-multiplication, break ties to even. Never divides.
const fn rne_argmin(num: i128, den: i128, g: i128, n_max: i128) -> i128 {
    // |num/den - n/g| comparison via |num*g - n*den| (den, g > 0)
    let mut best_n: i128 = 0;
    let mut best_err: i128 = i128::MAX;
    let mut n: i128 = 0;
    while n <= n_max {
        let e = num * g - n * den;
        let e = if e < 0 { -e } else { e };
        if e < best_err || (e == best_err && n % 2 == 0) {
            best_err = e;
            best_n = n;
        }
        n += 1;
    }
    best_n
}

/// CLAIM C: the two agree on every pair (quotients here reach 45/4 = 11.25, so the
/// scan bound 4*12 covers the range with margin).
const fn rounding_is_a_function_of_the_pair() -> bool {
    let mut k1: i128 = 0;
    while k1 <= 15 {
        let mut k2: i128 = 1;
        while k2 <= 15 {
            // exact quotient = (k1/4)/(k2/3) = 3*k1 / (4*k2)
            let num = 3 * k1;
            let den = 4 * k2;
            let a = rne_scaled(num, den, 4);
            let b = rne_argmin(num, den, 4, 48);
            if a != b {
                return false;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

const _CLAIM_C: () = assert!(rounding_is_a_function_of_the_pair());

/// CLAIM D: double rounding (to 1/12 first, then to 1/4) diverges from single
/// rounding on at least one input. Returns the count of divergent pairs.
const fn double_rounding_divergences() -> i128 {
    let mut divergent: i128 = 0;
    let mut k1: i128 = 0;
    while k1 <= 15 {
        let mut k2: i128 = 1;
        while k2 <= 15 {
            let num = 3 * k1;
            let den = 4 * k2;
            let once = rne_scaled(num, den, 4);
            // two-step: round to the 1/12 grid, then round that to the 1/4 grid
            let mid = rne_scaled(num, den, 12); // index on 1/12 grid
            let twice = rne_scaled(mid, 12, 4); // value mid/12, re-rounded to 1/4
            if once != twice {
                divergent += 1;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    divergent
}

const _CLAIM_D: () = assert!(double_rounding_divergences() > 0);
