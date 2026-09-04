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
//! The rounding region is isolated by a range wide enough that the band cannot
//! leave it. That is measured rather than assumed: wrapping and saturating differ
//! on every value outside a range and agree on every value inside one, so the two
//! policies agreeing at every position is what says no completion fired.

use super::{ALL_MODES, Which, adapt_at, bounds, dither, is_tie, position, residues, the_dither};
use crate::apply::Dither;
use crate::overflow::Policy;
use crate::rounding::Mode;
use crate::slots::Slot;
use crate::symmetry::{Reads, When, behaviour_of};

/// How far either way the band runs.
///
/// Small, dense and centred on zero, because the two things a mode can read
/// besides the residue are the sign of the slot and its parity, and both change
/// inside a band this size. A function rather than an item constant, for the
/// reason the ratio coordinate's suite gives: a const here is a coordinate
/// spelled in the host's own type and the contract lint refuses it in the one
/// crate otherwise allowed to name one.
fn band() -> i64 {
    64
}

/// The rounding region's answer at one position, with the completion measured not
/// to have fired.
fn rounded(mode: Mode, slot: i64, num: i64, den: i64, d: Dither) -> i64 {
    let wrapped = adapt_at(Which::Wide, mode, Policy::Wrap, position(slot, num, den), d);
    let saturated = adapt_at(
        Which::Wide,
        mode,
        Policy::Saturate,
        position(slot, num, den),
        d,
    );
    assert_eq!(
        wrapped, saturated,
        "the two policies disagreed at {slot}+{num}/{den}, so the position left the range and \
         the rounding region is not what was measured"
    );
    let (lo, hi) = bounds(Which::Wide);
    assert!(
        wrapped.index() > lo.index() && wrapped.index() < hi.index(),
        "the answer reached a bound of the wide range"
    );
    wrapped.index()
}

