//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Step 04. The other symmetry of the grid, which nobody in this corpus has
//! asked about.
//!
//! Translation is one symmetry of a grid and reflection through zero is the
//! other. The relocation law asks whether the map commutes with the first. This
//! step asks the second:
//!
//! ```text
//! adapt(-position) == adapt(-adapt(position))
//! ```
//!
//! stated with the right side re-adapting for the same reason relocation is: the
//! negation of an admitted slot need not itself be admitted, since a two's
//! complement range has one more slot below zero than above it.
//!
//! The property is worth a step of its own because it is what licenses computing
//! on a magnitude and putting the sign back, which is a real lowering and not a
//! curiosity. `mock/registry/*.toml` carries no row about it: a search for
//! `equivarian` finds seven rows in `law-the-later-topics.toml` and a search for
//! negation, reflection, odd symmetry or a sign fixup over the same files finds
//! one hit and it is about a conversion door.
//!
//! Negation of a position is written through the coordinate's own normalisation
//! rather than by hand: `Exact::between` takes a numerator outside `[0, den)` and
//! carries it into the slot, so negating the numerator is the whole of it.

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
    position,
    residues,
    round_only,
    verdict,
};

const BAND: i64 = 64;
const DITHER: (i64, i64) = (1, 2);

/// The negation of a position, through the coordinate rather than by hand.
///
/// `slot + num/den` negated is `-slot - num/den`, and `Exact::between` carries
/// the negative numerator into the slot, so the result is `-slot - 1` at
/// `(den - num)/den` off grid and `-slot` on it.
fn negated(slot: i64, num: i64, den: i64) -> arvo_format::apply::Exact {
    position(-slot, -num, den)
}

/// Whether the rounding region commutes with reflection, over a band.
///
/// The walk runs to the end whatever it finds, so a count beside a verdict is a
/// count over the whole band rather than a count up to the first failure.
fn rounding_reflects(mode: Mode) -> (bool, Option<(i64, i64, i64, i64, i64)>) {
    let d = dither(DITHER.0, DITHER.1);
    let mut held = true;
    let mut witness = None;
    for (num, den) in residues() {
        for slot in -BAND ..= BAND {
            let (here, iso_a) = round_only(Range::WideSigned, mode, position(slot, num, den), d);
            let (there, iso_b) = round_only(Range::WideSigned, mode, negated(slot, num, den), d);
            assert!(
                iso_a && iso_b,
                "the wide range did not isolate the rounding region"
            );
            if there != -here {
                held = false;
                if witness.is_none() {
                    witness = Some((slot, num, den, here, there));
                }
            }
        }
    }
    (held, witness)
}

/// Whether the completion region commutes with reflection, over a band that
/// leaves the range on both sides.
fn completion_reflects(range: Range, policy: Policy) -> (bool, Option<(i64, i64, i64)>, u64) {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let mut excursions = 0u64;
    let mut held = true;
    let mut witness = None;
    for y in (lo - 2 * span) ..= (hi + 2 * span) {
        if y < lo || y > hi {
            excursions += 1;
        }
        let direct = complete_only(range, policy, -y);
        let staged = complete_only(range, policy, -complete_only(range, policy, y));
        if direct != staged {
            held = false;
            if witness.is_none() {
                witness = Some((y, direct, staged));
            }
        }
    }
    (held, witness, excursions)
}

/// Whether the whole map commutes with reflection, over positions that leave the
/// range on both sides.
fn map_reflects(range: Range, mode: Mode, policy: Policy) -> (bool, u64) {
    let (lo, hi) = range.bounds();
    let span = hi - lo + 1;
    let d = dither(DITHER.0, DITHER.1);
    let mut triples = 0u64;
    let mut held = true;
    for (num, den) in residues() {
        for slot in (lo - span) ..= (hi + span) {
            triples += 1;
            let here = adapt_at(range, mode, policy, position(slot, num, den), d);
            let direct = adapt_at(range, mode, policy, negated(slot, num, den), d);
            let staged = adapt_at(range, mode, policy, position(-here, 0, 1), d);
            if direct != staged {
                held = false;
            }
        }
    }
    (held, triples)
}

