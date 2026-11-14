// The exact conversion past the slot index, admitted.
//
// The top of the signed 64-bit integers placed on an 8-bit grid at a quantum of
// 2^-70 lands near 2^133, past the index. The step holds its floor in four
// limbs, so the obligation reads nothing of the index.

use arvo_format::ambient::BinaryRationals;
use arvo_format::conversion::converted_position;
use arvo_format::format::{Format, Phase};
use arvo_format::points::Integer;
use arvo_format::quantum::Constant;
use arvo_format::slots::{Slot, Unsigned};

struct Deep;

impl Format for Deep {
    type Ambient = BinaryRationals;
    type Quantum = Constant<-70>;
    type Slots = Unsigned<8>;

    const PHASE: Phase = Phase::ZERO;
}

const _CONVERTED: () = {
    let _ = converted_position::<Integer<64>, Deep>(Slot::at((1 << 63) - 1));
};

fn main() {}
