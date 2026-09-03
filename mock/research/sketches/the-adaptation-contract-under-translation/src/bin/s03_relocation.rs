//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Step 03. The law over the whole map, and whether the two region properties
//! decide it.
//!
//! The law is relocation: adapting a translated position gives the same slot as
//! adapting the position and then translating.
//!
//! ```text
//! adapt(position + c) == adapt(adapt(position) + c)
//! ```
//!
//! for every position and every representable translation `c`. It needs no
//! operation, because it quantifies over every exact position rather than over
//! the ones some operation reaches. A multiply-add is the case where the
//! position is a product and `c` is the addend, so a fusion verdict is this law
//! read at the positions that operation reaches, and nothing else.
//!
//! The claim under test is that the law holds over a cell exactly when the
//! rounding region commutes with translation over the positions the cell reaches
//! and the completion region commutes over the values it reaches. All three are
//! measured here on the same reached sets, and the cell is reported broken if
//! they disagree.

use std::collections::BTreeSet;

use arvo_format::overflow::Policy;
use arvo_format::rounding::Mode;
use the_adaptation_contract_under_translation::{
    MODES,
    POLICIES,
    Range,
    adapt_at,
    complete_only,
    dither,
    is_tie,
    position,
    residues,
    round_only,
    verdict,
};

/// Which part of the cross a cell restricts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Restriction {
    /// Whether a position below zero is in the cell.
    negative_positions:    bool,
    /// Whether a translation below zero is in the cell.
    negative_translations: bool,
    /// Whether an exactly-half residue is in the cell.
    ties:                  bool,
}

