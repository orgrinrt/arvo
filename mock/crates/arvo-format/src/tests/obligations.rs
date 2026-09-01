//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What an implementor of each open contract owes, and the constructions that do
//! not supply it.
//!
//! A new instance earns admission by supplying the concept's obligations, and
//! closing the concept while opening the inventory is what makes admission a
//! check rather than a negotiation. Four contracts here are open, so four carry
//! an obligation.
//!
//! **Each obligation is asserted twice and the two halves are different tests.**
//! The verdict form is asserted here, at run time, against a construction that
//! compiles and is wrong; the refusal itself is a `compile_fail` doctest on the
//! `ADMITTED` const, because a refusal is a build failure and a build failure is
//! not expressible from inside a running test.
//!
//! **The refusal cannot be a `trybuild` case and that is not a preference.**
//! `trybuild` runs `cargo check`, the obligation is a const evaluated at
//! monomorphisation, and `check` skips codegen, so a `trybuild` case would report
//! a refused program as compiling. Measured rather than assumed:
//! `rustc --emit=metadata` accepts a program whose obligation fails while the
//! same invocation catches an ordinary type error, and a full build refuses it
//! with `E0080`. A doctest builds a binary, so it reaches the evaluation.

use crate::ambient::{
    is_admissible_ambient, Ambient, BinaryRationals, DecimalRationals, UnsignedBinaryRationals,
};
use crate::format::{is_admissible_format, Format};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{is_admissible_quantum, Constant, Indexed, Quantum};
use crate::slots::{Signed, Slots, Unsigned};
use crate::width::Width;

// --- the constructions that compile and are wrong ----------------------------

/// A quantum law over no magnitudes, which parameterises the empty set.
///
/// It compiles, which is the point: the trait is open and nothing stops it being
/// written. Kept permanently rather than built to look at and deleted.
pub struct NoMagnitudes;

impl Quantum for NoMagnitudes {
    const BASE: i32 = 0;
    const SLOPE: i32 = 0;
    const MAGNITUDES: u32 = 0;
}

/// A domain declaring a radix of one, which is not a positional notation.
///
/// At radix one the step never changes with the exponent, so every magnitude
/// names the same value and the quantum law carries no information.
pub struct UnaryRationals;

impl Ambient for UnaryRationals {
    const RADIX: u32 = 1;
    const SIGNED: bool = true;
}

/// A domain declaring a radix of zero, where the quantum is not a number at a
/// negative exponent at all.
pub struct NullaryRationals;

impl Ambient for NullaryRationals {
    const RADIX: u32 = 0;
    const SIGNED: bool = true;
}

/// A format whose phase denominator is zero, so its phase names no position.
pub struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE_NUM: i64 = 1;
    const PHASE_DEN: i64 = 0;
}

// --- the quantum law -----------------------------------------------------------

#[test]
fn the_law_rejects_a_quantum_over_no_magnitudes() {
    // `MAGNITUDES = 0` parameterises the empty set, and an empty set contains
    // nothing, the additive identity included. A predicate that reads only the
    // phase and the slot range has nothing to notice that with, which is what
    // makes this the case worth keeping.
    assert!(
        !is_admissible_quantum::<NoMagnitudes>(),
        "a law over no magnitudes was admitted"
    );
}

#[test]
fn the_law_admits_every_quantum_this_crate_ships() {
    // The control. A law that rejected everything would pass the test above and
    // establish nothing.
    assert!(is_admissible_quantum::<Constant<0>>());
    assert!(is_admissible_quantum::<Constant<-14>>());
    assert!(is_admissible_quantum::<Indexed<-14, 30>>());
    assert!(is_admissible_quantum::<Indexed<0, 1>>());
}

#[test]
fn the_quantum_law_separates_the_two_constructions_rather_than_answering_one_way() {
    assert_ne!(
        is_admissible_quantum::<Constant<0>>(),
        is_admissible_quantum::<NoMagnitudes>(),
        "the law gives the same verdict to a shipped law and one over no magnitudes"
    );
}

// --- the ambient domain --------------------------------------------------------

#[test]
fn the_law_rejects_a_radix_below_two() {
    // Neither is a positional notation. At one every magnitude names the same
    // value and at zero the quantum is not a number at a negative exponent, so
    // the cancellation the additive identity turns on is undefined in both.
    assert!(
        !is_admissible_ambient::<UnaryRationals>(),
        "a radix of one was admitted"
    );
    assert!(
        !is_admissible_ambient::<NullaryRationals>(),
        "a radix of zero was admitted"
    );
}

#[test]
fn the_law_admits_every_domain_this_crate_ships() {
    assert!(is_admissible_ambient::<BinaryRationals>());
    assert!(is_admissible_ambient::<UnsignedBinaryRationals>());
    assert!(is_admissible_ambient::<DecimalRationals>());
}

#[test]
fn the_ambient_law_separates_the_two_constructions_rather_than_answering_one_way() {
    assert_ne!(
        is_admissible_ambient::<BinaryRationals>(),
        is_admissible_ambient::<UnaryRationals>(),
        "the law gives the same verdict to a shipped domain and a radix of one"
    );
}

// --- the format ----------------------------------------------------------------

#[test]
fn the_law_rejects_a_format_whose_phase_denominator_is_zero() {
    assert!(
        !is_admissible_format::<NoDenominator>(),
        "a phase denominator of zero was admitted, and it names no position on the grid"
    );
}

#[test]
fn the_law_admits_every_format_this_crate_ships() {
    assert!(is_admissible_format::<Integer<8>>());
    assert!(is_admissible_format::<UFixed<13, -4>>());
    assert!(is_admissible_format::<Biased<7, -2, 1>>());
    assert!(is_admissible_format::<Floating<11, -14, 30>>());
}

#[test]
fn the_format_law_separates_the_two_constructions_rather_than_answering_one_way() {
    assert_ne!(
        is_admissible_format::<Integer<8>>(),
        is_admissible_format::<NoDenominator>(),
        "the law gives the same verdict to a shipped format and a zero denominator"
    );
}

// --- every contract that is open carries one -----------------------------------

#[test]
fn every_open_contract_in_this_crate_states_its_obligation_as_a_check() {
    // The class rather than the four instances. Each verdict function exists and
    // separates a shipped instance from a wrong one, which is what says the
    // obligation is a check rather than a sentence asking for something.
    //
    // `Slots` is here through a shipped range because its wrong constructions
    // live beside the slot laws in the parent module, where the width bound they
    // are about is also asserted.
    assert!(
        is_admissible_ambient::<BinaryRationals>() && !is_admissible_ambient::<UnaryRationals>()
    );
    assert!(is_admissible_quantum::<Constant<0>>() && !is_admissible_quantum::<NoMagnitudes>());
    assert!(is_admissible_format::<Integer<8>>() && !is_admissible_format::<NoDenominator>());
    assert!(crate::slots::is_admissible::<Signed<8>>());
    assert!(crate::slots::is_admissible::<Unsigned<8>>());

    // And the width that is a coordinate rather than a bound, so the four are not
    // all one assertion wearing four names.
    assert_eq!(<Signed<8> as Slots>::WIDTH, Width::bits(8));
}
