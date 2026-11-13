// The twin: an outside slot family over the whole signed range of its width is
// held, with its container named by its own row in the placement.
use arvo_format::overflow::Wrap;
use arvo_format::quantum::Constant;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, BinaryRationals, Format, Phase, Signature, Slot, Slots, Width};
use arvo_numeral::Num;
use arvo_placement::{Carrier8, SoleCarrier};

/// A slot family of this crate's own, and its row in the placement.
struct Mine;

impl Slots for Mine {
    const MAX: Slot = Slot::at(127);
    const MIN: Slot = Slot::at(-128);
    const WIDTH: Width = Width::bits(8);
}

impl SoleCarrier for Mine {
    type Carrier = Carrier8;
}

/// An integer format over it.
struct OverMine;

impl Format for OverMine {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Mine;

    const PHASE: Phase = Phase::ZERO;
}

fn main() {
    let _ = Num::<Signature<OverMine, Adapt<TowardZero, Wrap>>>::constant::<0>();
}
