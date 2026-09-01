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
//! The other three pin what the coordinates buy. Each was well-typed while the
//! contract was spelled in the host's own types, so none of them could have been
//! written down before, and each names a way a caller could have been wrong with
//! nothing to say so.

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
