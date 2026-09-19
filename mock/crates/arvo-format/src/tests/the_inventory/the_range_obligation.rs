//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What an outside slot range owes, and the constructions that do not meet it.
//!
//! Every range here is declared outside the shipped ladder and reached through
//! `is_admissible`, which returns the verdict the `ADMITTED` obligation enforces
//! without forcing it, so a construction that compiles and is wrong stays in the
//! suite as something the law is shown to reject.

use super::{OffsetFive, WiderThanItsSpan};
use crate::slots::{Signed, Slot, SlotCount, Slots, Unsigned, slot_count, slot_in_range};
use crate::width::Width;

/// A slot range from outside this crate that does not meet the contract.
///
/// A construction from outside this crate, values verbatim, kept permanently
/// rather than in a scratch file. It compiles, which is the point: the trait
/// is open and nothing stops it being written. What it does not do is pass the law
/// below, and using it does not build, which the `trybuild` case records.
struct RogueRange;

impl Slots for RogueRange {
    const MAX: Slot = Slot::at(-4611686018427387905);
    const MIN: Slot = Slot::at(4611686018427387904);
    const WIDTH: Width = Width::bits(63);
}

/// A width of zero, which admits nothing.
struct EmptyRange;

impl Slots for EmptyRange {
    const MAX: Slot = Slot::at(-1);
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::NONE;
}

/// A width of zero over one slot, ordered and addressed.
///
/// `EmptyRange` above is inverted as well as zero-width, so the ordering
/// condition refuses it on its own and the width condition is never the one
/// deciding. This one is ordered, and a width of zero addresses its single slot,
/// since two to the zero is one, so the only condition it fails is that the
/// width is at least one.
struct ZeroWidthOverOneSlot;

impl Slots for ZeroWidthOverOneSlot {
    const MAX: Slot = Slot::ZERO;
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::NONE;
}

/// The same single slot at a width of one, which differs from the one above in
/// the width alone.
struct OneBitOverOneSlot;

impl Slots for OneBitOverOneSlot {
    const MAX: Slot = Slot::ZERO;
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::bits(1);
}

#[test]
fn a_width_of_zero_is_refused_even_where_it_addresses_its_span() {
    assert!(
        !crate::slots::is_admissible::<ZeroWidthOverOneSlot>().get(),
        "a zero width over one slot was admitted, so the width's lower bound is not checked"
    );
    // The controls. The range is ordered, and the same slot at a width of one is
    // admitted, so the width is the one thing refusing it. A zero width addresses
    // the span as well as a width of one does: one slot, and two to the zero is one.
    assert!(
        <ZeroWidthOverOneSlot as Slots>::MIN
            .is_at_most(<ZeroWidthOverOneSlot as Slots>::MAX)
            .get()
    );
    assert!(
        crate::slots::is_admissible::<OneBitOverOneSlot>().get(),
        "one slot at a width of one was refused, so the zero-width refusal is not about the width"
    );
}

#[test]
fn the_law_rejects_a_range_that_does_not_meet_the_contract() {
    // The law returns a verdict, so the wrong construction can be reported on
    // without forcing the const that refuses it. Asserting that it rejects is the
    // shape a construction that compiles and is wrong wants.
    assert!(
        !crate::slots::is_admissible::<RogueRange>().get(),
        "an inverted range was admitted, which is the finding returning"
    );
    assert!(
        !crate::slots::is_admissible::<EmptyRange>().get(),
        "a zero-width range was admitted"
    );
}

#[test]
fn the_law_admits_every_range_this_crate_ships() {
    // The control. A law that rejected everything would pass the test above and
    // establish nothing, so it has to accept the shipped set.
    macro_rules! admits {
        ($($w:literal),+ $(,)?) => {
            $(
                assert!(
                    crate::slots::is_admissible::<Unsigned<$w>>().get(),
                    "unsigned {} was refused by the law", $w
                );
                assert!(
                    crate::slots::is_admissible::<Signed<$w>>().get(),
                    "signed {} was refused by the law", $w
                );
            )+
        };
    }
    admits!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    );

    // And the two foreign ranges in the parent, neither of which is a shipped
    // shape and both of which are admissible, so the law is about the
    // obligations rather than about the two constructions this crate writes.
    assert!(crate::slots::is_admissible::<OffsetFive>().get());
    assert!(crate::slots::is_admissible::<WiderThanItsSpan>().get());
}

#[test]
fn the_law_separates_the_two_constructions_rather_than_answering_one_way() {
    // Both directions in one place, so a law stuck at `true` or at `false` fails
    // here rather than passing one of the two tests above.
    let shipped = crate::slots::is_admissible::<Unsigned<13>>().get();
    let rogue = crate::slots::is_admissible::<RogueRange>().get();
    assert_ne!(
        shipped, rogue,
        "the law gives the same verdict to a shipped range and an inverted one"
    );
}

/// A range whose span the slot index cannot state.
///
/// `MIN <= MAX` holds and the width is in range, so an obligation checking only
/// those admits it. Its ends are the index's own, so the span between them is
/// `2^128 - 1` and the subtraction that asks whether the width addresses it
/// leaves the index while asking. The obligation computes it checked and reads
/// the overflow as the refusal. Computed unchecked it panics in a debug build
/// and wraps in a release one, and the wrapped difference is small enough that
/// the range is admitted.
struct SpanPastTheIndex;

