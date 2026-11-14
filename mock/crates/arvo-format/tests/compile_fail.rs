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
//! One pins the width bound on an outside range, which no impl set can hold: a
//! range from outside this crate is not refused by a missing impl, so the bound
//! in the obligation is the only thing between a sixty-five-bit declaration and
//! a crate that treats it as admitted.
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
//! One runs the other way. It pins the refusal the const generic parameters are
//! spelled around: a coordinate type cannot sit in that position, which is why
//! the declared widths carry a machine integer there and nowhere else.
//!
//! Five pin what addition refuses. Three are its obligation's three
//! refusals, each bound in a const item so the refusal is reached at check time,
//! and `is_addable` in the addition suite asks the same three questions from the
//! side that does not force them. The other two are the shape of the operation:
//! a format alone where a declared signature goes, and a host integer where a
//! slot goes.
//!
//! Three pin what multiplication does not admit, one pair for each condition a
//! consumer most plausibly reaches: a phase whose remainder denominator no exact
//! position holds, a magnitude-indexed quantum, and a range whose products the
//! slot index cannot carry. Each pair is a refused file and a twin that differs
//! from it in the offending token alone and compiles, so the refusal is pinned
//! to that token rather than to anything else in the file.
//!
//! The last three pin what forces the obligations in negation and subtraction,
//! in the same pairs: `negation_position` and `difference_position` over a
//! magnitude-indexed quantum, each against a constant one, and
//! `difference_position` over an outside range past the ladder against
//! `sum_position` over it, since the difference is the one step that forces the
//! slot range's own obligation. Deleting any of the three forcing lines turns a
//! case red.
//!
//! Five more pairs pin the quotient and the conversion: a constant divisor at the
//! zero member against one at one, `Divisor::of` over a range that holds the zero
//! against the same range starting at one, each operation over a
//! magnitude-indexed quantum against a constant one, and a conversion whose
//! remainder's denominator is `2^128` against one at `2^127`. After them one file
//! that has to compile names every item those two operations add from the crate
//! root, as an outside crate with no feature line would.
//!
//! Five more pin the member: a member constant one past the top against one at
//! the top and one below the bottom against one at the bottom, the adapted door
//! over a magnitude-indexed quantum against a constant one, the constant door
//! over a phase that does not denote against a half, and `Divisor::member` from
//! a range holding its zero slot, from an exponent below zero and one above it,
//! from a phase a half and a whole slot apart and from a range past the
//! divisor's, each against the zero-free range on the divisor's grid,
//! with the same range at radix ten admitted beside it, and from a range in
//! tenths against one in halves beside a divisor in halves.

#[test]
fn a_width_the_slot_range_cannot_carry_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/width_above_the_bound.rs");
}

/// The width bound on an outside range, which no impl set can hold.
///
/// A range from outside this crate is not refused by a missing impl, so the
/// bound in the obligation is the only thing between a sixty-five-bit
/// declaration and a crate that treats it as admitted.
#[test]
fn a_range_past_the_ladder_is_refused_where_it_is_used() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_range_past_the_ladder_is_refused_where_it_is_used.rs");
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
/// MathWorks documents a `fi` word length to 65535 and the ladder stops at 64,
/// so an ordinary declaration has no format. That refusal is what the standards
/// bound is for, and a refusal nothing pins can be deleted by accident, so it
/// lives here rather than in a paragraph.
#[test]
fn a_word_length_past_the_slot_ladder_is_refused_at_the_declaration() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/word_length_past_the_ladder.rs");
}

/// The IEEE 754 formats whose significand the ladder does not carry.
///
/// Binary128 and decimal128 each need 114 bits with the sign. The standard's
/// own parameters name them, so they are ordinary declarations, and the refusal
/// is the slot range's width bound met where the significand is used.
#[test]
fn the_128_bit_interchange_formats_are_refused_where_used() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/binary128_past_the_ladder.rs");
    t.compile_fail("tests/ui/decimal128_past_the_ladder.rs");
}

/// A sign and magnitude significand's own obligation: a signed ambient and a
/// precision of at least one, each refused where the range is read.
#[test]
fn a_significand_that_is_not_sign_and_magnitude_is_refused_where_used() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/an_unsigned_interchange_is_refused.rs");
    t.compile_fail("tests/ui/a_precision_of_zero_is_refused.rs");
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

/// Addition over the magnitude-indexed family, refused by the obligation.
///
/// The applied map carries no magnitude coordinate, so a sum of two members at
/// two quanta has no position to be handed as.
#[test]
fn addition_over_a_float_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/addition_over_a_float_is_refused.rs");
}

