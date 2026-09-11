//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What a reach is, and what a yes from a predicate handed one is a claim about.
//!
//! The cross next door measures the predicates against the map over the domains
//! a cell reaches. These arms are about the coordinate itself: that a reach
//! handed its ends backwards widens rather than inverting, that the conservative
//! reach really is the widest thing a caller can name, that a degenerate reach
//! licenses everything and says so, that the upper excursion is strict on the
//! grid and inclusive off it, and that the cross reaches the cases its verdicts
//! turn on.
//!
//! Two of them are here because the cross cannot reach them. `cell_reach` hands
//! every cell an upper translation bound of the range's highest slot, so the
//! completion region's low-side disjunct is never exercised true anywhere in the
//! 648 cells, and a defect confined to it would pass the whole sweep. And every
//! cell's positions run a whole span past the top, so the difference between a
//! strict and an inclusive upper excursion never decides a cell either.

use super::{
    ALL_MODES,
    CompletionRelocates,
    GridState,
    RANGES,
    RESTRICTIONS,
    Relocates,
    Which,
    at,
    bounds,
    measure_relocation,
    relocation_over,
};
use crate::overflow::{Policy, SHIPPED_POLICIES};
use crate::rounding::Mode;
use crate::slots::Slot;
use crate::symmetry::{Reach, rounding_is_translation_equivariant};

#[test]
fn the_control_the_cross_reaches_every_case_a_verdict_could_turn_on() {
    // The per-cell floor is derived rather than guessed, and the derivation was
    // wrong the first time, which is why it is written out. The smallest cell is
    // the symmetric range restricted to non-negative positions and non-negative
    // translations: eleven positions and four translations, which is 44 triples
    // at every residue the cell walks. Restricting the translations is what makes
    // a cell on a range reaching negatives smaller than one on a range that does
    // not, and that is the part the first floor missed.
    //
    // How many residues a cell walks is its grid state's, out of the seven
    // `residues` lists: one on the grid, the six that are not the midpoint off it,
    // and all seven with a tie. That count is asserted too, so the floor cannot be
    // met by walking residues the cell's grid state rules out.
    let mut cells = 0u32;
    let mut triples = 0u64;
    let mut excursions = 0u64;
    let mut ties = 0u64;
    let mut negatives = 0u64;
    let mut on_grid_cells = 0u32;
    for which in RANGES {
        for &mode in &ALL_MODES {
            for &policy in &SHIPPED_POLICIES {
                for r in RESTRICTIONS {
                    let cell = relocation_over(which, mode, policy, r);
                    let walked = match r.grid {
                        GridState::OnGrid => 1,
                        GridState::OffGrid => 6,
                        GridState::WithTies => 7,
                    };
                    assert_eq!(
                        cell.residues, walked,
                        "{which:?} {mode:?} {policy:?} {r:?} walked {} residues",
                        cell.residues
                    );
                    assert!(
                        cell.triples >= 44 * cell.residues,
                        "{which:?} {mode:?} {policy:?} {r:?} ran {} triples over {} residues, so \
                         the sweep shrank",
                        cell.triples,
                        cell.residues
                    );
                    assert!(
                        cell.excursions > 0,
                        "{which:?} {mode:?} {policy:?} {r:?} never left the range, so the \
                         completion region answered nothing in it"
                    );
                    if r.grid == GridState::OnGrid {
                        assert_eq!(
                            cell.ties, 0,
                            "an on-grid cell walked an exactly-half position"
                        );
                        on_grid_cells += 1;
                    }
                    cells += 1;
                    triples += cell.triples;
                    excursions += cell.excursions;
                    ties += cell.ties;
                    negatives += cell.negative_positions;
                }
            }
        }
    }
    assert_eq!(
        cells,
        (RANGES.len() * ALL_MODES.len() * SHIPPED_POLICIES.len() * RESTRICTIONS.len()) as u32
    );
    assert_eq!(
        cells, 648,
        "the cross is not the whole matrix of twelve restrictions"
    );
    assert_eq!(
        on_grid_cells,
        cells / 3,
        "a third of the restrictions are on the grid"
    );
    assert!(triples > 0, "the cross ran nothing");
    assert!(
        excursions > 0,
        "no position left the range: {excursions} of {triples}"
    );
    assert!(
        ties > 0,
        "no exactly-half position, so the tie rules are untested"
    );
    assert!(
        negatives > 0,
        "no negative position, so the sign the modes read never varies"
    );
}

