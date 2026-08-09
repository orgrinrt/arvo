//! Probe 3: the radix-ten instance, and the first non-vacuous exercise of the crossing
//! contract's injectivity statement.
//!
//! rustc --edition 2021 --crate-type lib probe_3_crossing_decimal.rs
//!
//! Every numeral the review has built has been radix two, where the encoding's
//! non-injectivity is exactly two data wide (a negative zero, and a NaN payload set) and
//! both are special cases sitting outside the ordinary finite arithmetic. Radix ten is
//! where it stops being a special case: the collapse is proportional to the format and it
//! lands on ordinary finite values.
//!
//! The derivation, which is the part worth carrying into the spec, is that the radix is
//! only the FIRST link of the chain and the rest follows without mentioning it again:
//!
//!   radix > 2  ->  no constant leading digit to hide  ->  the significand is stored
//!   unnormalised  ->  a value has one datum per representable exponent shift  ->  the
//!   encoding is not injective.
//!
//! Only the first arrow is about the radix. The design's axes already name every link:
//! `Numeral::Radix`, `Encoding::Fields` (hidden digit), `Encoding::Canonical` (which
//! cohort member `encode` selects). Nothing new is needed, and section 4 of file 54 states
//! the one place the standard and the design genuinely part company.

#![allow(dead_code)]

#[path = "vu_bias_sealed_adj.rs"]
pub mod bias;
#[path = "crossing.rs"]
pub mod crossing;
#[path = "numeral.rs"]
pub mod numeral;

use bias::nat::{H, I, O};
use crossing::*;
use numeral::*;

macro_rules! fmt_of {
    ($N:ty, normalised = $norm:expr, nan_data = $nd:expr, neg_zero = $nz:expr, cohort = $c:expr) => {
        Fmt {
            r: <$N as Numeral>::R as i64,
            p: <$N as Numeral>::P as u32,
            qmin: (<$N as Numeral>::EMIN - <$N as Numeral>::P as i64 + 1) as i32,
            qmax: (<$N as Numeral>::EMAX - <$N as Numeral>::P as i64 + 1) as i32,
            signed: <$N as Numeral>::SIGNED,
            inf: <$N as Numeral>::INF,
            nan_data: if <$N as Numeral>::NAN { $nd } else { 0 },
            normalised: $norm,
            gradual: <$N as Numeral>::GRADUAL,
            neg_zero: $nz,
            cohort_rule: $c,
        }
    };
}

pub const fn sections_agree(a: Fmt, b: Fmt) -> bool {
    let n = ndata(a);
    let mut d = 0;
    while d < n {
        let v = decode(a, d);
        if v.tag != 0 && encode(a, v) != encode(b, v) {
            return false;
        }
        d += 1;
    }
    true
}

type P2t = O<H>;
type P3t = I<H>;

// ---------------------------------------------------------------------------
// A. a decimal `Ranged` numeral, p = 2 decimal digits, e in [0, 2]
// ---------------------------------------------------------------------------
//
// The quantum exponents are e - p + 1, so q ranges over [-1, 1]: the same value can be
// spelled with three different exponents whenever its significand has room to shift.

pub type Dec = Fl<Ten, P2t, EZero, EPos<P2t>, Gradual, NoSpecials, Symmetric>;

pub const D_MIN: Fmt = fmt_of!(
    Dec,
    normalised = false,
    nan_data = 0,
    neg_zero = true,
    cohort = 0
);
pub const D_MAX: Fmt = fmt_of!(
    Dec,
    normalised = false,
    nan_data = 0,
    neg_zero = true,
    cohort = 1
);

const _: () = assert!(D_MIN.r == 10);
const _: () = assert!(D_MIN.p == 2);
const _: () = assert!(D_MIN.qmin == -1 && D_MIN.qmax == 1);

// Statements 1 and 2 survive radix ten untouched. They are the two the design leans on and
// neither notices the radix.
const _: () = assert!(s1_decode_after_encode_is_id_on_values(D_MIN));
const _: () = assert!(s2_canonicalisation_is_idempotent(D_MIN));
const _: () = assert!(s1_decode_after_encode_is_id_on_values(D_MAX));
const _: () = assert!(s2_canonicalisation_is_idempotent(D_MAX));

// Statement 3 is false, and the derived boolean predicts it under both sections.
const _: () = assert!(!s3_encode_after_decode_is_id_on_data(D_MIN));
const _: () = assert!(s3_predicted(D_MIN) == s3_encode_after_decode_is_id_on_data(D_MIN));
const _: () = assert!(s3_predicted(D_MAX) == s3_encode_after_decode_is_id_on_data(D_MAX));

// ---------------------------------------------------------------------------
// B. the size of the failure, which is the point
// ---------------------------------------------------------------------------
//
// In the binary model the collapse is one datum with no specials (the negative zero). Here
// it is forty-one out of six hundred, on ordinary finite values, and it grows with the
// format. The injectivity statement has not been exercised before; it has been satisfied
// vacuously in the sense that its one witness was a special case everyone already knew.

