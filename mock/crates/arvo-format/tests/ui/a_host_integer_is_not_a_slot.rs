// A host integer where the contract asks for a slot.
//
// A slot is a `Slot`, so a declaration handing the range a host integer does not
// typecheck. That is what lets an outside crate implement the contract without
// naming a host type on any line, and therefore without being refused by the
// bare-primitive lints for supplying the coordinates.

use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

struct Untyped;

impl Slots for Untyped {
    const MIN: i64 = 0;
    const MAX: Slot = Slot::at(7);
    const WIDTH: Width = Width::bits(3);
}

fn main() {}
