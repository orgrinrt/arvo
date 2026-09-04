//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The predicates against the map, cell by cell over the whole cross.
//!
//! Two disagreements are possible and they are not the same defect. A cell where
//! the predicate says a law holds and the map says it does not is unsound, and
//! one of those makes the predicate unshippable, because an arm gated on it
//! computes a wrong answer. A cell where the predicate refuses a law that holds
//! is conservative, and costs a lowering rather than a result. They are counted
//! separately here, because one number would hide which kind it came from.
//!
//! Both counts are asserted at zero. Conservative at zero is a stronger claim
//! than the predicate needs in order to be correct, and it is asserted anyway: a
//! conservative cell appearing later is a region the predicate stopped describing
//! exactly, which is worth knowing even where nothing is unsound.

use super::{
    ALL_MODES,
    CompletionReflects,
    CompletionRelocates,
    RANGES,
    RESTRICTIONS,
    Reflects,
    Relocates,
    Which,
    adapt_at,
    at,
    bounds,
    cell_reach,
    reflection_over,
    reflection_reach,
    relocation_over,
    the_dither,
};
use crate::apply::Exact;
use crate::overflow::{Policy, SHIPPED_POLICIES};
use crate::rounding::Mode;
use crate::slots::Slot;

#[test]
fn the_map_relocates_exactly_where_the_predicate_says() {
    let mut cells = 0u32;
    let mut unsound = 0u32;
    let mut conservative = 0u32;
    let mut licensed = 0u32;
    let mut refused = 0u32;
    for which in RANGES {
        for &mode in &ALL_MODES {
            for &policy in &SHIPPED_POLICIES {
                for r in RESTRICTIONS {
                    let said = at(which, mode, policy, &Relocates(cell_reach(which, r))).get();
                    let measured = relocation_over(which, mode, policy, r).law;
                    cells += 1;
                    if said {
                        licensed += 1;
                    } else {
                        refused += 1;
                    }
                    if said && !measured {
                        unsound += 1;
                    }
                    if !said && measured {
                        conservative += 1;
                    }
                    assert_eq!(
                        said, measured,
                        "{which:?} {mode:?} {policy:?} {r:?}: the predicate says {said} and the \
                         map says {measured}"
                    );
                }
            }
        }
    }
    assert_eq!(unsound, 0, "the predicate licensed a law the map refuses");
    assert_eq!(
        conservative, 0,
        "the predicate refused a law the map honours"
    );
    assert_eq!(
        cells,
        (RANGES.len() * ALL_MODES.len() * SHIPPED_POLICIES.len() * RESTRICTIONS.len()) as u32,
        "the cross visited {cells} cells, which is not the whole matrix"
    );
    assert!(
        licensed > 0 && refused > 0,
        "the predicate answered one way everywhere, so agreeing with the map establishes nothing: \
         {licensed} licensed and {refused} refused"
    );
}

#[test]
fn the_map_reflects_exactly_where_the_predicate_says() {
    let mut licensed = 0u32;
    let mut refused = 0u32;
    let mut walked = 0u64;
    for which in RANGES {
        for &mode in &ALL_MODES {
            for &policy in &SHIPPED_POLICIES {
                let said = at(which, mode, policy, &Reflects(reflection_reach(which))).get();
                let (measured, positions) = reflection_over(which, mode, policy);
                walked += positions;
                if said {
                    licensed += 1;
                } else {
                    refused += 1;
                }
                assert_eq!(
                    said, measured,
                    "{which:?} {mode:?} {policy:?}: the predicate says {said} and the map says \
                     {measured}"
                );
            }
        }
    }
    assert!(walked > 1_000, "the cross shrank to {walked} positions");
    assert!(
        licensed > 0 && refused > 0,
        "the predicate answered one way everywhere: {licensed} licensed and {refused} refused"
    );
}

