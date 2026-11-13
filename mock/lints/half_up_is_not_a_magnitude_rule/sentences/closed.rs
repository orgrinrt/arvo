//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The pairs that stood in the catalogue until the reader learned to read them.
//!
//! Each was a pair the reader got wrong on one named side, carried in
//! `catalogue.rs` as a known gap. They are here in the words they were
//! catalogued in, each with its own test asserting both sides, so a reader
//! change that reopens one of them turns that one test red and names it. The
//! word lists each fix rests on are walked whole in `shape_tests.rs`, since a
//! pair is one word of a list.
//!
//! They are kept apart from `corpus.rs` because that file's count is what says
//! whether one of its pairs was dropped, and these are seventeen more.

use super::{Pair, wrong_sides};

/// The closed pairs, in the order the catalogue held them.
pub(super) const CLOSED: &[Pair] = &[
    Pair {
        fires:    "`half_up` is nearest. Round every tie away from zero.",
        silent:   "`half_up` is nearest. MATLAB's Round takes every tie away from zero.",
        subject:  None,
        differs:  "whether `Round` opening the clause is an imperative or the MATLAB method",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is the mode. Fix the doc so a tie goes away from zero.",
        silent:   "`half_up` is the mode. MATLAB's Fix sends a tie away from zero.",
        subject:  None,
        differs:  "whether `Fix` opening the clause is an imperative or the MATLAB method",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is the mode. Ceiling aside, a tie goes away from zero.",
        silent:   "`half_up` is the mode. Ceiling sends a tie away from zero.",
        subject:  None,
        differs:  "whether `Ceiling` opens an aside or is the subject that sends the tie",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest. Round to it and a tie goes away from zero.",
        silent:   "`half_up` is nearest. Round is what goes away from zero.",
        subject:  None,
        differs:  "whether `Round` is a verb taking the mode as its object or the method",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. `-2.5` goes away from zero.",
        silent:   "`half_up` is a mode. `arvo-format` sends a tie away from zero.",
        subject:  None,
        differs:  "whether the code span opening the clause is a value the mode acts on or a \
                  subject of its own",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest, whereas it goes away from zero.",
        silent:   "`half_up` is nearest, whereas `roundTiesToAway` goes away from zero.",
        subject:  None,
        differs:  "whether the contrasted segment names the alias, under a name that is also a \
                  reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest, which goes away from zero.",
        silent:   "`half_up` is not Java's rule, which goes away from zero.",
        subject:  None,
        differs:  "whether the relative binds to the mode or to a rule named in words",
        measured: false,
    },
    Pair {
        fires:    "`half_up(x) = floor(x + q/2)` is stated first. It then answers ties away \
                  from zero.",
        silent:   "Seat 125 stated `half_up(x) = floor(x + q/2)` first. Seat 270 read blind \
                  and first answered ties away from zero.",
        subject:  None,
        differs:  "whether the clause after the naming opens on a pronoun or on a subject named \
                  in words, which is the shape of the governing ruling's `promotion`",
        measured: false,
    },
    Pair {
        fires:    "`half_up` sends a tie away from zero rather than toward positive infinity.",
        silent:   "`half_up` sends a tie toward positive infinity rather than away from zero.",
        subject:  None,
        differs:  "whether the reading or the denotation stands last before `rather than` in \
                  the reading's own segment",
        measured: false,
    },
    Pair {
        fires:    "`half_up` goes toward positive infinity, away from zero.",
        silent:   "`half_up` sends a tie toward positive infinity, where ties away from zero \
                  would give -3.",
        subject:  None,
        differs:  "whether the reading follows the denotation by a bare comma or by a comma \
                  introducing a contrast",
        measured: false,
    },
    Pair {
        fires:    "`half_up` rounds x (a tie) away from zero.",
        silent:   "The modes floor, away from zero, half_up (the ruled one), and ceil.",
        subject:  None,
        differs:  "whether a one-word aside between the name and the reading is read as a list \
                  neighbour or the item beside it holds a real list",
        measured: false,
    },
    Pair {
        fires:    "`half_up` doesn't send a tie toward zero; it sends it away from zero.",
        silent:   "`half_up` differs from `TowardZero`; it sends a tie away from zero.",
        subject:  None,
        differs:  "whether `toward zero` names the alias `TowardZero` in a code span or merely \
                  uses the ordinary English words",
        measured: false,
    },
    Pair {
        fires:    "`half_up` differs from `floor`, and it sends a tie away from zero.",
        silent:   "`half_up` differs from `floor`. It sends a tie away from zero.",
        subject:  None,
        differs:  "whether the pronoun opens a segment joined by a comma and a conjunction \
                  within one clause, where the subject governs, or opens the next clause after \
                  a full stop, where the name standing last does",
        measured: false,
    },
    Pair {
        fires:    "`half_up` returns the `away from zero` value at a tie.",
        silent:   "`half_up` has the `away from zero` value in it.",
        subject:  None,
        differs:  "whether `value` follows the span as a predicate the sentence asserts or as a \
                  word saying the span is quoted",
        measured: false,
    },
    Pair {
        fires:    "`half_up` moves a tie up in magnitude.",
        silent:   "`half_up` has an error that increases in magnitude with the width.",
        subject:  None,
        differs:  "whether the direction word stands in front of `in magnitude` as the tie's \
                  own direction or names how an error grows",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a nearest rule that reads nothing and so falls on the \
                  translation side, where `half_up` itself sends a tie away from zero.",
        silent:   "`half_up` is a nearest rule that reads nothing and so falls on the \
                  translation side, where the ties-away rule would fall on the reflection \
                  side.",
        subject:  None,
        differs:  "whether the segment names the mode itself or a rule named by a word that is \
                  also a reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. It sends a tie away from zero.",
        silent:   "`half_up` is a mode. The crate sends a tie away from zero.",
        subject:  None,
        differs:  "whether the clause opens with a pronoun for the mode or names a subject of \
                  its own in words, neither a claim about what the mode denotes",
        measured: false,
    },
];

