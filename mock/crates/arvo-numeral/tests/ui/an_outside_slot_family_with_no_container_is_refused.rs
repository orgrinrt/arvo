// An outside slot family over a whole range is held only through its row in the
// placement: with no `SoleCarrier` row no ladder container holds it. Asked for
// directly, the refusal carries the note of `SoleCarrier`. Through the numeral
// door the same family is refused as a `Sole`, which does not repeat that note,
// so this is the case that checks its wording.
use arvo_format::{Slot, Slots, Width};
use arvo_placement::SoleCarrier;

/// A slot family of this crate's own, and no row for it in the placement.
struct Mine;

impl Slots for Mine {
    const MAX: Slot = Slot::at(127);
    const MIN: Slot = Slot::at(-128);
    const WIDTH: Width = Width::bits(8);
}

fn held_alone<S: SoleCarrier>() {}

fn main() {
    held_alone::<Mine>();
}
