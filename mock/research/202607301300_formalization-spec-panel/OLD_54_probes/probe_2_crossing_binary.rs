//! Probe 2: the crossing contract run against the `Specials`-carrying type-level numeral,
//! radix two. This is the join the review had in two halves and never put together: the
//! numeral is a type, the contract is checked exhaustively over the datum space it names,
//! and every parameter of the check is read off the type rather than written by hand.
//!
//! rustc --edition 2021 --crate-type lib probe_2_crossing_binary.rs
//!
//! The `Encoding`-side parameters that the numeral does NOT carry (normalisation, the NaN
//! datum count, whether the negative-zero datum exists, which cohort member `encode`
//! selects) are supplied at the crossing site, which is the design's own coordinate split
//! made operational: `Numeral` fixes the value set, `Encoding` fixes the data, and the
//! crossing contract is the only place both are in scope at once.

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

/// Read the value-side axes off the numeral type; take the datum-side axes as arguments.
/// The quantum exponent of a row is `e - p + 1`, which is `quantum_exp` from file 50's
/// model (`50:78-83`) with the floor applied by the row layout rather than by an `if`.
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

type P2t = O<H>;
type P3t = I<H>;
type P6t = O<P3t>;

// ---------------------------------------------------------------------------
// A. the model float: radix 2, p = 3, e in [-2, 3], full IEEE specials
// ---------------------------------------------------------------------------

pub type ModelF = Fl<Two, P3t, ENeg<P2t>, EPos<P3t>, Gradual, IeeeSpecials, Symmetric>;

/// binary32's own NaN datum count is `2 * (2^(p-1) - 1)` for quiet plus signalling across
/// both signs; the model uses four, which is enough for the multiplicity to bite and small
/// enough for the exhaustive check to finish.
pub const F_MODEL: Fmt = fmt_of!(
    ModelF,
    normalised = true,
    nan_data = 4,
    neg_zero = true,
    cohort = 0
);

const _: () = assert!(F_MODEL.r == 2);
const _: () = assert!(F_MODEL.p == 3);
const _: () = assert!(F_MODEL.qmin == -4);
const _: () = assert!(F_MODEL.qmax == 1);

const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_MODEL));
const _: () = assert!(s2_canonicalisation_is_idempotent(F_MODEL));
/// Statement 3 is FALSE here, and the reason is the two oldest entries in the design's own
/// list: a signed zero and a NaN payload set.
const _: () = assert!(!s3_encode_after_decode_is_id_on_data(F_MODEL));
/// And the derived boolean predicts it.
const _: () = assert!(s3_predicted(F_MODEL) == s3_encode_after_decode_is_id_on_data(F_MODEL));

pub const MODEL_LIVE: i64 = live_data(F_MODEL);
pub const MODEL_VALUES: i64 = distinct_values(F_MODEL);
const _: () = assert!(MODEL_LIVE == 62);
const _: () = assert!(MODEL_VALUES == 58);
/// Four data collapse: the negative zero, and three of the four NaN data.
const _: () = assert!(MODEL_LIVE - MODEL_VALUES == 4);

// ---------------------------------------------------------------------------
// B. the injective corner: statement 3 is not vacuously false either
// ---------------------------------------------------------------------------
//
// Unsigned, no specials, normalised. Every value has exactly one datum, so the crossing
// contract's third statement is TRUE and the encoding is a bijection onto its live data.

pub type ModelU = Fl<Two, P3t, ENeg<P2t>, EPos<P3t>, Gradual, NoSpecials, NonNegative>;
pub const F_U: Fmt = fmt_of!(
    ModelU,
    normalised = true,
    nan_data = 0,
    neg_zero = false,
    cohort = 0
);

const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_U));
const _: () = assert!(s2_canonicalisation_is_idempotent(F_U));
const _: () = assert!(s3_encode_after_decode_is_id_on_data(F_U));
const _: () = assert!(s3_predicted(F_U) == s3_encode_after_decode_is_id_on_data(F_U));
const _: () = assert!(live_data(F_U) == distinct_values(F_U));

// ---------------------------------------------------------------------------
// C. the `Specials` product's four corners, each crossed
// ---------------------------------------------------------------------------
//
// The point of running all four rather than the two that already existed: the contract's
// truth values are a function of the `Specials` corner and the encoding's own NaN datum
// count, and nothing else changes between these four numerals.

pub type CNone = Fl<Two, P3t, ENeg<P2t>, EPos<P3t>, Gradual, NoSpecials, Symmetric>;
pub type CInf = Fl<Two, P3t, ENeg<P2t>, EPos<P3t>, Gradual, InfOnly, Symmetric>;
pub type CNan = Fl<Two, P3t, ENeg<P2t>, EPos<P3t>, Gradual, NanOnly, Symmetric>;
pub type CIeee = Fl<Two, P3t, ENeg<P2t>, EPos<P3t>, Gradual, IeeeSpecials, Symmetric>;

pub const F_NONE: Fmt = fmt_of!(
    CNone,
    normalised = true,
    nan_data = 0,
    neg_zero = true,
    cohort = 0
);
pub const F_INF: Fmt = fmt_of!(
    CInf,
    normalised = true,
    nan_data = 0,
    neg_zero = true,
    cohort = 0
);
pub const F_NAN: Fmt = fmt_of!(
    CNan,
    normalised = true,
    nan_data = 2,
    neg_zero = true,
    cohort = 0
);
pub const F_IEEE: Fmt = fmt_of!(
    CIeee,
    normalised = true,
    nan_data = 2,
    neg_zero = true,
    cohort = 0
);

