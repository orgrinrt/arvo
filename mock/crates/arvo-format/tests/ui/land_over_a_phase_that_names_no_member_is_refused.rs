// Landing over a phase that names no member, refused, the pair that turns on the
// phase's denominator.
//
// A phase over a zero denominator names no point, so there is no distance from
// it to measure and the format's obligation refuses the declaration where `land`
// forces it. Over a denominator of two the same numerator is a half, and the same
// call is admitted. The two files differ in the phase's denominator and nothing
// else.

use core::num::NonZeroU128;

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::BinaryRationals;
use arvo_format::apply::{Dither, Scaled, land};
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::Saturate;
use arvo_format::quantum::{Constant, Exponent};
use arvo_format::rounding::TowardZero;
use arvo_format::slots::Signed;

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 0);
}

const _LANDED: () = {
    let _ = land::<Signature<Probe, Adapt<TowardZero, Saturate>>>(
        Scaled::of(1, NonZeroU128::MIN, Exponent::ZERO),
        Dither::UNUSED,
    );
};

fn main() {}
