//! Arm G. Does the const call site have to be used?
//!
//! Arm C's const item is read in `main`. This one is not read at all. It
//! separates "a const item is evaluated at check time" from "a used const item
//! is", which is the difference between the obligation reaching every const call
//! site and reaching only the live ones.
//!
//! The control is arm C: it must refuse, or this arm is comparing against
//! nothing.

#![allow(dead_code)]

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase, has_additive_identity};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;
use arvo_format::width::Bool;

struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    const PHASE: Phase = Phase::of(1, 0);
}

const UNREAD: Bool = has_additive_identity::<NoDenominator>();

fn main() {
    println!("G: the unread const item did not stop this");
}