pub const D_DATA: i64 = ndata(D_MIN);
pub const D_LIVE: i64 = live_data(D_MIN);
pub const D_VALUES: i64 = distinct_values(D_MIN);
const _: () = assert!(D_DATA == 600);
const _: () = assert!(D_LIVE == 600);
const _: () = assert!(D_VALUES == 559);
const _: () = assert!(D_LIVE - D_VALUES == 41);

/// Every datum is live: an unnormalised encoding has no reserved significand range, so
/// `decode` is total on the finite region. That is the other side of the trade and it is
/// why decimal encodings waste nothing despite the redundancy.
const _: () = assert!(D_DATA == D_LIVE);

// ---------------------------------------------------------------------------
// C. the section is a genuine choice, and radix ten is where it becomes visible
// ---------------------------------------------------------------------------
//
// Under radix two with a hidden digit the two cohort rules are the same function (probe 2,
// section F). Under radix ten they are different functions on the same value set, which
// means `Encoding::Canonical` is carrying real content rather than a formality: the design
// owes a CHOICE here, and neither choice is derivable from the numeral.

const _: () = assert!(!sections_agree(D_MIN, D_MAX));

/// The witness, named rather than counted. The value 1 (m = 1, q = 0) is spelled `1 x 10^0`
/// and `10 x 10^-1`; the min-significand section picks the first, the max-significand
/// section the second, and both decode back to 1.
pub const ONE: Val = Val {
    tag: 1,
    neg: false,
    m: 1,
    q: 0,
};
const _: () = assert!(encode(D_MIN, ONE) != encode(D_MAX, ONE));
const _: () = assert!(val_eq(decode(D_MIN, encode(D_MIN, ONE)), ONE));
const _: () = assert!(val_eq(decode(D_MAX, encode(D_MAX, ONE)), ONE));

// ---------------------------------------------------------------------------
// D. the counterfactual: radix ten CAN be normalised, and it changes no value
// ---------------------------------------------------------------------------
//
// Constraining the significand to [10^(p-1), 10^p) in the normal rows gives the identical
// value set with fewer live data. So cohorts are not forced on decimal by the value set,
// and they are not an accident of the encoding either: they are a deliberate choice the
// standard makes because the cohort member (the "quantum") is meaningful to a decimal
// consumer. That is the fact section 4 of file 54 turns into a design statement.

pub const D_NORM: Fmt = fmt_of!(
    Dec,
    normalised = true,
    nan_data = 0,
    neg_zero = true,
    cohort = 0
);
const _: () = assert!(s1_decode_after_encode_is_id_on_values(D_NORM));
const _: () = assert!(s2_canonicalisation_is_idempotent(D_NORM));
const _: () = assert!(distinct_values(D_NORM) == D_VALUES);
const _: () = assert!(live_data(D_NORM) < D_LIVE);
const _: () = assert!(live_data(D_NORM) == 560);

// ---------------------------------------------------------------------------
// E. the quantum in the type: a decimal `Implicit` numeral has no cohort at all
// ---------------------------------------------------------------------------
//
// One exponent row, so no value has a second spelling, and the only collapse left is the
// signed zero the binary case already had. This is where a consumer who needs the quantum
// to be part of the number should be: the design's `Implicit` form puts the exponent in the
// type, which is the compile-time version of what IEEE's preferred-exponent rules do at
// runtime.

pub type DecFixed = Fl<Ten, P2t, EPos<P2t>, EPos<P2t>, Gradual, NoSpecials, Symmetric>;
pub const D_FIXED: Fmt = fmt_of!(
    DecFixed,
    normalised = false,
    nan_data = 0,
    neg_zero = true,
    cohort = 0
);
const _: () = assert!(D_FIXED.qmin == D_FIXED.qmax);
const _: () = assert!(s1_decode_after_encode_is_id_on_values(D_FIXED));
const _: () = assert!(s2_canonicalisation_is_idempotent(D_FIXED));
const _: () = assert!(live_data(D_FIXED) - distinct_values(D_FIXED) == 1);
/// Drop the sign domain to non-negative and the decimal fixed numeral is injective: the
/// crossing contract's third statement is TRUE for a radix-ten numeral, which is the check
/// that keeps "decimal is never injective" from becoming the next unexamined sentence.
pub type DecFixedU = Fl<Ten, P2t, EPos<P2t>, EPos<P2t>, Gradual, NoSpecials, NonNegative>;
pub const D_FIXED_U: Fmt = fmt_of!(
    DecFixedU,
    normalised = false,
    nan_data = 0,
    neg_zero = false,
    cohort = 0
);
const _: () = assert!(s3_encode_after_decode_is_id_on_data(D_FIXED_U));
const _: () = assert!(s3_predicted(D_FIXED_U));

