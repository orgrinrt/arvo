//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Which shifts and which negation each rule commutes with, over the rounding
//! region alone, and the position that breaks each one that does not.
//!
//! `the_classification` derives three facts per mode from four domains. This
//! file states the partition those facts come from, one law at a time: a shift
//! by one quantum, every odd shift, every even shift, every non-negative shift
//! over the non-negative side where nothing crosses zero, negation, and negation
//! away from a tie. Each rule's answer on each law is written down here from the
//! formula that defines the rule, not read from `behaviour_of`, and then walked.
//!
//! A failing cell is not left as a `false` in a table. Each one is asserted to
//! fail at a named position, and the same position is asserted to satisfy the
//! law under a rule that passes it, so the witness separates rather than failing
//! everything.
//!
//! The walk is the band `the_classification` uses, every slot from `-64` to
//! `64` at the residues of `residues()`, one of which is the tie, through the
//! wide range so no completion fires. The stochastic mode is walked at the
//! suite's fixed dither of one half, where it is nearest with a tie sent down.

use super::{Rule, band, is_tie, residues, rounded, the_dither};
use crate::rounding::{ALL_MODES, Mode};
use crate::symmetry::{
    Reach,
    rounding_is_reflection_equivariant,
    rounding_is_translation_equivariant,
};

/// Whether `r(x + c) = r(x) + c` at the position `slot + num/den`.
fn shift_holds_at(rule: Rule, slot: i128, num: i64, den: i64, c: i128) -> bool {
    let d = the_dither();
    rounded(rule, slot + c, num, den, d) == rounded(rule, slot, num, den, d) + c
}

/// Whether `r(-x) = -r(x)` at the position `slot + num/den`.
fn negation_holds_at(rule: Rule, slot: i128, num: i64, den: i64) -> bool {
    let d = the_dither();
    rounded(rule, -slot, -num, den, d) == -rounded(rule, slot, num, den, d)
}

/// A law over the band: its shifts, and which positions it is asked at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Law {
    /// A shift by one quantum, at every position.
    OneQuantum,
    /// Every odd shift from `-5` to `5`, at every position.
    OddShifts,
    /// Every even shift from `-4` to `4`, at every position.
    EvenShifts,
    /// Every shift from `1` to `5`, at a non-negative position, so neither the
    /// position nor its image is below zero.
    NonNegativeSide,
    /// Negation, at every position.
    Negation,
    /// Negation, at every position that is not a tie.
    NegationOffATie,
}

const LAWS: [Law; 6] = [
    Law::OneQuantum,
    Law::OddShifts,
    Law::EvenShifts,
    Law::NonNegativeSide,
    Law::Negation,
    Law::NegationOffATie,
];

/// Whether `rule` commutes with `law` over the whole band.
fn holds(rule: Rule, law: Law) -> bool {
    let shifts: &[i128] = match law {
        Law::OneQuantum => &[1],
        Law::OddShifts => &[-5, -3, -1, 1, 3, 5],
        Law::EvenShifts => &[-4, -2, 2, 4],
        Law::NonNegativeSide => &[1, 2, 3, 4, 5],
        Law::Negation | Law::NegationOffATie => &[],
    };
    let low = if law == Law::NonNegativeSide { 0 } else { -band() };
    for (num, den) in residues() {
        if law == Law::NegationOffATie && is_tie(num, den) {
            continue;
        }
        for slot in low ..= band() {
            let here = match law {
                Law::Negation | Law::NegationOffATie => negation_holds_at(rule, slot, num, den),
                _ => {
                    shifts
                        .iter()
                        .all(|&c| shift_holds_at(rule, slot, num, den, c))
                },
            };
            if !here {
                return false;
            }
        }
    }
    true
}

/// Each rule's answer on each law, from the formula that defines it.
///
/// `floor`, `ceil` and `half_up` are `floor(x + k)` for a constant `k`, which
/// commutes with every whole shift and with no negation; `half_up` is `k = 1/2`,
/// whose only asymmetry is where the tie goes, so it commutes with negation away
/// from a tie. `stochastic` at a dither of one half is `ceil(x - 1/2)`, the same
/// shape with the tie sent down. `toward_zero` is odd and reads the sign, so it
/// commutes with negation and with a shift that does not cross zero. `half_even`
/// is odd and reads the parity, so it commutes with negation and an even shift.
/// The ties-away rule is odd and reads the sign at a tie.
fn expected(rule: Rule, law: Law) -> bool {
    use Law::*;
    match rule {
        Rule::Shipped(Mode::Floor | Mode::Ceil) => {
            matches!(law, OneQuantum | OddShifts | EvenShifts | NonNegativeSide)
        },
        Rule::Shipped(Mode::HalfUp | Mode::Stochastic) => law != Negation,
        Rule::Shipped(Mode::TowardZero) | Rule::TiesAwayFromZero => {
            matches!(law, NonNegativeSide | Negation | NegationOffATie)
        },
        Rule::Shipped(Mode::HalfEven) => matches!(law, EvenShifts | Negation | NegationOffATie),
    }
}

