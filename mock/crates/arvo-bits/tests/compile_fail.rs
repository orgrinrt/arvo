//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What must not compile.
//!
//! `N` outside `1..=64` is refused at `cargo build`, not at `cargo check`: the
//! obligation is a const, evaluated where it is used, so a call reaches it at
//! codegen. A refusal that lives only in a paragraph can be deleted by
//! accident; `trybuild` holds the offending file and asserts the diagnostic.
//! Mirrors `arvo_format`'s own `tests/ui/cancelling_slot_forces_the_format_obligation.rs`
//! pattern: the call is public API, `ADMITTED` is forced internally, and the
//! panic surfaces as `E0080` at the `assert!` site.
//!
//! There are two public doors onto the obligation and both are held here.
//! `masked` forces it directly; `cast` forces it only by delegating to
//! `masked` at the target width, so its refusal is one delegation away from
//! disappearing under a refactor that nothing else would notice.

#[test]
fn a_width_of_zero_bits_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_width_of_zero_bits_is_refused.rs");
}

#[test]
fn a_width_past_sixty_four_bits_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_width_past_sixty_four_bits_is_refused.rs");
}

#[test]
fn a_cast_to_zero_bits_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_cast_to_zero_bits_is_refused.rs");
}

#[test]
fn a_cast_past_sixty_four_bits_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_cast_past_sixty_four_bits_is_refused.rs");
}
