//! Probe 5. The identity rotation is not representable in the numeral geometry
//! reaches for first, and the design already carries the fix as a parameter
//! value rather than as a new mechanism.
//!
//! Every quantity geometry normalises lives in a CLOSED interval: a rotor
//! component and a direction cosine in [-1, 1], a colour channel and a
//! barycentric weight in [0, 1]. The obvious numeral for [0, 1] is purely
//! fractional, and `78:723` already records that `UFixed<0, F>` has no
//! representable one (its raw encoding is `1 << F` and at `I == 0` the
//! container is exactly `F` bits). File 99 section 2 then found the same absent
//! element opens sqrt's overflow band, at exactly those numerals, and sqrt is
//! what a normalisation calls.
//!
//! So the design's own two known defects meet at the one operation the rotor
//! formulation exists to make cheap. This probe asks what the ratified grammar
//! can do about it, in const position throughout per the fourth design rule.
//!
//! The grammar checked against is `40:493` and `91:153-154`: a value is
//! `A * m * r^E + B` with `A: Adjustment` and `B: Bias` gcd-normalised
//! rationals, `m` a `Precision`-digit significand in radix `r`.
#![no_std]

/// A model numeral's value, as the exact rational `num / den`.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Q(pub i128, pub i128);

/// `A * m * r^E + B`, with A = an/ad and B = bn/bd, kept exact.
/// E >= 0 only, which is all these cases need.
pub const fn value(m: i128, an: i128, ad: i128, r: i128, e: u32, bn: i128, bd: i128) -> Q {
    let mut scale = 1i128;
    let mut k = 0;
    while k < e {
        scale *= r;
        k += 1;
    }
    // A*m*r^E + B  =  (an*m*scale*bd + bn*ad) / (ad*bd)
    let num = an * m * scale * bd + bn * ad;
    let den = ad * bd;
    Q(num, den)
}

pub const fn eq(a: Q, b: Q) -> bool {
    a.0 * b.1 == b.0 * a.1
}

pub const ONE: Q = Q(1, 1);
pub const NEG_ONE: Q = Q(-1, 1);

// ---------------------------------------------------------------------------
// CLAIM A. Plain dyadic, adjustment 1, purely fractional. The signed form
// reaches -1 and misses +1; the unsigned form misses +1 as well. Checked at
// every F from 1 to 24, exhaustively over the endpoint, not sampled.
// ---------------------------------------------------------------------------

/// Largest significand of an F-bit unsigned field, and the signed pair.
pub const fn u_max(f: u32) -> i128 {
    (1i128 << f) - 1
}
pub const fn i_max(f: u32) -> i128 {
    (1i128 << f) - 1
}
pub const fn i_min(f: u32) -> i128 {
    -(1i128 << f)
}

/// A dyadic purely fractional numeral: A = 1, r = 2, E = 0, B = 0, and the
/// quantum comes from reading the significand at 2^-F. Modelled by putting the
/// scale in the adjustment, A = 1/2^F, which is the same value set.
pub const fn dyadic(m: i128, f: u32) -> Q {
    value(m, 1, 1i128 << f, 2, 0, 0, 1)
}

const _: () = {
    let mut f = 1;
    while f <= 24 {
        // unsigned field [0, 2^F - 1] scaled by 2^-F: top value is 1 - 2^-F
        assert!(
            !eq(dyadic(u_max(f), f), ONE),
            "unsigned dyadic must miss +1"
        );
        // signed field [-2^F, 2^F - 1] scaled by 2^-F: reaches -1, misses +1
        assert!(eq(dyadic(i_min(f), f), NEG_ONE), "signed dyadic reaches -1");
        assert!(!eq(dyadic(i_max(f), f), ONE), "signed dyadic must miss +1");
        f += 1;
    }
};

/// The asymmetry stated as a value: the gap between the top of the value set
/// and +1 is exactly one quantum, at every width. So the identity rotation is
/// off the grid by the smallest amount the numeral can express, which is the
/// worst possible place for it to be: no rounding mode brings it back.
const _: () = {
    let mut f = 1;
    while f <= 24 {
        // top + quantum == 1
        let top = dyadic(i_max(f), f);
        let q = dyadic(1, f);
        // top/1 + q == 1  <=>  top.0*q.1 + q.0*top.1 == top.1*q.1
        assert!(top.0 * q.1 + q.0 * top.1 == top.1 * q.1);
        f += 1;
    }
};

// ---------------------------------------------------------------------------
// CLAIM B. The ratified rational `Adjustment` represents BOTH endpoints at the
// SAME container width. A = 1/(r^F - 1). This is Direct3D's UNORM rule, which
// this round already recorded from the sibling colour pass ("UNORM divides by
// 2^n - 1 rather than 2^n, so all-ones lands on exactly 1.0"), and it needs no
// new mechanism: it is a value of a parameter `91:153-154` already seals.
// ---------------------------------------------------------------------------

