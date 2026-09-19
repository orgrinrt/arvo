//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The clause reader, over every spelling, every reading and every escape, each
//! escape beside the same sentence without it.
//!
//! The sentences a review measured against the reader that stood before this one
//! are in `sentences.rs`, paired with their near-twins. What is here is the
//! vocabulary walked whole: every spelling against every reading, every negator,
//! every marker, every clause boundary.

use super::reading::{DIRECTIONS, ELSEWHERE, NEGATORS, QUALIFIED, READINGS, SPELLINGS, hits};

/// The readings found in `text`, as written.
fn found(text: &str) -> Vec<&'static str> {
    hits(text, None).into_iter().map(|h| h.reading).collect()
}

#[test]
fn every_spelling_with_every_reading_fires_once_and_names_both() {
    for s in SPELLINGS {
        for r in READINGS {
            // The magnitude reading is about where a tie goes only behind a
            // direction word, so the sentence carrying it has one.
            let direction = if *r == QUALIFIED { "up " } else { "" };
            let text = format!("The mode `{s}` takes a tie {direction}{r} here.");
            let h = hits(&text, None);
            assert_eq!(h.len(), 1, "{text}: {h:?}");
            assert_eq!(h[0].name, *s, "{text}");
            assert_eq!(h[0].reading, *r, "{text}");
            assert!(
                text[h[0].at ..].to_ascii_lowercase().starts_with(r),
                "the offset names the reading: {text}"
            );
        }
    }
}

#[test]
fn control_a_spelling_alone_and_a_reading_alone_are_silent() {
    for s in SPELLINGS {
        let text = format!("The mode `{s}` is `floor(x + q/2)`, a tie toward positive infinity.");
        assert!(found(&text).is_empty(), "{text}");
    }
    for r in READINGS {
        let direction = if *r == QUALIFIED { "up " } else { "" };
        let text = format!("A tie goes {direction}{r} under this other rule.");
        assert!(found(&text).is_empty(), "{text}");
    }
}

#[test]
fn the_magnitude_reading_counts_only_behind_a_direction_word() {
    // Two words that read a tie by its size, and the same two words bounding an
    // error, which is what most of the prose that carries them is doing.
    for d in DIRECTIONS {
        let text = format!("`half_up` moves a tie {d} in magnitude.");
        assert_eq!(found(&text), ["in magnitude"], "{text}");
    }
    assert!(found("The error of `half_up` is at most half a quantum in magnitude.").is_empty());
    assert!(found("`half_up` bounds the error by `q/2` in magnitude at every sign.").is_empty());
    // The direction has to be in the reading's own segment. Here it is about
    // where the mode rounds, and the magnitude is the error's.
    assert!(found("`half_up` rounds up, and the error is at most `q/2` in magnitude.").is_empty());
}

#[test]
fn a_reading_is_matched_without_case_and_a_name_with_it() {
    assert_eq!(found("`half_up` sends a tie Away From Zero."), [
        "away from zero"
    ]);
    // Java's and Python's constant is a different operation with a different
    // name, and saying that it goes away from zero is true.
    assert!(found("Java's `HALF_UP` sends a tie away from zero.").is_empty());
    assert!(found("`HALFUP` sends a tie away from zero.").is_empty());
}

#[test]
fn an_identifier_carrying_a_spelling_or_a_reading_is_not_one() {
    assert!(found("See `half_up_ties_away_from_zero` in the planted maps' list.").is_empty());
    assert!(found("`round_half_up_x` goes `away_from_zero_y`.").is_empty());
    assert!(found("`HalfUpper` goes away from zero.").is_empty());
    // A hyphen or a path separator is not part of an identifier, so these are
    // mentions.
    assert_eq!(found("`nearest-half-up` goes away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`Mode::HalfUp` goes away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`roundTiesToAway` is what `HalfUp` does."), [
        "roundtiestoaway"
    ]);
}