/// Addition over an outside format whose sum leaves the slot coordinate.
///
/// No shipped format reaches it, which is why the case declares its own: a
/// refusal only an outside format can reach is still a refusal the obligation
/// makes.
#[test]
fn addition_over_an_uncarried_sum_is_refused() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/addition_over_an_uncarried_sum_is_refused.rs");
}

/// Addition over a phase that does not denote, refused by design, and the same
/// declaration over the least denominator, whose remainder of `2^63` an exact
/// position holds, admitted.
#[test]
fn addition_over_a_phase_that_does_not_denote_is_refused_and_one_reduced_to_2_63_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/addition_over_a_phase_that_does_not_denote_is_refused.rs");
    t.pass("tests/ui/addition_over_a_phase_reduced_to_two_to_the_63_is_admitted.rs");
}

/// A format where a declared signature goes.
#[test]
fn a_format_is_not_a_signature() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_format_is_not_a_signature.rs");
}

/// A host integer where an operand goes.
#[test]
fn a_host_integer_is_not_an_operand() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_host_integer_is_not_an_operand.rs");
}

/// Multiplication over a phase whose remainder denominator no exact position
/// holds, not admitted, and the same declaration at half that phase
/// denominator, admitted.
#[test]
fn multiplication_over_a_remainder_no_position_holds_is_refused_and_one_it_holds_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/multiplication_over_a_remainder_no_position_holds_is_refused.rs");
    t.pass("tests/ui/multiplication_over_a_remainder_a_position_holds_is_admitted.rs");
}

/// Multiplication over a magnitude-indexed quantum, refused, and over a constant
/// quantum at the same exponent, admitted.
#[test]
fn multiplication_over_a_magnitude_indexed_quantum_is_refused_and_a_constant_one_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/multiplication_over_a_magnitude_indexed_quantum_is_refused.rs");
    t.pass("tests/ui/multiplication_over_a_constant_quantum_is_admitted.rs");
}

/// Multiplication over the unsigned 64-bit range, whose top corner is past the
/// slot index, admitted exactly and under saturation, clamping and wrapping;
/// and the quotient and the conversion past the index, admitted. No obligation
/// reads the index, so a refusal here would be a condition the design does not
/// have.
#[test]
fn a_position_past_the_slot_index_is_admitted_under_every_policy() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/multiplication_over_the_unsigned_64_bit_range_is_admitted.rs");
    t.pass("tests/ui/multiplication_saturating_past_the_index_is_admitted.rs");
    t.pass("tests/ui/multiplication_clamping_past_the_index_is_admitted.rs");
    t.pass("tests/ui/multiplication_wrapping_past_the_index_is_admitted.rs");
    t.pass("tests/ui/division_past_the_slot_index_is_admitted.rs");
    t.pass("tests/ui/conversion_past_the_slot_index_is_admitted.rs");
}

/// Negation over a magnitude-indexed quantum, refused because `negation_position`
/// forces addition's obligation, and over a constant quantum, admitted.
#[test]
fn negation_over_a_magnitude_indexed_quantum_is_refused_and_a_constant_one_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/negation_over_a_magnitude_indexed_quantum_is_refused.rs");
    t.pass("tests/ui/negation_over_a_constant_quantum_is_admitted.rs");
}

/// Subtraction over a magnitude-indexed quantum, refused because
/// `difference_position` forces addition's obligation, and over a constant
/// quantum, admitted.
#[test]
fn subtraction_over_a_magnitude_indexed_quantum_is_refused_and_a_constant_one_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/subtraction_over_a_magnitude_indexed_quantum_is_refused.rs");
    t.pass("tests/ui/subtraction_over_a_constant_quantum_is_admitted.rs");
}

/// Subtraction over an outside range past the ladder, refused because
/// `difference_position` alone forces the slot range's obligation, and addition
/// over the same declaration, admitted.
#[test]
fn subtraction_over_a_range_past_the_ladder_is_refused_and_addition_over_it_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/subtraction_over_a_range_past_the_ladder_is_refused.rs");
    t.pass("tests/ui/addition_over_a_range_past_the_ladder_is_admitted.rs");
}

/// A constant divisor at the zero member, refused where the constant is named,
/// and at one, admitted.
#[test]
fn division_by_a_constant_zero_is_refused_and_by_a_constant_one_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/division_by_a_constant_zero_is_refused.rs");
    t.pass("tests/ui/division_by_a_constant_one_is_admitted.rs");
}

