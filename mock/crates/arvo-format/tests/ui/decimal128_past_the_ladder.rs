// IEEE 754 decimal128, at the standard's own parameters, `p = 34` and
// `emax = 6144`. Its significand runs to `10^34 - 1`, which needs 114 bits with
// the sign, and the slot ladder stops at 64, so the significand's range is
// refused where it is used, with the diagnostic the slot range's obligation
// already supplies.

use arvo_format::format::Format;
use arvo_format::slots::{Slot, slot_in_range};
use arvo_format::standards::ieee754::Decimal;

const _REFUSED: () = {
    let _ = slot_in_range::<<Decimal<34, 6144> as Format>::Slots>(Slot::ZERO);
};

fn main() {}
