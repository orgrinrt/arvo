//! Arm M. `contains`, the crate's own spelling of the ratified membership
//! predicate, against a format whose phase does not denote.
//!
//! `Format::ADMITTED` is forced at exactly two sites in the crate,
//! `format.rs:322` and `format.rs:398`, which are `cancelling_slot` and
//! `has_additive_identity`. `contains` (`format.rs:246`) is not one of them and
//! never reads `PHASE` at all.
//!
//! So the question is whether a produced binary can ask membership of a format
//! the contract calls inadmissible, and get an answer.
//!
//! Predicted: check, build and run all succeed, and `contains` answers `true`
//! for an in-range coordinate of a set whose grid names no position.
//!
//! The control is the same coordinate out of range, which must answer `false`,
//! or `contains` answers `true` regardless of what it is handed and the arm
//! measures nothing.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{contains, Format, Phase};
use arvo_format::quantum::{Constant, Magnitude};
use arvo_format::slots::{Signed, Slot};

struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 0);
}

fn main() {
    let inside = contains::<NoDenominator>(Slot::at(3), Magnitude::at(0)).get();
    // The control: a slot the range cannot hold.
    let outside = contains::<NoDenominator>(Slot::at(1 << 40), Magnitude::at(0)).get();

    assert!(!outside, "the control was admitted, so the arm measures nothing");
    println!(
        "M: contains on a non-denoting format answered in-range={inside} out-of-range={outside}, \
         and PHASE.denotes()={}",
        <NoDenominator as Format>::PHASE.denotes().get()
    );
}
