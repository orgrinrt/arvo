//! Probe 1: a plain-`Direction` quantiser's error is a pure function of the
//! exact value's residue class modulo the quantum. An externally supplied
//! noise term, added before rounding, breaks that dependence, with zero new
//! mechanism beyond what the round-first amendment (file 28, section 4)
//! already introduces: `quantize(exact + noise)` on the same unbounded-grid
//! rounder used for the undithered case.
//!
//! Model: quantum Q = 10, round to nearest multiple of Q (ties away from
//! zero, unused by the chosen inputs so the tie rule does not matter here).
//! Two call sites, `i1` with exact value 3 and `i2` with exact value 23.
//! `23 mod 10 == 3 mod 10`, so a memoryless, deterministic `Direction` rule
//! delivers the SAME error at both sites: this is the mechanism of banding,
//! stated as a fact about the map rather than as an image. Two different
//! externally supplied noise terms (4 and -2, standing in for whatever a
//! caller's dither source produced) are added before rounding at each site.
//! The two dithered errors differ, because the noise broke the pure
//! dependence on `exact mod Q`.
//!
//! What this shows: `Direction` cannot decorrelate error from a fixed input
//! by construction, because it is a memoryless pure function of the exact
//! value alone. Decorrelation needs an extra input the map did not have
//! before, which is exactly what the round-first amendment's "round on the
//! unbounded-exponent extension of the grid" makes representable: feed the
//! extension `exact + noise` instead of `exact`, same rounder, no new axis.
//! Nothing here claims statistical independence; that is a stronger claim
//! about ensembles of noise draws, argued in the deliverable, not checked by
//! this probe. This probe checks only the mechanism: same undithered error,
//! different dithered error, from an extra input with no new state.

#![no_std]

const Q: i32 = 10;

/// Round to the nearest multiple of `Q`, correct for negative inputs via
/// Euclidean division so the residual carried by a shaper (probe 2) can be
/// negative without special-casing.
const fn round_to_nearest_multiple(v: i32, q: i32) -> i32 {
    let down = v.div_euclid(q) * q;
    let rem = v - down; // rem in [0, q)
    if rem * 2 < q {
        down
    } else {
        down + q
    }
}

const I1_EXACT: i32 = 3;
const I2_EXACT: i32 = 23;

// same residue class mod Q: 3 mod 10 == 23 mod 10 == 3
const _: () = assert!(I1_EXACT.rem_euclid(Q) == I2_EXACT.rem_euclid(Q));

const ERROR0_I1: i32 = round_to_nearest_multiple(I1_EXACT, Q) - I1_EXACT;
const ERROR0_I2: i32 = round_to_nearest_multiple(I2_EXACT, Q) - I2_EXACT;

// undithered: a pure function of exact value alone gives the same error at
// both sites, because both sites have the same residue. this IS banding.
const _: () = assert!(ERROR0_I1 == -3);
const _: () = assert!(ERROR0_I2 == -3);
const _: () = assert!(ERROR0_I1 == ERROR0_I2);

// two externally supplied noise samples, standing in for a caller's dither
// source (arvo owns neither value; both are just extra function arguments)
const NOISE_I1: i32 = 4;
const NOISE_I2: i32 = -2;

const ERRORD_I1: i32 = round_to_nearest_multiple(I1_EXACT + NOISE_I1, Q) - I1_EXACT;
const ERRORD_I2: i32 = round_to_nearest_multiple(I2_EXACT + NOISE_I2, Q) - I2_EXACT;

const _: () = assert!(ERRORD_I1 == 7);
const _: () = assert!(ERRORD_I2 == -3);

// the finding: same residue class, same undithered error, different
// dithered error. the extra input broke the pure dependence on the residue.
const _: () = assert!(ERRORD_I1 != ERRORD_I2);

// negative control: with noise pinned to zero at both sites, the dithered
// path degenerates back to the undithered one exactly, confirming the
// mechanism above is the noise term doing the work, not an artefact of
// having two call sites at all.
const ERROR_ZERO_NOISE_I1: i32 = round_to_nearest_multiple(I1_EXACT + 0, Q) - I1_EXACT;
const ERROR_ZERO_NOISE_I2: i32 = round_to_nearest_multiple(I2_EXACT + 0, Q) - I2_EXACT;
const _: () = assert!(ERROR_ZERO_NOISE_I1 == ERROR_ZERO_NOISE_I2);