// ---------------------------------------------------------------------------
// F. two encodings for one numeral, which is the standard's own shape
// ---------------------------------------------------------------------------
//
// IEEE's decimal formats ship two interchange encodings, BID and DPD. They are two
// `Encoding` instances under one `Numeral`: same radix, same precision, same exponent
// range, same value set, different data. The design's three-way cut predicts exactly this
// shape, and the crossing contract makes it checkable rather than asserted.
//
// The half that is easy and the half that is not:
//
//   - Repacking the significand digits (which is all DPD's declets are, next to BID's
//     binary integer) is a BIJECTION on the datum space. A bijection commutes with decode
//     and encode, so it cannot change any of the three statements. That is a theorem, not a
//     measurement, and it says the interesting difference between BID and DPD is not the
//     packing.
//   - What IS interesting is that a binary significand field wide enough to hold
//     `10^p - 1` also holds codes above it. Those are non-canonical, and the standard's
//     rule is that they read as zero. That is a THIRD non-injectivity, larger than the
//     cohorts, present in BID and absent in DPD, and living entirely on the `Encoding`
//     side of the design's coordinate split.
//
// Modelled below at p = 2: `10^2 = 100` values of significand in a seven-bit field of 128,
// so twenty-eight codes per row per sign are non-canonical.

const BID_SIG_FIELD: i64 = 128; // 2^7, the smallest field holding 10^2 - 1
const BID_ROWS: i64 = 3; // q in [-1, 1]
const BID_DATA: i64 = 2 * BID_ROWS * BID_SIG_FIELD;

const fn bid_decode(d: i64) -> Val {
    if d < 0 || d >= BID_DATA {
        return NONE;
    }
    let per_sign = BID_ROWS * BID_SIG_FIELD;
    let neg = d / per_sign == 1;
    let rem = d % per_sign;
    let row = rem / BID_SIG_FIELD;
    let sig = rem % BID_SIG_FIELD;
    if sig >= 100 {
        // non-canonical: the standard reads these as zero
        return Val {
            tag: 1,
            neg: false,
            m: 0,
            q: 0,
        };
    }
    strip(D_MIN, neg, sig, (row as i32) - 1)
}

const fn bid_encode(v: Val) -> i64 {
    // the canonical datum: same section as D_MIN, re-indexed into the wider field
    let c = encode(D_MIN, v);
    if c < 0 {
        return -1;
    }
    let per_sign_small = BID_ROWS * 100;
    let sign = c / per_sign_small;
    let rem = c % per_sign_small;
    let row = rem / 100;
    let sig = rem % 100;
    sign * BID_ROWS * BID_SIG_FIELD + row * BID_SIG_FIELD + sig
}

const fn bid_s1() -> bool {
    let mut d = 0;
    while d < BID_DATA {
        let v = bid_decode(d);
        if v.tag != 0 {
            let e = bid_encode(v);
            if e < 0 || !val_eq(bid_decode(e), v) {
                return false;
            }
        }
        d += 1;
    }
    true
}

const fn bid_s2() -> bool {
    let mut d = 0;
    while d < BID_DATA {
        let v = bid_decode(d);
        if v.tag != 0 {
            let c = bid_encode(v);
            if c < 0 {
                return false;
            }
            let vc = bid_decode(c);
            if vc.tag == 0 || bid_encode(vc) != c {
                return false;
            }
        }
        d += 1;
    }
    true
}

const fn bid_live() -> i64 {
    let mut d = 0;
    let mut c = 0;
    while d < BID_DATA {
        if bid_decode(d).tag != 0 {
            c += 1;
        }
        d += 1;
    }
    c
}

const fn bid_canonical() -> i64 {
    let mut d = 0;
    let mut c = 0;
    while d < BID_DATA {
        let v = bid_decode(d);
        if v.tag != 0 && bid_encode(v) == d {
            c += 1;
        }
        d += 1;
    }
    c
}

/// Both statements survive the second encoding untouched, against the identical numeral.
const _: () = assert!(bid_s1());
const _: () = assert!(bid_s2());
/// And the value set is the same one the first encoding produced.
const _: () = assert!(bid_canonical() == D_VALUES);
/// The datum space is not. 768 data carry 559 values, against 600 in the first encoding:
/// the non-canonical codes are 168 extra spellings of zero on their own, more than four
/// times the whole cohort collapse.
const _: () = assert!(BID_DATA == 768);
const _: () = assert!(bid_live() == 768);
const _: () = assert!(BID_DATA - bid_canonical() == 209);

// ---------------------------------------------------------------------------
// G. mulnum over two decimal numerals still works, and radix is a shared parameter
// ---------------------------------------------------------------------------

pub type DA = Fl<Ten, P2t, EZero, EPos<P2t>, Gradual, NoSpecials, Symmetric>;
pub type DB = Fl<Ten, P3t, EZero, EPos<P3t>, Gradual, NoSpecials, Symmetric>;
pub type DAB = <DA as MulNum<DB>>::Out;
const _: () = assert!(<DAB as Numeral>::R == 10);
const _: () = assert!(<DAB as Numeral>::P == 5);
const _: () = assert!(<DAB as Numeral>::EMAX == 5);

pub fn forced_decimal() -> i64 {
    <DAB as Numeral>::EMAX
}