#[test]
fn neither_region_alone_decides_either_law() {
    // If one of the two agreed with the law everywhere, the other would be doing
    // no work and the conjunction would be a pile rather than a rule. Measured
    // for the completion region against the law, over the same cross.
    let mut cells = 0u32;
    let mut completion_alone = 0u32;
    for which in RANGES {
        for &mode in &ALL_MODES {
            for &policy in &SHIPPED_POLICIES {
                for r in RESTRICTIONS {
                    let reach = cell_reach(which, r);
                    let completion = at(which, mode, policy, &CompletionRelocates(reach)).get();
                    let law = relocation_over(which, mode, policy, r).law;
                    cells += 1;
                    if completion == law {
                        completion_alone += 1;
                    }
                }
            }
        }
    }
    assert!(
        completion_alone < cells,
        "the completion region alone decides the relocation law in {completion_alone} of \
         {cells} cells, so the rounding half is not doing anything in this cross"
    );

    // And the same question on the reflection law, where the completion is the
    // half that varies with the range rather than with the mode.
    let mut reflection_cells = 0u32;
    let mut reflection_completion_alone = 0u32;
    for which in RANGES {
        for &mode in &ALL_MODES {
            for &policy in &SHIPPED_POLICIES {
                let completion = at(
                    which,
                    mode,
                    policy,
                    &CompletionReflects(reflection_reach(which)),
                )
                .get();
                let (law, _) = reflection_over(which, mode, policy);
                reflection_cells += 1;
                if completion == law {
                    reflection_completion_alone += 1;
                }
            }
        }
    }
    assert!(
        reflection_completion_alone < reflection_cells,
        "the completion region alone decides the reflection law in \
         {reflection_completion_alone} of {reflection_cells} cells"
    );
}

#[test]
fn a_clamp_on_a_two_s_complement_range_breaks_at_a_named_witness() {
    // Worked out on paper rather than derived by the expression under test.
    //
    // `Integer<3>` admits -4 to 3. Take the position 7, four above the top, and
    // translate it by -4, which is representable. Directly: 7 - 4 = 3, in range,
    // so 3. Staged: 7 saturates to 3, then 3 - 4 = -1, in range, so -1. The clamp
    // threw away the four the translation was about to give back.
    let d = the_dither();
    let direct = adapt_at(
        Which::Signed,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(3)),
        d,
    );
    let here = adapt_at(
        Which::Signed,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(7)),
        d,
    );
    let staged = adapt_at(
        Which::Signed,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(here.index() - 4)),
        d,
    );
    assert_eq!(direct, Slot::at(3));
    assert_eq!(here, Slot::at(3));
    assert_eq!(staged, Slot::at(-1));
    assert_ne!(direct, staged);

    // The control: the same pair under wrapping agrees, so the break is the
    // policy rather than the pair.
    let wrapped_direct = adapt_at(
        Which::Signed,
        Mode::Floor,
        Policy::Wrap,
        Exact::on_grid(Slot::at(3)),
        d,
    );
    let wrapped_here = adapt_at(
        Which::Signed,
        Mode::Floor,
        Policy::Wrap,
        Exact::on_grid(Slot::at(7)),
        d,
    );
    let wrapped_staged = adapt_at(
        Which::Signed,
        Mode::Floor,
        Policy::Wrap,
        Exact::on_grid(Slot::at(wrapped_here.index() - 4)),
        d,
    );
    assert_eq!(wrapped_direct, wrapped_staged);
}

#[test]
fn a_clamp_on_a_range_with_no_negative_slot_still_breaks_below_it() {
    // The case an argument from "every translation is non-negative" misses, and
    // it is why the region is about excursion sides crossed with translation
    // signs rather than about signedness.
    //
    // `UFixed<3, 0>` admits 0 to 7, so no translation can undo an excursion above
    // the top. One below the bottom can be undone: take the position -2 and
    // translate by 3. Directly: -2 + 3 = 1, in range, so 1. Staged: -2 clamps to
    // 0, then 0 + 3 = 3.
    let d = the_dither();
    let direct = adapt_at(
        Which::Unsigned,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(1)),
        d,
    );
    let here = adapt_at(
        Which::Unsigned,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(-2)),
        d,
    );
    let staged = adapt_at(
        Which::Unsigned,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(here.index() + 3)),
        d,
    );
    assert_eq!(direct, Slot::at(1));
    assert_eq!(here, Slot::ZERO);
    assert_eq!(staged, Slot::at(3));
    assert_ne!(direct, staged);

    // The control, which is the half the argument does cover: an excursion above
    // the top on this range cannot be undone, so the same schedule agrees there.
    let above_direct = adapt_at(
        Which::Unsigned,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(13)),
        d,
    );
    let above_here = adapt_at(
        Which::Unsigned,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(10)),
        d,
    );
    let above_staged = adapt_at(
        Which::Unsigned,
        Mode::Floor,
        Policy::Saturate,
        Exact::on_grid(Slot::at(above_here.index() + 3)),
        d,
    );
    assert_eq!(above_direct, above_staged);
}

