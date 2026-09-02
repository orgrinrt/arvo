//! Probe 1: the round-first-plus-specials pipeline (file 28's amendment, file
//! 27's `Specials`, verified for the positive half in `30_probes/probe_2`)
//! extended to the signed case, and a genuine asymmetry checked rather than
//! assumed.
//!
//! Probe 2 of file 30 disclaims, three times over (in its own header, its own
//! "what this does not show" section, and file 30's summary), that it says
//! nothing about "the negative half of the range". That disclaimer was never
//! converted into a stated claim to check. This probe converts it.
//!
//! The claim worth checking is not "does the pipeline work for negative
//! values too", which would be a symmetry assumption. It is the opposite:
//! **`TowardPositive` and `TowardNegative` are absolute directions on the
//! value line, not magnitude-relative ones, so their behaviour at the two
//! ends of a symmetric domain is NOT mirror images of each other.** Under
//! IEEE's roundTowardPositive attribute, positive overflow delivers +infinity
//! (there is a neighbour above), but negative "underflow" (a magnitude past
//! the domain's negative bound) delivers the negative largest finite, never
//! -infinity, because rounding toward positive never selects the more
//! negative of two candidates. roundTiesToEven and roundTowardZero, by
//! contrast, ARE odd-symmetric (round(-x) = -round(x)), so they behave as
//! mirror images. A design whose `Direction` vocabulary is genuinely absolute
//! (as `202607301200`'s five-position vocabulary is written) gets this right
//! for free from the SAME markers used on the positive side, with no
//! sign-conditional resolution logic anywhere. That is the claim checked
//! here, not assumed.
//!
//! Model: identical to `30_probes/probe_2` (radix 2, precision 3, emax 2,
//! scale 16), extended to signed exact values via a magnitude/sign split.
//! `Domain::Symmetric`, per file 30's split (`30:242-281`): the value range
//! is symmetric, infinities are signed data with distinct values at each end
//! (unlike signed zero, which collapses to one value), and file 27's
//! `Specials` member is the identity fact that makes both ends of that range
//! reachable at all.
//!
//! Every claim is a `const` assertion; compiling is the check.

#![no_std]

const SCALE: i64 = 16;
const P: u32 = 3;
// EMAX = 2 in the shared model (30_probes/probe_2); LARGEST_FINITE,
// FIRST_PAST_EMAX and RN_BOUNDARY below are its already-verified derived
// constants, carried over rather than re-derived.

const LARGEST_FINITE: i64 = 112; // magnitude
const FIRST_PAST_EMAX: i64 = 128; // magnitude, verified in probe 2
const RN_BOUNDARY: i64 = 120; // magnitude, verified in probe 2

const POS_INF: i64 = i64::MAX;
const NEG_INF: i64 = i64::MIN;

const fn pow2(n: u32) -> i64 {
    1i64 << n
}

/// Unchanged from probe 2: the scaled quantum of the binade containing a
/// MAGNITUDE `m >= SCALE`, extended past emax with no upper bound.
const fn quantum_at(m: i64) -> i64 {
    let mut e: u32 = 0;
    while SCALE * pow2(e + 1) <= m {
        e += 1;
    }
    SCALE * pow2(e) / pow2(P - 1)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    NearestTiesEven,
    TowardZero,
    TowardPositive,
    TowardNegative,
}

/// Round a non-negative magnitude on the unbounded grid, in the direction
/// that increasing magnitude corresponds to (`round_up`). This is the ONLY
/// place sign enters: everything above and below this function talks about
/// magnitude and the caller decides, from `(dir, sign)`, which way "up" goes.
const fn round_magnitude(m: i64, round_up: bool, ties_even: bool) -> i64 {
    if m < SCALE {
        return m;
    }
    let q = quantum_at(m);
    let down = (m / q) * q;
    let rem = m - down;
    if rem == 0 {
        return down;
    }
    if ties_even {
        if rem * 2 < q {
            down
        } else if rem * 2 > q {
            down + q
        } else if (down / q) % 2 == 0 {
            down
        } else {
            down + q
        }
    } else if round_up {
        down + q
    } else {
        down
    }
}

/// The absolute-direction mapping: for a value of the given sign, does this
/// `Dir` mean "increase the magnitude" or "decrease the magnitude"?
/// `NearestTiesEven` is handled by its own flag in `round_magnitude` and
/// never reaches this function.
const fn magnitude_goes_up(dir: Dir, positive: bool) -> bool {
    match dir {
        Dir::TowardZero => false,
        Dir::TowardPositive => positive, // value increases -> magnitude increases iff positive
        Dir::TowardNegative => !positive, // value decreases -> magnitude increases iff negative
        Dir::NearestTiesEven => unreachable!(),
    }
}