impl Restriction {
    fn name(self) -> String {
        format!(
            "neg_pos={} neg_c={} ties={}",
            self.negative_positions, self.negative_translations, self.ties
        )
    }
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

/// The dither every arm here uses. Fixed, because relocation is a question about
/// a function and the stochastic mode is one only once its decision is pinned.
const DITHER: (i64, i64) = (1, 2);

struct Cell {
    /// Whether the relocation law held at every triple.
    law:         bool,
    /// The first triple it failed at.
    law_witness: Option<(i64, i64, i64, i64, i64, i64)>,
    /// Whether the rounding region commuted over the positions reached.
    equivariant: bool,
    /// Whether the completion region commuted over the values reached.
    homomorphic: bool,
    triples:     u64,
    /// How many of the values reaching the completion sat outside the range,
    /// which is what makes the homomorphy question non-vacuous.
    excursions:  u64,
}

fn cell(range: Range, mode: Mode, policy: Policy, r: Restriction) -> Cell {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let d = dither(DITHER.0, DITHER.1);

    let position_lo = if r.negative_positions { lo - span } else { 0.max(lo - span) };
    let translation_lo = if r.negative_translations { lo } else { 0.max(lo) };

    let mut out = Cell {
        law:         true,
        law_witness: None,
        equivariant: true,
        homomorphic: true,
        triples:     0,
        excursions:  0,
    };

    // Everything the cell touches, gathered while the law is walked so the two
    // region questions are asked about exactly what the law saw.
    let mut positions_reached: BTreeSet<i64> = BTreeSet::new();
    let mut values_reached: BTreeSet<i64> = BTreeSet::new();
    let mut translations: BTreeSet<i64> = BTreeSet::new();

    for (num, den) in residues() {
        if !r.ties && is_tie(num, den) {
            continue;
        }
        for n in position_lo ..= (hi + span) {
            let here = adapt_at(range, mode, policy, position(n, num, den), d);
            // The value that reaches the completion on the staged side.
            let (rounded, _) = round_only(Range::WideSigned, mode, position(n, num, den), d);
            positions_reached.insert(n);
            values_reached.insert(rounded);
            if rounded < lo || rounded > hi {
                out.excursions += 1;
            }
            for c in translation_lo ..= hi {
                translations.insert(c);
                positions_reached.insert(n + c);
                out.triples += 1;
                let direct = adapt_at(range, mode, policy, position(n + c, num, den), d);
                let staged = adapt_at(range, mode, policy, position(here + c, 0, 1), d);
                if direct != staged {
                    out.law = false;
                    if out.law_witness.is_none() {
                        out.law_witness = Some((n, num, den, c, direct, staged));
                    }
                }
            }
        }
    }

    // The rounding region, over exactly the positions the law walked.
    for (num, den) in residues() {
        if !r.ties && is_tie(num, den) {
            continue;
        }
        let mut offsets: BTreeSet<i64> = BTreeSet::new();
        for &n in &positions_reached {
            let (got, isolated) = round_only(Range::WideSigned, mode, position(n, num, den), d);
            assert!(
                isolated,
                "the wide range did not isolate the rounding region"
            );
            offsets.insert(got - n);
        }
        if offsets.len() > 1 {
            out.equivariant = false;
        }
    }

    // The completion region, over exactly the values and translations it saw.
    for &y in &values_reached {
        for &c in &translations {
            let direct = complete_only(range, policy, y + c);
            let staged = complete_only(range, policy, complete_only(range, policy, y) + c);
            if direct != staged {
                out.homomorphic = false;
            }
        }
    }

    out
}

fn main() {
    println!("step 03: relocation over the whole map, against the two region properties");
    println!("law: adapt(position + c) == adapt(adapt(position) + c)");
    println!();

    let mut cells = 0u32;
    let mut agreements = 0u32;
    let mut disagreements = Vec::new();
    let mut law_true = 0u32;
    let mut law_false = 0u32;
    let mut e_true = 0u32;
    let mut e_false = 0u32;
    let mut h_true = 0u32;
    let mut h_false = 0u32;
    let mut excursions_total = 0u64;
    let mut triples_total = 0u64;

    for range in Range::SMALL {
        for &mode in &MODES {
            for &policy in &POLICIES {
                for r in RESTRICTIONS {
                    let c = cell(range, mode, policy, r);
                    cells += 1;
                    triples_total += c.triples;
                    excursions_total += c.excursions;
                    if c.law {
                        law_true += 1;
                    } else {
                        law_false += 1;
                    }
                    if c.equivariant {
                        e_true += 1;
                    } else {
                        e_false += 1;
                    }
                    if c.homomorphic {
                        h_true += 1;
                    } else {
                        h_false += 1;
                    }
                    if c.law == (c.equivariant && c.homomorphic) {
                        agreements += 1;
                    } else {
                        disagreements.push(format!(
                            "range={range:?} mode={mode:?} policy={policy:?} {} law={} E={} H={} witness={:?}",
                            r.name(),
                            c.law,
                            c.equivariant,
                            c.homomorphic,
                            c.law_witness
                        ));
                    }
                    // The full cross is what the report carries, so a reader can
                    // find the cell they care about rather than a summary of it.
                    println!(
                        "cell range={range:?} mode={mode:?} policy={policy:?} {} law={} E={} H={} triples={} excursions={} witness={}",
                        r.name(),
                        c.law,
                        c.equivariant,
                        c.homomorphic,
                        c.triples,
                        c.excursions,
                        match c.law_witness {
                            None => "none".to_string(),
                            Some((n, num, den, cc, direct, staged)) =>
                                format!(
                                    "position {n}+{num}/{den} translated by {cc}: direct={direct} staged={staged}"
                                ),
                        }
                    );
                }
            }
        }
    }

    println!();
    println!("--- the controls ---");
    println!(
        "cells={cells} triples={triples_total} excursions={excursions_total} \
         law true/false={law_true}/{law_false} E true/false={e_true}/{e_false} \
         H true/false={h_true}/{h_false}"
    );

    verdict(
        "R1",
        &format!(
            "the law holds in a cell exactly when both region properties do: {agreements} of \
             {cells} cells agree"
        ),
        agreements == cells,
    );
    for d in &disagreements {
        println!("R1 disagreement: {d}");
    }
    verdict(
        "R2",
        "the cross reaches cells where the law holds and cells where it fails",
        law_true > 0 && law_false > 0,
    );
    verdict(
        "R3",
        "the cross reaches cells where each region property holds and where it fails",
        e_true > 0 && e_false > 0 && h_true > 0 && h_false > 0,
    );
    verdict(
        "R4",
        "the values reaching the completion left the range, so the homomorphy question was \
         not vacuous",
        excursions_total > 0,
    );

    // R5. Neither property alone decides the law. If either did, one of the two
    // would be doing no work and the pair would be a pile rather than a rule.
    let mut e_alone = 0u32;
    let mut h_alone = 0u32;
    for range in Range::SMALL {
        for &mode in &MODES {
            for &policy in &POLICIES {
                for r in RESTRICTIONS {
                    let c = cell(range, mode, policy, r);
                    if c.law == c.equivariant {
                        e_alone += 1;
                    }
                    if c.law == c.homomorphic {
                        h_alone += 1;
                    }
                }
            }
        }
    }
    verdict(
        "R5",
        &format!(
            "neither property alone decides the law: equivariance alone agrees in {e_alone} of \
             {cells} cells and homomorphy alone in {h_alone}"
        ),
        e_alone < cells && h_alone < cells,
    );

    // R6. The hand-worked case, so the law is reproducible from two calls rather
    // than from a sweep.
    //
    // `Integer<5>` admits -16 to 15, saturating, floor. Take the position 15 and
    // a half, and translate by -16. Directly: 15.5 - 16 = -0.5, which floors to
    // -1, in range. Staged: 15.5 floors to 15, in range, then 15 - 16 = -1, in
    // range. Those agree, because floor commutes and no excursion happened.
    let agree_direct = adapt_at(
        Range::SmallSigned,
        Mode::Floor,
        Policy::Saturate,
        position(-1, 1, 2),
        dither(1, 2),
    );
    let agree_staged = adapt_at(
        Range::SmallSigned,
        Mode::Floor,
        Policy::Saturate,
        position(15 - 16, 0, 1),
        dither(1, 2),
    );
    // And the case that breaks: the position 31, which is 16 above the top,
    // translated by -16. Directly: 15, in range. Staged: 31 saturates to 15,
    // then 15 - 16 = -1.
    let break_direct = adapt_at(
        Range::SmallSigned,
        Mode::Floor,
        Policy::Saturate,
        position(31 - 16, 0, 1),
        dither(1, 2),
    );
    let break_staged = adapt_at(
        Range::SmallSigned,
        Mode::Floor,
        Policy::Saturate,
        position(
            adapt_at(
                Range::SmallSigned,
                Mode::Floor,
                Policy::Saturate,
                position(31, 0, 1),
                dither(1, 2),
            ) - 16,
            0,
            1,
        ),
        dither(1, 2),
    );
    verdict(
        "R6",
        &format!(
            "the hand-worked pair: the in-range case agrees at {agree_direct} and {agree_staged}, \
             and the excursion breaks at direct={break_direct} staged={break_staged}"
        ),
        agree_direct == agree_staged && break_direct == 15 && break_staged == -1,
    );
}