fn main() {
    println!("step 04: does the map commute with reflection through zero");
    println!("law: adapt(-position) == adapt(-adapt(position))");
    println!();

    println!("--- the rounding region ---");
    let mut reflecting_modes = BTreeSet::new();
    for &mode in &MODES {
        let (held, witness) = rounding_reflects(mode);
        if held {
            reflecting_modes.insert(format!("{mode:?}"));
        }
        println!("mode={mode:?} reflects={held} witness={}", match witness {
            None => "none".to_string(),
            Some((slot, num, den, here, there)) =>
                format!(
                    "position {slot}+{num}/{den} rounds to {here} and its negation to {there}, \
                     which is not {}",
                    -here
                ),
        });
    }

    println!();
    println!("--- the completion region ---");
    for range in Range::SMALL {
        for &policy in &POLICIES {
            let (held, witness, excursions) = completion_reflects(range, policy);
            println!(
                "range={range:?} symmetric={} policy={policy:?} reflects={held} excursions={excursions} witness={}",
                range.is_symmetric(),
                match witness {
                    None => "none".to_string(),
                    Some((y, direct, staged)) => format!("y={y}: direct={direct} staged={staged}"),
                }
            );
        }
    }

    println!();
    println!("--- the whole map ---");
    let mut map_cells = 0u32;
    let mut map_agreements = 0u32;
    let mut triples_total = 0u64;
    for range in Range::SMALL {
        for &mode in &MODES {
            for &policy in &POLICIES {
                let (held, triples) = map_reflects(range, mode, policy);
                triples_total += triples;
                let (r_holds, _) = rounding_reflects(mode);
                let (c_holds, ..) = completion_reflects(range, policy);
                map_cells += 1;
                if held == (r_holds && c_holds) {
                    map_agreements += 1;
                }
                println!(
                    "cell range={range:?} mode={mode:?} policy={policy:?} map={held} \
                     rounding={r_holds} completion={c_holds} triples={triples}"
                );
            }
        }
    }

    println!();
    println!("--- the controls ---");

    verdict(
        "N1",
        &format!(
            "the whole map reflects exactly when both regions do: {map_agreements} of \
             {map_cells} cells agree, over {triples_total} positions"
        ),
        map_agreements == map_cells,
    );
    verdict(
        "N2",
        &format!(
            "the instrument separates: {} of six modes reflect and the rest do not",
            reflecting_modes.len()
        ),
        !reflecting_modes.is_empty() && reflecting_modes.len() < MODES.len(),
    );

    // N3. The complementarity, which is the finding. On a domain reaching
    // negatives, translation equivariance and reflection equivariance partition
    // the six shipped modes: every mode has exactly one of them.
    let mut both = Vec::new();
    let mut neither = Vec::new();
    for &mode in &MODES {
        let (reflects, _) = rounding_reflects(mode);
        let translates = translation_holds_on_the_full_domain(mode);
        if reflects && translates {
            both.push(format!("{mode:?}"));
        }
        if !reflects && !translates {
            neither.push(format!("{mode:?}"));
        }
        println!("partition mode={mode:?} translates={translates} reflects={reflects}");
    }
    verdict(
        "N3",
        &format!(
            "over the six shipped modes on a domain reaching negatives, the two symmetries \
             partition: {} have both and {} have neither",
            both.len(),
            neither.len()
        ),
        both.is_empty() && neither.is_empty(),
    );

    // N4. The hand-worked witness for a mode that does not reflect, so the
    // negative half of N3 rests on two calls rather than on a loop.
    //
    // `Floor` at one and a half rounds to 1. Its negation is minus one and a
    // half, which floors to -2, and -1 is not -2.
    let d = dither(DITHER.0, DITHER.1);
    let (down, _) = round_only(Range::WideSigned, Mode::Floor, position(1, 1, 2), d);
    let (up, _) = round_only(Range::WideSigned, Mode::Floor, negated(1, 1, 2), d);
    verdict(
        "N4",
        &format!(
            "floor sends one and a half to {down} and its negation to {up}, and {up} is not {}",
            -down
        ),
        down == 1 && up == -2,
    );

    // N5. And one that does, at the same position, so the two are separated by
    // the mode rather than by the position.
    let (t_down, _) = round_only(Range::WideSigned, Mode::TowardZero, position(1, 1, 2), d);
    let (t_up, _) = round_only(Range::WideSigned, Mode::TowardZero, negated(1, 1, 2), d);
    verdict(
        "N5",
        &format!(
            "toward zero sends one and a half to {t_down} and its negation to {t_up} at the \
             same position"
        ),
        t_down == 1 && t_up == -1,
    );

    // N6. The symmetric range is what the completion needs and the two's
    // complement one is what it does not have. Asserted as the separating pair
    // rather than read off the table.
    let (sym, ..) = completion_reflects(Range::SmallSymmetric, Policy::Saturate);
    let (asym, ..) = completion_reflects(Range::SmallSigned, Policy::Saturate);
    verdict(
        "N6",
        "saturation reflects on a symmetric range and not on a two's complement one",
        sym && !asym,
    );

    // N7. Wrapping reflects on every range measured, symmetric or not, because
    // negation is an automorphism of the cyclic group the wrap reduces into. The
    // separating pair above is about saturation and this says so.
    let wrap_everywhere = Range::SMALL
        .iter()
        .all(|&range| completion_reflects(range, Policy::Wrap).0);
    verdict(
        "N7",
        "wrapping reflects on every range measured, asymmetric ones included",
        wrap_everywhere,
    );
}

/// Whether the rounding region commutes with translation on a domain reaching
/// negatives, measured here rather than carried from step 01, so this step
/// stands alone.
fn translation_holds_on_the_full_domain(mode: Mode) -> bool {
    let d = dither(DITHER.0, DITHER.1);
    for (num, den) in residues() {
        let mut offsets = BTreeSet::new();
        for slot in -BAND ..= BAND {
            let (got, iso) = round_only(Range::WideSigned, mode, position(slot, num, den), d);
            assert!(iso, "the wide range did not isolate the rounding region");
            offsets.insert(got - slot);
        }
        if offsets.len() > 1 {
            return false;
        }
    }
    true
}
