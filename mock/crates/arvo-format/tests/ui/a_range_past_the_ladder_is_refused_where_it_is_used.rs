// An outside slot range declared at sixty-five bits, refused where it is used.
//
// The shipped ladder stops at 64 because there is no impl past it, and that
// refusal is a trait bound. An outside range is not held by the impl set at all:
// it is ordered, and sixty-five bits address its `2^64 + 1` slots, so the only
// thing that refuses it is the width bound in the obligation itself, the only
// guard standing between this declaration and a crate that treats it as
// admitted.

use arvo_format::slots::{Slot, Slots, slot_in_range};
use arvo_format::width::Width;

struct WiderThanTheLadder;

impl Slots for WiderThanTheLadder {
    const MAX: Slot = Slot::at(u64::MAX as i128 + 1);
    const MIN: Slot = Slot::ZERO;
    const WIDTH: Width = Width::bits(65);
}

const _REFUSED: () = {
    let _ = slot_in_range::<WiderThanTheLadder>(Slot::ZERO);
};

fn main() {}
