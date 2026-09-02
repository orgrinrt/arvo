// A9. Not the assigned question. `proposal::the_additive_identity_is_decided_by_
// the_phase_being_a_whole_multiple_of_the_quantum` says in its `note` that the
// shipped `has_additive_identity` implements the superseded claim, that a nonzero
// phase leaves no identity. This arm reads the shipped function back at a whole
// step and at a half step, so the registry's claim about the source is checkable
// rather than remembered.
use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{has_additive_identity, Format, Phase};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

struct WholeStep;
impl Format for WholeStep {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 1);
}

struct HalfStep;
impl Format for HalfStep {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 2);
}

fn main() {
    let whole = has_additive_identity::<WholeStep>().get();
    let half = has_additive_identity::<HalfStep>().get();
    println!("A9 whole step (1/1, nonzero) = {whole}");
    println!("A9 half step  (1/2, nonzero) = {half}");
    assert!(whole, "the shipped function still implements the superseded claim");
    assert!(!half, "the control moved: a half step must carry no identity");
}
