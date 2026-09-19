//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What a mode reads, derived by walking the map rather than read off the
//! declaration.
//!
//! `behaviour_of` says three things about each of the six names, and the whole of
//! this file is deriving those three by running `adapt` over domains that differ
//! in one restriction at a time and asserting the shipped answer equals what came
//! back. Without this the two region predicates would rest on a table, which is
//! the shape this crate deleted four predicates for.
//!
//! The derivations return rather than assert, so a wrong classification can be
//! named in a test and shown to be rejected. An arm that only ever compared the
//! shipped answer to itself would pass whatever either said.
//!
//! The six shipped names fill three rows of the classification. The fourth row,
//! a rule reading the sign only at a tie, is filled by the planted ties-away rule,
//! which is what shows the instrument can tell four rows apart when no shipped
//! mode sits in one of them.

use super::{ALL_MODES, Rule, band, dither, is_tie, residues, rounded, shipped_rules, the_dither};
use crate::apply::Dither;
use crate::rounding::Mode;
use crate::symmetry::{Reads, When, behaviour_of};

/// Whether the rounding region commutes with translation over a domain.
///
/// The domain is a band of slots and a set of residues. Translating a position
/// moves its slot and leaves its residue alone, so the region commutes over the
/// band exactly when the offset it adds is the same at every slot in it, for
/// every residue.
fn commutes_over(rule: Rule, negatives: bool, ties: bool, step: i128, d: Dither) -> bool {
    let low = if negatives { -band() } else { 0 };
    for (num, den) in residues() {
        if !ties && is_tie(num, den) {
            continue;
        }
        let mut first = 0i128;
        let mut seen = false;
        let mut slot = low;
        while slot <= band() {
            let offset = rounded(rule, slot, num, den, d) - slot;
            if seen {
                if first != offset {
                    return false;
                }
            } else {
                first = offset;
                seen = true;
            }
            slot += step;
        }
    }
    true
}

/// Whether the rounding region commutes with reflection through zero.
///
/// Negating a position is negating its numerator: `Exact::between` carries the
/// negative numerator into the slot, so `slot + num/den` becomes `-slot - 1` at
/// `(den - num)/den` off the grid and `-slot` on it.
fn reflects(rule: Rule) -> bool {
    for (num, den) in residues() {
        for slot in -band() ..= band() {
            let here = rounded(rule, slot, num, den, the_dither());
            let there = rounded(rule, -slot, -num, den, the_dither());
            if there != -here {
                return false;
            }
        }
    }
    true
}

/// What the map says a rule reads besides the residue.
///
/// Returns rather than asserts, so the control below can name a wrong answer and
/// show it rejected.
fn derived_reads(rule: Rule) -> Reads {
    if commutes_over(rule, true, true, 1, the_dither()) {
        return Reads::Nothing;
    }
    let sign_defeats_it = commutes_over(rule, false, true, 1, the_dither());
    let parity_defeats_it = commutes_over(rule, true, true, 2, the_dither());
    assert!(
        sign_defeats_it != parity_defeats_it,
        "{rule:?} is defeated by both restrictions or by neither, so the classification cannot \
         be derived: sign {sign_defeats_it}, parity {parity_defeats_it}"
    );
    if sign_defeats_it { Reads::Sign } else { Reads::Parity }
}

/// What the map says about when a rule reads it.
fn derived_when(rule: Rule) -> When {
    if commutes_over(rule, true, true, 1, the_dither()) {
        When::Never
    } else if commutes_over(rule, true, false, 1, the_dither()) {
        When::AtATie
    } else {
        When::EveryOffGridPosition
    }
}

/// The row of the four domains a rule produces, as a bit index.
fn row(rule: Rule) -> u16 {
    (commutes_over(rule, true, true, 1, the_dither()) as u16)
        | ((commutes_over(rule, false, true, 1, the_dither()) as u16) << 1)
        | ((commutes_over(rule, true, false, 1, the_dither()) as u16) << 2)
        | ((commutes_over(rule, true, true, 2, the_dither()) as u16) << 3)
}

#[test]
fn the_classification_of_every_mode_agrees_with_what_the_map_reads() {
    for &mode in &ALL_MODES {
        let shipped = behaviour_of(mode);
        assert_eq!(
            shipped.reads(),
            derived_reads(Rule::Shipped(mode)),
            "{mode:?} is declared to read {:?} and the map disagrees",
            shipped.reads()
        );
        assert_eq!(
            shipped.when(),
            derived_when(Rule::Shipped(mode)),
            "{mode:?} is declared to read it {:?} and the map disagrees",
            shipped.when()
        );
    }
}

