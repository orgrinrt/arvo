//! The crossing contract as a `const fn` model, parameterised by exactly the axes the
//! type-level numeral and its encoding carry, so a probe drives it from a numeral type
//! rather than from hand-written parameters.
//!
//! The contract is the settled three-statement form (`49:161-168`, from files 30 and 31):
//!
//!   1. `decode . encode = id` on values, always.
//!   2. `encode . decode` is idempotent on data, always. This is canonicalisation.
//!   3. `encode . decode = id` on data iff the encoding is injective, a derived boolean.
//!
//! Statement 3's "derived boolean" has been asserted and never derived. This file derives
//! it, as a closed-form predicate over the axes, and checks the prediction against
//! exhaustive enumeration of the datum space at model widths, in both directions.
//!
//! Coordinates. A value is `(sign, m, q)` denoting `(-1)^sign * m * radix^q`, identified up
//! to the strip normal form (m carries no factor of radix), plus the two special classes.
//! The strip form is the VALUE identity and is not a choice. Which representable datum
//! `encode` picks is the SECTION, and it is a choice: `Encoding::Canonical`'s content.
//! Keeping those two apart is the whole content of the crossing contract, and radix ten is
//! where they come apart, because there the section is not determined by the value set.

#![allow(dead_code)]

// tag: 0 none, 1 finite, 2 infinity, 3 NaN
#[derive(Copy, Clone)]
pub struct Val {
    pub tag: u8,
    pub neg: bool,
    pub m: i64,
    pub q: i32,
}

pub const NONE: Val = Val {
    tag: 0,
    neg: false,
    m: 0,
    q: 0,
};

pub const fn val_eq(a: Val, b: Val) -> bool {
    if a.tag != b.tag {
        return false;
    }
    match a.tag {
        0 => true,
        3 => true, // one NaN value class; payloads are datum facts
        2 => a.neg == b.neg,
        _ => a.neg == b.neg && a.m == b.m && a.q == b.q,
    }
}

/// Every axis the model reads. Each field names where it lives in the design.
#[derive(Copy, Clone)]
pub struct Fmt {
    /// `Numeral::Radix`.
    pub r: i64,
    /// `Numeral::Precision`, in digits of the radix.
    pub p: u32,
    /// Quantum exponent of the bottom row, derived from `Ranged`'s EMIN and the precision.
    pub qmin: i32,
    /// Quantum exponent of the top row.
    pub qmax: i32,
    /// `Numeral::Domain`, the value fact.
    pub signed: bool,
    /// `Numeral::Exponent`'s nested `Specials::INF`.
    pub inf: bool,
    /// How many NaN data the encoding reserves. Zero means `Specials::NAN` is false.
    pub nan_data: i64,
    /// `Encoding::Fields`: does the significand carry a hidden leading digit, so that only
    /// normalised significands are stored? Radix two can; radix ten cannot, because there
    /// is no constant leading digit to hide.
    pub normalised: bool,
    /// `Numeral`'s `Underflow`: is the bottom row extended down to zero?
    pub gradual: bool,
    /// `Encoding::Canonical`: does the would-be negative zero datum exist as a zero, or is
    /// it repurposed? OFP8's `FNUZ` variants repurpose it, which removes the oldest
    /// non-injectivity in the review (`30_probes/probe_3`'s own witness).
    pub neg_zero: bool,
    /// `Encoding::Canonical`: which member of a cohort `encode` selects. 0 selects the
    /// smallest significand (largest exponent), 1 the largest significand (smallest
    /// exponent). Only has content when the significand is stored unnormalised.
    pub cohort_rule: u8,
}

pub const fn ipow(r: i64, k: u32) -> i64 {
    let mut acc = 1i64;
    let mut i = 0;
    while i < k {
        acc *= r;
        i += 1;
    }
    acc
}

const fn nsign(f: Fmt) -> i64 {
    if f.signed {
        2
    } else {
        1
    }
}

/// Rows of the finite datum region. A normalised encoding has one subnormal row plus one
/// row per normal exponent; an unnormalised encoding has one row per exponent and no
/// normal/subnormal distinction at all, which is the structural reason radix ten has
/// cohorts and radix two does not.
const fn nrows(f: Fmt) -> i64 {
    let normals = (f.qmax - f.qmin + 1) as i64;
    if f.normalised {
        normals + 1
    } else {
        normals
    }
}

const fn nsig(f: Fmt) -> i64 {
    ipow(f.r, f.p)
}

const fn nfinite(f: Fmt) -> i64 {
    nsign(f) * nrows(f) * nsig(f)
}