#[test]
fn seventeen_pairs_closed() {
    // A count, so a pair dropped rather than kept is a failure here.
    assert_eq!(CLOSED.len(), 17);
}

/// One closed pair, as a test asserting the reader gets both of its sides
/// right.
macro_rules! closed_case {
    ($name:ident, $i:expr) => {
        #[test]
        fn $name() {
            let wrong = wrong_sides(std::iter::once(&CLOSED[$i]));
            assert!(
                wrong.is_empty(),
                "CLOSED[{}] reads wrong again: {wrong:?}",
                $i
            );
        }
    };
}

closed_case!(round_opening_a_clause_on_an_object_is_an_imperative, 0);
closed_case!(fix_opening_a_clause_on_an_object_is_an_imperative, 1);
closed_case!(ceiling_opening_an_aside_is_not_the_clauses_subject, 2);
closed_case!(round_as_a_verb_taking_the_mode_is_not_the_method, 3);
closed_case!(a_code_span_value_opening_a_clause_is_not_a_subject, 4);
closed_case!(the_alias_named_under_a_reading_word_is_a_name, 5);
closed_case!(a_relative_binds_to_a_rule_the_mode_is_denied_to_be, 6);
closed_case!(a_clause_opening_on_a_seat_has_a_subject_of_its_own, 7);
closed_case!(
    a_negator_denying_the_denotation_behind_the_reading_leaves_it,
    8
);
closed_case!(a_denotation_excuses_only_a_reading_set_against_it, 9);
closed_case!(a_parenthetical_beside_the_reading_is_not_a_list_item, 10);
closed_case!(toward_zero_behind_a_verbs_object_is_a_direction, 11);
closed_case!(a_pronoun_in_one_clause_binds_past_a_denied_rule, 12);
closed_case!(value_behind_a_verb_of_giving_is_what_the_mode_gives, 13);
closed_case!(in_magnitude_nearest_an_error_is_the_errors, 14);
closed_case!(the_ties_away_rule_named_in_words_is_a_name, 15);
closed_case!(the_crate_named_as_subject_has_the_clause, 16);
