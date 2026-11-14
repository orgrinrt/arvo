// The exact quotient past the slot index, admitted.
//
// Over the signed 64-bit range at a quantum of 2^-64, the minimum over minus
// one is the position 2^127, one past the index. The step holds its floor in
// four limbs, so the obligation reads nothing of the index.

use arvo_format::ambient::BinaryRationals;
use arvo_format::division::{Divisor, quotient_position};
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::{Signed, Slot};

struct Fine;

impl Format for Fine {
    type Ambient = BinaryRationals;
    type Quantum = Constant<-64>;
    type Slots = Signed<64>;

    const PHASE: Phase = Phase::ZERO;
}

const _QUOTIENT: () = {
    let _ = quotient_position::<Fine>(Slot::at(-(1 << 63)), Divisor::constant::<-1>());
};

fn main() {}
