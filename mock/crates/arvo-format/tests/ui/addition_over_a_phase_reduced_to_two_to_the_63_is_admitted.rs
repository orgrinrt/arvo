// Addition over a phase that does not denote, refused, the pair that turns on
// the phase's denominator.
//
// A phase over a zero denominator names no member, so there is no sum to
// position, and the obligation refuses the format by design. Over the least
// denominator the same odd numerator reduces to a remainder of 2^63, which an
// exact position holds, and the same declaration is admitted. The two files
// differ in the phase's denominator and nothing else.

use arvo_format::addition::sum_position;
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Signed, Slot};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(i64::MAX, i64::MIN);
}

const _AT: () = {
    let _ = sum_position::<Probe>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
