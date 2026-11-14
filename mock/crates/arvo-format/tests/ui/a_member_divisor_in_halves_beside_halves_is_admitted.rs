// The slots 1 to 255 in halves beside the bytes in halves, admitted: one
// quantum and one phase.
//
// A divisor taken from a member of a second format `G` beside `UFixed<8, -1>`,
// the bytes in halves. The pair turns on the radix of `G` alone: at exponent
// `-1` radix ten steps in tenths and radix two in halves, so the two files
// differ in the ambient `G` names and nothing else.

use arvo_format::format::{Format, Phase};
use arvo_format::points::UFixed;
use arvo_format::quantum::Constant;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;
use arvo_format::{Divisor, Member};

struct Range;

impl Slots for Range {
    const MAX: Slot = Slot::at(255);
    const MIN: Slot = Slot::at(1);
    const WIDTH: Width = Width::bits(16);
}

struct G;

impl Format for G {
    type Ambient = arvo_format::ambient::BinaryRationals;
    type Quantum = Constant<-1>;
    type Slots = Range;

    const PHASE: Phase = Phase::of(0, 1);
}

const DIVISOR: Divisor<UFixed<8, -1>> = Divisor::member(Member::<G>::constant::<7>());

fn main() {
    let _ = DIVISOR.slot();
}
