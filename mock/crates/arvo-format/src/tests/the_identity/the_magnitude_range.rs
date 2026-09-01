//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The magnitude the identity is found at, and what the bounded search costs.
//!
//! The existential runs over the magnitude, so the answer can come from anywhere
//! in the declared range rather than from its bottom. These arms move that
//! coordinate alone, with every other one held where the arm above it left it,
//! and they carry the search bound's derivation as assertions rather than as a
//! paragraph.
//!
//! The wide intermediate's mutant is here too, because what it separates is a
//! cancelling slot one past what an index carries, which is the same coordinate
//! seen from its far end.

use notko::Maybe;

use super::{the_narrow_cancelling_slot, Grid, Shrinking};
use crate::ambient::BinaryRationals;
use crate::format::{
    cancelling_slot, contains, has_additive_identity, step_exponent, Format, Phase,
};
use crate::points::Biased;
use crate::quantum::{Constant, Exponent, Indexed, Magnitude, Quantum};
use crate::slots::{slot_in_range, Signed, Slot, Slots};
use crate::width::Width;

// --- the magnitude range, which is the coordinate a constant quantum hides ----

#[test]
fn a_whole_phase_out_of_reach_low_down_is_found_at_a_higher_magnitude() {
    // Every coordinate here is one this crate ships, so an outside `Format`
    // reaches this with no outside `Quantum` at all. The quantum doubles per
    // magnitude, so the same absolute phase is half as many steps at each one:
    // slot -4 at magnitude zero, outside `Signed<2>`, and slot -2 at magnitude
    // one, which is the range's own lowest index.
    type Growing = Grid<BinaryRationals, Indexed<0, 2>, Signed<2>, 4, 1>;
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-4))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(1)),
        Maybe::Is(Slot::at(-2))
    );
    assert!(
        has_additive_identity::<Growing>().get(),
        "the cancelling slot in range at magnitude one was not found"
    );

    // The control, and it is what says the search found it rather than the first
    // magnitude: cutting the magnitude range to one takes the identity away while
    // every other coordinate stays where it was.
    type OneMagnitude = Grid<BinaryRationals, Indexed<0, 1>, Signed<2>, 4, 1>;
    assert_eq!(
        cancelling_slot::<OneMagnitude>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-4))
    );
    assert!(
        !has_additive_identity::<OneMagnitude>().get(),
        "cutting the magnitude range to one did not take the identity back"
    );
}

#[test]
fn the_identity_survives_a_shrinking_quantum() {
    // The half that refutes the whole-multiple reading outright, and the arm this
    // crate carried red while the predicate answered at one magnitude. The step
    // is radix^0 at magnitude zero and radix^-1 at magnitude one, so it halves,
    // and a phase of one half is exactly one step up there.
    type HalfOnShrinking = Grid<BinaryRationals, Shrinking<3>, Signed<8>, 1, 2>;

    // The geometry, worked out rather than taken from the predicate.
    assert_eq!(
        step_exponent::<HalfOnShrinking>(Magnitude::SMALLEST),
        Exponent::ZERO
    );
    assert_eq!(
        step_exponent::<HalfOnShrinking>(Magnitude::at(1)),
        Exponent::of(-1)
    );

    // So 1/2 + (-1) * 1/2 is zero, at coordinates the format admits.
    assert!(contains::<HalfOnShrinking>(Slot::at(-1), Magnitude::at(1)).get());
    assert_eq!(
        cancelling_slot::<HalfOnShrinking>(Magnitude::SMALLEST),
        Maybe::Isnt
    );
    assert_eq!(
        cancelling_slot::<HalfOnShrinking>(Magnitude::at(1)),
        Maybe::Is(Slot::at(-1))
    );
    assert!(has_additive_identity::<HalfOnShrinking>().get());

    // The control that says this arm is about the magnitude rather than about the
    // phase: the same phase over a law that does not shrink genuinely has no
    // identity, and that is the region the reduction does cover.
    assert!(!has_additive_identity::<Biased<8, 0, 1>>().get());

    // And the second control: one magnitude and the same fractional phase has no
    // identity, so the magnitude range is what made the difference rather than
    // the slope's sign on its own.
    type OneMagnitude = Grid<BinaryRationals, Shrinking<1>, Signed<8>, 1, 2>;
    assert!(
        !has_additive_identity::<OneMagnitude>().get(),
        "cutting the magnitude range to one did not take the identity back"
    );

    // And the phase still has to become whole eventually. A denominator of three
    // never divides a power of two, so no magnitude cancels it.
    type NeverWhole = Grid<BinaryRationals, Shrinking<40>, Signed<62>, 1, 3>;
    assert!(
        !has_additive_identity::<NeverWhole>().get(),
        "a phase whose denominator no power of the radix divides gained an identity"
    );
}

#[test]
fn the_magnitude_the_identity_is_found_at_is_not_always_the_first() {
    // Stated on its own because it is the whole content of the law: the
    // existential runs over the magnitude, so the answer can come from anywhere
    // in the range rather than from its bottom.
    type Growing = Grid<BinaryRationals, Indexed<0, 4>, Signed<2>, 16, 1>;
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-16))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(1)),
        Maybe::Is(Slot::at(-8))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(2)),
        Maybe::Is(Slot::at(-4))
    );
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(3)),
        Maybe::Is(Slot::at(-2))
    );
    assert!(has_additive_identity::<Growing>().get());

    // Only magnitude three answers, so a search reading any single magnitude
    // gives the wrong answer whichever one it reads. If a later change made a
    // second magnitude answer, this arm would stop showing that the search is
    // what did it, which is why the count is asserted rather than assumed.
    let mut answering = 0;
    for index in 0..<Indexed<0, 4> as Quantum>::MAGNITUDES.count() {
        if let Maybe::Is(slot) = cancelling_slot::<Growing>(Magnitude::at(index)) {
            if slot_in_range::<Signed<2>>(slot).get() {
                answering += 1;
            }
        }
    }
    assert_eq!(
        answering, 1,
        "the witness is not unique, so this arm does not show the search mattered"
    );
}

