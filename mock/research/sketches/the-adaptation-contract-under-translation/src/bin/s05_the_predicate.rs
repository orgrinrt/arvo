//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Step 05. Does the predicate that is meant to ship agree with the map.
//!
//! Steps 01 to 04 measure what the map does. This step declares the const
//! predicate a consumer would gate an arm on and asks whether it says the same
//! thing, cell by cell, over the same cross.
//!
//! Two numbers matter and they are different. **Unsound** is a cell where the
//! predicate says the law holds and the map says it does not, and a single one
//! of those makes the predicate unshippable, because an arm gated on it computes
//! a wrong answer. **Conservative** is a cell where the predicate refuses a law
//! that in fact holds, which costs a lowering and never a result. The report
//! carries both rather than one agreement count, because collapsing them would
//! hide which kind of disagreement a number came from.

use arvo_format::overflow::Policy;
use arvo_format::rounding::Mode;
use the_adaptation_contract_under_translation::{
    MODES,
    POLICIES,
    Range,
    adapt_at,
    dither,
    is_tie,
    position,
    residues,
    verdict,
};

#[path = "../predicate.rs"]
mod predicate;

use predicate::{
    Reach,
    Reads,
    adaptation_reflects,
    adaptation_relocates,
    behaviour_of,
    rounding_is_reflection_equivariant,
};

const DITHER: (i64, i64) = (1, 2);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Restriction {
    negative_positions:    bool,
    negative_translations: bool,
    ties:                  bool,
}

const RESTRICTIONS: [Restriction; 8] = [
    Restriction {
        negative_positions:    true,
        negative_translations: true,
        ties:                  true,
    },
    Restriction {
        negative_positions:    true,
        negative_translations: true,
        ties:                  false,
    },
    Restriction {
        negative_positions:    true,
        negative_translations: false,
        ties:                  true,
    },
    Restriction {
        negative_positions:    true,
        negative_translations: false,
        ties:                  false,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: true,
        ties:                  true,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: true,
        ties:                  false,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: false,
        ties:                  true,
    },
    Restriction {
        negative_positions:    false,
        negative_translations: false,
        ties:                  false,
    },
];

/// The reach a cell hands the predicate, derived from the cell's own bounds.
fn reach_of(range: Range, r: Restriction) -> Reach {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    Reach {
        position_low:     if r.negative_positions { lo - span } else { 0.max(lo - span) },
        position_high:    hi + span,
        translation_low:  if r.negative_translations { lo } else { 0.max(lo) },
        translation_high: hi,
        ties:             r.ties,
    }
}

/// The relocation law, measured, over the same cell the reach describes.
fn relocation_holds(range: Range, mode: Mode, policy: Policy, r: Restriction) -> bool {
    let reach = reach_of(range, r);
    let d = dither(DITHER.0, DITHER.1);
    let mut held = true;
    for (num, den) in residues() {
        if !r.ties && is_tie(num, den) {
            continue;
        }
        for n in reach.position_low ..= reach.position_high {
            let here = adapt_at(range, mode, policy, position(n, num, den), d);
            for c in reach.translation_low ..= reach.translation_high {
                let direct = adapt_at(range, mode, policy, position(n + c, num, den), d);
                let staged = adapt_at(range, mode, policy, position(here + c, 0, 1), d);
                if direct != staged {
                    held = false;
                }
            }
        }
    }
    held
}

/// The reflection law, measured.
fn reflection_holds(range: Range, mode: Mode, policy: Policy) -> bool {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let d = dither(DITHER.0, DITHER.1);
    let mut held = true;
    for (num, den) in residues() {
        for slot in (lo - span) ..= (hi + span) {
            let here = adapt_at(range, mode, policy, position(slot, num, den), d);
            let direct = adapt_at(range, mode, policy, position(-slot, -num, den), d);
            let staged = adapt_at(range, mode, policy, position(-here, 0, 1), d);
            if direct != staged {
                held = false;
            }
        }
    }
    held
}

/// Whether the rounded slot is always the position's own slot or the one above,
/// which is what lets the excursion sides be derived from the position bounds.
fn rounding_stays_within_one_slot() -> (bool, u64) {
    let d = dither(DITHER.0, DITHER.1);
    let mut held = true;
    let mut checked = 0u64;
    for &mode in &MODES {
        for (num, den) in residues() {
            for slot in -256i64 ..= 256 {
                let got = adapt_at(
                    Range::WideSigned,
                    mode,
                    Policy::Wrap,
                    position(slot, num, den),
                    d,
                );
                checked += 1;
                if got != slot && got != slot + 1 {
                    held = false;
                }
            }
        }
    }
    (held, checked)
}

