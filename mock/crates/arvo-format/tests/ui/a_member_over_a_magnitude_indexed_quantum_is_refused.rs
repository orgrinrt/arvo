// A member through the adapted door over a magnitude-indexed quantum, refused:
// membership there is a slot and a magnitude, and a member holds the slot
// alone. `adapt` forces the slot range and nothing else, so the refusal is the
// member's own. It differs from `a_member_over_a_constant_quantum_is_admitted`
// in the quantum the format names.

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
    type Quantum = quantum::Indexed<-3, 4>;
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