const fn ninf(f: Fmt) -> i64 {
    if f.inf {
        nsign(f)
    } else {
        0
    }
}

pub const fn ndata(f: Fmt) -> i64 {
    nfinite(f) + ninf(f) + f.nan_data
}

/// The exponent and significand range of a finite row.
const fn row_q(f: Fmt, row: i64) -> i32 {
    if f.normalised {
        if row == 0 {
            f.qmin
        } else {
            f.qmin + (row as i32) - 1
        }
    } else {
        f.qmin + (row as i32)
    }
}

const fn row_sig_lo(f: Fmt, row: i64) -> i64 {
    if f.normalised && row > 0 {
        ipow(f.r, f.p - 1)
    } else {
        0
    }
}

const fn row_sig_hi(f: Fmt, row: i64) -> i64 {
    if f.normalised && row == 0 {
        ipow(f.r, f.p - 1)
    } else {
        nsig(f)
    }
}

/// The strip normal form: the unique identity of a finite value.
pub const fn strip(f: Fmt, neg: bool, mut m: i64, mut q: i32) -> Val {
    if m == 0 {
        return Val {
            tag: 1,
            neg: false,
            m: 0,
            q: 0,
        };
    }
    while m % f.r == 0 {
        m /= f.r;
        q += 1;
    }
    Val { tag: 1, neg, m, q }
}

/// `decode`. Total on the datum space by construction where a datum is in it; data outside
/// a row's own significand range are not data at all and report `NONE`.
pub const fn decode(f: Fmt, d: i64) -> Val {
    if d < 0 || d >= ndata(f) {
        return NONE;
    }
    let nf = nfinite(f);
    if d < nf {
        let per_sign = nrows(f) * nsig(f);
        let sign = d / per_sign;
        let rem = d % per_sign;
        let row = rem / nsig(f);
        let sig = rem % nsig(f);
        if sig < row_sig_lo(f, row) || sig >= row_sig_hi(f, row) {
            return NONE;
        }
        // Abrupt underflow removes the subnormal row except for zero itself.
        if f.normalised && row == 0 && !f.gradual && sig != 0 {
            return NONE;
        }
        let neg = sign == 1;
        // A repurposed negative-zero datum is not a finite datum.
        if sig == 0 && neg && !f.neg_zero {
            return Val {
                tag: 3,
                neg: false,
                m: 0,
                q: 0,
            };
        }
        // An unnormalised encoding spells zero once per row; every one of them is a zero.
        return strip(f, neg, sig, row_q(f, row));
    }
    let d2 = d - nf;
    if d2 < ninf(f) {
        return Val {
            tag: 2,
            neg: d2 == 1,
            m: 0,
            q: 0,
        };
    }
    Val {
        tag: 3,
        neg: false,
        m: 0,
        q: 0,
    }
}

const fn finite_index(f: Fmt, neg: bool, row: i64, sig: i64) -> i64 {
    let sign = if neg { 1 } else { 0 };
    sign * nrows(f) * nsig(f) + row * nsig(f) + sig
}

/// The row a given `(sig, q)` spelling lives in, or -1 if that spelling is not a datum.
const fn row_of(f: Fmt, sig: i64, q: i32) -> i64 {
    let mut row = 0;
    let n = nrows(f);
    while row < n {
        if row_q(f, row) == q && sig >= row_sig_lo(f, row) && sig < row_sig_hi(f, row) {
            if f.normalised && row == 0 && !f.gradual && sig != 0 {
                return -1;
            }
            return row;
        }
        row += 1;
    }
    -1
}

/// `encode`. The section of `decode`: a choice of datum per value, which is what
/// `Encoding::Canonical` supplies. Returns -1 when the value is not representable.
pub const fn encode(f: Fmt, v: Val) -> i64 {
    match v.tag {
        0 => -1,
        3 => {
            if f.nan_data > 0 {
                nfinite(f) + ninf(f)
            } else if !f.neg_zero {
                // The repurposed negative-zero datum IS the NaN datum in an FNUZ encoding.
                finite_index(f, true, 0, 0)
            } else {
                -1
            }
        }
        2 => {
            if f.inf {
                nfinite(f) + if v.neg { 1 } else { 0 }
            } else {
                -1
            }
        }
        _ => {
            if v.m == 0 {
                // Zero. A normalised encoding spells it once, in the subnormal row. An
                // unnormalised encoding spells it once per row, so zero has a cohort of its
                // own and the same `Canonical` rule selects among them: rule 0 takes the
                // largest exponent, rule 1 the smallest.
                let n = nrows(f);
                let mut row = if f.cohort_rule == 0 { n - 1 } else { 0 };
                let mut steps = 0;
                while steps < n {
                    if row_of(f, 0, row_q(f, row)) == row {
                        return finite_index(f, false, row, 0);
                    }
                    if f.cohort_rule == 0 {
                        row -= 1;
                    } else {
                        row += 1;
                    }
                    steps += 1;
                }
                return -1;
            }
            if v.neg && !f.signed {
                return -1;
            }
            // Walk the cohort: (m * r^k, q - k) for k = 0, 1, ...
            let mut best: i64 = -1;
            let mut k = 0u32;
            let mut m = v.m;
            let mut q = v.q;
            while m < nsig(f) {
                let row = row_of(f, m, q);
                if row >= 0 {
                    let idx = finite_index(f, v.neg, row, m);
                    if f.cohort_rule == 0 {
                        // smallest significand wins: the first hit is the answer
                        return idx;
                    }
                    best = idx;
                }
                m *= f.r;
                q -= 1;
                k += 1;
                if k > 64 {
                    break;
                }
            }
            best
        }
    }
}

