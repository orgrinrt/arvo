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
//! compiles and is wrong. The refusal itself is a build failure, which is not
//! expressible from inside a running test, so it lives in `tests/ui/` and in the
//! `compile_fail` doctests on each `ADMITTED`.
//!
//! **Which of those two can see a refusal depends on where the obligation is
//! forced, not on the tool.** An obligation is a const, and a const is evaluated
//! where it is used. Forced from a runtime call it is evaluated at codegen, which
//! `cargo check` skips, so a `trybuild` case written that way reports a refused
//! program as compiling and a doctest, which builds a binary, catches it. Forced
//! in a `const` item it is evaluated at check time, and then a `trybuild` case
//! sees it: `tests/ui/a_law_over_no_magnitudes_is_refused.rs` binds one and is
//! refused with `E0080` under `cargo check`.
//!
//! So both shapes ship, and each covers what the other cannot: the doctests cover
//! a declaration reached only through a runtime call, and the `trybuild` cases
//! cover the const-bound form with the exact diagnostic pinned.

use crate::ambient::{
    is_admissible_ambient, Ambient, BinaryRationals, DecimalRationals, Radix,
    UnsignedBinaryRationals,
};
use crate::format::{contains, has_additive_identity, is_admissible_format, Format, Phase};
use crate::points::{Biased, Floating, Integer, UFixed};
use crate::quantum::{
    is_admissible_quantum, Constant, Exponent, Indexed, Magnitude, MagnitudeCount, Quantum,
};
use crate::slots::{Signed, Slot, Slots, Unsigned};
use crate::width::{Bool, Width};

// --- the constructions that compile and are wrong ----------------------------

/// A quantum law over no magnitudes, which parameterises the empty set.
///
/// It compiles, which is the point: the trait is open and nothing stops it being
/// written. Kept permanently rather than built to look at and deleted.
pub struct NoMagnitudes;

impl Quantum for NoMagnitudes {
    const BASE: Exponent = Exponent::ZERO;
    const SLOPE: Exponent = Exponent::ZERO;
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(0);
}

/// A step law whose exponent leaves what an `Exponent` carries before it reaches
/// its largest magnitude.
///
/// The second condition the quantum contract states. Every coordinate here is one
/// an implementor can write, and the law names no quantum at the magnitudes it
/// claims.
pub struct ReachesPastTheExponent;

impl Quantum for ReachesPastTheExponent {
    const BASE: Exponent = Exponent::ZERO;
    const SLOPE: Exponent = Exponent::of(i32::MAX);
    const MAGNITUDES: MagnitudeCount = MagnitudeCount::of(4);
}

/// A domain declaring a radix of one, which is not a positional notation.
///
/// At radix one the step never changes with the exponent, so every magnitude
/// names the same value and the quantum law carries no information.
pub struct UnaryRationals;

impl Ambient for UnaryRationals {
    const RADIX: Radix = Radix::of(1);
    const SIGNED: Bool = Bool::TRUE;
}

/// A domain declaring a radix of zero, where the quantum is not a number at a
/// negative exponent at all.
pub struct NullaryRationals;

impl Ambient for NullaryRationals {
    const RADIX: Radix = Radix::of(0);
    const SIGNED: Bool = Bool::TRUE;
}

/// A format whose phase denominator is zero, so its phase names no position.
pub struct NoDenominator;

impl Format for NoDenominator {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 0);
}

// --- the quantum law -----------------------------------------------------------

#[test]
fn the_law_rejects_a_quantum_over_no_magnitudes() {
    // `MAGNITUDES = 0` parameterises the empty set, and an empty set contains
    // nothing, the additive identity included. A predicate that reads only the
    // phase and the slot range has nothing to notice that with, which is what
    // makes this the case worth keeping.
    assert!(
        !is_admissible_quantum::<NoMagnitudes>().get(),
        "a law over no magnitudes was admitted"
    );
}

#[test]
fn the_law_rejects_a_step_law_that_runs_off_the_exponent() {
    // The second condition, and it is a separate construction rather than the
    // same one read twice: this law ranges over four magnitudes, so it passes the
    // first condition outright, and it still names no quantum at its largest.
    assert!(
        !is_admissible_quantum::<ReachesPastTheExponent>().get(),
        "a law whose exponent leaves the type before its largest magnitude was admitted"
    );

    // The control that says the two conditions are two: this construction meets
    // the magnitude condition, so a verdict reading only that would admit it.
    assert!(<ReachesPastTheExponent as Quantum>::MAGNITUDES.count() >= 1);
}

