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

use super::{Grid, Shrinking, the_narrow_cancelling_slot};
use crate::ambient::BinaryRationals;
use crate::format::{
    Format,
    Phase,
    cancelling_slot,
    contains,
    has_additive_identity,
    step_exponent,
};
use crate::points::Biased;
use crate::quantum::{Constant, Exponent, Indexed, Magnitude, Quantum};
use crate::slots::{Signed, Slot, Slots, slot_in_range};
use crate::tests::the_inventory::AtTheBottom as AtTheBottomOfTheIndex;
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
    type NeverWhole = Grid<BinaryRationals, Shrinking<40>, Signed<64>, 1, 3>;
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
    for index in 0 .. <Indexed<0, 4> as Quantum>::MAGNITUDES.count() {
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
    // The first half of the derivation on the two bounds. A growing quantum
    // scales the denominator, which passes every numerator a phase declares
    // within 64 steps.
    type Growing = Grid<BinaryRationals, Indexed<0, 300>, Signed<64>, 4, 1>;
    assert!(cancelling_slot::<Growing>(Magnitude::at(2)).is());
    assert_eq!(cancelling_slot::<Growing>(Magnitude::at(3)), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Growing>(Magnitude::at(64)), Maybe::Isnt);
    assert_eq!(cancelling_slot::<Growing>(Magnitude::at(200)), Maybe::Isnt);
    assert_eq!(
        cancelling_slot::<Growing>(Magnitude::at(u32::MAX)),
        Maybe::Isnt
    );

    // A shrinking quantum scales the numerator, and from a half it answers at
    // every magnitude until the slot leaves the index: `-2^126` at 127, the
    // index's own least value at 128, and nothing from 129 on.
    type Shrink = Grid<BinaryRationals, Shrinking<300>, Signed<64>, 1, 2>;
    assert!(cancelling_slot::<Shrink>(Magnitude::at(1)).is());
    assert_eq!(
        cancelling_slot::<Shrink>(Magnitude::at(127)),
        Maybe::Is(Slot::at(-(1i128 << 126)))
    );
    assert_eq!(
        cancelling_slot::<Shrink>(Magnitude::at(128)),
        Maybe::Is(Slot::at(i128::MIN))
    );
    assert_eq!(cancelling_slot::<Shrink>(Magnitude::at(129)), Maybe::Isnt);
    assert_eq!(
        cancelling_slot::<Shrink>(Magnitude::at(u32::MAX)),
        Maybe::Isnt
    );
}

/// A phase that takes the longest scaling there is before it answers.
///
/// One over `2^62`: sixty-two steps reduce the denominator to one, and a hundred
/// and twenty-seven more double the numerator from minus one to the index's least
/// value, so the cancelling slot is `i128::MIN` at magnitude 189. A range sitting
/// at the bottom of the index holds it, so the identity exists and is found only
/// by a search reaching that far.
type FoundAtTheFarEnd =
    Grid<BinaryRationals, Shrinking<400>, AtTheBottomOfTheIndex, 1, { 1i64 << 62 }>;

#[test]
fn the_search_bound_reaches_the_longest_scaling_that_answers() {
    assert_eq!(
        cancelling_slot::<FoundAtTheFarEnd>(Magnitude::at(189)),
        Maybe::Is(Slot::at(i128::MIN))
    );
    assert!(
        has_additive_identity::<FoundAtTheFarEnd>().get(),
        "the search stopped before the one magnitude whose slot the range holds"
    );

    // The magnitudes around it: one below is a slot the range does not hold, and
    // one above leaves the index.
    assert_eq!(
        cancelling_slot::<FoundAtTheFarEnd>(Magnitude::at(188)),
        Maybe::Is(Slot::at(-(1i128 << 126)))
    );
    assert_eq!(
        cancelling_slot::<FoundAtTheFarEnd>(Magnitude::at(190)),
        Maybe::Isnt
    );

    // A phase written with a common factor of the radix: four over two to the
    // sixty-second is one over two to the sixtieth, found at 187. A factor the
    // radix shares is cancelled by the scaling itself, so this one answers the
    // same with or without the reduction; the arm below is the one that needs it.
    type Unreduced =
        Grid<BinaryRationals, Shrinking<400>, AtTheBottomOfTheIndex, 4, { 1i64 << 62 }>;
    assert_eq!(
        cancelling_slot::<Unreduced>(Magnitude::at(187)),
        Maybe::Is(Slot::at(i128::MIN))
    );
    assert!(has_additive_identity::<Unreduced>().get());
}

