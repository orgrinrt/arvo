// A member through the constant door over a phase of a half, admitted. It
// differs from `a_member_over_a_phase_that_does_not_denote_is_refused` in the
// phase alone.

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

    const PHASE: Phase = Phase::of(1, 2);
}

const MEMBER: Member<Probe> = Member::constant::<0>();

fn main() {
    let _ = MEMBER.slot();
}
