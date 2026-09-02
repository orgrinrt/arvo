// A transitive reach is decided by the branch, and the branch is decided by the
// declaration.
//
// `has_additive_identity` forces the format's own obligation and the quantum's
// directly. It reaches the slot range's only through `slot_in_range`, inside its
// loop, on a magnitude where `cancelling_slot` answers `Is`. A phase of zero is
// cancelled by slot zero at every magnitude, so this format reaches that call on
// the first pass and the inverted range refuses.
//
// The other half is a const item in `src/tests/obligations.rs`: the same verb over
// the same inverted range under a phase that cancels nowhere, which builds. Same
// verb, same slot range, opposite outcome, decided by the phase.
//
// Bound in a const item for the reason the sibling cases are: the obligation is a
// const, evaluated where it is used, so a runtime call reaches it at codegen and
// `cargo check` never gets there. A const item reaches it at check time, which is
// what `trybuild` runs.
//
// The distinction only exists here. Monomorphising the verb from a runtime call
// instantiates `slot_in_range::<F::Slots>` whichever branch would have run, so at
// codegen the range is forced for every declaration and nothing about the phase
// matters.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{has_additive_identity, Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

struct InvertedSlots;

impl Slots for InvertedSlots {
    const MIN: Slot = Slot::at(8);
    const MAX: Slot = Slot::at(-8);
    const WIDTH: Width = Width::bits(8);
}

struct CancelsAtEveryMagnitude;

impl Format for CancelsAtEveryMagnitude {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = InvertedSlots;
    const PHASE: Phase = Phase::of(0, 1);
}

const _REFUSED: () = {
    let _ = has_additive_identity::<CancelsAtEveryMagnitude>();
};

fn main() {}