#[test]
fn a_symmetric_range_is_what_a_clamp_needs_to_reflect() {
    // Worked out rather than swept. Take the position 4, one above the top of
    // both ranges. On `-4` to `3` the clamp gives 3, whose negation is -3, which
    // is in range; the negated position is -4, which is in range and stays -4. On
    // `-3` to `3` the clamp gives 3, negated to -3; the negated position is -4,
    // which clamps to -3. The asymmetric range has a slot with no positive twin
    // and the symmetric one does not.
    let d = the_dither();
    for (which, agrees) in [(Which::Signed, false), (Which::Symmetric, true)] {
        let here = adapt_at(
            which,
            Mode::TowardZero,
            Policy::Saturate,
            Exact::on_grid(Slot::at(4)),
            d,
        );
        let direct = adapt_at(
            which,
            Mode::TowardZero,
            Policy::Saturate,
            Exact::on_grid(Slot::at(-4)),
            d,
        );
        let staged = adapt_at(
            which,
            Mode::TowardZero,
            Policy::Saturate,
            Exact::on_grid(Slot::at(-here.index())),
            d,
        );
        assert_eq!(
            direct == staged,
            agrees,
            "{which:?}: direct {direct:?} against staged {staged:?}"
        );
    }

    // The control: the two ranges genuinely differ in the way the argument names,
    // and nothing else about them was changed.
    let (signed_lo, signed_hi) = bounds(Which::Signed);
    let (symmetric_lo, symmetric_hi) = bounds(Which::Symmetric);
    assert_eq!(signed_hi, symmetric_hi);
    assert_ne!(signed_lo, symmetric_lo);
    assert_eq!(symmetric_lo.index(), -symmetric_hi.index());
    assert_ne!(signed_lo.index(), -signed_hi.index());
}

#[test]
fn wrapping_relocates_and_reflects_over_every_cell_measured() {
    // The homomorphism, stated as the arm a design gates on rather than left
    // implicit in the cross above.
    for which in RANGES {
        for &mode in &ALL_MODES {
            for r in RESTRICTIONS {
                let reach = cell_reach(which, r);
                assert!(
                    at(which, mode, Policy::Wrap, &CompletionRelocates(reach)).get(),
                    "{which:?} {mode:?} {r:?}: wrapping refused to commute with translation"
                );
            }
            assert!(
                at(
                    which,
                    mode,
                    Policy::Wrap,
                    &CompletionReflects(reflection_reach(which))
                )
                .get(),
                "{which:?} {mode:?}: wrapping refused to commute with reflection"
            );
        }
    }

    // The control: a clamp does not, on the same cells, so the claim above is
    // about wrapping rather than about the cells.
    let refused = RANGES
        .iter()
        .flat_map(|&which| {
            RESTRICTIONS.iter().map(move |&r| {
                at(
                    which,
                    Mode::Floor,
                    Policy::Saturate,
                    &CompletionRelocates(cell_reach(which, r)),
                )
                .get()
            })
        })
        .filter(|held| !held)
        .count();
    assert!(
        refused > 0,
        "a clamp commuted over every cell too, so the arm above is not about the policy"
    );
}

#[test]
fn the_control_every_range_kind_the_predicates_read_is_in_the_cross() {
    // The predicates ask two questions of a range: whether its lowest slot is
    // below zero, and whether that slot is the negation of its highest. Three
    // kinds is the whole classification, and this is the arm that fails if a
    // fourth appears or if two of the three collapse.
    let mut reaching_negatives = 0;
    let mut symmetric = 0;
    for which in RANGES {
        let (lo, hi) = bounds(which);
        if lo.index() < 0 {
            reaching_negatives += 1;
        }
        if lo.index() == -hi.index() {
            symmetric += 1;
        }
    }
    assert_eq!(
        reaching_negatives, 2,
        "the cross should carry two ranges reaching a negative slot and one that does not"
    );
    assert_eq!(
        symmetric, 1,
        "the cross should carry exactly one range symmetric about zero"
    );
    assert_eq!(RANGES.len(), 3);
}
