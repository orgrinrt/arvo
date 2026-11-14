//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What must not compile, each beside a twin that differs in the offending
//! token and does.
//!
//! The refusals that are failed constants surface at `cargo build`, not at
//! `cargo check`, since a constant is evaluated where it is used; `trybuild`
//! builds each file and holds the diagnostic, so a refusal cannot go away
//! without a test saying so. A twin that stopped compiling would make its pair
//! say nothing, which is why each pair is one test.

fn pair(refused: &str, twin: &str) {
    let t = trybuild::TestCases::new();
    t.compile_fail(format!("tests/ui/{refused}.rs"));
    t.pass(format!("tests/ui/{twin}.rs"));
}

#[test]
fn a_width_of_zero_bits_is_refused() {
    pair(
        "a_width_of_zero_bits_is_refused",
        "a_width_of_one_bit_compiles",
    );
}

#[test]
fn a_width_past_sixty_four_bits_is_refused() {
    pair(
        "a_width_past_sixty_four_bits_is_refused",
        "a_width_of_sixty_four_bits_compiles",
    );
}

#[test]
fn a_constant_outside_the_range_is_refused() {
    pair(
        "a_constant_outside_the_range_is_refused",
        "a_constant_at_the_end_of_the_range_compiles",
    );
}

#[test]
fn a_sum_across_two_signatures_is_refused() {
    pair(
        "a_sum_across_two_signatures_is_refused",
        "a_sum_within_one_signature_compiles",
    );
}

#[test]
fn a_host_integer_where_a_value_goes_is_refused() {
    pair(
        "a_host_integer_where_a_value_goes_is_refused",
        "a_value_where_a_value_goes_compiles",
    );
}

#[test]
fn a_value_where_a_host_integer_goes_is_refused() {
    pair(
        "a_value_where_a_host_integer_goes_is_refused",
        "the_slot_where_a_slot_goes_compiles",
    );
}

#[test]
fn the_field_named_from_outside_is_refused() {
    pair(
        "the_field_named_from_outside_is_refused",
        "the_slot_named_from_outside_compiles",
    );
}

#[test]
fn clamp_is_refused() {
    pair("clamp_is_refused", "saturate_compiles");
}

#[test]
fn stochastic_is_refused() {
    pair("stochastic_is_refused", "half_even_compiles");
}

#[test]
fn a_phased_format_is_refused() {
    pair(
        "a_phased_format_is_refused",
        "a_format_at_zero_phase_compiles",
    );
}

#[test]
fn the_identity_of_a_format_without_one_is_refused() {
    pair(
        "the_identity_of_a_format_without_one_is_refused",
        "the_identity_of_a_format_with_one_compiles",
    );
}

#[test]
fn a_difference_across_two_signatures_is_refused() {
    pair(
        "a_difference_across_two_signatures_is_refused",
        "a_difference_within_one_signature_compiles",
    );
}

#[test]
fn a_product_past_the_four_limbs_is_refused() {
    pair(
        "a_product_past_the_four_limbs_is_refused",
        "a_product_inside_the_four_limbs_compiles",
    );
}

#[test]
fn the_one_of_a_format_without_one_is_refused() {
    pair(
        "the_one_of_a_format_without_one_is_refused",
        "the_one_of_a_format_with_one_compiles",
    );
}

#[test]
fn an_override_naming_clamp_is_refused() {
    pair(
        "an_override_naming_clamp_is_refused",
        "an_override_naming_saturate_compiles",
    );
}

#[test]
fn an_override_naming_stochastic_is_refused() {
    pair(
        "an_override_naming_stochastic_is_refused",
        "an_override_naming_half_even_compiles",
    );
}

#[test]
fn a_crossing_to_another_format_is_refused() {
    pair(
        "a_crossing_to_another_format_is_refused",
        "a_crossing_to_a_sibling_compiles",
    );
}

#[test]
fn the_index_of_a_signed_range_as_an_unsigned_integer_is_refused() {
    pair(
        "the_index_of_a_signed_range_as_an_unsigned_integer_is_refused",
        "the_index_of_a_signed_range_as_a_signed_integer_compiles",
    );
}

#[test]
fn the_index_of_nine_bits_as_a_byte_is_refused() {
    pair(
        "the_index_of_nine_bits_as_a_byte_is_refused",
        "the_index_of_nine_bits_as_sixteen_compiles",
    );
}

#[test]
fn an_index_from_a_128_bit_integer_is_refused() {
    pair(
        "an_index_from_a_128_bit_integer_is_refused",
        "an_index_from_a_64_bit_integer_compiles",
    );
}

#[test]
fn the_carrier_by_projection_is_refused() {
    pair(
        "the_carrier_by_projection_is_refused",
        "the_slot_by_a_generic_consumer_compiles",
    );
}

#[test]
fn the_carrier_by_a_fully_qualified_projection_is_refused() {
    pair(
        "the_carrier_by_a_fully_qualified_projection_is_refused",
        "the_slot_by_a_fully_qualified_path_compiles",
    );
}

#[test]
fn the_carrier_map_by_path_is_refused() {
    pair(
        "the_carrier_map_by_path_is_refused",
        "the_slot_by_path_compiles",
    );
}

#[test]
fn the_machine_integers_by_path_is_refused() {
    pair(
        "the_machine_integers_by_path_is_refused",
        "the_slot_by_path_compiles",
    );
}

#[test]
fn an_outside_slot_family_over_part_of_a_range_is_refused() {
    pair(
        "an_outside_slot_family_over_part_of_a_range_is_refused",
        "an_outside_slot_family_over_a_whole_range_compiles",
    );
}

#[test]
fn an_outside_policy_that_clamps_is_refused() {
    pair(
        "an_outside_policy_that_clamps_is_refused",
        "an_outside_policy_that_wraps_compiles",
    );
}

#[test]
fn a_disarmed_row_naming_the_wrong_container_is_refused() {
    pair(
        "a_disarmed_row_naming_the_wrong_container_is_refused",
        "a_disarmed_row_naming_the_right_container_compiles",
    );
}
