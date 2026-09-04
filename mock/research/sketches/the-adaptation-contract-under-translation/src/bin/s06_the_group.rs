//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Step 06. Are translation and reflection the right two, or two picked off a
//! list.
//!
//! The isometries of a grid spaced one quantum apart are the maps `x -> e*x + c`
//! with `e` in `{+1, -1}` and `c` a whole number of quanta. Translation and
//! reflection are the two generators of that group, so if the map commuting with
//! a composite is decided by whether it commutes with each generator, the two
//! laws are the whole of what "commutes with a symmetry of the grid" means and
//! there is no third to find.
//!
//! Sufficiency is immediate: a map commuting with two maps commutes with their
//! composite. What is worth measuring is necessity, because a composite can hold
//! accidentally where a generator fails, and if it does then the composite is a
//! law in its own right and the two generators do not cover it.
//!
//! So this step measures three verdicts per cell, all from `adapt`, and compares
//! the composite against the conjunction of the two.

use the_adaptation_contract_under_translation::{
    MODES,
    POLICIES,
    Range,
    adapt_at,
    dither,
    position,
    residues,
    verdict,
};

const DITHER: (i64, i64) = (1, 2);

/// The verdict of one law over one cell, and the count of what it walked.
struct Verdict {
    held:   bool,
    walked: u64,
}

/// The relocation law: adapting a translated position against adapting and then
/// translating.
fn relocation(
    range: Range,
    mode: arvo_format::rounding::Mode,
    policy: arvo_format::overflow::Policy,
) -> Verdict {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let d = dither(DITHER.0, DITHER.1);
    let mut out = Verdict {
        held:   true,
        walked: 0,
    };
    for (num, den) in residues() {
        for n in (lo - span) ..= (hi + span) {
            let here = adapt_at(range, mode, policy, position(n, num, den), d);
            for c in lo ..= hi {
                out.walked += 1;
                let direct = adapt_at(range, mode, policy, position(n + c, num, den), d);
                let staged = adapt_at(range, mode, policy, position(here + c, 0, 1), d);
                if direct != staged {
                    out.held = false;
                }
            }
        }
    }
    out
}

/// The reflection law.
fn reflection(
    range: Range,
    mode: arvo_format::rounding::Mode,
    policy: arvo_format::overflow::Policy,
) -> Verdict {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let d = dither(DITHER.0, DITHER.1);
    let mut out = Verdict {
        held:   true,
        walked: 0,
    };
    for (num, den) in residues() {
        for n in (lo - span) ..= (hi + span) {
            out.walked += 1;
            let here = adapt_at(range, mode, policy, position(n, num, den), d);
            let direct = adapt_at(range, mode, policy, position(-n, -num, den), d);
            let staged = adapt_at(range, mode, policy, position(-here, 0, 1), d);
            if direct != staged {
                out.held = false;
            }
        }
    }
    out
}

/// The composite law at one orientation: adapting `e * position + c` against
/// adapting and then applying the same map to the answer.
fn composite(
    range: Range,
    mode: arvo_format::rounding::Mode,
    policy: arvo_format::overflow::Policy,
    orientation: i64,
) -> Verdict {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let d = dither(DITHER.0, DITHER.1);
    let mut out = Verdict {
        held:   true,
        walked: 0,
    };
    for (num, den) in residues() {
        for n in (lo - span) ..= (hi + span) {
            let here = adapt_at(range, mode, policy, position(n, num, den), d);
            for c in lo ..= hi {
                out.walked += 1;
                // `e * (n + num/den) + c` is `e*n + c` at residue `e*num/den`,
                // and the coordinate normalises a negative numerator into the
                // slot, so no case split is needed here.
                let direct = adapt_at(
                    range,
                    mode,
                    policy,
                    position(orientation * n + c, orientation * num, den),
                    d,
                );
                let staged = adapt_at(
                    range,
                    mode,
                    policy,
                    position(orientation * here + c, 0, 1),
                    d,
                );
                if direct != staged {
                    out.held = false;
                }
            }
        }
    }
    out
}

