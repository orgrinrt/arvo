//! Probe 2: file 28's round-first amendment plus file 27's `Specials` member
//! reproduce IEEE 754's overflow behaviour exactly, for three rounding
//! attributes including the round-to-nearest threshold, with no new axis and
//! no new `Resolution`.
//!
//! Both files proposed their piece and neither checked that the two together
//! produce IEEE's actual rule. File 27 claimed something stronger and wrong:
//! that with infinity representable, "past the top is unreachable"
//! (`27:188-193`), so overflow-to-infinity stops being an out-of-range
//! resolution. It does not. IEEE 754-2019 (7.4) declares overflow when the
//! result rounded **as though the exponent range were unbounded** exceeds the
//! largest finite, and under roundTiesToEven the boundary is
//! `2^emax * (2 - 2^-p)`, the midpoint between the largest finite and the
//! first value of the binade that does not exist. That is not the midpoint
//! between the largest finite and infinity, which does not exist. So the
//! over-range position stays inhabited, and what closes the gap is:
//!
//!   1. round-first, which supplies the unbounded grid the midpoint lives on;
//!   2. `Specials` on the numeral, which makes infinity a representable datum;
//!   3. the existing rule that a `Direction` at an out-of-range position means
//!      "the neighbour that exists", now derived against a set that has one
//!      above the largest finite.
//!
//! Model: radix 2, precision p = 3 (one hidden bit, two stored), emax = 2.
//! Scaled by 16 so every quantity is an integer: real v is `v * 16`.
//!
//!   binade e = 2: scaled quantum 16, values 64, 80, 96, 112 (real 4 to 7)
//!   largest finite: 112 (real 7)
//!   unbounded continuation e = 3: scaled quantum 32, first value 128 (real 8)
//!   roundTiesToEven overflow boundary: real 7.5, scaled 120
//!
//! Every claim is a `const` assertion, so compiling is the check.

#![no_std]

const SCALE: i64 = 16;
const P: u32 = 3;
const EMAX: i32 = 2;

/// (2 - 2^(1-p)) * 2^emax, scaled: the largest finite value.
const LARGEST_FINITE: i64 = 112;

/// Infinity, as a representable datum outside the rational image.
const INF: i64 = i64::MAX;

const fn pow2(n: u32) -> i64 {
    1i64 << n
}

/// Scaled quantum of the binade containing `x`, on the grid extended past
/// `emax` with no upper bound. `x` is at least `SCALE` (real 1), which keeps
/// this out of the subnormal region the probe is not about.
const fn quantum_at(x: i64) -> i64 {
    let mut e: u32 = 0;
    while SCALE * pow2(e + 1) <= x {
        e += 1;
    }
    SCALE * pow2(e) / pow2(P - 1)
}

// ---- the direction triple, on the unbounded grid ---------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    NearestTiesEven,
    TowardZero,
    TowardPositive,
}

/// Round a positive exact value onto the UNBOUNDED grid by `dir`.
///
/// At a binade boundary the even-multiple rule and IEEE's even-significand
/// rule coincide: the value above the top of a binade is the next binade's
/// first, whose multiple index of the lower binade's quantum is even.
const fn round_unbounded(x: i64, dir: Dir) -> i64 {
    if x < SCALE {
        return x; // outside the probe's scope, passed through
    }
    let q = quantum_at(x);
    let down = (x / q) * q;
    let rem = x - down;
    if rem == 0 {
        return down;
    }
    match dir {
        Dir::TowardZero => down,
        Dir::TowardPositive => down + q,
        Dir::NearestTiesEven => {
            if rem * 2 < q {
                down
            } else if rem * 2 > q {
                down + q
            } else if (down / q) % 2 == 0 {
                down
            } else {
                down + q
            }
        }
    }
}

/// The round-to-nearest overflow boundary, derived from the format rather
/// than written as a literal, so a wrong derivation fails to compile.
const FIRST_PAST_EMAX: i64 = SCALE * pow2((EMAX + 1) as u32);
const RN_BOUNDARY: i64 = (LARGEST_FINITE + FIRST_PAST_EMAX) / 2;
const _: () = assert!(FIRST_PAST_EMAX == 128);
const _: () = assert!(RN_BOUNDARY == 120); // real 7.5 = 2^emax * (2 - 2^-p)

// ---- the design's pipeline -------------------------------------------------

/// Round on the unbounded grid by the in-range direction triple, then
/// classify the ROUNDED result against the range and resolve.
///
/// `over_range` is an ordinary `Direction`: `TowardPositive` names the
/// neighbour above, which is infinity when the numeral carries specials and
/// the largest finite when it does not; `TowardZero` names the neighbour
/// below, which is the largest finite either way.
const fn design_pipeline(x: i64, in_range: Dir, over_range: Dir, specials: bool) -> i64 {
    let r = round_unbounded(x, in_range);
    if r <= LARGEST_FINITE {
        return r;
    }
    match over_range {
        Dir::TowardPositive => {
            if specials {
                INF
            } else {
                LARGEST_FINITE
            }
        }
        _ => LARGEST_FINITE,
    }
}