#[test]
fn every_negator_bound_to_one_of_the_two_lets_it_through() {
    for n in NEGATORS {
        let between = format!("`half_up` is {n} ties away from zero.");
        assert!(found(&between).is_empty(), "{between}");
        let before = format!("{n} ties away from zero is `half_up`.");
        assert!(found(&before).is_empty(), "{before}");
        // In the name's own segment, ahead of the name, which is the other half
        // of the binding rule.
        let at_the_name = format!("Ties away from zero, {n} `half_up`, is the alias.");
        assert!(found(&at_the_name).is_empty(), "{at_the_name}");
    }
    for n in ["isn't", "doesn't", "won't"] {
        let text = format!("`half_up` {n} go away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
}

#[test]
fn control_the_same_sentences_without_the_negator_fire() {
    assert_eq!(found("`half_up` is ties away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("Ties away from zero is `half_up`."), [
        "away from zero"
    ]);
    assert_eq!(found("`half_up` does go away from zero."), [
        "away from zero"
    ]);
}

#[test]
fn a_negator_in_another_segment_is_about_that_segment() {
    // The sentence asserts the reading of the mode and denies it of something
    // else. A reader taking any negator before the later of the two reads this
    // as a contrast and says nothing, which is how the reading got into prose
    // in the first place.
    assert_eq!(
        found("Unlike `half_even`, `half_up` sends a tie away from zero."),
        ["away from zero"]
    );
    assert_eq!(
        found("With no dither to read, `half_up` sends a tie away from zero."),
        ["away from zero"]
    );
    // A conjunction ends the negator's reach inside the segment too.
    assert_eq!(
        found("`half_up` is not the even rule but sends a tie away from zero."),
        ["away from zero"]
    );
}

#[test]
fn a_negator_after_both_is_about_something_else_and_does_not_let_it_through() {
    assert_eq!(
        found("`half_up` takes a tie away from zero, not toward positive infinity."),
        ["away from zero"]
    );
}

#[test]
fn a_negator_matched_as_a_word_not_inside_one() {
    // `no` inside `nothing`, `not` inside `notation`, `vs` inside nothing at all.
    assert_eq!(found("In this notation `half_up` goes away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("Nothing else: `half_up` goes away from zero."), [
        "away from zero"
    ]);
}

#[test]
fn every_marker_for_another_rule_lets_the_clause_through() {
    for m in ELSEWHERE {
        let text = format!("The {m} `half_up` takes a tie away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    assert_eq!(found("The `half_up` takes a tie away from zero."), [
        "away from zero"
    ]);
}

#[test]
fn a_marker_in_the_next_clause_does_not_reach_back() {
    assert_eq!(
        found("`half_up` takes a tie away from zero. The planted map does too."),
        ["away from zero"]
    );
}

#[test]
fn every_clause_boundary_separates_the_two() {
    for cut in [". ", "; ", ": ", "? ", "! ", "\n\n", ".\n", ";"] {
        let text = format!("`half_up` goes up{cut}ties away from zero is the alias.");
        assert!(found(&text).is_empty(), "{text:?}");
        let joined = format!("`half_up` goes up{cut}and takes ties away from zero.");
        assert!(found(&joined).is_empty(), "{joined:?}");
    }
}

#[test]
fn a_clause_opening_with_a_pronoun_takes_the_name_the_one_before_it_left() {
    // The sentence is about the mode and says so once, which is how anybody
    // writes two sentences about one thing.
    assert_eq!(
        found("`half_up` is nearest. Its tie goes away from zero."),
        ["away from zero"]
    );
    assert_eq!(found("`half_up` is nearest. It goes away from zero."), [
        "away from zero"
    ]);
}

#[test]
fn control_a_pronoun_points_at_whatever_was_named_last() {
    // The clause before names the other operation last, so the pronoun after it
    // is about that and not about the mode.
    assert!(
        found("`half_up` is not the `HALF_UP` of Java. That one goes away from zero.").is_empty()
    );
    // Nothing named at all, so the pronoun points at nothing this reads.
    assert!(found("A tie at -2.5 goes to -2. It is ties away from zero.").is_empty());
    // And a clause that opens with a pronoun after a blank line starts afresh.
    assert!(found("`half_up` is nearest.\n\nIts tie goes away from zero.").is_empty());
}

#[test]
fn control_what_is_not_a_boundary_keeps_the_clause_whole() {
    assert_eq!(found("`half_up` at 2.5 goes away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`ruling::half_up` goes away from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`half_up` sends a tie\naway from zero."), [
        "away from zero"
    ]);
    assert_eq!(found("`half_up`, away from zero."), ["away from zero"]);
}

#[test]
fn a_list_with_the_two_in_different_items_is_silent() {
    assert!(found("rounding: in {floor, ceil, toward_zero, away from zero, half_up}").is_empty());
    assert!(found("The modes floor, away from zero, and half_up.").is_empty());
}

#[test]
fn control_a_list_with_both_in_one_item_fires() {
    assert_eq!(
        found("rounding: in {floor, half_up away from zero, ceil}"),
        ["away from zero"]
    );
}

#[test]
fn control_a_parenthetical_is_not_a_list() {
    // Two commas, and an item of it carrying a clause rather than a term. The
    // reader that counted commas let both of these through.
    assert_eq!(
        found(
            "`half_up` rounds to nearest and, for negative inputs, rounds the tie to the \
               larger magnitude."
        ),
        ["larger magnitude"]
    );
    assert_eq!(
        found("For `half_up` the tie at -2.5 goes to -3, away from zero, like Java."),
        ["away from zero"]
    );
}

#[test]
fn a_subject_names_the_mode_in_every_clause() {
    let h = hits("Nearest, a tie sent away from zero.", Some("HalfUp"));
    assert_eq!(h.len(), 1, "{h:?}");
    assert_eq!(h[0].name, "HalfUp");
    let two = hits(
        "Nearest. Ties go away from zero. Also up in magnitude.",
        Some("HalfUp"),
    );
    assert_eq!(two.len(), 2, "each clause is about the item: {two:?}");
}

#[test]
fn control_a_subject_still_takes_every_escape() {
    assert!(hits("Not a tie sent away from zero.", Some("HalfUp")).is_empty());
    assert!(hits("The planted rule goes away from zero.", Some("HalfUp")).is_empty());
    assert!(hits("Nearest, a tie toward positive infinity.", Some("HalfUp")).is_empty());
    assert!(hits("Sent away from zero.", None).is_empty());
    // And it does not reach past a pronoun pointing at something else, which is
    // how a doc block contrasting the mode with Java's rule reads.
    assert!(
        hits(
            "Nearest. It is not the `HALF_UP` of Java; that one is ties away from zero.",
            Some("HalfUp")
        )
        .is_empty()
    );
}

#[test]
fn a_table_row_pairs_its_first_cell_with_the_others() {
    assert_eq!(found("| `half_up` | ties away from zero |"), [
        "away from zero"
    ]);
    assert_eq!(
        found("| mode | tie |\n|---|---|\n| `half_up` | up in magnitude |\n"),
        ["in magnitude"]
    );
}

#[test]
fn control_a_row_that_contrasts_or_names_something_else_is_silent() {
    assert!(found("| `half_up` | not away from zero |").is_empty());
    assert!(found("| `half_up` | the planted rule went away from zero |").is_empty());
    assert!(found("| floor | away from zero |").is_empty());
    assert!(found("| away from zero | `half_up` |").is_empty());
}

#[test]
fn a_row_ends_the_clause_it_interrupts() {
    assert!(found("`half_up` goes\n| a | b |\naway from zero.").is_empty());
}

#[test]
fn an_offset_counts_from_the_passage_in_a_later_clause_and_a_later_row() {
    // The first clause and the first row are long enough that an offset taken
    // from the clause or the row rather than the passage lands elsewhere.
    let text = "| a long first row | with cells |\n\
                A first clause that runs on for a while. `half_up` goes away from zero.\n\
                | `half_up` | up in magnitude |";
    let h = hits(text, None);
    assert_eq!(h.len(), 2, "{h:?}");
    assert!(text[h[0].at ..].starts_with("away from zero"), "{h:?}");
    assert!(text[h[1].at ..].starts_with("in magnitude"), "{h:?}");
}

#[test]
fn the_rulings_own_note_read_verbatim_is_clean() {
    // `ruling::half_up_denotes_ties_toward_positive_infinity`'s `note`, as the
    // registry carries it. The lint does not read a ratified ruling, so this is
    // not what keeps the gate off it: it is the sentence the whole repository is
    // meant to be able to write, and a reader refusing it would be refusing the
    // canon's own words.
    let note = "**A note travels with the name, as the floor note travels with the retired \
                word**: `half_up` is not Java's or Python's `HALF_UP`; on a signed domain a tie \
                at `-2.5 q` goes to `-2 q`. A reader wanting ties away from zero writes the \
                alias.";
    assert!(found(note).is_empty(), "{note}");
}

#[test]
fn the_crates_own_prose_about_the_alias_reads_clean() {
    // Prose that has to name both, and does it by contrast.
    for text in [
        "It is not the `HALF_UP` of Java or Python, which sends that tie the other way; that \
         operation is not one of the six and is reached by shifting the position half a step \
         away from zero and then rounding with `TowardZero`.",
        "The first of each pair is `half_up`, against the ties-away alias.",
        "`half_up` sends a tie toward positive infinity whatever the sign, so a tie at -2.5 goes \
         to -2.",
    ] {
        assert!(found(text).is_empty(), "{text}");
    }
}

#[test]
fn the_sentence_the_crate_shipped_fires() {
    // What `Mode::HalfUp` said before the ruling, as nearly as it was written.
    assert!(
        !hits(
            "To the nearest, and a tie goes away from zero.",
            Some("HalfUp")
        )
        .is_empty()
    );
    assert!(!found("`half_up` sends a tie to the neighbour of greater magnitude.").is_empty());
    assert!(!found("`HalfUp` reads the sign at a tie.").is_empty());
    assert!(!found("`HalfUp` commutes with reflection.").is_empty());
}

#[test]
fn a_reading_quoted_as_a_name_is_a_mention_rather_than_a_reading() {
    // Prose about the corpus rather than about the mode: the registry's residue
    // rows count the entries that read one way against the rows that name the
    // mode, and every one of them has to be able to say so.
    assert!(
        found("All three rows naming `half_up` appear for their `away from zero` entries.")
            .is_empty()
    );
    assert!(found("The `away from zero` spelling appears in `half_up`'s rows.").is_empty());
}

#[test]
fn control_a_code_span_alone_is_not_a_mention() {
    // Without the word saying the span is a name, the clause is asserting it.
    assert_eq!(found("`roundTiesToAway` is what `HalfUp` does."), [
        "roundtiestoaway"
    ]);
    assert_eq!(found("`half_up` sends a tie `away from zero`."), [
        "away from zero"
    ]);
}
