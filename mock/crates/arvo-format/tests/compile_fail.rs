//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What must not compile.
//!
//! A refusal is a build failure, and a build failure is expressible: `trybuild`
//! holds the offending file and asserts the diagnostic. Without this the bound is
//! a thing that happens to be true of the current source rather than a thing
//! anything checks.
//!
//! The first case was previously a runtime test asserting a constant against the
//! literal its own definition set, whose doc argued a refusal could not be
//! asserted from inside a running test. That argument was wrong and this is what
//! it should have been.
//!
//! Three of the rest pin what the coordinates buy. Each was well-typed while the
//! contract was spelled in the host's own types, so none of them could have been
//! written down before, and each names a way a caller could have been wrong with
//! nothing to say so.
//!
//! Two more pin what a quantum law owes. Both conditions were stated where nothing
//! held them, so both of those constructions used to compile and answer, and a
//! runtime test cannot catch either because an obligation refuses rather than
//! returning.
//!
//! One pins where the slot ladder stops, against a word length an industrial
//! convention admits and this stack has no format for. A refusal that lives only
//! in a paragraph can be deleted by accident.
//!
//! Three pin which verbs force an obligation, from the refusing side, which is
//! the half the runtime arms cannot reach: the two verbs that force the format's
//! own, and the slot range reached through a call rather than forced. Each of
//! those was a claim about a set, and a claim about a set of two needs both of
//! its members pinned or deleting one member leaves the suite green.
//!
//! The last one runs the other way. It pins the refusal the const generic
//! parameters are spelled around: a coordinate type cannot sit in that position,
//! which is why the declared widths carry a machine integer there and nowhere
//! else.

#[test]
fn a_width_the_slot_range_cannot_carry_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/width_above_the_bound.rs");
}

#[test]
fn a_host_integer_at_a_coordinate_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_host_integer_is_not_a_slot.rs");
}

#[test]
fn an_index_and_an_extent_do_not_convert_into_each_other() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/an_index_is_not_an_extent.rs");
}

#[test]
fn the_two_ratios_are_two_coordinates() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_phase_is_not_a_fraction.rs");
}

/// A law over no magnitudes, refused where it is forced.
///
/// The condition lived in a doc comment on `MAGNITUDES` and nothing held it, so
/// this construction compiled and two functions in one file then disagreed about
/// whether zero was in its set.
#[test]
fn a_quantum_law_over_no_magnitudes_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_law_over_no_magnitudes_is_refused.rs");
}

/// A step law whose exponent leaves the exponent, refused the same way.
///
/// This condition was written nowhere at all, so the arithmetic wrapped in the
/// exponent's own width and the crate answered with a law it does not have.
#[test]
fn a_step_law_that_runs_off_the_exponent_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_step_law_that_runs_off_the_exponent_is_refused.rs");
}

/// The word lengths an industrial convention admits and the ladder does not.
///
/// MathWorks documents a `fi` word length to 65535 and the ladder stops at 62,
/// so an ordinary declaration has no format. That refusal is what the standards
/// bound is for, and a refusal nothing pins can be deleted by accident, so it
/// lives here rather than in a paragraph.
#[test]
fn a_word_length_past_the_slot_ladder_is_refused_at_the_declaration() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/word_length_past_the_ladder.rs");
}

/// Which verbs force the format's own obligation, pinned from the refusing side.
///
/// Three runtime arms pin the routes that reach a value without meeting it, and
/// nothing pinned the positive half, which is the claim that turned out to be
/// wrong: the design named `apply`, which forces the slot range's obligation and
/// never this one. `has_additive_identity` is one of the two functions that do.
#[test]
fn the_identity_search_forces_the_format_obligation() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/the_identity_search_forces_the_format_obligation.rs");
}

/// The other forcing verb, which nothing held.
///
/// Two functions force the format's obligation and one arm pinned one of them,
/// so deleting the force site from `cancelling_slot` left the suite green. A
/// claim about a set of two needs both of its members pinned.
#[test]
fn the_cancelling_slot_search_forces_the_format_obligation() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/cancelling_slot_forces_the_format_obligation.rs");
}

/// The slot range reached through a call rather than forced directly.
///
/// `has_additive_identity` reaches `slot_in_range` only on a magnitude where
/// `cancelling_slot` answers `Is`, so in a const evaluation a phase that cancels
/// decides whether the slot range's obligation is met at all. The other half, the
/// same verb over the same inverted range under a phase that cancels nowhere,
/// builds and is a const item beside the obligation arms.
#[test]
fn a_cancelling_phase_reaches_the_slot_range_and_a_non_cancelling_one_does_not() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_cancelling_phase_reaches_the_slot_range.rs");
}

/// The refusal every const generic parameter in this crate is spelled around.
///
/// `Width` is the crate's own count of bits and cannot be the type of a const
/// generic parameter, which is the reason the declared widths carry a machine
/// integer instead. Established once by compiling a file by hand, and a hand
/// check that answers and then goes away leaves the next reader to redo it.
#[test]
fn an_arvo_type_as_a_const_parameter_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/an_arvo_type_as_a_const_parameter.rs");
}