fn main() {
    println!("step 05: does the shipped predicate agree with the map");
    println!();

    let mut cells = 0u32;
    let mut exact = 0u32;
    let mut unsound = Vec::new();
    let mut conservative = Vec::new();

    for range in Range::SMALL {
        let (lo, hi) = range.bounds();
        for &mode in &MODES {
            for &policy in &POLICIES {
                for r in RESTRICTIONS {
                    let reach = reach_of(range, r);
                    let said = adaptation_relocates(mode, policy, lo, hi, reach);
                    let measured = relocation_holds(range, mode, policy, r);
                    cells += 1;
                    if said == measured {
                        exact += 1;
                    } else if said && !measured {
                        unsound.push(format!(
                            "range={range:?} mode={mode:?} policy={policy:?} restriction={r:?}"
                        ));
                    } else {
                        conservative.push(format!(
                            "range={range:?} mode={mode:?} policy={policy:?} restriction={r:?}"
                        ));
                    }
                    println!(
                        "relocation range={range:?} mode={mode:?} policy={policy:?} \
                         neg_pos={} neg_c={} ties={} predicate={said} measured={measured}",
                        r.negative_positions, r.negative_translations, r.ties
                    );
                }
            }
        }
    }

    println!();
    let mut reflection_cells = 0u32;
    let mut reflection_exact = 0u32;
    let mut reflection_unsound = Vec::new();
    for range in Range::SMALL {
        let (lo, hi) = range.bounds();
        let span = hi - lo + 1;
        // Reflection carries no translation, and the positions are the band the
        // measurement walks.
        let reach = Reach {
            position_low:     lo - span,
            position_high:    hi + span,
            translation_low:  0,
            translation_high: 0,
            ties:             true,
        };
        for &mode in &MODES {
            for &policy in &POLICIES {
                let said = adaptation_reflects(mode, policy, lo, hi, reach);
                let measured = reflection_holds(range, mode, policy);
                reflection_cells += 1;
                if said == measured {
                    reflection_exact += 1;
                } else if said && !measured {
                    reflection_unsound
                        .push(format!("range={range:?} mode={mode:?} policy={policy:?}"));
                }
                println!(
                    "reflection range={range:?} mode={mode:?} policy={policy:?} \
                     predicate={said} measured={measured}"
                );
            }
        }
    }

    println!();
    println!("--- the controls ---");

    verdict(
        "P1",
        &format!(
            "the relocation predicate is never unsound: {} cells said the law holds where the \
             map says it does not",
            unsound.len()
        ),
        unsound.is_empty(),
    );
    for u in &unsound {
        println!("P1 unsound: {u}");
    }
    verdict(
        "P2",
        &format!(
            "the relocation predicate is exact on {exact} of {cells} cells, with \
             {} conservative refusals",
            conservative.len()
        ),
        exact == cells,
    );
    for c in &conservative {
        println!("P2 conservative: {c}");
    }
    verdict(
        "P3",
        &format!(
            "the reflection predicate is never unsound and is exact on {reflection_exact} of \
             {reflection_cells} cells",
        ),
        reflection_unsound.is_empty() && reflection_exact == reflection_cells,
    );
    for u in &reflection_unsound {
        println!("P3 unsound: {u}");
    }

    // P4. The bound the excursion derivation rests on: the rounded slot is the
    // position's own slot or the one above it, never anything else. If it were
    // not, deriving the excursion sides from the position bounds would be
    // unsound rather than conservative.
    let (within, checked) = rounding_stays_within_one_slot();
    verdict(
        "P4",
        &format!(
            "the rounded slot is the position's slot or the next one, over {checked} positions"
        ),
        within,
    );

    // P5. The coincidence the classification refuses to derive. Over the six
    // names this vocabulary carries, reflection equivariance equals reading
    // something beyond the residue. It is measured and recorded here rather than
    // built into `behaviour_of`, because a nearest rule whose tie goes toward
    // positive infinity would break it and the vocabulary is closed at six only
    // by the design saying so.
    let coincidence = MODES.iter().all(|&mode| {
        rounding_is_reflection_equivariant(mode)
            == !matches!(behaviour_of(mode).reads, Reads::Nothing)
    });
    verdict(
        "P5",
        "over the six shipped names, commuting with reflection equals reading something \
         beyond the residue",
        coincidence,
    );

    // P6. The predicate separates. If it answered the same way everywhere it
    // would agree with nothing and pass P1 by saying no to all of it.
    let mut said_true = 0u32;
    let mut said_false = 0u32;
    for range in Range::SMALL {
        let (lo, hi) = range.bounds();
        for &mode in &MODES {
            for &policy in &POLICIES {
                for r in RESTRICTIONS {
                    if adaptation_relocates(mode, policy, lo, hi, reach_of(range, r)) {
                        said_true += 1;
                    } else {
                        said_false += 1;
                    }
                }
            }
        }
    }
    verdict(
        "P6",
        &format!("the predicate answers both ways: {said_true} yes and {said_false} no"),
        said_true > 0 && said_false > 0,
    );

    // P7. The conservative default. A consumer that declares nothing gets a no
    // everywhere the law is not unconditional, which is what makes `EVERYTHING`
    // safe to be the default rather than convenient.
    let mut default_yes = Vec::new();
    for range in Range::SMALL {
        let (lo, hi) = range.bounds();
        for &mode in &MODES {
            for &policy in &POLICIES {
                if adaptation_relocates(mode, policy, lo, hi, Reach::EVERYTHING) {
                    default_yes.push(format!("{range:?}/{mode:?}/{policy:?}"));
                }
            }
        }
    }
    println!("P7 detail: the default licenses {:?}", default_yes);
    verdict(
        "P7",
        "the conservative default licenses only cells where both regions commute unconditionally",
        default_yes.iter().all(|s| {
            s.contains("Wrap")
                && (s.contains("Floor") || s.contains("Ceil") || s.contains("Stochastic"))
        }),
    );
}
