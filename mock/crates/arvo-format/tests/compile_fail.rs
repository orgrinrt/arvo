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
