//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The word lists that tell a name from an ordinary word, a subject from a
//! value, an error's size from a tie's, and a contrast from an apposition,
//! each walked whole beside the same sentence without the word.
//!
//! Each of these was a shape the catalogue carried, and the pair it carried is
//! in `sentences/closed.rs`. A pair is one word of the list, so the walks are
//! here.

use super::reading::{
    DENOTES,
    ERRORS,
    IMPERATIVE_NEXT,
    NAME_LEADS,
    NEGATORS,
    OTHER_NAMES,
    SUBJECTS,
    hits,
};

/// The readings found in `text`, as written.
fn found(text: &str) -> Vec<&'static str> {
    hits(text, None).into_iter().map(|h| h.reading).collect()
}

/// A word with its first letter upper-cased, as it opens a sentence.
fn cap(word: &str) -> String {
    let mut c = word.chars();
    c.next()
        .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
        .unwrap_or_default()
}

/// Every form an other name takes that is also an English word under its
/// capital: one word, one capital. `alias` is left out, since it is a negator
/// as well and the walk would be asking about the negator.
fn ordinary_forms() -> Vec<String> {
    let mut out: Vec<String> = OTHER_NAMES
        .iter()
        .filter(|n| !NEGATORS.contains(n))
        .map(|n| cap(n))
        .filter(|f| {
            let mut c = f.chars();
            c.next().is_some_and(|c| c.is_ascii_uppercase()) && c.all(|c| c.is_ascii_lowercase())
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

#[test]
fn the_ordinary_forms_are_the_method_names_english_also_has() {
    // Computed, so this names what the walks below are over.
    assert_eq!(ordinary_forms(), [
        "Ceil",
        "Ceiling",
        "Convergent",
        "Fix",
        "Floor",
        "Round",
        "Stochastic"
    ]);
}

#[test]
fn every_method_name_opening_a_clause_as_an_imperative_leaves_the_clause_to_the_mode() {
    for form in ordinary_forms() {
        for next in IMPERATIVE_NEXT {
            let text = format!("`half_up` is nearest. {form} {next} tie away from zero.");
            assert_eq!(found(&text), ["away from zero"], "{text}");
        }
        // The control: the same name followed by a verb is the clause's
        // subject, and the clause is about that rule.
        let text = format!("`half_up` is nearest. {form} takes every tie away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
}

#[test]
fn every_lead_makes_the_lower_case_toward_zero_the_rule() {
    for lead in NAME_LEADS {
        let text =
            format!("`half_up` is nearest, {lead} toward zero. It sends a tie away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    // The control: behind a verb's object the two words are a direction, and
    // the pronoun after them is about the mode.
    assert_eq!(
        found("`half_up` sends a tie toward zero. It sends a tie away from zero."),
        ["away from zero"]
    );
}

#[test]
fn every_subject_noun_opening_a_clause_takes_the_clause() {
    for noun in SUBJECTS {
        for article in ["", "The "] {
            let text = format!("`half_up` is a mode. {article}{noun} sends a tie away from zero.");
            assert!(found(&text).is_empty(), "{text}");
        }
    }
    // The control: a noun that is not a subject of its own carries the mode.
    assert_eq!(
        found("`half_up` is a mode. The tie goes away from zero."),
        ["away from zero"]
    );
}

#[test]
fn every_error_noun_nearest_the_magnitude_makes_it_the_errors() {
    for noun in ERRORS {
        let text = format!("`half_up` has an {noun} that grows up in magnitude.");
        assert!(found(&text).is_empty(), "{text}");
        // The control: a tie standing nearer the two words gives them back to
        // the tie.
        let tie = format!("`half_up` keeps the {noun} small and moves a tie up in magnitude.");
        assert_eq!(found(&tie), ["in magnitude"], "{tie}");
    }
}

#[test]
fn every_denotation_set_off_by_a_bare_comma_is_an_apposition() {
    for d in DENOTES {
        let bare = format!("`half_up` goes {d}, away from zero.");
        assert_eq!(found(&bare), ["away from zero"], "{bare}");
        // The control: the same segment opening on `where` is a contrast.
        let contrast = format!("`half_up` goes {d}, where away from zero would give -3.");
        assert!(found(&contrast).is_empty(), "{contrast}");
    }
}

#[test]
fn every_negator_denying_the_denotation_behind_the_reading_leaves_the_reading() {
    for n in NEGATORS {
        for d in DENOTES {
            let text = format!("`half_up` sends a tie away from zero {n} {d}.");
            assert_eq!(found(&text), ["away from zero"], "{text}");
            // The control: the same two swapped, the negator ahead of the
            // reading and so bound to it.
            let swapped = format!("`half_up` sends a tie {d} {n} away from zero.");
            assert!(found(&swapped).is_empty(), "{swapped}");
        }
        // And behind the reading with no denotation behind it, the negator
        // binds the reading, as it always did.
        let alone = format!("`half_up` sends a tie away from zero {n} at all.");
        assert!(found(&alone).is_empty(), "{alone}");
    }
}

#[test]
fn every_negator_ahead_of_a_rule_in_words_gives_a_relative_to_that_rule() {
    for n in NEGATORS {
        let text = format!("`half_up` is {n} Java's rule, which goes away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    // The control: the same rule in words, affirmed of the mode, is the mode.
    assert_eq!(
        found("`half_up` is a nearest rule, which goes away from zero."),
        ["away from zero"]
    );
}

#[test]
fn an_alias_spelling_that_is_a_reading_is_a_name_where_it_heads_its_segment() {
    for alias in ["roundTiesToAway", "ties-away", "ties_away"] {
        let subject = format!("`half_up` is nearest, whereas `{alias}` goes away from zero.");
        assert!(found(&subject).is_empty(), "{subject}");
        let noun = format!("`half_up` is nearest, where the {alias} rule would reflect.");
        assert!(found(&noun).is_empty(), "{noun}");
        // The controls: the same spelling as what the mode is, in the mode's
        // segment or in one pointing back at it, is a reading.
        let mode = format!("`half_up` is `{alias}`.");
        assert!(!found(&mode).is_empty(), "{mode}");
        let back = format!("`half_up` is nearest, whereas it is `{alias}`.");
        assert!(!found(&back).is_empty(), "{back}");
    }
}
