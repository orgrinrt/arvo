// Addition over a phase whose remainder no position can hold, refused.
//
// An odd numerator over the least denominator reduces to a denominator of
// 2^63, which the remainder of an exact position does not carry. Saturating it
// would land the position a whole slot above the sum at the largest odd
// numerator, so the obligation refuses the format instead.

use arvo_format::addition::sum_position;
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Signed, Slot};

struct UnheldRemainder;

impl Format for UnheldRemainder {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(i64::MAX, i64::MIN);
}

const _REFUSED: () = {
    let _ = sum_position::<UnheldRemainder>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
