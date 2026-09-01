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