/// Every rule the partition is stated over.
fn rules() -> [Rule; 7] {
    [
        Rule::Shipped(Mode::Floor),
        Rule::Shipped(Mode::Ceil),
        Rule::Shipped(Mode::HalfUp),
        Rule::Shipped(Mode::Stochastic),
        Rule::Shipped(Mode::TowardZero),
        Rule::Shipped(Mode::HalfEven),
        Rule::TiesAwayFromZero,
    ]
}

#[test]
fn every_rule_commutes_with_exactly_the_laws_its_formula_says() {
    for rule in rules() {
        for law in LAWS {
            assert_eq!(
                holds(rule, law),
                expected(rule, law),
                "{rule:?} under {law:?}"
            );
        }
    }
    // Every shipped mode is among the rules walked.
    for mode in ALL_MODES {
        assert!(
            rules().contains(&Rule::Shipped(mode)),
            "{mode:?} is not walked"
        );
    }
}

#[test]
fn the_control_every_law_separates_the_rules() {
    // A law every rule passes, or none does, would say nothing about any of
    // them, and the table above would still agree with it.
    for law in LAWS {
        let passing = rules().iter().filter(|&&rule| holds(rule, law)).count();
        assert!(
            passing > 0 && passing < rules().len(),
            "{law:?} answered the same for every rule"
        );
    }
}

#[test]
fn the_shipped_predicates_at_the_widest_reach_agree_with_the_measured_partition() {
    // At `Reach::EVERYTHING` a predicate can lean on no restriction of the
    // domain, so it has to say exactly what the unrestricted walk measures: a
    // shift by one quantum for the translation predicate, and negation at every
    // position for the reflection one.
    for mode in ALL_MODES {
        let rule = Rule::Shipped(mode);
        assert_eq!(
            rounding_is_translation_equivariant(mode, Reach::EVERYTHING).get(),
            holds(rule, Law::OneQuantum),
            "{mode:?} translation"
        );
        assert_eq!(
            rounding_is_reflection_equivariant(mode).get(),
            holds(rule, Law::Negation),
            "{mode:?} reflection"
        );
    }
}

/// A position a law fails at, and the rule it fails under.
struct Witness {
    rule:    Rule,
    slot:    i128,
    num:     i64,
    den:     i64,
    shift:   i128,
    passing: Rule,
}

/// Every failing cell's witness. A shift of zero names negation.
fn witnesses() -> [Witness; 8] {
    let tz = Rule::Shipped(Mode::TowardZero);
    let he = Rule::Shipped(Mode::HalfEven);
    let hu = Rule::Shipped(Mode::HalfUp);
    let tie = |rule: Rule, slot: i128, shift: i128, passing: Rule| {
        Witness {
            rule,
            slot,
            num: 1,
            den: 2,
            shift,
            passing,
        }
    };
    [
        // `-q/2` to `q/2` crosses zero: `toward_zero` sends both to zero.
        tie(tz, -1, 1, hu),
        // `q/2` to `3q/2`: the even neighbour is `0` for one and `2` for the other.
        tie(he, 0, 1, hu),
        // `-q/2` to `3q/2`, an even shift across zero.
        tie(tz, -1, 2, he),
        // The ties-away alias crosses zero at a tie the same way.
        tie(Rule::TiesAwayFromZero, -1, 1, hu),
        // Negation at `q/2`: each of these sends `q/2` and `-q/2` to the same side.
        tie(hu, 0, 0, tz),
        tie(Rule::Shipped(Mode::Floor), 0, 0, he),
        tie(Rule::Shipped(Mode::Ceil), 0, 0, he),
        tie(
            Rule::Shipped(Mode::Stochastic),
            0,
            0,
            Rule::TiesAwayFromZero,
        ),
    ]
}

#[test]
fn every_failing_cell_fails_at_a_named_position_another_rule_passes() {
    for w in witnesses() {
        let law_at = |rule: Rule| {
            if w.shift == 0 {
                negation_holds_at(rule, w.slot, w.num, w.den)
            } else {
                shift_holds_at(rule, w.slot, w.num, w.den, w.shift)
            }
        };
        assert!(!law_at(w.rule), "{:?} holds at its own witness", w.rule);
        assert!(
            law_at(w.passing),
            "{:?} fails at the witness for {:?}",
            w.passing,
            w.rule
        );
    }
}

#[test]
fn the_witnesses_answer_as_the_formulas_do() {
    // The numbers behind the two witnesses the ruling turns on, as values rather
    // than as a comparison, so a reader sees the cell.
    let d = the_dither();
    let hu = Rule::Shipped(Mode::HalfUp);
    // `half_up` sends `-q/2` to `0` and `q/2` to `q`: not odd, but a shift.
    assert_eq!(rounded(hu, -1, 1, 2, d), 0);
    assert_eq!(rounded(hu, 0, 1, 2, d), 1);
    // The ties-away alias sends `-q/2` to `-q` and `q/2` to `q`: odd, not a shift.
    assert_eq!(rounded(Rule::TiesAwayFromZero, -1, 1, 2, d), -1);
    assert_eq!(rounded(Rule::TiesAwayFromZero, 0, 1, 2, d), 1);
    // `toward_zero` sends both to `0`.
    assert_eq!(rounded(Rule::Shipped(Mode::TowardZero), -1, 1, 2, d), 0);
    assert_eq!(rounded(Rule::Shipped(Mode::TowardZero), 0, 1, 2, d), 0);
}
