// IEEE 754 binary128, at the standard's own parameters, `p = 113` and
// `emax = 16383`. Its significand needs 114 bits with the sign, and the slot
// ladder stops at 64, so the significand's range is refused where it is used,
// with the diagnostic the slot range's obligation already supplies.

use arvo_format::format::Format;
use arvo_format::slots::{Slot, slot_in_range};
use arvo_format::standards::ieee754::Binary;

const _REFUSED: () = {
    let _ = slot_in_range::<<Binary<113, 16383> as Format>::Slots>(Slot::ZERO);
};

fn main() {}
