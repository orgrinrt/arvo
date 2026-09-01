// A host integer where the contract asks for a slot.
//
// The whole of what this round changed, pinned as a refusal. Before it, the
// coordinate was an `i64` and this compiled, which is why an outside crate had to
// write a host type to implement the contract at all and was refused by the
// bare-primitive lints for doing so.

use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

struct Untyped;

impl Slots for Untyped {
    const MIN: i64 = 0;
    const MAX: Slot = Slot::at(7);
    const WIDTH: Width = Width::bits(3);
}

fn main() {}
