// An outside slot family is held on the terms a shipped one is: a range that is
// only part of its width's whole range is refused, whichever crate declares it.
use arvo_format::overflow::Wrap;
use arvo_format::quantum::Constant;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, BinaryRationals, Format, Phase, Signature, Slot, Slots, Width};
use arvo_numeral::Num;
use arvo_placement::{Carrier8, SoleCarrier};

/// A slot family of this crate's own, and its row in the placement.
struct Mine;

impl Slots for Mine {
    const MAX: Slot = Slot::at(100);
    const MIN: Slot = Slot::at(-100);
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
