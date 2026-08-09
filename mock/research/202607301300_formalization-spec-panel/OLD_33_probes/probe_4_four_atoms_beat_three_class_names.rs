//! Probe 4: the recovery map's three declared class names do not cover the
//! design's own five recovery maps; four atomic properties do.
//!
//! The consolidation classifies a recovery map `phi` as one of three things
//! (26_consolidation_two.md:76-82): a **homomorphism** (commutes with the
//! operation), a **partial identity** (returns its argument unchanged wherever
//! it returns at all), or a **retraction** (total, fixes the representable set
//! pointwise, order preserving). Each implies a law by a short proof.
//!
//! The design has three declared class names and, once the settled identity
//! contract's dithered entry point is counted, five maps to classify. This probe
//! computes four atomic properties for each map and shows the coverage gap
//! precisely, rather than arguing about it.
//!
//!   T  total:        defined everywhere on the exact domain.
//!   F  fixes:        `phi(x) = x` for every x the destination numeral holds.
//!   M  monotone:     order preserving where defined.
//!   H  homomorphic:  `phi(x op y) ~= phi(phi x op phi y)` (Kleene equality),
//!                    quantified over the EXACT domain, not over the destination
//!                    numeral. Quantifying over the destination numeral makes the
//!                    condition vacuous for every map that fixes it pointwise,
//!                    which is four of the five below. An earlier draft of this
//!                    probe did exactly that and reported clamping as a
//!                    homomorphism; the const assertion refused, which is the
//!                    check working.
//!
//! CLAIM A. Each of the five maps has a distinct (T,F,M,H) signature, so the
//! four atoms separate all five.
//!
//! CLAIM B. Two of the five sit in none of the three declared classes:
//! `SubstituteZero` (total and fixing but not order preserving, so not a
//! retraction; not homomorphic; not partial, since it is total) and the confined
//! dithered entry point (total and monotone but not fixing, so none of the
//! three). A design that ships three class names has nowhere to put either, and
//! the second of the two arrives from the settled identity half rather than from
//! anything the algebra half already had.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_4_four_atoms_beat_three_class_names.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

// Destination numeral N, eight values. Exact domain E, wide enough to hold the
// sums the homomorphism condition forms.
const N_LO: i32 = -4;
const N_HI: i32 = 3;
const E_LO: i32 = -64;
const E_HI: i32 = 63;
// The homomorphism quantifier's own range, chosen so that x + y stays inside E.
const H_LO: i32 = -32;
const H_HI: i32 = 31;
const NSPAN: i32 = N_HI - N_LO + 1; // 8

type PV = (i32, bool);

// ---------------------------------------------------------------------------
// The five recovery maps. Four are the design's own `Resolution` instances; the
// fifth is the dithered entry point the settled identity contract adds, with the
// confinement repair applied (31_arntzen_settling_the_identity_contract.md:386-388).
// ---------------------------------------------------------------------------

const PHI_WRAP: u8 = 0; // ReduceModulo
const PHI_CLAMP: u8 = 1; // Direction-resolved clamp at both ends
const PHI_REFUSE: u8 = 2; // Refuse, which is Precise's own rule
const PHI_ZERO: u8 = 3; // SubstituteZero
const PHI_DITHER: u8 = 4; // confined quantize(x + noise), noise fixed at +1
const PHI_COUNT: u8 = 5;

const DITHER_NOISE: i32 = 1;

