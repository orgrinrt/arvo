//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The same claim brute force cannot afford past eight bits, carried over every
//! admitted width by sample and by witness instead.
//!
//! The claim is quantified over every value the format holds rather than over a
//! width a member-indexed `Table` can walk, so this is the instrument the width
//! walk and the widest-range fractional check both share: sixty-four triples is
//! affordable at every width the design admits.

use super::super::{NearTheTop, boundary_members, pair_for_sum, signatures};
use crate::adapt::DeclaredSignature;
use crate::addition::{add, addition_is_associative};
use crate::ambient::BinaryRationals;
use crate::apply::Dither;
use crate::format::Format;
use crate::overflow::SHIPPED_POLICIES;
use crate::quantum::Constant;
use crate::rounding::ALL_MODES;
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::tests::dispatch::{self, PerFormat, PerSignature};
use crate::tests::grid::Grid;

/// Whether `boundary_members`' sample ever diverges under a licensed
/// signature.
///
/// The claim it checks, that a licensed cell never diverges, is stated over
/// every value the format holds rather than over a width the brute-force
/// `Table` can afford, so this is the instrument the width walk and the
/// widest-range fractional check both share: sixty-four triples is affordable
/// at every width the design admits, and a member-indexed `Table` is not past
/// eight bits. `boundary_members` rather than `edges`, because the claim is
/// quantified over stored operands and `edges` reaches past them on purpose.
fn assert_licensed_sample_holds<S: DeclaredSignature>(min: Slot, max: Slot) {
    for a in boundary_members(min, max) {
        for b in boundary_members(min, max) {
            let ab = add::<S>(a, b, Dither::UNUSED);
            for c in boundary_members(min, max) {
                let bc = add::<S>(b, c, Dither::UNUSED);
                assert_eq!(
                    add::<S>(ab, c, Dither::UNUSED),
                    add::<S>(a, bc, Dither::UNUSED),
                    "a licensed cell diverged at {a:?} + {b:?} + {c:?}, [{min:?}, {max:?}]"
                );
            }
        }
    }
}

/// The verdict at one whole-phase signature: a licensed cell against the edge
/// sample above, a refused cell against the witness the design names.
///
/// At a whole phase a refusal always names a triple that diverges, one past
/// the top of the range or one past the bottom, whichever the range affords:
/// `pair_for_sum` builds the pair summing to that position and the opposite
/// end supplies the third operand, the same construction `completion_at`
/// reasons about, read back into an actual triple through `add` rather than
/// trusted from the reach machinery alone.
struct WholePhaseWitness;

impl PerSignature for WholePhaseWitness {
    type Out = ();

    fn run<S: DeclaredSignature>(&self) {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        if addition_is_associative::<S>().get() {
            assert_licensed_sample_holds::<S>(min, max);
            return;
        }
        let diverges = |a: Slot, b: Slot, c: Slot| -> bool {
            let ab = add::<S>(a, b, Dither::UNUSED);
            let bc = add::<S>(b, c, Dither::UNUSED);
            add::<S>(ab, c, Dither::UNUSED) != add::<S>(a, bc, Dither::UNUSED)
        };
        let top = max.index().checked_add(1).is_some_and(|sum| {
            pair_for_sum(min.index(), max.index(), sum).is_some_and(|(a, b)| diverges(a, b, min))
        });
        let bottom = min.index().checked_sub(1).is_some_and(|sum| {
            pair_for_sum(min.index(), max.index(), sum).is_some_and(|(a, b)| diverges(a, b, max))
        });
        assert!(
            top || bottom,
            "refused cell [{min:?}, {max:?}] named no divergent witness"
        );
    }
}

/// `WholePhaseWitness` over every mode and policy, at one format.
#[derive(Default)]
struct WidthWalk {
    formats: usize,
}

impl PerFormat for WidthWalk {
    fn run<F: Format>(&mut self) {
        for mode in ALL_MODES {
            for policy in SHIPPED_POLICIES {
                dispatch::at::<F, WholePhaseWitness>(mode, policy, &WholePhaseWitness);
            }
        }
        self.formats += 1;
    }
}

#[test]
fn the_verdict_holds_a_witness_or_a_sample_over_every_admitted_width() {
    let mut walk = WidthWalk::default();
    dispatch::every_width(&mut walk);
    assert_eq!(walk.formats, 64 * 2);
}

/// The licensed half of the verdict's claim, at the widest ranges under a
/// fractional phase.
///
/// The same formats `the_operation.rs`'s
/// `addition_is_total_at_the_widest_ranges_under_a_fractional_phase` carries:
/// the widest shipped ranges, and an outside range at the index's top, where the
/// positions the verdict reasons about sit at the edge of what the index holds.
/// A refused cell is tallied rather than witnessed: the witness the design names
/// is stated for a whole phase, and none of these is one.
struct WidestFractional;

impl PerSignature for WidestFractional {
    type Out = bool;

    fn run<S: DeclaredSignature>(&self) -> bool {
        let min = <<S::Format as Format>::Slots as Slots>::MIN;
        let max = <<S::Format as Format>::Slots as Slots>::MAX;
        let licensed = addition_is_associative::<S>().get();
        if licensed {
            assert_licensed_sample_holds::<S>(min, max);
        }
        licensed
    }
}

#[test]
fn the_verdicts_licensed_half_holds_at_the_widest_ranges_under_a_fractional_phase() {
    type F1 = Grid<BinaryRationals, Constant<0>, Signed<64>, 1, 3>;
    type F2 = Grid<BinaryRationals, Constant<0>, Signed<64>, -1, 2>;
    type F3 = Grid<BinaryRationals, Constant<0>, Unsigned<64>, -1, 3>;
    type F4 = Grid<BinaryRationals, Constant<0>, Unsigned<64>, 1, 2>;
    type F5 = Grid<BinaryRationals, Constant<0>, NearTheTop, 1, 2>;
    let mut licensed = 0u32;
    let mut cells = 0u32;
    for mode in ALL_MODES {
        for policy in SHIPPED_POLICIES {
            for is_licensed in [
                dispatch::at::<F1, WidestFractional>(mode, policy, &WidestFractional),
                dispatch::at::<F2, WidestFractional>(mode, policy, &WidestFractional),
                dispatch::at::<F3, WidestFractional>(mode, policy, &WidestFractional),
                dispatch::at::<F4, WidestFractional>(mode, policy, &WidestFractional),
                dispatch::at::<F5, WidestFractional>(mode, policy, &WidestFractional),
            ] {
                cells += 1;
                if is_licensed {
                    licensed += 1;
                }
            }
        }
    }
    assert_eq!(cells, 5 * signatures() as u32);
    assert!(
        licensed > 0,
        "the licensed half of this claim went unchecked"
    );
}