/// Round a SIGNED exact value on the unbounded grid, absolute direction.
const fn round_unbounded_signed(x: i64, dir: Dir) -> i64 {
    if x == 0 {
        return 0;
    }
    let positive = x > 0;
    let m = if positive { x } else { -x };
    let rm = match dir {
        Dir::NearestTiesEven => round_magnitude(m, false, true),
        _ => round_magnitude(m, magnitude_goes_up(dir, positive), false),
    };
    if positive {
        rm
    } else {
        -rm
    }
}

/// The design's pipeline, both ends: round on the unbounded grid, then
/// classify the rounded MAGNITUDE against the range and resolve. `over` names
/// the resolution at the positive-overflow position, `under` at the
/// negative-underflow position, matching the spec's separate `OverRange` /
/// `UnderRange` fields (`202607301200`'s `Quantisation` trait). `specials`
/// gates whether either infinity is a representable datum at all.
const fn design_pipeline_signed(
    x: i64,
    in_range: Dir,
    over: Dir,
    under: Dir,
    specials: bool,
) -> i64 {
    let r = round_unbounded_signed(x, in_range);
    let m = if r >= 0 { r } else { -r };
    if m <= LARGEST_FINITE {
        return r;
    }
    if r > 0 {
        // over-range: the neighbour above exists iff `over` reaches outward
        // (TowardPositive) and specials make it representable.
        match over {
            Dir::TowardPositive => {
                if specials {
                    POS_INF
                } else {
                    LARGEST_FINITE
                }
            }
            _ => LARGEST_FINITE,
        }
    } else {
        // under-range: the neighbour below exists iff `under` reaches
        // outward (TowardNegative) and specials make it representable.
        match under {
            Dir::TowardNegative => {
                if specials {
                    NEG_INF
                } else {
                    -LARGEST_FINITE
                }
            }
            _ => -LARGEST_FINITE,
        }
    }
}

// ---- three oracles, both ends, written from IEEE 754-2019 directly --------

/// roundTiesToEven: odd-symmetric. Overflows to +-infinity at the same
/// magnitude boundary on both ends.
const fn oracle_rn_signed(x: i64) -> i64 {
    let positive = x >= 0;
    let m = if positive { x } else { -x };
    if m >= RN_BOUNDARY {
        if positive {
            POS_INF
        } else {
            NEG_INF
        }
    } else {
        round_unbounded_signed(x, Dir::NearestTiesEven)
    }
}

/// roundTowardZero: odd-symmetric. Never delivers infinity, on either end.
const fn oracle_rz_signed(x: i64) -> i64 {
    let r = round_unbounded_signed(x, Dir::TowardZero);
    if r > LARGEST_FINITE {
        LARGEST_FINITE
    } else if r < -LARGEST_FINITE {
        -LARGEST_FINITE
    } else {
        r
    }
}

/// roundTowardPositive: NOT symmetric. Delivers +infinity for any positive
/// overflow, but NEVER delivers -infinity: a very negative exact value
/// rounds toward positive means picking the LESS negative candidate, which
/// is bounded below by the negative largest finite.
const fn oracle_rp_signed(x: i64) -> i64 {
    if x > LARGEST_FINITE {
        POS_INF
    } else if x < -LARGEST_FINITE {
        -LARGEST_FINITE // never -infinity under this attribute
    } else {
        round_unbounded_signed(x, Dir::TowardPositive)
    }
}

// ---- exhaustive agreement, both ends, real -9 through 9 (scaled) ----------

const fn agrees_signed(in_range: Dir, over: Dir, under: Dir, which: u8) -> bool {
    let mut x = -SCALE * 9;
    while x <= SCALE * 9 {
        if x != 0 {
            let d = design_pipeline_signed(x, in_range, over, under, true);
            let o = match which {
                0 => oracle_rn_signed(x),
                1 => oracle_rz_signed(x),
                _ => oracle_rp_signed(x),
            };
            if d != o {
                return false;
            }
        }
        x += 1;
    }
    true
}

/// roundTiesToEven: saturate outward on both ends (TowardPositive over,
/// TowardNegative under), which is the symmetric pairing.
const _: () = assert!(agrees_signed(
    Dir::NearestTiesEven,
    Dir::TowardPositive,
    Dir::TowardNegative,
    0
));

