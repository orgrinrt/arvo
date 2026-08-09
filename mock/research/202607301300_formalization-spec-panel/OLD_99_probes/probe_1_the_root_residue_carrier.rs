//! Probe 1: the root-residue carrier.
//!
//! Question: does file 24's relocation (rounding moves to the narrowing) and file 43's
//! carrier-kind move (the exact intermediate changes kind rather than transferring as a
//! template) extend to `Sqrt`? What is sqrt's finite exact carrier, what width does it
//! need, can it tie, and is its overflow band inhabited?
//!
//! Model: same-numeral unsigned sqrt. Operand and result on the dyadic grid k/2^F,
//! indices 0..2^P. The true result index is t = sqrt(k * 2^F); the floor candidate is
//! m = isqrt(k << F) and the residue is r = (k << F) - m^2, so that
//!     m^2 + r = k * 2^F,   0 <= r <= 2m.
//! Correct rounding to nearest is a one-comparison function of the pair (m, r):
//!     round up  <=>  t > m + 1/2  <=>  4(m^2 + r) > (2m+1)^2  <=>  r > m.
//! A tie would require 4r = 4m + 1: even = odd, impossible. So nearest-rounding sqrt
//! on a dyadic grid CANNOT tie, which is why no direction triple's tie rule is ever
//! consulted (the design-level parallel of IEEE sqrt never raising an inexact tie
//! anomaly, and of hardware sqrt needing no sticky tie path).
//!
//! Every claim below is exhaustive over the full index set at each (P, F), evaluated
//! in const position, and pinned against Python pre-computations run before this file
//! was written (see OUTCOMES.md). Compilation of this file is the verification.

// exact integer sqrt, restoring, bit by bit; const-evaluable, O(bits) steps.
const fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = x.div_ceil(2);
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

const fn icbrt(n: u64) -> u64 {
    let mut m = 0u64;
    while (m + 1) * (m + 1) * (m + 1) <= n {
        m += 1;
    }
    m
}

/// One exhaustive sweep. Returns (exact_hits, round_ups, ties, max_residue,
/// max_operand_bits, overflow_count, zero_flush_count).
const fn sqrt_sweep(p: u32, f: u32) -> (u64, u64, u64, u64, u32, u64, u64) {
    let m_max = (1u64 << p) - 1;
    let (mut exact, mut ups, mut ties, mut over, mut zflush) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut max_r = 0u64;
    let mut max_w = 0u32;
    let mut k = 0u64;
    while k <= m_max {
        let x = k << f; // scaled operand; the whole carrier is (m, r) over this integer
        let m = isqrt(x);
        let r = x - m * m;
        if r > max_r {
            max_r = r;
        }
        let w = 64 - x.leading_zeros();
        if w > max_w {
            max_w = w;
        }
        let res = if r == 0 {
            exact += 1;
            m
        } else {
            // tie test, exact: (2m+1)^2 == 4x. parity makes it unsatisfiable;
            // the check is here so the claim is measured rather than argued.
            if (2 * m + 1) * (2 * m + 1) == 4 * x {
                ties += 1;
            }
            if r > m {
                ups += 1;
                m + 1
            } else {
                m
            }
        };
        if res > m_max {
            over += 1;
        }
        if k >= 1 && res == 0 {
            zflush += 1;
        }
        k += 1;
    }
    (exact, ups, ties, max_r, max_w, over, zflush)
}

/// CLAIM A: the residue rule agrees with a definition-shaped oracle that never
/// computes a square root: argmin over candidate indices c of |k*2^F - c^2| in
/// cross-multiplied form (compare 4x against (2c+1)^2 window boundaries).
const fn oracle_agrees(p: u32, f: u32) -> bool {
    let m_max = (1u64 << p) - 1;
    let mut k = 0u64;
    while k <= m_max {
        let x = k << f;
        // fast path: residue rule
        let m = isqrt(x);
        let r = x - m * m;
        let fast = if r == 0 {
            m
        } else if r > m {
            m + 1
        } else {
            m
        };
        // oracle: scan every candidate c in 0..=m_max+1, pick the c whose squared
        // distance |x - c^2| is least, exact integers only; ties would surface as
        // two minima, and none ever does (the tie claim, re-checked by another route).
        let mut best = 0u64;
        let mut best_d = u64::MAX;
        let mut double_min = false;
        let mut c = 0u64;
        while c <= m_max + 1 {
            // |sqrt(x) - c| ordering is NOT |x - c^2| ordering in general, so the
            // oracle compares by the true metric in cross-multiplied form:
            // |sqrt(x) - c| < |sqrt(x) - b| for c < b  <=>  x*4 > ... ; for a
            // monotone scan it suffices to test "is sqrt(x) closer to c than to
            // c+1", i.e. sqrt(x) < c + 1/2, i.e. 4x < (2c+1)^2, and take the
            // first c where that holds.
            if 4 * x < (2 * c + 1) * (2 * c + 1) {
                best = c;
                best_d = 0;
                break;
            }
            if 4 * x == (2 * c + 1) * (2 * c + 1) {
                double_min = true; // a genuine tie
            }
            c += 1;
        }
        if best_d != 0 || double_min || best != fast {
            return false;
        }
        k += 1;
    }
    true
}