#[test]
fn an_on_grid_reach_is_strict_at_the_top_and_the_map_agrees() {
    // The reach whose highest position is the highest slot, which no cell in the
    // cross reaches. On the grid the rounding region returns the position itself,
    // so nothing rounds above the top, and an inclusive upper excursion would read
    // an excursion that never happens and refuse a relocation the map honours.
    // Off the grid the position at the top rounds above it under `Ceil`, so the
    // same bounds really do reach past it there.
    let (lo, hi) = bounds(Which::Signed);
    let on = Reach::of(lo, hi).translated_by(lo, hi).on_grid();
    let off = Reach::of(lo, hi).translated_by(lo, hi).without_ties();

    // The coordinate first. On the grid nothing is off it and no tie is reached,
    // and taking the ties away from a reach with none leaves it where it was
    // rather than moving it off the grid it never left.
    assert!(!on.reaches_off_the_grid().get());
    assert!(!on.reaches_a_tie().get());
    assert_eq!(on.without_ties(), on);
    assert!(off.reaches_off_the_grid().get());
    assert!(!off.reaches_a_tie().get());
    assert_ne!(on, off);

    // The upper excursion is what the grid state moves. The lower one is not,
    // because rounding never goes down past the position's own slot.
    assert!(!on.reaches_above(hi).get());
    assert!(off.reaches_above(hi).get());
    assert!(!on.reaches_below(lo).get());
    assert!(!off.reaches_below(lo).get());

    // On the grid every mode is licensed under every policy, and the map honours
    // every one of them.
    for &mode in &ALL_MODES {
        for &policy in &SHIPPED_POLICIES {
            assert!(
                at(Which::Signed, mode, policy, &Relocates(on)).get(),
                "{mode:?} {policy:?}: an on-grid reach inside the range was refused"
            );
            assert!(
                measure_relocation(Which::Signed, mode, policy, on, GridState::OnGrid).law,
                "{mode:?} {policy:?}: the map broke relocation on the grid inside the range"
            );
        }
    }

    // Off the grid under a clamp the predicate refuses, and the map agrees under
    // `Ceil`: the top slot plus a quarter rounds to one past the top, saturates
    // back, and a negative translation then lands one lower than the direct sum.
    for policy in [Policy::Saturate, Policy::Clamp] {
        assert!(
            !at(Which::Signed, Mode::Ceil, policy, &Relocates(off)).get(),
            "{policy:?}: an off-grid reach at the top was licensed"
        );
        assert!(
            !measure_relocation(Which::Signed, Mode::Ceil, policy, off, GridState::OffGrid).law,
            "{policy:?}: the map honoured a relocation the rounding above the top breaks"
        );
    }

    // The control: under wrapping the same off-grid reach is licensed and holds,
    // so the refusal above is the clamp at the top rather than the reach.
    assert!(at(Which::Signed, Mode::Ceil, Policy::Wrap, &Relocates(off)).get());
    assert!(
        measure_relocation(
            Which::Signed,
            Mode::Ceil,
            Policy::Wrap,
            off,
            GridState::OffGrid
        )
        .law
    );
}

#[test]
fn a_translation_band_with_no_positive_translation_licenses_a_low_excursion() {
    // The other case the cross cannot reach. `completion_is_translation_homomorphic`
    // decides the low side by `reaches_below(lowest).not()` or
    // `reaches_a_positive_translation().not()`, and the second is false in every
    // cell of the sweep, because `cell_reach` always hands the range's highest
    // slot as the upper translation bound and that is positive on all three
    // ranges. Its mirror on the high side is exercised both ways, through the
    // `negative_translations` axis. So a defect confined to this disjunct passes
    // 648 cells and this arm is what would catch it.
    //
    // The positions stop one slot short of the top so the high side is decided by
    // `reaches_above` rather than by the translations, which is what leaves the
    // verdict resting on the low side alone.
    let (lo, hi) = bounds(Which::Signed);
    let span = hi.index() - lo.index() + 1;
    let positions_low = Slot::at(lo.index() - span);
    let positions_high = Slot::at(hi.index() - 1);

    let no_way_back =
        Reach::of(positions_low, positions_high).translated_by(Slot::at(lo.index()), Slot::ZERO);
    assert!(no_way_back.reaches_below(lo).get());
    assert!(!no_way_back.reaches_above(hi).get());
    assert!(!no_way_back.reaches_a_positive_translation().get());
    for &mode in &ALL_MODES {
        assert!(
            at(
                Which::Signed,
                mode,
                Policy::Saturate,
                &CompletionRelocates(no_way_back)
            )
            .get(),
            "{mode:?} refused a low excursion that no translation in the band can undo"
        );
    }

    // The control, and it is what says the yes above came from the disjunct
    // rather than from a reach that licenses everything: the same positions with
    // a positive translation in the band are refused.
    let a_way_back =
        Reach::of(positions_low, positions_high).translated_by(Slot::at(lo.index()), hi);
    assert!(a_way_back.reaches_a_positive_translation().get());
    for &mode in &ALL_MODES {
        assert!(
            !at(
                Which::Signed,
                mode,
                Policy::Saturate,
                &CompletionRelocates(a_way_back)
            )
            .get(),
            "{mode:?} licensed a low excursion a positive translation can undo"
        );
    }

    // And the second control: under wrapping the answer is yes whichever band it
    // is handed, so the refusal above is the clamp throwing the distance away
    // rather than anything about the reach.
    for &mode in &ALL_MODES {
        assert!(
            at(
                Which::Signed,
                mode,
                Policy::Wrap,
                &CompletionRelocates(a_way_back)
            )
            .get(),
            "{mode:?} refused a reach under a policy that is a homomorphism"
        );
    }
}

