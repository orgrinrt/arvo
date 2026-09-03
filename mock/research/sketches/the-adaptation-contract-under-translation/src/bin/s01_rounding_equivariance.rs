//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Step 01. Does the rounding region commute with translation, and on which
//! domain.
//!
//! A position is `slot + residue`. Translating by a whole number of quanta moves
//! the slot and leaves the residue alone, so the rounding region commutes with
//! translation exactly when the slot it picks is `slot + offset` for an offset
//! that depends on the residue and not on the slot.
//!
//! So the measurement is: fix a residue, walk the slot across a band, collect
//! the offsets, and ask whether the set has one member. Nothing is declared
//! here. The offsets come out of `arvo_format::apply::adapt`.
//!
//! Five domains, because a mode may read the sign of the slot or its parity and
//! each is defeated by a different restriction. A domain with no negative
//! positions defeats the sign. A domain with no exactly-half residue defeats
//! whatever a tie rule reads. A domain of even slots defeats the parity and not
//! the sign, which is what separates the two rather than leaving one inferred.

use std::collections::BTreeSet;

use arvo_format::rounding::Mode;
use the_adaptation_contract_under_translation::{
    MODES,
    Range,
    dither,
    is_tie,
    position,
    residues,
    round_only,
    verdict,
};

/// A domain, as the restriction that names it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Domain {
    name:      &'static str,
    /// Whether the band reaches below zero.
    negatives: bool,
    /// Whether an exactly-half residue is in it.
    ties:      bool,
    /// The stride across the band. Two is the even-slot domain, and it is the
    /// domain of translations by an even number of quanta.
    step:      i64,
}

const DOMAINS: [Domain; 5] = [
    Domain {
        name:      "full",
        negatives: true,
        ties:      true,
        step:      1,
    },
    Domain {
        name:      "no-negatives",
        negatives: false,
        ties:      true,
        step:      1,
    },
    Domain {
        name:      "no-ties",
        negatives: true,
        ties:      false,
        step:      1,
    },
    Domain {
        name:      "no-negatives-no-ties",
        negatives: false,
        ties:      false,
        step:      1,
    },
    Domain {
        name:      "even-slots",
        negatives: true,
        ties:      true,
        step:      2,
    },
];

/// A band of slots to walk, named so a report says which one produced a verdict.
#[derive(Clone, Copy)]
struct Band {
    name:   &'static str,
    centre: i64,
    reach:  i64,
}

/// Three bands, because a verdict that depended on where the band sat would be
/// a fact about the band. The far ones are still deep inside the wide range, so
/// the rounding region is what is measured at all three.
const BANDS: [Band; 3] = [
    Band {
        name:   "at-zero",
        centre: 0,
        reach:  64,
    },
    Band {
        name:   "far",
        centre: 1_000_000,
        reach:  64,
    },
    Band {
        name:   "very-far",
        centre: 1 << 37,
        reach:  64,
    },
];

/// What a sweep over one mode, one domain and one band found.
struct Sweep {
    equivariant:       bool,
    witness:           Option<(i64, i64, i64, i64, i64, i64)>,
    escapes:           u64,
    visited:           u64,
    ties_reached:      u64,
    negatives_reached: u64,
}

