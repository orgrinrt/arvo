// A divisor read at run time over a range that leaves the zero out, the pair
// that turns on the phase.
//
// At a whole phase the slot `-n` names the zero whether or not the range holds
// it, and a slot is not typed as a member, so a slot read at run time could be
// that one and telling it apart would be a check on the value. `Divisor::of` is
// refused there. At a half-offset phase no slot names the zero and it is
// admitted. The two files differ in the phase and nothing else.

use arvo_format::ambient::BinaryRationals;
use arvo_format::division::Divisor;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

struct Range;

impl Slots for Range {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(8);
}

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Range;

    const PHASE: Phase = Phase::ZERO;
}

const _DIVISOR: Divisor<Probe> = Divisor::of(Slot::at(7));

fn main() {}