/// CLAIM E: the same carrier shape for the cube root: m = icbrt(k << 2F),
/// residue r = X - m^3, round up iff 8X > (2m+1)^3, tie parity-impossible.
const fn cbrt_sweep(p: u32, f: u32) -> (u64, u64, u64) {
    let m_max = (1u64 << p) - 1;
    let (mut exact, mut ups, mut ties) = (0u64, 0u64, 0u64);
    let mut k = 0u64;
    while k <= m_max {
        let x = k << (2 * f);
        let m = icbrt(x);
        let r = x - m * m * m;
        if r == 0 {
            exact += 1;
        } else {
            let b = (2 * m + 1) * (2 * m + 1) * (2 * m + 1);
            if b == 8 * x {
                ties += 1;
            }
            if 8 * x > b {
                ups += 1;
            }
        }
        k += 1;
    }
    (exact, ups, ties)
}

// ---- pinned expectations, Python pre-computed (OUTCOMES.md), asserted in const position ----

const S_2_2: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(2, 2);
const S_3_3: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(3, 3);
const S_4_2: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(4, 2);
const S_4_4: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(4, 4);
const S_6_6: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(6, 6);
const S_8_4: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(8, 4);
const S_8_8: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(8, 8);
const S_2_4: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(2, 4); // far < 1 - q
const S_3_6: (u64, u64, u64, u64, u32, u64, u64) = sqrt_sweep(3, 6); // far < 1 - q

const _: () = {
    // CLAIM B: ties are impossible, every (P, F), full index set.
    assert!(S_2_2.2 == 0 && S_3_3.2 == 0 && S_4_2.2 == 0 && S_4_4.2 == 0);
    assert!(S_6_6.2 == 0 && S_8_4.2 == 0 && S_8_8.2 == 0 && S_2_4.2 == 0 && S_3_6.2 == 0);
    // CLAIM C: overflow band empty exactly when M >= 2^F - 1 (far point >= 1 - q),
    // inhabited otherwise, with the Python-pinned witness counts.
    assert!(S_2_2.5 == 0 && S_3_3.5 == 0 && S_4_2.5 == 0 && S_4_4.5 == 0);
    assert!(S_6_6.5 == 0 && S_8_4.5 == 0 && S_8_8.5 == 0);
    assert!(S_2_4.5 == 3 && S_3_6.5 == 7); // the no-ONE numerals: sqrt escapes upward
                                           // CLAIM C': no nonzero operand ever rounds to zero: UnderRange-to-zero empty always.
    assert!(
        S_2_2.6 == 0
            && S_3_3.6 == 0
            && S_4_2.6 == 0
            && S_4_4.6 == 0
            && S_6_6.6 == 0
            && S_8_4.6 == 0
            && S_8_8.6 == 0
            && S_2_4.6 == 0
            && S_3_6.6 == 0
    );
    // CLAIM D: the carrier is linear-width. The widest integer the whole decision
    // touches is the scaled operand itself, P + F bits (16 at (8,8)), and the residue
    // is below 2^((P+F)/2 + 1) (508 at (8,8)). Pinned against Python.
    assert!(S_8_8.4 == 16 && S_8_8.3 == 508);
    assert!(S_4_4.4 == 8 && S_4_4.3 == 28);
    assert!(S_6_6.4 == 12 && S_6_6.3 == 124);
    // exact-hit counts, pinned (perfect squares of the scaled operand):
    assert!(S_2_2.0 == 2 && S_3_3.0 == 2 && S_4_2.0 == 4 && S_4_4.0 == 4);
    assert!(S_6_6.0 == 8 && S_8_4.0 == 16 && S_8_8.0 == 16);
    // round-up counts, pinned:
    assert!(S_2_2.1 == 1 && S_3_3.1 == 4 && S_4_2.1 == 6 && S_4_4.1 == 8);
    assert!(S_6_6.1 == 34 && S_8_4.1 == 128 && S_8_8.1 == 134);
};

const _: () = {
    // CLAIM A: residue rule == never-rooting oracle, exhaustive.
    assert!(oracle_agrees(4, 4));
    assert!(oracle_agrees(6, 6));
    assert!(oracle_agrees(8, 8));
    assert!(oracle_agrees(8, 4));
};

const C_4_2: (u64, u64, u64) = cbrt_sweep(4, 2);
const C_6_3: (u64, u64, u64) = cbrt_sweep(6, 3);
const _: () = {
    // CLAIM E: the cube root has the same carrier shape, ties parity-impossible,
    // counts pinned against Python: (2, 7, 0) and (4, 30, 0).
    assert!(C_4_2.0 == 2 && C_4_2.1 == 7 && C_4_2.2 == 0);
    assert!(C_6_3.0 == 4 && C_6_3.1 == 30 && C_6_3.2 == 0);
};
