// A disarmed outside row naming the 8-bit container for a 16-bit family is
// refused where a value is built, though its own obligation checks nothing.
use arvo_format::overflow::Wrap;
use arvo_format::quantum::Constant;
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, BinaryRationals, Format, Phase, Signature, Slot, Slots, Width};
use arvo_numeral::Num;
use arvo_placement::{Carrier8, SoleCarrier};

/// A 16-bit slot family of this crate's own.
struct Mine;

impl Slots for Mine {
    const MAX: Slot = Slot::at(65535);
    const MIN: Slot = Slot::at(0);
    const WIDTH: Width = Width::bits(16);
}

/// Its row, with the placement's obligation written over by one that checks
/// nothing.
impl SoleCarrier for Mine {
    type Carrier = Carrier8;

    const ADMITTED: () = ();
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
    let _ = Num::<Signature<OverMine, Adapt<TowardZero, Wrap>>>::constant::<300>();
}
