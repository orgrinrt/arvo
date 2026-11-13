// A binary interchange format at a precision of zero, whose significand holds
// only zero. The significand's obligation refuses it where the range is read,
// here by the range predicate.

use arvo_format::format::Format;
use arvo_format::slots::{Slot, slot_in_range};
use arvo_format::standards::ieee754::Binary;

const _REFUSED: () = {
    let _ = slot_in_range::<<Binary<0, 1> as Format>::Slots>(Slot::ZERO);
};

fn main() {}