fn main() {
    println!("step 06: is the pair of laws the whole symmetry group of the grid");
    println!("composite: adapt(e*position + c) against adapt(e*adapt(position) + c)");
    println!();

    let mut cells = 0u32;
    let mut agreements = 0u32;
    let mut accidental = Vec::new();
    let mut missed = Vec::new();
    let mut walked = 0u64;
    let mut composite_true = 0u32;
    let mut composite_false = 0u32;

    for range in Range::SMALL {
        for &mode in &MODES {
            for &policy in &POLICIES {
                let translates = relocation(range, mode, policy);
                let reflects = reflection(range, mode, policy);
                for orientation in [1i64, -1] {
                    let both = composite(range, mode, policy, orientation);
                    walked += both.walked + translates.walked + reflects.walked;
                    // The generators the composite is built from: a positive
                    // orientation is a translation alone, a negative one is a
                    // reflection followed by a translation.
                    let predicted = if orientation > 0 {
                        translates.held
                    } else {
                        translates.held && reflects.held
                    };
                    cells += 1;
                    if both.held {
                        composite_true += 1;
                    } else {
                        composite_false += 1;
                    }
                    if both.held == predicted {
                        agreements += 1;
                    } else if both.held {
                        accidental.push(format!(
                            "{range:?} {mode:?} {policy:?} orientation {orientation}: the \
                             composite holds where a generator fails"
                        ));
                    } else {
                        missed.push(format!(
                            "{range:?} {mode:?} {policy:?} orientation {orientation}: both \
                             generators hold and the composite does not"
                        ));
                    }
                    println!(
                        "cell range={range:?} mode={mode:?} policy={policy:?} orientation={orientation} composite={} translates={} reflects={} predicted={predicted}",
                        both.held, translates.held, reflects.held
                    );
                }
            }
        }
    }

    println!();
    println!("--- the controls ---");
    println!(
        "cells={cells} walked={walked} composite true/false={composite_true}/{composite_false}"
    );

    verdict(
        "G1",
        &format!(
            "the composite holds exactly where its generators do: {agreements} of {cells} cells"
        ),
        agreements == cells,
    );
    for a in &accidental {
        println!("G1 accidental: {a}");
    }
    for m in &missed {
        println!("G1 missed: {m}");
    }

    verdict(
        "G2",
        "the composite answers both ways, so agreeing with the conjunction is not agreeing with \
         a constant",
        composite_true > 0 && composite_false > 0,
    );

    // G3. The two orientations are not the same walk. If they were, the negative
    // one would be the positive one under another name and the reflection half
    // would be doing nothing.
    let mut differing = 0u32;
    for range in Range::SMALL {
        for &mode in &MODES {
            for &policy in &POLICIES {
                let forward = composite(range, mode, policy, 1).held;
                let backward = composite(range, mode, policy, -1).held;
                if forward != backward {
                    differing += 1;
                }
            }
        }
    }
    verdict(
        "G3",
        &format!("the two orientations disagree somewhere: {differing} cells"),
        differing > 0,
    );

    // G4. The identity composite, which is the vacuous case. Orientation one with
    // the translation fixed at zero is `adapt(x) == adapt(adapt(x))`, which is
    // the map being a retraction and holds everywhere. Reported so a reader sees
    // what a trivially true cell looks like next to the ones above.
    let mut retraction_holds = true;
    let d = dither(DITHER.0, DITHER.1);
    for range in Range::SMALL {
        let (lo, hi) = range.bounds();
        let span = hi - lo + 1;
        for &mode in &MODES {
            for &policy in &POLICIES {
                for (num, den) in residues() {
                    for n in (lo - span) ..= (hi + span) {
                        let once = adapt_at(range, mode, policy, position(n, num, den), d);
                        let twice = adapt_at(range, mode, policy, position(once, 0, 1), d);
                        if once != twice {
                            retraction_holds = false;
                        }
                    }
                }
            }
        }
    }
    verdict(
        "G4",
        "the map is a retraction, which is the composite at the identity and is what makes the \
         outer rounding of a staged schedule dead",
        retraction_holds,
    );
}
