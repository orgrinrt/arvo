// A slot range that does not hold zero, the pair that turns on the quantum.
//
// Slots 8 to 15 at quanta 1 and 4 hold 8 to 15 and 32 to 60 in fours, and a
// point between them rounds out of range at both, so the finest grid in range
// is not the nearest member there; landing on such a format is not yet
// admitted. The same slots on a constant quantum are one grid. The two files
// differ in the quantum and nothing else.

use core::num::NonZeroU128;

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::BinaryRationals;
use arvo_format::apply::{Dither, Scaled, land};
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::Saturate;
use arvo_format::quantum::{Exponent, MagnitudeCount, Quantum};
use arvo_format::rounding::TowardZero;
use arvo_format::slots::{Slot, Slots};
use arvo_format::width::Width;

struct Upper;

impl Slots for Upper {
    const MAX: Slot = Slot::at(15);
    const MIN: Slot = Slot::at(8);
    const WIDTH: Width = Width::bits(4);
}

struct ByFour;

impl Quantum for ByFour {
    const BASE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(2);
    const SLOPE: Exponent = Exponent::of(2);
}

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = ByFour;
    type Slots = Upper;

    const PHASE: Phase = Phase::ZERO;
}

const _LANDED: () = {
    let _ = land::<Signature<Probe, Adapt<TowardZero, Saturate>>>(
        Scaled::of(1, NonZeroU128::MIN, Exponent::ZERO),
        Dither::UNUSED,
    );
};

fn main() {}
