// The other of the two verbs that force the format's own obligation.
//
// `Format::ADMITTED` is forced at `cancelling_slot` and at
// `has_additive_identity`, and only the second was pinned. Deleting
// `let () = <F as Format>::ADMITTED;` from `cancelling_slot` left the whole
// suite green, so half the claim the design makes was held by nothing.
//
// Same shape as the sibling arm: the obligation is a const, evaluated where it
// is used, so binding the call in a const item is what makes the refusal
// reachable at check time rather than only at codegen. The result is discarded
// inside the const body rather than named, because naming `Maybe` here would
// pull a dependency in and the refusal has nothing to do with the return type.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{cancelling_slot, Format, Phase};
use arvo_format::quantum::{Constant, Magnitude};
use arvo_format::slots::Signed;

struct PhaseNamesNoPosition;

impl Format for PhaseNamesNoPosition {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 0);
}

const _REFUSED: () = {
    let _ = cancelling_slot::<PhaseNamesNoPosition>(Magnitude::SMALLEST);
};

fn main() {}
