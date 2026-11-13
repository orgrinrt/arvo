// An IEEE 754 interchange format over an unsigned ambient. The significand is
// sign and magnitude, so its range names values below zero that an unsigned
// domain does not carry, and the significand's obligation refuses it where the
// range is read, here by the range predicate.

use arvo_format::ambient::UnsignedBinaryRationals;
use arvo_format::format::Format;
use arvo_format::slots::{Slot, slot_in_range};
use arvo_format::standards::ieee754::Interchange;

type Unsigned = Interchange<UnsignedBinaryRationals, 24, 127>;

const _REFUSED: () = {
    let _ = slot_in_range::<<Unsigned as Format>::Slots>(Slot::ZERO);
};

fn main() {}