#[test]
fn half_up_reads_nothing_and_the_ties_away_rule_reads_the_sign_at_a_tie() {
    // The ruling's two readings side by side, derived rather than declared.
    // `half_up` is `floor(x + q/2)` and commutes with every translation; the
    // ties-away alias commutes with reflection and reads the sign at a tie.
    let half_up = Rule::Shipped(Mode::HalfUp);
    assert_eq!(derived_reads(half_up), Reads::Nothing);
    assert_eq!(derived_when(half_up), When::Never);
    assert!(!reflects(half_up));

    let away = Rule::TiesAwayFromZero;
    assert_eq!(derived_reads(away), Reads::Sign);
    assert_eq!(derived_when(away), When::AtATie);
    assert!(reflects(away));
}

#[test]
fn the_control_a_wrong_classification_would_be_caught() {
    // The arm above compares two answers, and it is worth something only if the
    // derived one can disagree. Five wrong classifications are named here and
    // each is rejected, so a shipped table saying any of them would fail rather
    // than being confirmed by an assertion comparing it to itself.
    assert_ne!(
        derived_reads(Rule::Shipped(Mode::HalfEven)),
        Reads::Sign,
        "the derivation cannot tell parity from sign"
    );
    assert_ne!(
        derived_reads(Rule::Shipped(Mode::TowardZero)),
        Reads::Nothing,
        "the derivation cannot tell a mode that reads the sign from one that reads nothing"
    );
    assert_ne!(
        derived_when(Rule::Shipped(Mode::HalfEven)),
        When::EveryOffGridPosition,
        "the derivation cannot tell a rule that fires only at a tie from one that always does"
    );
    assert_ne!(
        derived_when(Rule::Shipped(Mode::TowardZero)),
        When::AtATie,
        "the derivation cannot tell a rule that always fires from one that fires at a tie"
    );
    assert_ne!(
        derived_reads(Rule::Shipped(Mode::HalfUp)),
        Reads::Sign,
        "the derivation cannot tell the ruled half_up from the ties-away alias"
    );
}

#[test]
fn the_reflection_fact_of_every_mode_agrees_with_the_map() {
    for &mode in &ALL_MODES {
        assert_eq!(
            behaviour_of(mode).reflects().get(),
            reflects(Rule::Shipped(mode)),
            "{mode:?} is declared to reflect {} and the map disagrees",
            behaviour_of(mode).reflects().get()
        );
    }
    // The control: the reflection walk answers both ways, so agreeing with the
    // shipped fact is not agreeing with a constant.
    let reflecting = shipped_rules().filter(|&rule| reflects(rule)).count();
    assert!(
        reflecting > 0 && reflecting < ALL_MODES.len(),
        "the reflection walk answered the same way for every mode"
    );
}

#[test]
fn the_two_symmetries_partition_the_six_shipped_names() {
    // A measured fact about these six rather than a theorem about rounding: every
    // shipped mode commutes with exactly one of the two. `half_up` is the case
    // that makes it a measurement worth keeping, since it is a nearest rule that
    // reads nothing and so falls on the translation side, where the ties-away
    // alias would fall on the reflection side.
    let mut both = 0;
    let mut neither = 0;
    for rule in shipped_rules() {
        let translates = commutes_over(rule, true, true, 1, the_dither());
        let reflects_here = reflects(rule);
        if translates && reflects_here {
            both += 1;
        }
        if !translates && !reflects_here {
            neither += 1;
        }
    }
    assert_eq!(both, 0, "a mode commutes with both symmetries");
    assert_eq!(neither, 0, "a mode commutes with neither symmetry");
}

#[test]
fn the_control_the_two_symmetries_are_not_the_same_question() {
    // The partition above would also hold if one of the two were the negation of
    // the other by construction. It is not: each is measured separately, and the
    // six split four and two, `floor`, `ceil`, `half_up` and `stochastic`
    // translating and `toward_zero` and `half_even` reflecting.
    let translating: [bool; 6] =
        ALL_MODES.map(|mode| commutes_over(Rule::Shipped(mode), true, true, 1, the_dither()));
    let reflecting: [bool; 6] = ALL_MODES.map(|mode| reflects(Rule::Shipped(mode)));
    for (i, &mode) in ALL_MODES.iter().enumerate() {
        let translates = matches!(
            mode,
            Mode::Floor | Mode::Ceil | Mode::HalfUp | Mode::Stochastic
        );
        assert_eq!(translating[i], translates, "{mode:?} translation");
        assert_eq!(reflecting[i], !translates, "{mode:?} reflection");
    }
    assert_eq!(translating.iter().filter(|&&t| t).count(), 4);
    assert_eq!(reflecting.iter().filter(|&&r| r).count(), 2);
}