impl Slots for SpanPastTheIndex {
    const MAX: Slot = Slot::at(i128::MAX);
    const MIN: Slot = Slot::at(i128::MIN);
    const WIDTH: Width = Width::bits(64);
}

/// A range one slot wider than its declared width addresses, at the widest width.
///
/// The boundary of the addressing condition where it is tightest: `2^64 + 1`
/// slots at a width addressing `2^64`.
struct OneSlotPastSixtyFour;

impl Slots for OneSlotPastSixtyFour {
    const MAX: Slot = Slot::at(u64::MAX as i128);
    const MIN: Slot = Slot::at(-1);
    const WIDTH: Width = Width::bits(64);
}

/// A range the ladder does not reach, declared at a width that addresses it.
///
/// Ordered, and sixty-five bits address its `2^64 + 1` slots, so the only
/// condition it fails is the bound on the width itself. Without that bound it is
/// admitted, which is what this construction exists to show.
struct WiderThanTheLadder;

impl Slots for WiderThanTheLadder {
    const MAX: Slot = Slot::at(u64::MAX as i128 + 1);
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::bits(65);
}

/// A range at the very top of the slot index, eight bits wide.
///
/// An outside range placed where the shipped ones never sit. Admissible: ordered,
/// an admitted width, and a span the width addresses.
pub(crate) struct AtTheTop;

impl Slots for AtTheTop {
    const MAX: Slot = Slot::at(i128::MAX);
    const MIN: Slot = Slot::at(i128::MAX - 255);
    const WIDTH: Width = Width::bits(8);
}

/// A range at the very bottom of the slot index, eight bits wide.
pub(crate) struct AtTheBottom;

impl Slots for AtTheBottom {
    const MAX: Slot = Slot::at(i128::MIN + 255);
    const MIN: Slot = Slot::at(i128::MIN);
    const WIDTH: Width = Width::bits(8);
}

/// A range whose declared width cannot address it.
struct WidthTooNarrow;

impl Slots for WidthTooNarrow {
    const MAX: Slot = Slot::at(1000);
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::bits(4);
}

#[test]
fn the_law_rejects_a_range_that_passes_the_easy_obligations() {
    // The case a weaker obligation admitted. Kept permanently because it is the
    // one that looks admissible: nothing about it is inverted and its width is in
    // range, and it still breaks the only thing the range is for.
    assert!(
        !crate::slots::is_admissible::<SpanPastTheIndex>().get(),
        "a span the index cannot state was admitted"
    );
    assert!(
        !crate::slots::is_admissible::<WidthTooNarrow>().get(),
        "a width that cannot address its own range was admitted"
    );
    assert!(
        !crate::slots::is_admissible::<OneSlotPastSixtyFour>().get(),
        "a range of 2^64 + 1 slots was admitted at a width addressing 2^64"
    );
    assert!(
        !crate::slots::is_admissible::<WiderThanTheLadder>().get(),
        "a width past the ladder was admitted because it addresses its span"
    );

    // And the reasons are distinct from the inverted case, so the law is not
    // rejecting everything that is not a shipped shape.
    assert!(
        <SpanPastTheIndex as Slots>::MIN
            .is_at_most(<SpanPastTheIndex as Slots>::MAX)
            .get()
    );
    assert!(
        <WidthTooNarrow as Slots>::MIN
            .is_at_most(<WidthTooNarrow as Slots>::MAX)
            .get()
    );
    assert!(
        <OneSlotPastSixtyFour as Slots>::MIN
            .is_at_most(<OneSlotPastSixtyFour as Slots>::MAX)
            .get()
    );
    assert!(
        <WiderThanTheLadder as Slots>::MIN
            .is_at_most(<WiderThanTheLadder as Slots>::MAX)
            .get()
    );

    // A declaration may be wider than its span and not narrower, which is the
    // asymmetry the obligation carries: a range addressable by more bits than it
    // needs is coherent, and one its width cannot address is not.
    assert!(crate::slots::is_admissible::<WiderThanItsSpan>().get());
}

#[test]
fn the_law_admits_a_range_anywhere_the_index_reaches() {
    // No condition ties a range to the neighbourhood of zero: a range at either
    // end of the index, at a width addressing it, is admitted, and counting it
    // reads what its ends say.
    assert!(crate::slots::is_admissible::<AtTheTop>().get());
    assert!(crate::slots::is_admissible::<AtTheBottom>().get());
    assert_eq!(slot_count::<AtTheTop>(), SlotCount::of(256));
    assert_eq!(slot_count::<AtTheBottom>(), SlotCount::of(256));
    assert!(slot_in_range::<AtTheTop>(Slot::at(i128::MAX)).get());
    assert!(!slot_in_range::<AtTheTop>(Slot::at(i128::MAX - 256)).get());
    assert!(slot_in_range::<AtTheBottom>(Slot::at(i128::MIN)).get());
    assert!(!slot_in_range::<AtTheBottom>(Slot::at(i128::MIN + 256)).get());
}