#[test]
fn the_law_admits_every_quantum_this_crate_ships() {
    // The control. A law that rejected everything would pass the tests above and
    // establish nothing.
    assert!(is_admissible_quantum::<Constant<0>>().get());
    assert!(is_admissible_quantum::<Constant<-14>>().get());
    assert!(is_admissible_quantum::<Indexed<-14, 30>>().get());
    assert!(is_admissible_quantum::<Indexed<0, 1>>().get());
}

#[test]
fn the_quantum_law_separates_the_two_constructions_rather_than_answering_one_way() {
    assert_ne!(
        is_admissible_quantum::<Constant<0>>(),
        is_admissible_quantum::<NoMagnitudes>(),
        "the law gives the same verdict to a shipped law and one over no magnitudes"
    );
    assert_ne!(
        is_admissible_quantum::<Indexed<0, 4>>(),
        is_admissible_quantum::<ReachesPastTheExponent>(),
        "the law gives the same verdict to a shipped law and one that runs off the exponent"
    );
}

// --- the ambient domain --------------------------------------------------------

#[test]
fn the_law_rejects_a_radix_below_two() {
    // Neither is a positional notation. At one every magnitude names the same
    // value and at zero the quantum is not a number at a negative exponent, so
    // the cancellation the additive identity turns on is undefined in both.
    assert!(
        !is_admissible_ambient::<UnaryRationals>().get(),
        "a radix of one was admitted"
    );
    assert!(
        !is_admissible_ambient::<NullaryRationals>().get(),
        "a radix of zero was admitted"
    );
}

#[test]
fn the_law_admits_every_domain_this_crate_ships() {
    assert!(is_admissible_ambient::<BinaryRationals>().get());
    assert!(is_admissible_ambient::<UnsignedBinaryRationals>().get());
    assert!(is_admissible_ambient::<DecimalRationals>().get());
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
        !is_admissible_format::<NoDenominator>().get(),
        "a phase denominator of zero was admitted, and it names no position on the grid"
    );
}

#[test]
fn the_law_admits_every_format_this_crate_ships() {
    assert!(is_admissible_format::<Integer<8>>().get());
    assert!(is_admissible_format::<UFixed<13, -4>>().get());
    assert!(is_admissible_format::<Biased<7, -2, 1>>().get());
    assert!(is_admissible_format::<Floating<11, -14, 30>>().get());
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
    // live beside the slot laws in `the_inventory`, where the width bound they
    // are about is also asserted.
    assert!(
        is_admissible_ambient::<BinaryRationals>().get()
            && !is_admissible_ambient::<UnaryRationals>().get()
    );
    assert!(
        is_admissible_quantum::<Constant<0>>().get()
            && !is_admissible_quantum::<NoMagnitudes>().get()
    );
    assert!(
        is_admissible_format::<Integer<8>>().get()
            && !is_admissible_format::<NoDenominator>().get()
    );
    assert!(crate::slots::is_admissible::<Signed<8>>().get());
    assert!(crate::slots::is_admissible::<Unsigned<8>>().get());

    // And the width that is a coordinate rather than a bound, so the four are not
    // all one assertion wearing four names.
    assert_eq!(<Signed<8> as Slots>::WIDTH, Width::bits(8));
}

#[test]
fn every_verdict_returns_the_stacks_truth_value_rather_than_the_hosts() {
    // Structural, and it is the one property the four share that a runtime
    // assertion about their answers cannot reach: each returns a `Bool`, so a
    // caller writing one into a signature writes the stack's type. If a verdict
    // returned the host's, this stops compiling.
    let verdicts: [Bool; 4] = [
        is_admissible_ambient::<BinaryRationals>(),
        is_admissible_quantum::<Constant<0>>(),
        is_admissible_format::<Integer<8>>(),
        crate::slots::is_admissible::<Signed<8>>(),
    ];
    assert_eq!(verdicts, [Bool::TRUE; 4]);
}

// --- how far an obligation reaches, which is not as far as it reads ----------
//
// An obligation is a const and a const is evaluated where it is used, so the
// guarantee is exactly the set of verbs that use it. `Format::ADMITTED` is forced
// at two of them. The three arms below are the routes that reach a value without
// meeting either, each one a property the design now states outright and each one
// established by hand before it was written down here.
//
// **They pass, and that is what they are for.** A route the design admits exists
// is pinned by an arm that passes while it is open and fails the moment it
// closes, which is what makes a later round's closing visible rather than silent.

