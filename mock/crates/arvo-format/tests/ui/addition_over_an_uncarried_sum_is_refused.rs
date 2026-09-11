// Addition over a format whose sum leaves the slot coordinate, refused.
//
// No shipped format reaches this: the widest range is 62 bits and `Biased`
// counts its phase in halves. An outside format declaring a phase of many
// whole quanta does, and without the refusal the step would narrow a position
// no slot index holds.

use arvo_format::addition::sum_position;
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Signed, Slot};

struct FarPhase;

impl Format for FarPhase {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(i64::MAX, 1);
}

const _REFUSED: () = {
    let _ = sum_position::<FarPhase>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