const fn phi(which: u8, x: i32) -> PV {
    match which {
        PHI_WRAP => (N_LO + (x - N_LO).rem_euclid(NSPAN), true),
        PHI_CLAMP => {
            if x < N_LO {
                (N_LO, true)
            } else if x > N_HI {
                (N_HI, true)
            } else {
                (x, true)
            }
        }
        PHI_REFUSE => {
            if x < N_LO || x > N_HI {
                (0, false)
            } else {
                (x, true)
            }
        }
        PHI_ZERO => {
            if x < N_LO || x > N_HI {
                (0, true)
            } else {
                (x, true)
            }
        }
        _ => {
            // Confine first, then perturb, then clamp: the repair that restores
            // totality without letting the perturbation manufacture a refusal.
            let c = if x < N_LO {
                N_LO
            } else if x > N_HI {
                N_HI
            } else {
                x
            };
            let p = c + DITHER_NOISE;
            if p < N_LO {
                (N_LO, true)
            } else if p > N_HI {
                (N_HI, true)
            } else {
                (p, true)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The four atoms.
// ---------------------------------------------------------------------------

const fn total(p: u8) -> bool {
    let mut x = E_LO;
    while x <= E_HI {
        if !phi(p, x).1 {
            return false;
        }
        x += 1;
    }
    true
}

const fn fixes(p: u8) -> bool {
    let mut x = N_LO;
    while x <= N_HI {
        let r = phi(p, x);
        if !r.1 || r.0 != x {
            return false;
        }
        x += 1;
    }
    true
}

const fn monotone(p: u8) -> bool {
    let mut x = E_LO;
    while x <= E_HI {
        let mut y = x;
        while y <= E_HI {
            let (rx, ry) = (phi(p, x), phi(p, y));
            if rx.1 && ry.1 && rx.0 > ry.0 {
                return false;
            }
            y += 1;
        }
        x += 1;
    }
    true
}

const fn homomorphic(p: u8) -> bool {
    let mut x = H_LO;
    while x <= H_HI {
        let mut y = H_LO;
        while y <= H_HI {
            let direct = phi(p, x + y);
            let (px, py) = (phi(p, x), phi(p, y));
            let nested = if px.1 && py.1 {
                phi(p, px.0 + py.0)
            } else {
                (0, false)
            };
            if direct.1 != nested.1 {
                return false;
            }
            if direct.1 && direct.0 != nested.0 {
                return false;
            }
            y += 1;
        }
        x += 1;
    }
    true
}

/// The four atoms packed into one nibble, so signatures can be compared.
const fn signature(p: u8) -> u8 {
    (total(p) as u8)
        | ((fixes(p) as u8) << 1)
        | ((monotone(p) as u8) << 2)
        | ((homomorphic(p) as u8) << 3)
}

// ---------------------------------------------------------------------------
// CLAIM A: five distinct signatures.
// ---------------------------------------------------------------------------

const SIGS: [u8; 5] = [
    signature(PHI_WRAP),
    signature(PHI_CLAMP),
    signature(PHI_REFUSE),
    signature(PHI_ZERO),
    signature(PHI_DITHER),
];

const fn all_distinct() -> bool {
    let mut i = 0;
    while i < 5 {
        let mut j = i + 1;
        while j < 5 {
            if SIGS[i] == SIGS[j] {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const _: () = assert!(all_distinct());

// The individual columns, stated so the signature nibbles are readable rather
// than only distinct.
const _: () = assert!(total(PHI_WRAP) && fixes(PHI_WRAP) && !monotone(PHI_WRAP));
const _: () = assert!(homomorphic(PHI_WRAP));

const _: () = assert!(total(PHI_CLAMP) && fixes(PHI_CLAMP) && monotone(PHI_CLAMP));
const _: () = assert!(!homomorphic(PHI_CLAMP));

const _: () = assert!(!total(PHI_REFUSE) && fixes(PHI_REFUSE) && monotone(PHI_REFUSE));
const _: () = assert!(!homomorphic(PHI_REFUSE));

const _: () = assert!(total(PHI_ZERO) && fixes(PHI_ZERO) && !monotone(PHI_ZERO));
const _: () = assert!(!homomorphic(PHI_ZERO));

const _: () = assert!(total(PHI_DITHER) && !fixes(PHI_DITHER) && monotone(PHI_DITHER));
const _: () = assert!(!homomorphic(PHI_DITHER));

// ---------------------------------------------------------------------------
// CLAIM B: two maps sit in none of the three declared classes.
// ---------------------------------------------------------------------------

/// The consolidation's own three class names, written as predicates over the
/// atoms so the coverage question is decidable rather than rhetorical.
const fn is_homomorphism(p: u8) -> bool {
    homomorphic(p)
}
const fn is_partial_identity(p: u8) -> bool {
    // "returns its argument unchanged wherever it returns at all", which is
    // fixing plus a domain restricted to the destination numeral.
    fixes(p) && !total(p)
}
const fn is_retraction(p: u8) -> bool {
    total(p) && fixes(p) && monotone(p)
}
const fn classified(p: u8) -> bool {
    is_homomorphism(p) || is_partial_identity(p) || is_retraction(p)
}

const _: () = assert!(classified(PHI_WRAP));
const _: () = assert!(classified(PHI_CLAMP));
const _: () = assert!(classified(PHI_REFUSE));

// The two the three names cannot place.
const _: () = assert!(!classified(PHI_ZERO));
const _: () = assert!(!classified(PHI_DITHER));

// And the classes are not merely incomplete, they overlap: clamping is a
// retraction and refusal is a partial identity, but nothing prevents a map from
// satisfying two of the three at once, so "the class" is not a function of the
// map even where a class exists. The four atoms are a signature; the three names
// are a partial, overlapping cover.
const _: () = assert!(is_retraction(PHI_CLAMP) && !is_homomorphism(PHI_CLAMP));

/// Present so the same file can be compiled as a binary and print the table.
fn main() {
    let names = [
        "ReduceModulo",
        "Clamp",
        "Refuse",
        "SubstituteZero",
        "DitherConfined",
    ];
    println!("map              T F M H  classified");
    let mut p = 0u8;
    while p < PHI_COUNT {
        println!(
            "{:<16} {} {} {} {}  {}",
            names[p as usize],
            total(p) as u8,
            fixes(p) as u8,
            monotone(p) as u8,
            homomorphic(p) as u8,
            classified(p) as u8
        );
        p += 1;
    }
}
