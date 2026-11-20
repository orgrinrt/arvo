//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What must not compile, and the one thing beside it that must.
//!
//! `N` outside `1..=64` is refused at `cargo build`, not at `cargo check`: the
//! obligation is a const, evaluated where it is used, so a call reaches it at
//! codegen. A refusal that lives only in a paragraph can be deleted by
//! accident; `trybuild` holds the offending file and asserts the diagnostic.
//! Mirrors `arvo_format`'s own `tests/ui/cancelling_slot_forces_the_format_obligation.rs`
//! pattern: the call is public API, `ADMITTED` is forced internally, and the
//! panic surfaces as `E0080` at the `assert!` site.
//!
//! The doors onto the bound are `masked`, which forces it directly, `cast`,
//! which forces it by delegating to `masked` at the target width, and
//! `BitWidth::WIDTH`, which forces it before giving the width back.
//!
//! An index or an amount at the width is refused the same way, by an assertion
//! each of the six methods taking one forces itself. Each is held in a
//! constant, in a call at run time through the inherent method, and in a call
//! at run time through a function generic over its trait, which reaches the
//! assertion only through the trait's impl.
//!
//! All of it is one `TestCases` with one case that must pass. Without a case
//! that must pass, `trybuild` runs `cargo check` rather than `cargo build`,
//! and a refusal in a call at run time, which only codegen reaches, would pass
//! check and prove nothing.

const REFUSED: &[&str] = &[
    "a_width_of_zero_bits_is_refused",
    "a_width_past_sixty_four_bits_is_refused",
    "a_cast_to_zero_bits_is_refused",
    "a_cast_past_sixty_four_bits_is_refused",
    "the_width_of_zero_bits_is_refused",
    "the_width_past_sixty_four_bits_is_refused",
    "a_bit_at_the_width_is_refused",
    "a_written_bit_at_the_width_is_refused",
    "a_shift_by_the_width_is_refused",
    "a_shift_down_by_the_width_is_refused",
    "a_rotation_by_the_width_is_refused",
    "a_rotation_down_by_the_width_is_refused",
    "a_bit_at_the_width_through_the_trait_is_refused",
    "a_call_of_bit_at_the_width_is_refused",
    "a_call_of_with_bit_at_the_width_is_refused",
    "a_call_of_shift_up_at_the_width_is_refused",
    "a_call_of_shift_down_at_the_width_is_refused",
    "a_call_of_rotate_up_at_the_width_is_refused",
    "a_call_of_rotate_down_at_the_width_is_refused",
    "a_generic_call_of_bit_at_the_width_is_refused",
    "a_generic_call_of_with_bit_at_the_width_is_refused",
    "a_generic_call_of_shift_up_at_the_width_is_refused",
    "a_generic_call_of_shift_down_at_the_width_is_refused",
    "a_generic_call_of_rotate_up_at_the_width_is_refused",
    "a_generic_call_of_rotate_down_at_the_width_is_refused",
];

#[test]
fn what_must_not_compile_is_refused_at_build() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/every_index_below_the_width_builds.rs");
    for case in REFUSED {
        t.compile_fail(format!("tests/ui/{case}.rs"));
    }
}