#[test]
fn the_control_a_degenerate_reach_licenses_everything_and_says_so() {
    // A reach of one position with no translation and no tie cannot exhibit a
    // difference between two positions, so every mode's rounding region commutes
    // over it. A yes from this predicate is a claim about the reach it was
    // handed, and this says so out loud rather than in a comment.
    let one = Reach::of(Slot::ZERO, Slot::ZERO).without_ties();
    for &mode in &ALL_MODES {
        assert!(
            rounding_is_translation_equivariant(mode, one).get(),
            "{mode:?} refused a reach with no tie and no negative position"
        );
    }
    // And the conservative reach refuses the three that read something, which is
    // the other end of the same instrument.
    let refused = ALL_MODES
        .iter()
        .filter(|&&mode| !rounding_is_translation_equivariant(mode, Reach::EVERYTHING).get())
        .count();
    assert_eq!(
        refused, 3,
        "the conservative reach should refuse exactly the modes that read something"
    );
}

#[test]
fn a_reach_handed_its_ends_backwards_widens_rather_than_inverting() {
    let forwards = Reach::of(Slot::at(-4), Slot::at(7));
    let backwards = Reach::of(Slot::at(7), Slot::at(-4));
    assert_eq!(forwards, backwards);
    assert_eq!(backwards.positions_low(), Slot::at(-4));
    assert_eq!(backwards.positions_high(), Slot::at(7));

    let translated = Reach::of(Slot::ZERO, Slot::ZERO).translated_by(Slot::at(3), Slot::at(-3));
    assert_eq!(translated.translations_low(), Slot::at(-3));
    assert_eq!(translated.translations_high(), Slot::at(3));

    // The control: ordering is not the identity on everything, so the assertion
    // above is about a pair that needed it.
    assert_ne!(
        Reach::of(Slot::at(-4), Slot::at(7)),
        Reach::of(Slot::ZERO, Slot::at(7))
    );
}

#[test]
fn the_conservative_reach_is_the_widest_thing_a_caller_can_declare() {
    let everything = Reach::EVERYTHING;
    assert!(everything.reaches_off_the_grid().get());
    assert!(everything.reaches_a_tie().get());
    assert!(everything.reaches_a_negative_position().get());
    assert!(everything.reaches_a_negative_translation().get());
    assert!(everything.reaches_a_positive_translation().get());
    assert!(everything.reaches_below(Slot::ZERO).get());
    assert!(everything.reaches_above(Slot::ZERO).get());

    // The saturating add is what keeps that true. Both bounds already sit at the
    // ends of the coordinate, so the union of the positions and the translated
    // positions cannot be reached by adding them.
    assert_eq!(everything.lowest_rounded_position(), Slot::at(i64::MIN));
    assert_eq!(everything.highest_rounded_position(), Slot::at(i64::MAX));
}

#[test]
fn a_negative_translation_carries_a_non_negative_position_below_zero() {
    // The defect the predicate had in its first revision, kept as an arm. A reach
    // whose positions start at zero still reaches a negative position once a
    // negative translation is in it, and the sign a mode reads varies there.
    let no_translation = Reach::of(Slot::ZERO, Slot::at(16));
    assert!(!no_translation.reaches_a_negative_position().get());
    assert!(rounding_is_translation_equivariant(Mode::TowardZero, no_translation).get());

    let translated = no_translation.translated_by(Slot::at(-16), Slot::at(16));
    assert!(translated.reaches_a_negative_position().get());
    assert!(!rounding_is_translation_equivariant(Mode::TowardZero, translated).get());

    // The control: a non-negative translation leaves the answer alone, so the
    // difference is the sign of the translation rather than its presence.
    let forwards = no_translation.translated_by(Slot::ZERO, Slot::at(16));
    assert!(!forwards.reaches_a_negative_position().get());
    assert!(rounding_is_translation_equivariant(Mode::TowardZero, forwards).get());

    // And the fourth disjunct, which no sign argument reaches: the same negative
    // translation over a reach on the grid is licensed, because the rounding
    // region is never entered and the sign it would read is never read.
    assert!(rounding_is_translation_equivariant(Mode::TowardZero, translated.on_grid()).get());
}