/// A divisor read at run time over a range that leaves the zero out, refused at
/// a whole phase, where a non-member slot still names the zero, and admitted at
/// a half-offset phase, where none does.
#[test]
fn a_runtime_divisor_is_refused_at_a_whole_phase_and_admitted_at_a_half_offset_one() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_runtime_divisor_over_a_whole_phase_is_refused.rs");
    t.pass("tests/ui/a_runtime_divisor_over_a_half_offset_phase_is_admitted.rs");
}

/// A member constant one past the top of its range, refused where the constant
/// is named, and at the top, admitted.
#[test]
fn a_member_constant_past_the_top_is_refused_and_at_the_top_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_member_constant_past_the_top_is_refused.rs");
    t.pass("tests/ui/a_member_constant_at_the_top_is_admitted.rs");
}

/// A member constant one below the bottom of its range, refused where the
/// constant is named, and at the bottom, admitted.
#[test]
fn a_member_constant_below_the_bottom_is_refused_and_at_the_bottom_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_member_constant_below_the_bottom_is_refused.rs");
    t.pass("tests/ui/a_member_constant_at_the_bottom_is_admitted.rs");
}

/// What both member doors owe of the format, one pair per door: the adapted door
/// over a magnitude-indexed quantum against a constant one, and the constant door
/// over a phase that does not denote against a half.
#[test]
fn a_member_is_refused_over_a_magnitude_indexed_quantum_or_a_phase_that_does_not_denote() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_member_over_a_magnitude_indexed_quantum_is_refused.rs");
    t.pass("tests/ui/a_member_over_a_constant_quantum_is_admitted.rs");
    t.compile_fail("tests/ui/a_member_over_a_phase_that_does_not_denote_is_refused.rs");
    t.pass("tests/ui/a_member_over_a_half_phase_is_admitted.rs");
}

/// A divisor taken from a member of a second format, refused for each condition
/// of the door, each file one token from the zero-free range on the divisor's
/// grid, which is admitted, as is the same range at radix ten, where at exponent
/// zero the quantum is one. The radix is refused where it moves the quantum: a
/// range in tenths beside a divisor in halves, beside the same range in halves.
#[test]
fn a_member_divisor_is_refused_off_the_grid_past_the_range_or_over_its_zero() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/a_member_divisor_from_a_range_holding_its_zero_is_refused.rs");
    t.compile_fail("tests/ui/a_member_divisor_from_another_exponent_is_refused.rs");
    t.compile_fail("tests/ui/a_member_divisor_from_an_exponent_above_zero_is_refused.rs");
    t.compile_fail("tests/ui/a_member_divisor_from_another_phase_is_refused.rs");
    t.compile_fail("tests/ui/a_member_divisor_from_a_phase_a_whole_slot_apart_is_refused.rs");
    t.compile_fail("tests/ui/a_member_divisor_from_a_range_past_the_divisors_is_refused.rs");
    t.pass("tests/ui/a_member_divisor_from_a_zero_free_range_on_the_grid_is_admitted.rs");
    t.pass("tests/ui/a_member_divisor_from_another_radix_at_exponent_zero_is_admitted.rs");
    t.compile_fail("tests/ui/a_member_divisor_in_tenths_beside_halves_is_refused.rs");
    t.pass("tests/ui/a_member_divisor_in_halves_beside_halves_is_admitted.rs");
}

/// Division over a magnitude-indexed quantum, refused, and over a constant
/// quantum at the same exponent, admitted.
#[test]
fn division_over_a_magnitude_indexed_quantum_is_refused_and_a_constant_one_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/division_over_a_magnitude_indexed_quantum_is_refused.rs");
    t.pass("tests/ui/division_over_a_constant_quantum_is_admitted.rs");
}

/// Conversion from a magnitude-indexed quantum, refused, and from a constant
/// quantum at the same exponent, admitted.
#[test]
fn conversion_over_a_magnitude_indexed_quantum_is_refused_and_a_constant_one_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/conversion_over_a_magnitude_indexed_quantum_is_refused.rs");
    t.pass("tests/ui/conversion_over_a_constant_quantum_is_admitted.rs");
}

/// Conversion whose remainder's denominator is `2^128`, refused, and `2^127`,
/// admitted.
#[test]
fn conversion_over_a_remainder_no_position_holds_is_refused_and_one_it_holds_admitted() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/conversion_over_a_remainder_no_position_holds_is_refused.rs");
    t.pass("tests/ui/conversion_over_a_remainder_a_position_holds_is_admitted.rs");
}

/// Every item the quotient and the conversion add, reached from the crate root
/// by an outside crate with no feature line.
#[test]
fn the_quotient_and_conversion_surface_is_reached_from_the_root() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/the_quotient_and_conversion_surface_is_reached_from_the_root.rs");
}