const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_NONE));
const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_INF));
const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_NAN));
const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_IEEE));

const _: () = assert!(s2_canonicalisation_is_idempotent(F_NONE));
const _: () = assert!(s2_canonicalisation_is_idempotent(F_INF));
const _: () = assert!(s2_canonicalisation_is_idempotent(F_NAN));
const _: () = assert!(s2_canonicalisation_is_idempotent(F_IEEE));

// Statement 1 and 2 are invariant across the whole `Specials` product. Statement 3 is not,
// and it moves for a reason the encoding names rather than the numeral.
const _: () = assert!(s3_predicted(F_NONE) == s3_encode_after_decode_is_id_on_data(F_NONE));
const _: () = assert!(s3_predicted(F_INF) == s3_encode_after_decode_is_id_on_data(F_INF));
const _: () = assert!(s3_predicted(F_NAN) == s3_encode_after_decode_is_id_on_data(F_NAN));
const _: () = assert!(s3_predicted(F_IEEE) == s3_encode_after_decode_is_id_on_data(F_IEEE));

/// The infinities are two distinct VALUES, so adding them changes the value count by two
/// and leaves injectivity exactly where it was. That is the sentence the design should
/// carry: `Specials::INF` never touches the crossing contract, and `Specials::NAN` always
/// does when the encoding reserves more than one datum for it.
const _: () = assert!(distinct_values(F_INF) == distinct_values(F_NONE) + 2);
const _: () = assert!(distinct_values(F_NAN) == distinct_values(F_NONE) + 1);
const _: () = assert!(distinct_values(F_IEEE) == distinct_values(F_NONE) + 3);

// ---------------------------------------------------------------------------
// D. OFP8 E4M3's own `FNUZ` variant: an encoding choice that RESTORES injectivity
// ---------------------------------------------------------------------------
//
// `E4M3FNUZ` spends the negative-zero datum on NaN. That removes the review's oldest
// non-injectivity witness (`30_probes/probe_3`'s sign-magnitude zero) by construction, and
// with a single NaN datum the encoding becomes injective while still carrying a special.
// So the third statement is genuinely a derived boolean over the encoding's own choices,
// not a property of whether the numeral has specials.

pub type Fnuz = Fl<Two, P4x, ENeg<P6t>, EPos<P8x>, Gradual, NanOnly, Symmetric>;
type P4x = O<P2t>;
type P8x = O<P4x>;

pub const F_FNUZ: Fmt = fmt_of!(
    Fnuz,
    normalised = true,
    nan_data = 0,
    neg_zero = false,
    cohort = 0
);
const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_FNUZ));
const _: () = assert!(s2_canonicalisation_is_idempotent(F_FNUZ));
const _: () = assert!(s3_encode_after_decode_is_id_on_data(F_FNUZ));
const _: () = assert!(s3_predicted(F_FNUZ) == s3_encode_after_decode_is_id_on_data(F_FNUZ));

// ---------------------------------------------------------------------------
// E. abrupt underflow, crossed
// ---------------------------------------------------------------------------

pub type ModelAbrupt = Fl<Two, P3t, ENeg<P2t>, EPos<P3t>, Abrupt, NoSpecials, Symmetric>;
pub const F_ABRUPT: Fmt = fmt_of!(
    ModelAbrupt,
    normalised = true,
    nan_data = 0,
    neg_zero = true,
    cohort = 0
);
const _: () = assert!(s1_decode_after_encode_is_id_on_values(F_ABRUPT));
const _: () = assert!(s2_canonicalisation_is_idempotent(F_ABRUPT));
/// Abrupt underflow removes values, and it removes data with them, so it moves neither
/// statement's truth value. It does make `decode` partial on a region of the datum space
/// that was total, which is the fourth thing the crossing contract has to be stated over
/// and currently is not.
const _: () = assert!(live_data(F_ABRUPT) < live_data(F_NONE));

// ---------------------------------------------------------------------------
// F. the section is a genuine choice, and the two choices are different functions
// ---------------------------------------------------------------------------
//
// Both cohort rules satisfy statements 1 and 2 and fail 3 identically, which would be a
// vacuous observation if the two rules agreed everywhere. They do not agree, and the
// witness is zero, whose cohort under an unnormalised encoding spans every row. The
// binary model has no finite cohort at all, so this check belongs to probe 3; what is
// asserted here is the negative half, that under a normalised encoding the two rules ARE
// the same function, which is the precise sense in which radix two hides the choice.

pub const F_MODEL_MAX: Fmt = fmt_of!(
    ModelF,
    normalised = true,
    nan_data = 4,
    neg_zero = true,
    cohort = 1
);

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

const _: () = assert!(sections_agree(F_MODEL, F_MODEL_MAX));

/// Forced through a signature, not left inert.
pub fn crossing_holds<N: Numeral>(f: Fmt) -> (bool, bool, bool) {
    let _ = <N as Numeral>::R;
    (
        s1_decode_after_encode_is_id_on_values(f),
        s2_canonicalisation_is_idempotent(f),
        s3_encode_after_decode_is_id_on_data(f),
    )
}

pub fn call_crossing() -> (bool, bool, bool) {
    crossing_holds::<ModelF>(F_MODEL)
}