/// roundTowardZero: clamp inward on both ends (anything but the outward
/// marker clamps, per the pipeline's own fallback arm).
const _: () = assert!(agrees_signed(
    Dir::TowardZero,
    Dir::TowardZero,
    Dir::TowardZero,
    1
));

/// roundTowardPositive: outward on the TOP only (TowardPositive), inward on
/// the bottom (the under-range marker is NOT TowardNegative, because nothing
/// past the negative bound is ever selected under this attribute).
const _: () = assert!(agrees_signed(
    Dir::TowardPositive,
    Dir::TowardPositive,
    Dir::TowardZero,
    2
));

// ---- the asymmetry stated as its own assertion, not inferred from the loop
// above --------------------------------------------------------------------

/// Deep positive overflow under roundTowardPositive: +infinity.
const _: () = assert!(oracle_rp_signed(SCALE * 100) == POS_INF);
/// Deep negative "overflow" under the SAME attribute: the negative largest
/// finite, never -infinity. This is the asymmetry the disclaimer in file 30's
/// probe 2 left unchecked.
const _: () = assert!(oracle_rp_signed(-SCALE * 100) == -LARGEST_FINITE);
const _: () = assert!(oracle_rp_signed(-SCALE * 100) != NEG_INF);

/// The design's own pipeline reproduces the asymmetry with NO sign-
/// conditional code in the resolution logic itself (`design_pipeline_signed`
/// dispatches on `over` vs `under` exactly as the spec's separate
/// `OverRange`/`UnderRange` fields already do; the only sign-awareness lives
/// in `round_unbounded_signed`, which is shared with the in-range case and
/// already exists in the design for ordinary directed rounding).
const _: () = assert!(
    design_pipeline_signed(
        SCALE * 100,
        Dir::TowardPositive,
        Dir::TowardPositive,
        Dir::TowardZero,
        true
    ) == POS_INF
);
const _: () = assert!(
    design_pipeline_signed(
        -SCALE * 100,
        Dir::TowardPositive,
        Dir::TowardPositive,
        Dir::TowardZero,
        true
    ) == -LARGEST_FINITE
);

/// Without specials, both ends clamp regardless of attribute, symmetric
/// magnitude-for-magnitude with the positive case probe 2 already checked.
const _: () = assert!(
    design_pipeline_signed(
        -SCALE * 100,
        Dir::TowardPositive,
        Dir::TowardPositive,
        Dir::TowardNegative,
        false
    ) == -LARGEST_FINITE
);

/// roundTiesToEven at the boundary, both signs, mirrors probe 2's positive
/// boundary result exactly (odd symmetry, checked rather than assumed).
const _: () = assert!(round_unbounded_signed(RN_BOUNDARY, Dir::NearestTiesEven) == FIRST_PAST_EMAX);
const _: () =
    assert!(round_unbounded_signed(-RN_BOUNDARY, Dir::NearestTiesEven) == -FIRST_PAST_EMAX);
const _: () = assert!(
    design_pipeline_signed(
        RN_BOUNDARY,
        Dir::NearestTiesEven,
        Dir::TowardPositive,
        Dir::TowardNegative,
        true
    ) == POS_INF
);
const _: () = assert!(
    design_pipeline_signed(
        -RN_BOUNDARY,
        Dir::NearestTiesEven,
        Dir::TowardPositive,
        Dir::TowardNegative,
        true
    ) == NEG_INF
);

// ---- negative control: mirroring the resolution naively (using the SAME
// marker, TowardPositive, to mean "outward" at BOTH ends) reproduces the
// wrong attribute and disagrees with the true roundTowardPositive oracle ----

const fn design_pipeline_wrongly_mirrored(x: i64, over: Dir, _under: Dir) -> i64 {
    // deliberately reuses `over`'s marker as if it meant "outward" at the
    // under-range position too, which is the bug this probe exists to catch
    let r = round_unbounded_signed(x, Dir::TowardPositive);
    let m = if r >= 0 { r } else { -r };
    if m <= LARGEST_FINITE {
        return r;
    }
    if r > 0 {
        match over {
            Dir::TowardPositive => POS_INF,
            _ => LARGEST_FINITE,
        }
    } else {
        match over /* bug: should be `under` */ {
            Dir::TowardPositive => NEG_INF,
            _ => -LARGEST_FINITE,
        }
    }
}

const _: () = assert!(
    design_pipeline_wrongly_mirrored(-SCALE * 100, Dir::TowardPositive, Dir::TowardZero)
        != oracle_rp_signed(-SCALE * 100)
);