// ---- three independent oracles, written from IEEE 754-2019 7.4 -------------
//
// Each is written from the standard's own description of the attribute, not
// derived from the pipeline above, so agreement is a check rather than a
// restatement.

/// roundTiesToEven: overflow when the magnitude reaches the boundary, and the
/// delivered result is infinity.
const fn oracle_rn(x: i64) -> i64 {
    if x >= RN_BOUNDARY {
        INF
    } else {
        round_unbounded(x, Dir::NearestTiesEven)
    }
}

/// roundTowardZero: overflow delivers the largest finite, never infinity.
const fn oracle_rz(x: i64) -> i64 {
    let r = round_unbounded(x, Dir::TowardZero);
    if r > LARGEST_FINITE {
        LARGEST_FINITE
    } else {
        r
    }
}

/// roundTowardPositive: any positive result above the largest finite
/// delivers infinity, with no boundary at all.
const fn oracle_rp(x: i64) -> i64 {
    if x > LARGEST_FINITE {
        INF
    } else {
        round_unbounded(x, Dir::TowardPositive)
    }
}

// ---- exhaustive agreement over real 1 through 9 ----------------------------

const fn agrees(in_range: Dir, over_range: Dir, which: u8) -> bool {
    let mut x = SCALE;
    while x <= SCALE * 9 {
        let d = design_pipeline(x, in_range, over_range, true);
        let o = match which {
            0 => oracle_rn(x),
            1 => oracle_rz(x),
            _ => oracle_rp(x),
        };
        if d != o {
            return false;
        }
        x += 1;
    }
    true
}

const _: () = assert!(agrees(Dir::NearestTiesEven, Dir::TowardPositive, 0));
const _: () = assert!(agrees(Dir::TowardZero, Dir::TowardZero, 1));
const _: () = assert!(agrees(Dir::TowardPositive, Dir::TowardPositive, 2));

/// The pairings above are exactly IEEE's mode coupling, and the coupling is
/// not free: pairing roundTiesToEven's in-range direction with the
/// largest-finite over-range resolution disagrees with roundTiesToEven, so
/// the four rows of the `conv-ieee754` alias table carry real information
/// rather than restating a default.
const fn coupling_matters() -> bool {
    let mut x = SCALE;
    while x <= SCALE * 9 {
        if design_pipeline(x, Dir::NearestTiesEven, Dir::TowardZero, true) != oracle_rn(x) {
            return true; // found a disagreement, which is the claim
        }
        x += 1;
    }
    false
}
const _: () = assert!(coupling_matters());

// ---- the three cells that carry the finding --------------------------------

/// Below the boundary: the exact value IS past the largest finite, and IEEE
/// still delivers the largest finite with no overflow. This is the band file
/// 28's probe found classify-first gets wrong, now shown to be the same band
/// that makes the specials story work.
const _: () = assert!(118 > LARGEST_FINITE);
const _: () = assert!(
    design_pipeline(118, Dir::NearestTiesEven, Dir::TowardPositive, true) == LARGEST_FINITE
);

/// At the boundary: the tie is between the largest finite (odd multiple) and
/// the first value past emax (even multiple), so ties-to-even goes up, lands
/// over range, and delivers infinity. IEEE says magnitudes at or above the
/// boundary overflow, so the boundary case needs no rule of its own.
const _: () = assert!(round_unbounded(RN_BOUNDARY, Dir::NearestTiesEven) == FIRST_PAST_EMAX);
const _: () =
    assert!(design_pipeline(RN_BOUNDARY, Dir::NearestTiesEven, Dir::TowardPositive, true) == INF);

/// Without specials the identical pipeline delivers the largest finite, which
/// is SystemC and MATLAB's saturating behaviour. One identity member decides
/// it; no `Policy` member changes, and no second overflow vocabulary exists.
const _: () = assert!(
    design_pipeline(
        RN_BOUNDARY,
        Dir::NearestTiesEven,
        Dir::TowardPositive,
        false
    ) == LARGEST_FINITE
);
const _: () = assert!(
    design_pipeline(SCALE * 9, Dir::NearestTiesEven, Dir::TowardPositive, false) == LARGEST_FINITE
);

// ---- what this does NOT show ----------------------------------------------
//
// It does not show the five-position vocabulary is complete for float once
// specials exist. It shows overflow-to-infinity needs no new `Resolution`,
// given round-first and given that "the neighbour that exists" is derived
// against a value set the numeral declares. Underflow and subnormals, the
// sign of a zero result, NaN propagation, and the negative half of the range
// are all outside this probe; the deliverable says which remain open.
