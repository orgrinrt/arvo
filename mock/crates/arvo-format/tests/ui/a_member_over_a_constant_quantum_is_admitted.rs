// A member through the adapted door over a constant quantum, admitted. It
// differs from `a_member_over_a_magnitude_indexed_quantum_is_refused` in the
// quantum the format names.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::ambient::BinaryRationals;
use arvo_format::apply::{Dither, Exact};
use arvo_format::format::{Format, Phase};
use arvo_format::overflow::Clamp;
use arvo_format::rounding::TowardZero;
use arvo_format::slots::{Signed, Slot};
use arvo_format::{Member, quantum};

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = quantum::Constant<-3>;
    type Slots = Signed<4>;

    const PHASE: Phase = Phase::ZERO;
}

const MEMBER: Member<Probe> = Member::adapted::<Signature<Probe, Adapt<TowardZero, Clamp>>>(
    Exact::on_grid(Slot::ZERO),
    Dither::UNUSED,
);

fn main() {
    let _ = MEMBER.slot();
}