fn sweep(range: Range, mode: Mode, domain: Domain, band: Band, dith: (i64, i64)) -> Sweep {
    let (base_lo, _) = range.bounds();
    // A band centred away from zero has no negative half to restrict, so the
    // no-negatives domains are only meaningful on the band at zero. The walk
    // still runs, and the report says which band it ran on.
    let lo = if domain.negatives && base_lo < 0 {
        band.centre - band.reach
    } else {
        (band.centre - band.reach).max(0)
    };
    let hi = band.centre + band.reach;
    let mut out = Sweep {
        equivariant:       true,
        witness:           None,
        escapes:           0,
        visited:           0,
        ties_reached:      0,
        negatives_reached: 0,
    };
    let d = dither(dith.0, dith.1);
    for (num, den) in residues() {
        if !domain.ties && is_tie(num, den) {
            continue;
        }
        let mut offsets: BTreeSet<i64> = BTreeSet::new();
        let mut first: Option<(i64, i64)> = None;
        let mut slot = lo;
        while slot <= hi {
            let (got, isolated) = round_only(range, mode, position(slot, num, den), d);
            if !isolated {
                out.escapes += 1;
            }
            out.visited += 1;
            if is_tie(num, den) {
                out.ties_reached += 1;
            }
            if slot < 0 {
                out.negatives_reached += 1;
            }
            let offset = got - slot;
            offsets.insert(offset);
            match first {
                None => first = Some((slot, offset)),
                Some((s0, o0)) => {
                    if o0 != offset && out.witness.is_none() {
                        out.witness = Some((s0, o0, slot, offset, num, den));
                    }
                },
            }
            slot += domain.step;
        }
        if offsets.len() > 1 {
            out.equivariant = false;
        }
    }
    out
}

/// The dithers the stochastic mode is measured at. Each is fixed, so at each one
/// the mode is a function of the residue alone and the question has an answer;
/// the verdict must be the same at every one or the measurement is about the
/// dither rather than about the mode.
const DITHERS: [(i64, i64); 4] = [(0, 1), (1, 8), (1, 2), (7, 8)];