#[test]
fn a_phase_written_with_a_factor_the_radix_does_not_share_is_found_at_every_magnitude() {
    // Nine over three is three, and twelve over six is two. Each carries a
    // common factor the radix does not divide, so a scaling that looks for the
    // radix in what is left of the denominator finds none and would answer that
    // no number of steps makes the phase whole. In lowest terms the denominator
    // is one and every magnitude answers.
    type NineOverThree = Grid<BinaryRationals, Shrinking<4>, Signed<8>, 9, 3>;
    type TwelveOverSix = Grid<BinaryRationals, Shrinking<4>, Signed<8>, 12, 6>;
    for (magnitude, three, two) in
        [(0u32, -3i128, -2i128), (1, -6, -4), (2, -12, -8), (3, -24, -16)]
    {
        assert_eq!(
            cancelling_slot::<NineOverThree>(Magnitude::at(magnitude)),
            Maybe::Is(Slot::at(three)),
            "nine over three at magnitude {magnitude}"
        );
        assert_eq!(
            cancelling_slot::<TwelveOverSix>(Magnitude::at(magnitude)),
            Maybe::Is(Slot::at(two)),
            "twelve over six at magnitude {magnitude}"
        );
    }

    // The control: the same phases in lowest terms give the same answers, so the
    // arm is about how the phase is written and nothing else.
    type Three = Grid<BinaryRationals, Shrinking<4>, Signed<8>, 3, 1>;
    type Two = Grid<BinaryRationals, Shrinking<4>, Signed<8>, 2, 1>;
    for magnitude in 0u32 .. 4 {
        let m = Magnitude::at(magnitude);
        assert_eq!(
            cancelling_slot::<NineOverThree>(m),
            cancelling_slot::<Three>(m)
        );
        assert_eq!(
            cancelling_slot::<TwelveOverSix>(m),
            cancelling_slot::<Two>(m)
        );
    }
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

/// A slot range sitting at the bottom of what a 64-bit integer carries.
///
/// Admissible: it is not inverted, its span is three, and three bits address it.
/// It exists so a negation that wrapped to `i64::MIN` would land inside a real
/// range rather than harmlessly outside every shipped one.
struct AtTheBottomSlots;

impl Slots for AtTheBottomSlots {
    const MAX: Slot = Slot::at(i64::MIN as i128 + 3);
    const MIN: Slot = Slot::at(i64::MIN as i128);
    const WIDTH: Width = Width::bits(3);
}

/// A whole-multiple phase whose cancelling slot is one past what a 64-bit integer
/// carries, and which the slot index holds.
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
fn a_phase_whose_cancelling_slot_leaves_the_phase_width_answers_where_it_lands() {
    // The phase is a whole multiple, so the divisibility half says yes and the
    // answer turns entirely on where the cancelling slot lands: two to the
    // sixty-third, a slot the index holds and this range does not.
    assert!(<AtTheBottom as Format>::PHASE.is_whole_multiple().get());
    assert_eq!(
        cancelling_slot::<AtTheBottom>(Magnitude::SMALLEST),
        Maybe::Is(Slot::at(1i128 << 63))
    );
    assert!(!has_additive_identity::<AtTheBottom>().get());

    // The mutant, run rather than described. Taking the negation in the phase's
    // own width wraps two to the sixty-third down to `i64::MIN`, which this range
    // admits, so the narrow form answers yes to a position the range does not
    // hold. That is what the wide intermediate is for, and it is why the range
    // above is declared where it is rather than at some convenient width.
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

/// A phase whose scaling drives the numerator to the least value the slot index
/// holds, over a denominator of minus one.
///
/// A shrinking quantum multiplies the numerator by the radix once per magnitude,
/// so from two to the sixty-second the sixty-fifth magnitude reaches `-2^127`.
/// The quotient over minus one is `2^127`, one past the index's top, and the
/// remainder of that pair is the one signed remainder that overflows.
type ScaledToTheLeastValue = Grid<BinaryRationals, Shrinking<66>, Signed<8>, { 1i64 << 62 }, -1>;

#[test]
fn a_phase_scaled_to_the_least_index_value_over_minus_one_answers_isnt() {
    // The pair a checked division is there for. Unchecked, the remainder
    // overflows and the search diverges on a format that compiles.
    assert_eq!(
        cancelling_slot::<ScaledToTheLeastValue>(Magnitude::at(65)),
        Maybe::Isnt
    );

    // The control one magnitude down, where the same division is ordinary: the
    // numerator is `-2^126` and the quotient `2^126`, a slot the index holds.
    assert_eq!(
        cancelling_slot::<ScaledToTheLeastValue>(Magnitude::at(64)),
        Maybe::Is(Slot::at(1i128 << 126))
    );

    // And the identity search, which walks both, answers rather than diverging.
    assert!(!has_additive_identity::<ScaledToTheLeastValue>().get());
}