#[test]
fn the_rounded_slot_is_the_position_slot_or_the_one_above_it() {
    // The bound the reach's derivation of the excursion sides rests on. Without
    // it, deriving whether a rounded value can leave the range from the position
    // bounds would be unsound rather than conservative.
    let mut walked = 0u64;
    let mut stayed = 0u64;
    let mut climbed = 0u64;
    for rule in shipped_rules() {
        for (num, den) in residues() {
            for slot in -band() ..= band() {
                let got = rounded(rule, slot, num, den, the_dither());
                walked += 1;
                if got == slot {
                    stayed += 1;
                } else if got == slot + 1 {
                    climbed += 1;
                }
                assert!(
                    got == slot || got == slot + 1,
                    "{rule:?} sent {slot}+{num}/{den} to {got}, which is neither neighbour"
                );
            }
        }
    }
    assert!(walked > 0, "the walk ran nothing");
    assert!(
        stayed > 0 && climbed > 0,
        "the walk reached only one of the two neighbours, so the bound is untested on the other"
    );
}

#[test]
fn no_verdict_here_moves_with_the_dither() {
    // Five modes ignore the dither and the sixth does not, which the applied
    // map's own suite measures. What matters here is different: at any one fixed
    // dither the stochastic mode is a function of the position, so the question
    // this file asks has an answer, and the answer must not depend on which
    // dither was fixed.
    for rule in shipped_rules() {
        let first = commutes_over(rule, true, true, 1, the_dither());
        for d in [dither(0, 1), dither(1, 8), dither(7, 8), dither(999, 1000)] {
            assert_eq!(
                commutes_over(rule, true, true, 1, d),
                first,
                "{rule:?} changed its verdict at a different dither"
            );
        }
    }
}

#[test]
fn the_control_the_derivation_can_tell_the_four_domains_apart() {
    // If the four domains gave the same answer for every rule the derivation
    // would be reading one bit and reporting three. The six shipped names fill
    // three rows, reading nothing, the sign everywhere off the grid, and the
    // parity at a tie; the planted ties-away rule fills the fourth, the sign at
    // a tie. Four distinct rows is what the classification has cases for, so
    // anything less means the instrument is coarser than the table it checks.
    let mut shipped = 0u16;
    for rule in shipped_rules() {
        shipped |= 1u16 << row(rule);
    }
    assert_eq!(
        shipped.count_ones(),
        3,
        "the six shipped names produce {} distinct rows",
        shipped.count_ones()
    );
    let away = 1u16 << row(Rule::TiesAwayFromZero);
    assert_eq!(
        shipped & away,
        0,
        "the ties-away rule shares a row with a shipped mode"
    );
    assert_eq!((shipped | away).count_ones(), 4);
}

#[test]
fn the_control_a_slot_outside_the_wide_range_would_be_caught() {
    // The isolation `rounded` asserts is only worth something if the assertion
    // can fire. Wrapping and saturating do disagree outside a range, measured on
    // the narrow one so the disagreement is reachable.
    use super::{Which, adapt_at, bounds, position};
    use crate::overflow::Policy;
    use crate::slots::Slot;
    let d = the_dither();
    let (_, hi) = bounds(Which::Signed);
    let outside = position(hi.index() + 3, 0, 1);
    assert_ne!(
        adapt_at(Which::Signed, Mode::Floor, Policy::Wrap, outside, d),
        adapt_at(Which::Signed, Mode::Floor, Policy::Saturate, outside, d),
        "the two policies agree outside the range, so the isolation check is vacuous"
    );
    // And they agree inside it, which is the other half of what makes the check
    // an isolation rather than a coincidence.
    let inside = position(Slot::ZERO.index(), 1, 4);
    assert_eq!(
        adapt_at(Which::Signed, Mode::Floor, Policy::Wrap, inside, d),
        adapt_at(Which::Signed, Mode::Floor, Policy::Saturate, inside, d)
    );
}