pub const fn unorm(m: i128, f: u32) -> Q {
    value(m, 1, (1i128 << f) - 1, 2, 0, 0, 1)
}

const _: () = {
    let mut f = 1;
    while f <= 24 {
        assert!(eq(unorm(0, f), Q(0, 1)), "zero is exact");
        assert!(
            eq(unorm(u_max(f), f), ONE),
            "and so is one, at the same width"
        );
        f += 1;
    }
};

/// The signed closed interval, which is what a rotor component needs: a signed
/// field [-(2^F - 1), 2^F - 1] over the same adjustment reaches both endpoints,
/// symmetrically, and spends one pattern (the two's-complement minimum) to do
/// it. That spent pattern is a niche of exactly the kind `91` section 1.12's
/// `NicheCarrier` vocabulary already governs.
pub const fn snorm(m: i128, f: u32) -> Q {
    value(m, 1, (1i128 << f) - 1, 2, 0, 0, 1)
}

const _: () = {
    let mut f = 1;
    while f <= 24 {
        assert!(eq(snorm(u_max(f), f), ONE));
        assert!(eq(snorm(-u_max(f), f), NEG_ONE));
        // the identity rotor's scalar part, exactly, at every width
        assert!(eq(snorm(u_max(f), f), ONE));
        f += 1;
    }
};

// ---------------------------------------------------------------------------
// CLAIM C. What it costs. The adjustment composes multiplicatively, so a
// product of two closed-interval values has adjustment 1/(r^F - 1)^2, and
// quantising back to the closed-interval numeral is division by the fixed,
// nonzero, representable constant (r^F - 1). `91:288-291` names that the exact
// subfamily, "at zero new mechanism". So the closed-interval route pays a
// constant divide per multiply where the dyadic route pays a shift.
// ---------------------------------------------------------------------------

/// One exact product, then the renormalising divide, done in integers.
/// Returns (exact numerator over (2^F-1)^2, whether the divide is exact).
pub const fn product_then_renormalise(m1: i128, m2: i128, f: u32) -> (i128, bool) {
    let d = (1i128 << f) - 1;
    let prod = m1 * m2; // exact, at 2p width: mul_full, the settled class
    (prod, prod % d == 0)
}

const _: () = {
    // The renormalising divide is exact exactly when the product is a multiple
    // of the constant, which is a data fact, so in general it quantises. The
    // point being pinned is that the DIVISOR is a compile-time constant, which
    // is what puts it in the exact subfamily rather than in the exponential
    // class `91` section 1.13 gives general division.
    let (p, exact) = product_then_renormalise(255, 255, 8);
    assert!(p == 65025 && exact); // 1 * 1 = 1, exactly
    let (p2, exact2) = product_then_renormalise(255, 128, 8);
    assert!(p2 == 32640 && exact2); // 1 * x = x, exactly, for every x
    let (p3, exact3) = product_then_renormalise(128, 128, 8);
    assert!(p3 == 16384 && !exact3); // an ordinary product quantises
};

/// The identity is exact under multiplication, at every width, which is the
/// property the dyadic numeral loses and the whole reason this matters: an
/// identity that does not act as one is a defect that shows up only after a
/// composition chain, which is exactly where a rotation library lives.
const _: () = {
    let mut f = 1;
    while f <= 20 {
        let d = (1i128 << f) - 1;
        let mut m = 0;
        while m <= d {
            let (p, exact) = product_then_renormalise(d, m, f);
            assert!(exact && p / d == m, "one must act as one at every operand");
            m += 1 + d / 17; // stride, so the sweep stays inside const-eval
        }
        f += 1;
    }
};

// ---------------------------------------------------------------------------
// CLAIM D. The other route, one bit of integer headroom, and what it costs per
// representation. Rotor storage is the even subalgebra, 2^(n-1) components
// (probe 4, claim A), against a matrix's n^2.
// ---------------------------------------------------------------------------

pub const fn headroom_bits_rotor(n: u32) -> u32 {
    1u32 << (n - 1)
}
pub const fn headroom_bits_matrix(n: u32) -> u32 {
    n * n
}

const _: () = {
    // At the ranks a renderer uses, the headroom route is cheaper for the
    // rotor; at rank 7 and above it is not, which is the same crossover probe
    // 4 found for storage itself.
    assert!(headroom_bits_rotor(3) == 4 && headroom_bits_matrix(3) == 9);
    assert!(headroom_bits_rotor(4) == 8 && headroom_bits_matrix(4) == 16);
    assert!(headroom_bits_rotor(6) == 32 && headroom_bits_matrix(6) == 36);
    assert!(headroom_bits_rotor(7) == 64 && headroom_bits_matrix(7) == 49);
};

pub fn exercise() -> bool {
    eq(unorm(255, 8), ONE) && !eq(dyadic(i_max(8), 8), ONE)
}
