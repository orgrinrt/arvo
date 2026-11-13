// Subtraction over an outside slot range declared at sixty-five bits, the pair
// that pins `difference_position` forcing the slot range's own obligation.
//
// Addition's obligation admits this format: its sums are carried, its phase is
// zero and its quantum constant. What refuses it is the width bound in the slot
// range's obligation, and of the steps only `difference_position` forces that,
// so subtraction over it does not compile while addition over the same
// declaration does. The two files differ in the step they name and nothing
// else.

use arvo_format::addition::sum_position;
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

struct WiderThanTheLadder;

impl Slots for WiderThanTheLadder {
    const MAX: Slot = Slot::at(u64::MAX as i128 + 1);
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::bits(65);
}

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Constant<0>;
    type Slots = WiderThanTheLadder;

    const PHASE: Phase = Phase::ZERO;
}

const _AT: () = {
    let _ = sum_position::<Probe>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