/// Whether the rounding region commutes with translation over a domain.
///
/// The domain is a band of slots and a set of residues. Translating a position
/// moves its slot and leaves its residue alone, so the region commutes over the
/// band exactly when the offset it adds is the same at every slot in it, for
/// every residue.
fn commutes_over(mode: Mode, negatives: bool, ties: bool, step: i64, d: Dither) -> bool {
    let low = if negatives { -band() } else { 0 };
    for (num, den) in residues() {
        if !ties && is_tie(num, den) {
            continue;
        }
        let mut first = 0i64;
        let mut seen = false;
        let mut slot = low;
        while slot <= band() {
            let offset = rounded(mode, slot, num, den, d) - slot;
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
fn reflects(mode: Mode) -> bool {
    for (num, den) in residues() {
        for slot in -band() ..= band() {
            let here = rounded(mode, slot, num, den, the_dither());
            let there = rounded(mode, -slot, -num, den, the_dither());
            if there != -here {
                return false;
            }
        }
    }
    true
}

/// What the map says a mode reads besides the residue.
///
/// Returns rather than asserts, so the control below can name a wrong answer and
/// show it rejected.
fn derived_reads(mode: Mode) -> Reads {
    if commutes_over(mode, true, true, 1, the_dither()) {
        return Reads::Nothing;
    }
    let sign_defeats_it = commutes_over(mode, false, true, 1, the_dither());
    let parity_defeats_it = commutes_over(mode, true, true, 2, the_dither());
    assert!(
        sign_defeats_it != parity_defeats_it,
        "{mode:?} is defeated by both restrictions or by neither, so the classification cannot \
         be derived: sign {sign_defeats_it}, parity {parity_defeats_it}"
    );
    if sign_defeats_it { Reads::Sign } else { Reads::Parity }
}

/// What the map says about when a mode reads it.
fn derived_when(mode: Mode) -> When {
    if commutes_over(mode, true, true, 1, the_dither()) {
        When::Never
    } else if commutes_over(mode, true, false, 1, the_dither()) {
        When::AtATie
    } else {
        When::EveryOffGridPosition
    }
}

#[test]
fn the_classification_of_every_mode_agrees_with_what_the_map_reads() {
    for &mode in &ALL_MODES {
        let shipped = behaviour_of(mode);
        assert_eq!(
            shipped.reads(),
            derived_reads(mode),
            "{mode:?} is declared to read {:?} and the map disagrees",
            shipped.reads()
        );
        assert_eq!(
            shipped.when(),
            derived_when(mode),
            "{mode:?} is declared to read it {:?} and the map disagrees",
            shipped.when()
        );
    }
}

#[test]
fn the_control_a_wrong_classification_would_be_caught() {
    // The arm above compares two answers, and it is worth something only if the
    // derived one can disagree. Four wrong classifications are named here and
    // each is rejected, so a shipped table saying any of them would fail rather
    // than being confirmed by an assertion comparing it to itself.
    assert_ne!(
        derived_reads(Mode::HalfEven),
        Reads::Sign,
        "the derivation cannot tell parity from sign"
    );
    assert_ne!(
        derived_reads(Mode::TowardZero),
        Reads::Nothing,
        "the derivation cannot tell a mode that reads the sign from one that reads nothing"
    );
    assert_ne!(
        derived_when(Mode::HalfUp),
        When::EveryOffGridPosition,
        "the derivation cannot tell a rule that fires only at a tie from one that always does"
    );
    assert_ne!(
        derived_when(Mode::TowardZero),
        When::AtATie,
        "the derivation cannot tell a rule that always fires from one that fires at a tie"
    );
}

#[test]
fn the_reflection_fact_of_every_mode_agrees_with_the_map() {
    for &mode in &ALL_MODES {
        assert_eq!(
            behaviour_of(mode).reflects().get(),
            reflects(mode),
            "{mode:?} is declared to reflect {} and the map disagrees",
            behaviour_of(mode).reflects().get()
        );
    }
    // The control: the reflection walk answers both ways, so agreeing with the
    // shipped fact is not agreeing with a constant.
    let reflecting = ALL_MODES.iter().filter(|&&mode| reflects(mode)).count();
    assert!(
        reflecting > 0 && reflecting < ALL_MODES.len(),
        "the reflection walk answered the same way for every mode"
    );
}

#[test]
fn the_two_symmetries_partition_the_six_shipped_names() {
    // A measured fact about these six rather than a theorem about rounding. A
    // nearest rule whose tie went toward positive infinity would read nothing
    // beyond the residue and still commute with reflection away from a tie, so
    // the day a seventh name lands this is the arm that reports it.
    let mut both = 0;
    let mut neither = 0;
    for &mode in &ALL_MODES {
        let translates = commutes_over(mode, true, true, 1, the_dither());
        let reflects_here = reflects(mode);
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
    // the other by construction. It is not: each is measured separately and each
    // splits the six three and three, so the partition is a fact rather than a
    // restatement.
    let translating = ALL_MODES
        .iter()
        .filter(|&&mode| commutes_over(mode, true, true, 1, the_dither()))
        .count();
    let reflecting = ALL_MODES.iter().filter(|&&mode| reflects(mode)).count();
    assert_eq!(translating, 3);
    assert_eq!(reflecting, 3);
}

#[test]
fn the_rounded_slot_is_the_position_slot_or_the_one_above_it() {
    // The bound the reach's derivation of the excursion sides rests on. Without
    // it, deriving whether a rounded value can leave the range from the position
    // bounds would be unsound rather than conservative.
    let mut walked = 0u64;
    let mut stayed = 0u64;
    let mut climbed = 0u64;
    for &mode in &ALL_MODES {
        for (num, den) in residues() {
            for slot in -band() ..= band() {
                let got = rounded(mode, slot, num, den, the_dither());
                walked += 1;
                if got == slot {
                    stayed += 1;
                } else if got == slot + 1 {
                    climbed += 1;
                }
                assert!(
                    got == slot || got == slot + 1,
                    "{mode:?} sent {slot}+{num}/{den} to {got}, which is neither neighbour"
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
    for &mode in &ALL_MODES {
        let first = commutes_over(mode, true, true, 1, the_dither());
        for d in [dither(0, 1), dither(1, 8), dither(7, 8), dither(999, 1000)] {
            assert_eq!(
                commutes_over(mode, true, true, 1, d),
                first,
                "{mode:?} changed its verdict at a different dither"
            );
        }
    }
}

#[test]
fn the_control_the_derivation_can_tell_the_four_domains_apart() {
    // If the four domains gave the same answer for every mode the derivation
    // would be reading one bit and reporting three. Four distinct rows is what
    // the classification has cases for, so anything less means the instrument is
    // coarser than the table it checks.
    let mut seen = 0u16;
    for &mode in &ALL_MODES {
        let index = (commutes_over(mode, true, true, 1, the_dither()) as u16)
            | ((commutes_over(mode, false, true, 1, the_dither()) as u16) << 1)
            | ((commutes_over(mode, true, false, 1, the_dither()) as u16) << 2)
            | ((commutes_over(mode, true, true, 2, the_dither()) as u16) << 3);
        seen |= 1u16 << index;
    }
    assert_eq!(
        seen.count_ones(),
        4,
        "the four domains produce {} distinct rows and the classification has four cases",
        seen.count_ones()
    );
}

#[test]
fn the_control_a_slot_outside_the_wide_range_would_be_caught() {
    // The isolation `rounded` asserts is only worth something if the assertion
    // can fire. Wrapping and saturating do disagree outside a range, measured on
    // the narrow one so the disagreement is reachable.
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
