// Addition over a format whose sum leaves the slot coordinate, refused.
//
// No shipped format reaches this: the widest range is 64 bits and sits more
// than sixty bits inside the index. An outside range may sit anywhere the index
// reaches, and one near its top, with a phase of many whole quanta, has a sum of
// two members the index does not hold. Without the refusal the step would
// narrow a position no slot index holds.

use arvo_format::addition::sum_position;
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

struct NearTheTop;

impl Slots for NearTheTop {
    const MAX: Slot = Slot::at(i128::MAX / 2);
    const MIN: Slot = Slot::at(i128::MAX / 2 - 3);
    const WIDTH: Width = Width::bits(2);
}

struct FarPhase;

impl Format for FarPhase {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = NearTheTop;

    const PHASE: Phase = Phase::of(i64::MAX, 1);
}

const _REFUSED: () = {
    let _ = sum_position::<FarPhase>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
