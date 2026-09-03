// The two declarations every arm shares. `Broken` is the offending one: a phase
// of one over zero, which names no position on the grid. `Sound` is the control,
// identical in every coordinate but the denominator.
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

pub struct Broken;

impl Format for Broken {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 0);
}

pub struct Sound;

impl Format for Sound {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 2);
}
