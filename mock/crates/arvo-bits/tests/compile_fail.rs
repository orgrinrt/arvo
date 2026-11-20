//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What must not compile, and the two things beside it that must.
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
//! An index or an amount at or past the width is refused by the tier's one
//! guard, `Within::NEW`, which every method taking one goes through. Each of
//! the six is held at `Bits<8>` and eight in a constant, in a call at run time
//! through the inherent method, and in a call at run time through a function
//! generic over its trait; at the edges, `Bits<1>` at one and `Bits<64>` at
//! sixty-four; and past the width. A container written in the case with no
//! assertion of its own is refused past its width, and the two ways round the
//! guard, an impl of the caller's trait by hand and a proof built by hand, are
//! refused.
//!
//! All of it is one `TestCases` with cases that must pass. Without one,
//! `trybuild` runs `cargo check` rather than `cargo build`, and a refusal in a
//! call at run time, which only codegen reaches, would pass check and prove
//! nothing. The index mutant, set by `RUSTFLAGS`, never reaches these cases,
//! since `trybuild` removes `RUSTFLAGS` from its own build.

const BUILT: &[&str] = &[
    "every_index_below_the_width_builds",
    "a_container_of_its_own_builds_below_its_width",
    "a_packed_word_that_tiles_builds",
];

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
    "a_bit_at_one_of_one_bit_is_refused",
    "a_written_bit_at_one_of_one_bit_is_refused",
    "a_bit_at_sixty_four_of_sixty_four_bits_is_refused",
    "a_written_bit_at_sixty_four_of_sixty_four_bits_is_refused",
    "a_shift_up_by_sixty_four_of_sixty_four_bits_is_refused",
    "a_shift_down_by_sixty_four_of_sixty_four_bits_is_refused",
    "a_rotation_up_by_sixty_four_of_sixty_four_bits_is_refused",
    "a_rotation_down_by_sixty_four_of_sixty_four_bits_is_refused",
    "a_bit_at_the_largest_index_is_refused",
    "a_shift_down_past_the_width_is_refused",
    "a_container_of_its_own_past_its_width_is_refused",
    "a_container_of_its_own_past_its_width_in_a_constant_is_refused",
    "an_impl_of_the_callers_trait_by_hand_is_refused",
    "a_proof_built_by_hand_is_refused",
    "a_packed_word_whose_fields_fall_short_is_refused",
    "a_call_of_get_on_a_short_word_is_refused",
    "a_field_its_layout_does_not_list_is_refused",
    "a_call_of_with_on_a_field_not_listed_is_refused",
    "a_field_listed_twice_is_refused",
    "an_impl_of_the_fields_of_a_packed_word_by_hand_is_refused",
    "a_field_zero_bits_wide_is_refused",
    "a_field_sixty_five_bits_wide_is_refused",
    "a_container_zero_bits_wide_is_refused",
    "a_container_sixty_five_bits_wide_is_refused",
];

#[test]
fn what_must_not_compile_is_refused_at_build() {
    let t = trybuild::TestCases::new();
    for case in BUILT {
        t.pass(format!("tests/ui/{case}.rs"));
    }
    for case in REFUSED {
        t.compile_fail(format!("tests/ui/{case}.rs"));
    }
}
