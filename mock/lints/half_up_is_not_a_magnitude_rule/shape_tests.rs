//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The word lists that tell a name from an ordinary word, a subject from a
//! value, an error's size from a tie's, and a contrast from an apposition,
//! each walked from both sides of its edge.
//!
//! A walk of a list's own words through the frame that consults the list says
//! only that the list is consulted. So each list here is walked twice: its own
//! words one way, and a set of words outside it, in the same frame, the other
//! way. A list too wide reads one of the outside words as its own and fails the
//! second walk; a list too narrow fails the first. The outside words are the
//! ones the neighbours in `sentences/neighbours.rs` turned on, and each walk
//! asserts they are outside the list, so neither half can pass by being empty.

use super::reading::{
    CONTRASTS,
    COUNTED,
    DENOTES,
    ERRORS,
    IMPERATIVE_NEXT,
    IMPERATIVE_PREPOSITIONS,
    MOTIONS,
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

/// Asserts no word of `outside` is in `list`, so a walk over `outside` asks
/// about the edge of the list rather than about the list again.
fn outside(list: &[&str], outside: &[&str]) {
    for w in outside {
        assert!(
            !list.contains(w),
            "{w} is in the list it is meant to stand outside"
        );
    }
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
fn a_method_name_is_an_imperative_behind_an_object_and_the_method_behind_anything_else() {
    // Words a method opening a clause as its subject is followed by: a verb, or
    // a preposition naming what it rounds to.
    let verbs = ["sends", "takes", "is", "goes", "rounds", "does"];
    let names = ["nearest", "even", "zero"];
    outside(IMPERATIVE_NEXT, &verbs);
    outside(IMPERATIVE_NEXT, &names);
    assert!(ordinary_forms().contains(&"Round".to_string()));
    for form in ordinary_forms() {
        for next in IMPERATIVE_NEXT {
            let text = format!("`half_up` is nearest. {form} {next} tie away from zero.");
            assert_eq!(found(&text), ["away from zero"], "{text}");
        }
        for verb in verbs {
            let text = format!("`half_up` is nearest. {form} {verb} a tie away from zero.");
            assert!(found(&text).is_empty(), "{text}");
        }
        for prep in IMPERATIVE_PREPOSITIONS {
            // Behind a preposition, an object makes the imperative.
            for next in ["the", "it", "every", "each"] {
                let text =
                    format!("`half_up` is nearest. {form} {prep} {next} tie away from zero.");
                assert_eq!(found(&text), ["away from zero"], "{text}");
            }
            // And a word naming what it rounds to makes the method.
            for name in names {
                let text = format!(
                    "`half_up` is nearest. {form} {prep} {name} sends a tie away from zero."
                );
                assert!(found(&text).is_empty(), "{text}");
            }
        }
    }
}

#[test]
fn toward_zero_is_the_rule_behind_a_lead_or_an_operations_gerund_and_a_direction_elsewhere() {
    // A rule named here takes the pronoun after it; a direction leaves it to
    // the mode.
    for lead in NAME_LEADS {
        let text =
            format!("`half_up` is nearest, {lead} toward zero. It sends a tie away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    for gerund in ["rounding", "truncating", "chopping"] {
        let text =
            format!("`half_up` is nearest, {gerund} toward zero. It sends a tie away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    // Behind a gerund of motion, or behind what moves, it is where a thing goes.
    let moved = ["tie", "ties", "value", "sends", "rounds", "moves", "goes"];
    outside(NAME_LEADS, &moved);
    outside(NAME_LEADS, MOTIONS);
    for word in moved.iter().chain(MOTIONS) {
        let text =
            format!("`half_up` is nearest, {word} toward zero. It sends a tie away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
}

#[test]
fn a_subject_noun_takes_the_clause_unless_the_clause_hands_it_back_with_it() {
    let others = ["tie", "value", "result", "number", "slot"];
    outside(SUBJECTS, &others);
    for noun in SUBJECTS {
        for article in ["", "The "] {
            let text = format!("`half_up` is a mode. {article}{noun} sends a tie away from zero.");
            assert!(found(&text).is_empty(), "{text}");
            // The same subject reporting what the mode does is about the mode.
            let back = format!(
                "`half_up` is a mode. {article}{noun} shows it sends a tie away from zero."
            );
            assert_eq!(found(&back), ["away from zero"], "{back}");
            // `them` points at several things, never at one mode.
            let them =
                format!("`half_up` is a mode. {article}{noun} reads them as going away from zero.");
            assert!(found(&them).is_empty(), "{them}");
        }
    }
    for noun in others {
        let text = format!("`half_up` is a mode. The {noun} goes away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
}

#[test]
fn a_bare_number_names_a_seat_unless_the_word_behind_it_counts() {
    let acts = ["read", "said", "stated", "answered", "found"];
    outside(COUNTED, &acts);
    for act in acts {
        let text = format!("`half_up` sends ties up. 270 {act} the ties as going away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
    for counted in COUNTED {
        let text = format!("`half_up` sends ties up. 5 {counted} the ties go away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
}

#[test]
fn a_rule_named_by_its_owner_takes_the_clause_and_one_owned_by_the_mode_does_not() {
    for owner in ["Java's", "Python's", "MATLAB's", "IEEE's"] {
        for noun in ["rule", "rounding", "mode", "method"] {
            let text = format!(
                "`half_up` maps `-2.5` to `-2`. `-2.5` under {owner} {noun} goes away from zero."
            );
            assert!(found(&text).is_empty(), "{text}");
        }
    }
    for owner in ["its", "the mode's", "this mode's"] {
        let text = format!(
            "`half_up` maps `-2.5` to `-2`. `-2.5` under {owner} rule goes away from zero."
        );
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
    // The ruled mode's own authorities own no other rule: under their rule the
    // clause is the mode's, in the same frame where an alias owner's is not.
    for owner in ["op's", "the registry's", "the panel's", "arvo's", "the ruling's", "the canon's"]
    {
        for noun in ["rule", "rounding"] {
            let own = format!(
                "`half_up` is the ruled mode. Under {owner} {noun} a tie goes away from zero."
            );
            assert_eq!(found(&own), ["away from zero"], "{own}");
        }
    }
    for owner in ["Java's", "Python's", "MATLAB's", "IEEE's"] {
        let alias =
            format!("`half_up` is the ruled mode. Under {owner} rule a tie goes away from zero.");
        assert!(found(&alias).is_empty(), "{alias}");
    }
    // An owner's thing that is not a rule leaves the clause to the mode.
    for owned in ["table", "example", "test"] {
        let text =
            format!("`half_up` maps `-2.5` to `-2`. `-2.5` in Java's {owned} goes away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
}

#[test]
fn an_error_noun_nearest_the_magnitude_makes_it_the_errors_and_any_other_noun_does_not() {
    let others = ["tie", "value", "result", "slot"];
    outside(ERRORS, &others);
    for noun in ERRORS {
        let text = format!("`half_up` has an {noun} that grows up in magnitude.");
        assert!(found(&text).is_empty(), "{text}");
        let tie = format!("`half_up` keeps the {noun} small and moves a tie up in magnitude.");
        assert_eq!(found(&tie), ["in magnitude"], "{tie}");
    }
    for noun in others {
        let text = format!("`half_up` has a {noun} that goes up in magnitude.");
        assert_eq!(found(&text), ["in magnitude"], "{text}");
    }
}

#[test]
fn a_reading_behind_the_denotation_is_a_contrast_only_where_its_segment_opens_one() {
    let joins = ["and", "so", "then", "as well as", "along with"];
    outside(CONTRASTS, &joins);
    outside(NEGATORS, &joins);
    for d in DENOTES {
        let bare = format!("`half_up` goes {d}, away from zero.");
        assert_eq!(found(&bare), ["away from zero"], "{bare}");
        for c in CONTRASTS {
            let contrast = format!("`half_up` goes {d}, {c} away from zero would give -3.");
            assert!(found(&contrast).is_empty(), "{contrast}");
        }
        // Written out rather than taken from `NEGATORS`, so a contrast the
        // list loses is a failure here rather than one fewer word walked.
        for n in [
            "not",
            "rather than",
            "instead of",
            "unlike",
            "as opposed to",
            "in contrast to",
            "contrary to",
        ] {
            let set = format!("`half_up` is {d}, {n} away from zero.");
            assert!(found(&set).is_empty(), "{set}");
        }
        for j in joins {
            let joined = format!("`half_up` is {d}, {j} away from zero.");
            assert_eq!(found(&joined), ["away from zero"], "{joined}");
        }
    }
}

#[test]
fn a_negator_behind_the_reading_denies_the_denotation_unless_a_copula_makes_it_an_identity() {
    for n in NEGATORS {
        for d in DENOTES {
            let act = format!("`half_up` sends a tie away from zero {n} {d}.");
            assert_eq!(found(&act), ["away from zero"], "{act}");
            let identity = format!("For `half_up`, away from zero is {n} {d}.");
            assert!(found(&identity).is_empty(), "{identity}");
            let swapped = format!("`half_up` sends a tie {d} {n} away from zero.");
            assert!(found(&swapped).is_empty(), "{swapped}");
        }
        let alone = format!("`half_up` sends a tie away from zero {n} at all.");
        assert!(found(&alone).is_empty(), "{alone}");
    }
}

#[test]
fn a_relative_after_a_denied_rule_is_the_rules_and_after_a_denied_quality_the_modes() {
    for n in NEGATORS {
        let rule = format!("`half_up` is {n} Java's rule, which goes away from zero.");
        assert!(found(&rule).is_empty(), "{rule}");
        let article = format!("`half_up` is {n} the truncating rule, which goes away from zero.");
        assert!(found(&article).is_empty(), "{article}");
        // `alias` is a rule's name as well as a negator, so it names a rule
        // here whatever follows it.
        if OTHER_NAMES.contains(n) {
            continue;
        }
        // A rule described with no article is a rule, and a description of the
        // mode with one is not.
        for rule in ["symmetric rounding", "commercial rounding", "a ties-away rule"] {
            let text = format!("`half_up` is {n} {rule}, which sends a tie away from zero.");
            assert!(found(&text).is_empty(), "{text}");
        }
        for description in ["the slow one", "a careful choice", "the default"] {
            let text =
                format!("`half_up` is {n} {description}, which is why a tie goes away from zero.");
            assert_eq!(found(&text), ["away from zero"], "{text}");
        }
        for quality in ["biased", "exact", "signed"] {
            let text =
                format!("`half_up` is {n} {quality}, which means a tie goes away from zero.");
            assert_eq!(found(&text), ["away from zero"], "{text}");
        }
    }
    for act in ["does not truncate", "never sends a tie down", "has no bias"] {
        let text = format!("`half_up` {act}, which means a tie goes away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
    assert_eq!(
        found("`half_up` is a nearest rule, which goes away from zero."),
        ["away from zero"]
    );
}

#[test]
fn an_alias_spelling_that_is_a_reading_is_a_name_unless_its_segment_points_back() {
    for alias in ["roundTiesToAway", "ties-away", "ties_away"] {
        let subject = format!("`half_up` is nearest, whereas `{alias}` goes away from zero.");
        assert!(found(&subject).is_empty(), "{subject}");
        let noun = format!("`half_up` is nearest, where the {alias} rule would reflect.");
        assert!(found(&noun).is_empty(), "{noun}");
        for owner in ["Java", "Python", "MATLAB"] {
            let other = format!("`half_up` is nearest, and {alias} is what {owner} does.");
            assert!(found(&other).is_empty(), "{other}");
        }
        for back in ["what it does", "its rule", "what they call it"] {
            let text = format!("`half_up` is nearest, and {alias} is {back}.");
            assert!(!found(&text).is_empty(), "{text}");
        }
        let mode = format!("`half_up` is `{alias}`.");
        assert!(!found(&mode).is_empty(), "{mode}");
    }
}

#[test]
#[ignore = "catalogue: `it` in a clause opening on a subject is read as the mode, so where it is \
            the subject itself or what the subject read the clause is refused, the open pronoun call; \
            tracked #arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi"]
fn it_as_the_subjects_own_pronoun_leaves_the_clause_to_the_subject() {
    for noun in SUBJECTS {
        for text in [
            format!("`half_up` is a mode. The {noun}, as it ships, sends a tie away from zero."),
            format!(
                "`half_up` is a mode. The {noun} sends a tie away from zero, which it should not."
            ),
        ] {
            assert!(found(&text).is_empty(), "{text}");
        }
    }
}

#[test]
#[ignore = "catalogue: a seat number before a word `COUNTED` holds is read as a count, so the \
            clause carries the mode and is refused; tracked \
            #arvo_s_half_up_reader_reads_a_bare_number_before_or_or_in_as"]
fn a_seat_number_before_a_counting_word_is_still_a_seat() {
    for (word, rest) in [
        ("or", "271 answered"),
        ("in", "the panel read"),
        ("in", "its first answer read"),
    ] {
        assert!(COUNTED.contains(&word));
        let text = format!("`half_up` sends ties up. 270 {word} {rest} ties away from zero.");
        assert!(found(&text).is_empty(), "{text}");
    }
}

#[test]
#[ignore = "catalogue: a bare number before a verb is read as a seat, so a count of ties takes the \
            clause from the mode; tracked #arvo_s_half_up_reader_reads_a_bare_number_before_a_verb_as_a"]
fn a_count_before_a_verb_is_still_a_count() {
    for verb in ["go", "land", "round"] {
        let text = format!("`half_up` sends ties up. 5 {verb} away from zero.");
        assert_eq!(found(&text), ["away from zero"], "{text}");
    }
}