/// A format whose phase names no position on the grid.
///
/// `Phase::of(1, 0)` is stored as it was written, so this compiles and the
/// obligation on `Format` is what refuses it, at the two verbs that force it.
struct PhaseNamesNoPosition;

impl Format for PhaseNamesNoPosition {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 0);
}

/// The same format with the obligation written over, which any implementor may do.
struct DisarmedObligation;

impl Format for DisarmedObligation {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;
    const PHASE: Phase = Phase::of(1, 0);
    const ADMITTED: () = ();
}

#[test]
fn contains_answers_for_a_format_whose_phase_does_not_denote() {
    // `contains` is this crate's spelling of the membership predicate, and it
    // forces the quantum's obligation and the slot range's, through
    // `magnitude_in_range` and `slot_in_range`. What it never reaches is this
    // one, so a declaration inadmissible in its *phase* answers through it while
    // one inadmissible in its magnitudes or its slots is refused. The fixture is
    // the first kind, which is why the arm can run at all.
    //
    // The assertion is that it answers at all; which way it answers is the slot
    // and magnitude ranges' business and neither of those is what is wrong here.
    let answered = contains::<PhaseNamesNoPosition>(Slot::ZERO, Magnitude::SMALLEST);
    assert_eq!(answered, Bool::TRUE);

    // The control, and it is what makes the arm mean anything: the same call on a
    // format that does denote answers the same way, so the arm above is reporting
    // that the obligation was not consulted rather than that the phase was read.
    let control = contains::<Integer<8>>(Slot::ZERO, Magnitude::SMALLEST);
    assert_eq!(answered, control);
}

#[test]
fn a_coordinate_is_readable_off_the_impl_without_the_obligation_firing() {
    // The shortest route of the three. Nothing between a reader and the
    // declaration, so nothing to force.
    assert_eq!(
        <PhaseNamesNoPosition as Format>::PHASE.denotes(),
        Bool::FALSE
    );
    assert_eq!(<Integer<8> as Format>::PHASE.denotes(), Bool::TRUE);
}

#[test]
fn an_implementor_writes_over_the_obligation_and_the_forcing_verb_finds_nothing() {
    // Routed through a verb that forces, which is the whole content of the arm.
    // `contains` never reaches this obligation, so calling the two declarations
    // through it produces the same answer for a reason that has nothing to do
    // with the override, and an arm built that way passes identically whether
    // `ADMITTED` exists or not.
    //
    // `has_additive_identity` forces `<F as Format>::ADMITTED`. That this line
    // compiles at all is the assertion: the same call on `PhaseNamesNoPosition`
    // does not, which the `tests/ui` arm beside this file pins.
    assert_eq!(has_additive_identity::<DisarmedObligation>(), Bool::FALSE);

    // The control, and it is what stops the line above from being a fact about
    // `has_additive_identity` rather than about the override: a format that
    // leaves the default in place and does denote answers the other way, so the
    // verb is not simply returning `FALSE` for everything.
    assert_eq!(has_additive_identity::<Integer<8>>(), Bool::TRUE);

    // And the two are the same declaration apart from the obligation, so the arm
    // is about the override rather than about anything else that differs.
    assert_eq!(
        <DisarmedObligation as Format>::PHASE,
        <PhaseNamesNoPosition as Format>::PHASE
    );
}

#[test]
fn adapt_forces_the_slot_range_and_not_the_format() {
    use crate::adapt::{Adapt, Signature};
    use crate::apply::{adapt, Dither, Exact};
    use crate::overflow::Saturate;
    use crate::rounding::Floor;

    // The design said for a while that `apply` forces the format's obligation.
    // It does not: `adapt` forces `<<S::Format as Format>::Slots as Slots>::ADMITTED`
    // and reaches the format only through it. That this compiles and returns is
    // the proof, because the format underneath declares a phase that names no
    // position and the format's own obligation refuses exactly that.
    type OverANonDenotingFormat = Signature<PhaseNamesNoPosition, Adapt<Floor, Saturate>>;
    let landed = adapt::<OverANonDenotingFormat>(Exact::on_grid(Slot::ZERO), Dither::UNUSED);

    // The control: the same call over a format that does denote lands on the same
    // slot, so the line above is reporting that the format's obligation was never
    // consulted rather than that this position is special.
    type OverADenotingFormat = Signature<Integer<8>, Adapt<Floor, Saturate>>;
    assert_eq!(
        landed,
        adapt::<OverADenotingFormat>(Exact::on_grid(Slot::ZERO), Dither::UNUSED)
    );
}
