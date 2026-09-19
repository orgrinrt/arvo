//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The clause reader, over every spelling, every reading and every escape, each
//! escape beside the same sentence without it.

use super::reading::{ELSEWHERE, NEGATORS, READINGS, SPELLINGS, hits};

/// The readings found in `text`, as written.
fn found(text: &str) -> Vec<&'static str> {
    hits(text, None).into_iter().map(|h| h.reading).collect()
}

#[test]
fn every_spelling_with_every_reading_fires_once_and_names_both() {
    for s in SPELLINGS {
        for r in READINGS {
            let text = format!("The mode `{s}` takes a tie {r} here.");
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
        let text = format!("A tie goes {r} under this other rule.");
        assert!(found(&text).is_empty(), "{text}");
    }
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
fn every_negator_before_the_later_of_the_two_lets_it_through() {
    for n in NEGATORS {
        let between = format!("`half_up` is {n} ties away from zero.");
        assert!(found(&between).is_empty(), "{between}");
        let before = format!("{n} ties away from zero is `half_up`.");
        assert!(found(&before).is_empty(), "{before}");
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
fn a_subject_names_the_mode_in_every_clause() {
    let h = hits("Nearest, a tie sent away from zero.", Some("HalfUp"));
    assert_eq!(h.len(), 1, "{h:?}");
    assert_eq!(h[0].name, "HalfUp");
    let two = hits(
        "Nearest. Ties go away from zero. Also in magnitude.",
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
}

#[test]
fn a_table_row_pairs_its_first_cell_with_the_others() {
    assert_eq!(found("| `half_up` | ties away from zero |"), [
        "away from zero"
    ]);
    assert_eq!(
        found("| mode | tie |\n|---|---|\n| `half_up` | in magnitude |\n"),
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
                | `half_up` | in magnitude |";
    let h = hits(text, None);
    assert_eq!(h.len(), 2, "{h:?}");
    assert!(text[h[0].at ..].starts_with("away from zero"), "{h:?}");
    assert!(text[h[1].at ..].starts_with("in magnitude"), "{h:?}");
}

#[test]
fn the_rulings_own_note_and_the_crates_own_docs_read_clean() {
    // Prose that has to name both, and does it by contrast.
    for text in [
        "`half_up` is not Java's or Python's `HALF_UP`; on a signed domain a tie at `-2.5 q` \
         goes to `-2 q`. A reader wanting ties away from zero writes the shift and `toward_zero`.",
        "It is not the `HALF_UP` of Java or Python, which sends that tie the other way; that \
         operation is not one of the six and is reached by shifting the position half a step \
         away from zero and then rounding with `TowardZero`.",
        "The first of each pair is `half_up`, against the ties-away alias.",
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
