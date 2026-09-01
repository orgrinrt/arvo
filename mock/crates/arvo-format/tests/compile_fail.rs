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
//! The case here was previously a runtime test asserting a constant against the
//! literal its own definition set, whose doc argued a refusal could not be
//! asserted from inside a running test. That argument was wrong and this is what
//! it should have been.

#[test]
fn a_width_the_slot_range_cannot_carry_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/width_above_the_bound.rs");
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
