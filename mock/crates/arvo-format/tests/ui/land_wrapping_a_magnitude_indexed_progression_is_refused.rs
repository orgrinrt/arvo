// Wrapping on a set that is one progression over two magnitudes, the pair that
// turns on the quantum.
//
// Slots 0 and 1 at quanta 1 and 2 hold 0, 1 and 2, one progression, so the
// wrapping row admits wrapping there; the wrap is modulo the progression's
// count, not the top magnitude's slot range, and it is not built yet. The same
// slots on a constant quantum wrap as any grid does. The two files differ in the
// quantum and nothing else.

use core::num::NonZeroU128;

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::BinaryRationals;
use arvo_format::apply::{Dither, Scaled, land};
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::Wrap;
use arvo_format::quantum;
use arvo_format::quantum::Exponent;
use arvo_format::rounding::TowardZero;
use arvo_format::slots::Unsigned;

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Indexed<0, 2>;
    type Slots = Unsigned<1>;

    const PHASE: Phase = Phase::ZERO;
}

const _LANDED: () = {
    let _ = land::<Signature<Probe, Adapt<TowardZero, Wrap>>>(
        Scaled::of(1, NonZeroU128::MIN, Exponent::ZERO),
        Dither::UNUSED,
    );
};

fn main() {}
