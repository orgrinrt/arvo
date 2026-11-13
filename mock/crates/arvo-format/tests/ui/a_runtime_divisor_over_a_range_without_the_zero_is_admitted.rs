// A divisor read at run time over a range that holds the zero, the pair that
// turns on where the range starts.
//
// A slot read at run time could be the zero member, and telling it apart would
// be a check on the value, so `Divisor::of` is refused over a format that has
// one. Over the same range starting at one there is no zero member and it is
// admitted. The two files differ in the range's lowest slot and nothing else.

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