fn main() {
    println!("step 01: does the rounding region commute with translation");
    println!("residues over denominators 2, 3, 4, 5, 8, 16; three bands; five domains");
    println!();

    let mut escapes_total = 0u64;
    let mut ties_total = 0u64;
    let mut negatives_total = 0u64;
    let mut unstable_cells = 0u32;
    let mut band_disagreements = 0u32;
    let mut any_equivariant_on_full = false;
    let mut any_broken_on_full = false;

    for &mode in &MODES {
        for domain in DOMAINS {
            // The verdict at the band on zero is the one reported; the other two
            // bands are the control that it is not a fact about where the band
            // sat.
            let mut per_band = Vec::new();
            for band in BANDS {
                let mut verdicts = BTreeSet::new();
                let mut shown: Option<Sweep> = None;
                for &d in &DITHERS {
                    let s = sweep(Range::WideSigned, mode, domain, band, d);
                    verdicts.insert(s.equivariant);
                    escapes_total += s.escapes;
                    ties_total += s.ties_reached;
                    negatives_total += s.negatives_reached;
                    if shown.is_none() {
                        shown = Some(s);
                    }
                }
                if verdicts.len() > 1 {
                    unstable_cells += 1;
                }
                per_band.push((band.name, shown.expect("at least one dither")));
            }
            let (_, ref s) = per_band[0];
            // A band away from zero has no negatives in it whatever the domain
            // says, so only the domains that keep the negatives are comparable
            // across bands.
            if domain.negatives {
                let first = per_band[0].1.equivariant;
                for (_, other) in per_band.iter().skip(1) {
                    // Away from zero the sign never changes, so a sign-reading
                    // mode looks equivariant there. That is the expected
                    // difference and it is reported rather than counted as a
                    // disagreement; what would be a disagreement is the far band
                    // breaking where the band at zero held.
                    if first && !other.equivariant {
                        band_disagreements += 1;
                    }
                }
            }
            println!(
                "cell mode={mode:?} domain={} equivariant={} visited={} far={} very_far={} witness={}",
                domain.name,
                s.equivariant,
                s.visited,
                per_band[1].1.equivariant,
                per_band[2].1.equivariant,
                match s.witness {
                    None => "none".to_string(),
                    Some((s0, o0, s1, o1, num, den)) =>
                        format!(
                            "slot {s0} offset {o0} against slot {s1} offset {o1} at residue {num}/{den}"
                        ),
                }
            );
            if domain.name == "full" {
                if s.equivariant {
                    any_equivariant_on_full = true;
                } else {
                    any_broken_on_full = true;
                }
            }
        }
    }

    println!();
    println!("--- the controls ---");

    verdict(
        "W1",
        "no answer left the wide range, so the rounding region is what was measured",
        escapes_total == 0,
    );
    verdict(
        "W2",
        "the residue set reaches an exactly-half position",
        ties_total > 0,
    );
    verdict(
        "W3",
        "the slot band reaches a negative position",
        negatives_total > 0,
    );
    verdict(
        "W4",
        "the instrument separates: some mode commutes on the full domain and some does not",
        any_equivariant_on_full && any_broken_on_full,
    );
    verdict(
        "W4b",
        "no cell's verdict moved with the dither, so the stochastic answer is about the mode",
        unstable_cells == 0,
    );
    verdict(
        "W4c",
        "no cell holding at the band on zero broke at a band far from it",
        band_disagreements == 0,
    );

    // W5. The degenerate domain. A band of one slot cannot exhibit a difference
    // between two slots, so every mode must report equivariant there. A "yes"
    // from this instrument is a claim about the domain it swept, and this says
    // so out loud rather than in a comment.
    let one_slot = Band {
        name:   "one-slot",
        centre: 0,
        reach:  0,
    };
    let all_yes_on_one_slot = MODES
        .iter()
        .all(|&mode| sweep(Range::WideSigned, mode, DOMAINS[0], one_slot, (1, 2)).equivariant);
    verdict(
        "W5",
        "collapsing the band to one slot reports every mode equivariant, which is what a \
         vacuous domain looks like",
        all_yes_on_one_slot,
    );

    // W6. Every witness re-checked by calling the map at exactly the two
    // positions it names, so a failure verdict rests on two answers a reader can
    // reproduce rather than on a set the sweep accumulated.
    let mut witnesses = 0u32;
    let mut confirmed = 0u32;
    for &mode in &MODES {
        for domain in DOMAINS {
            let s = sweep(Range::WideSigned, mode, domain, BANDS[0], (1, 2));
            if let Some((s0, o0, s1, o1, num, den)) = s.witness {
                witnesses += 1;
                let (a, _) = round_only(
                    Range::WideSigned,
                    mode,
                    position(s0, num, den),
                    dither(1, 2),
                );
                let (b, _) = round_only(
                    Range::WideSigned,
                    mode,
                    position(s1, num, den),
                    dither(1, 2),
                );
                if a - s0 == o0 && b - s1 == o1 && o0 != o1 {
                    confirmed += 1;
                }
            }
        }
    }
    verdict(
        "W6",
        &format!("every witness re-checked against the map: {confirmed} of {witnesses}"),
        witnesses > 0 && witnesses == confirmed,
    );

    // W7. The same verdict on a format whose own range has no negatives, so the
    // no-negatives result is about the domain rather than about which format
    // supplied it.
    let mut agree = 0u32;
    let mut compared = 0u32;
    for &mode in &MODES {
        let signed_nonneg = sweep(Range::WideSigned, mode, DOMAINS[1], BANDS[0], (1, 2));
        let unsigned = sweep(Range::WideUnsigned, mode, DOMAINS[0], BANDS[0], (1, 2));
        compared += 1;
        if signed_nonneg.equivariant == unsigned.equivariant {
            agree += 1;
        }
    }
    verdict(
        "W7",
        &format!(
            "the no-negatives verdict is the same on an unsigned format: {agree} of {compared} modes"
        ),
        agree == compared,
    );

    // W8. The even-slot domain separates parity from sign. A mode reading the
    // parity is equivariant there because every translation in it is even; a
    // mode reading the sign is not, because the band still crosses zero. If both
    // modes behaved alike here the classification would be an inference rather
    // than a measurement.
    let parity_reader = sweep(
        Range::WideSigned,
        Mode::HalfEven,
        DOMAINS[4],
        BANDS[0],
        (1, 2),
    );
    let sign_reader = sweep(
        Range::WideSigned,
        Mode::TowardZero,
        DOMAINS[4],
        BANDS[0],
        (1, 2),
    );
    verdict(
        "W8",
        "the even-slot domain separates the parity reader from the sign reader",
        parity_reader.equivariant && !sign_reader.equivariant,
    );
}