// --- the search bound, which is the derivation pinned ------------------------

#[test]
fn the_search_bound_is_past_where_a_radix_of_two_can_still_answer() {
    // The first half of the derivation on the two bounds: at a radix of at least
    // two the running product leaves the wider width within 127 scaling steps, so
    // every magnitude past the bound answers `Isnt` and stopping there loses
    // nothing.
    type Growing = Grid<BinaryRationals, Indexed<0, 300>, Signed<62>, 4, 1>;
    assert!(cancelling_slot::<Growing>(Magnitude::at(2)).is());
    assert_eq!(cancelling_slot::<Growing>(Magnitude::at(127)), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Growing>(Magnitude::at(200)), Maybe::Isnt);
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(u32::MAX)),
        Maybe::Isnt
    );

    type Shrink = Grid<BinaryRationals, Shrinking<300>, Signed<62>, 1, 2>;
    assert!(cancelling_slot::<Shrink>(Magnitude::at(1)).is());
    assert_eq!(cancelling_slot::<Shrink>(Magnitude::at(127)), Maybe::Isnt);
    assert_eq!(
        cancelling_slot::<Shrink>(Magnitude::at(u32::MAX)),
        Maybe::Isnt
    );
}

#[test]
fn the_search_bound_loses_nothing_where_the_quantum_does_not_move() {
    // The second half: at a zero slope every magnitude gives the same equation,
    // so one answers them all and a bound cannot cut anything off.
    type Flat = Grid<BinaryRationals, Constant<3>, Signed<8>, 4, 1>;
    assert_eq!(
        cancelling_slot::<Flat>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(-4))
    );
    assert_eq!(
        cancelling_slot::<Flat>(Magnitude::at(1)),
        Maybe::Is(Slot::at(-4))
    );
    assert_eq!(
        cancelling_slot::<Flat>(Magnitude::at(127)),
        Maybe::Is(Slot::at(-4))
    );
    assert_eq!(
        cancelling_slot::<Flat>(Magnitude::at(u32::MAX)),
        Maybe::Is(Slot::at(-4))
    );
}

#[test]
fn a_magnitude_range_past_the_bound_still_finds_what_is_below_it() {
    // The bound cuts the search and not the answer. A format declaring far more
    // magnitudes than the bound still gets the identity that sits at a low one.
    type Wide = Grid<BinaryRationals, Indexed<0, 100_000>, Signed<8>, 4, 1>;
    assert!(has_additive_identity::<Wide>().get());

    // And one whose phase no magnitude can cancel does not, which is the honest
    // statement of what the bound costs: nothing, because no magnitude past it
    // can answer at a radix of at least two.
    type NoAnswer = Grid<BinaryRationals, Indexed<0, 100_000>, Signed<2>, 1, 3>;
    assert!(!has_additive_identity::<NoAnswer>().get());
}

// --- the wide intermediate, and the mutant that says why it is there ----------

/// A slot range sitting at the bottom of what an index carries.
///
/// Admissible: it is not inverted, its span is three, and three bits address it.
/// It exists so a negation that wrapped to `i64::MIN` would land inside a real
/// range rather than harmlessly outside every shipped one.
struct AtTheBottomSlots;

impl Slots for AtTheBottomSlots {
    const MIN: Slot = Slot::at(i64::MIN);
    const MAX: Slot = Slot::at(i64::MIN + 3);
    const WIDTH: Width = Width::bits(3);
}

/// A whole-multiple phase whose cancelling slot is one past what an index carries.
struct AtTheBottom;

impl Format for AtTheBottom {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = AtTheBottomSlots;
    const PHASE: Phase = Phase::of(i64::MIN, 1);
}

/// The same range with a phase whose cancelling slot it does hold.
struct NearTheBottom;

impl Format for NearTheBottom {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = AtTheBottomSlots;
    const PHASE: Phase = Phase::of(-(i64::MIN + 3), 1);
}

#[test]
fn a_phase_whose_cancelling_slot_leaves_the_index_answers_no() {
    // The phase is a whole multiple, so the divisibility half says yes and the
    // answer turns entirely on where the cancelling slot lands.
    assert!(<AtTheBottom as Format>::PHASE.is_whole_multiple().get());
    assert_eq!(
        cancelling_slot::<AtTheBottom>(Magnitude::SMALLEST),
        Maybe::Isnt
    );
    assert!(!has_additive_identity::<AtTheBottom>().get());

    // The mutant, run rather than described. Taking the negation in the index's
    // own width wraps two to the sixty-third down to `i64::MIN`, which this range
    // admits, so the narrow form answers yes to a position no slot can name. That
    // is what the wide intermediate is for, and it is why the range above is
    // declared where it is rather than at some convenient width.
    assert!(
        slot_in_range::<AtTheBottomSlots>(the_narrow_cancelling_slot(
            <AtTheBottom as Format>::PHASE
        ))
        .get()
    );

    // The control: the same range with a reachable cancelling slot answers yes,
    // so the arm above is about the overshoot rather than about a range that
    // refuses everything.
    assert!(has_additive_identity::<NearTheBottom>().get());

    // And the mutant agrees with the predicate wherever nothing overflows, so it
    // is the one value that separates them rather than a function that differs
    // everywhere.
    assert!(
        slot_in_range::<AtTheBottomSlots>(the_narrow_cancelling_slot(
            <NearTheBottom as Format>::PHASE
        ))
        .get()
    );
}