// ---------------------------------------------------------------------------
// the three statements, exhaustive over the datum space
// ---------------------------------------------------------------------------

/// Statement 1: `decode . encode = id` on values. Every value reached by decoding a datum
/// re-encodes to a datum that decodes back to the same value.
pub const fn s1_decode_after_encode_is_id_on_values(f: Fmt) -> bool {
    let n = ndata(f);
    let mut d = 0;
    while d < n {
        let v = decode(f, d);
        if v.tag != 0 {
            let e = encode(f, v);
            if e < 0 {
                return false;
            }
            if !val_eq(decode(f, e), v) {
                return false;
            }
        }
        d += 1;
    }
    true
}

/// Statement 2: `encode . decode` is idempotent on data.
pub const fn s2_canonicalisation_is_idempotent(f: Fmt) -> bool {
    let n = ndata(f);
    let mut d = 0;
    while d < n {
        let v = decode(f, d);
        if v.tag != 0 {
            let c = encode(f, v);
            if c < 0 {
                return false;
            }
            let vc = decode(f, c);
            if vc.tag == 0 {
                return false;
            }
            if encode(f, vc) != c {
                return false;
            }
        }
        d += 1;
    }
    true
}

/// Statement 3, measured: `encode . decode = id` on data.
pub const fn s3_encode_after_decode_is_id_on_data(f: Fmt) -> bool {
    let n = ndata(f);
    let mut d = 0;
    while d < n {
        let v = decode(f, d);
        if v.tag != 0 && encode(f, v) != d {
            return false;
        }
        d += 1;
    }
    true
}

/// Statement 3, derived. The closed form the design asserted and never wrote down.
///
/// An encoding is injective exactly when no value has two data. There are precisely four
/// sources of a second datum in this design's axes, and each is named by one axis:
///
///   - a signed zero, unless the encoding repurposes the datum (`Canonical`),
///   - more than one NaN datum (`Fields` reserved codes, gated by `Specials::NAN`),
///   - cohorts, which exist exactly when the significand is stored unnormalised over more
///     than one exponent with room to shift (`Fields`, and `Numeral::Radix` forces the
///     unnormalised case for every radix above two),
///   - both infinity data, which are distinct values, so they are never a source.
pub const fn s3_predicted(f: Fmt) -> bool {
    let signed_zero_dup = f.signed && f.neg_zero;
    let nan_dup = f.nan_data > 1;
    let cohorts = !f.normalised && f.p >= 2 && f.qmax > f.qmin;
    // An unnormalised encoding also spells zero once per row, which is a cohort of zeros
    // even at p = 1.
    let zero_cohort = !f.normalised && f.qmax > f.qmin;
    !signed_zero_dup && !nan_dup && !cohorts && !zero_cohort
}

/// The count of distinct values and of live data, so a probe can report the collapse rather
/// than only its boolean.
pub const fn live_data(f: Fmt) -> i64 {
    let n = ndata(f);
    let mut d = 0;
    let mut c = 0;
    while d < n {
        if decode(f, d).tag != 0 {
            c += 1;
        }
        d += 1;
    }
    c
}

/// Distinct values, counted as canonical data (one per value by statement 1 plus statement
/// 2, both of which the probes assert before reading this number).
pub const fn distinct_values(f: Fmt) -> i64 {
    let n = ndata(f);
    let mut d = 0;
    let mut c = 0;
    while d < n {
        let v = decode(f, d);
        if v.tag != 0 && encode(f, v) == d {
            c += 1;
        }
        d += 1;
    }
    c
}
