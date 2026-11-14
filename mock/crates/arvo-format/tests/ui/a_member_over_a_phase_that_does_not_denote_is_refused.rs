// A member through the constant door over a phase that names no position on
// the grid, refused. The slot is in range, so the refusal is the format's own
// obligation, forced by the door. It differs from
// `a_member_over_a_half_phase_is_admitted` in the phase alone.

use arvo_format::Member;
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

struct Probe;

impl Format for Probe {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 0);
}

const MEMBER: Member<Probe> = Member::constant::<0>();

fn main() {
    let _ = MEMBER.slot();
}
