//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Whether a word that could be a name is standing as one, and whether a
//! clause opens on a subject the lists have no name for.
//!
//! Some of what the reader matches is ordinary English as well as a name:
//! MATLAB's method names under their capital, the lower-case `toward zero`, and
//! the alias's spellings that are also readings. And a clause can open on a
//! subject no list names: a crate, a seat, a number. Each question here is
//! answered from the words right beside the match, read against closed lists in
//! `vocabulary.rs`, and none of them decides a pairing on its own.

use super::vocabulary::{
    ALIAS_OWNERS,
    ARTICLES,
    CONJUNCTIONS,
    COUNTED,
    IMPERATIVE_NEXT,
    IMPERATIVE_PREPOSITIONS,
    MOTIONS,
    NAME_LEADS,
    OTHER_NAMES,
    PERSONAL,
    RULE_NOUNS,
    SUBJECTS,
};

/// Whether the other rule's name, found at `at` in `text` in the form `form`,
/// stands there as the name rather than as an ordinary word.
pub(super) fn stands_as_a_name(text: &str, at: usize, form: &str) -> bool {
    if form == "toward zero" {
        return toward_zero_names(text, at);
    }
    !imperative(text, at, form)
}

/// Whether a capitalised ordinary word opening `text` at `at` is an imperative
/// or a plain word: the word right behind it is one `IMPERATIVE_NEXT` holds, or
/// one of `IMPERATIVE_PREPOSITIONS` with one of `IMPERATIVE_NEXT` behind that.
///
/// Only a single word with one capital is asked, which is the shape a MATLAB
/// method name and an English word share. `TowardZero` and `HALF_UP` are never
/// English and are not asked.
fn imperative(text: &str, at: usize, form: &str) -> bool {
    let mut chars = form.chars();
    let ordinary = chars.next().is_some_and(|c| c.is_ascii_uppercase())
        && chars.all(|c| c.is_ascii_lowercase());
    if !ordinary || text[.. at].chars().any(char::is_alphanumeric) {
        return false;
    }
    let rest = &text[at + form.len() ..];
    let next = next_word(rest);
    if IMPERATIVE_NEXT.contains(&next.as_str()) {
        return true;
    }
    IMPERATIVE_PREPOSITIONS.contains(&next.as_str()) && {
        let past = rest.trim_start_matches(char::is_whitespace);
        IMPERATIVE_NEXT.contains(&next_word(&past[next.len() ..]).as_str())
    }
}

/// The word right behind a match, lowered, where only whitespace stands
/// between. Punctuation in between gives no word.
fn next_word(text: &str) -> String {
    text.trim_start_matches(char::is_whitespace)
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '\'')
        .collect::<String>()
        .to_ascii_lowercase()
}

/// The word right ahead of `at`, lowered, where only whitespace stands between.
fn word_before(text: &str, at: usize) -> Option<String> {
    let head = text[.. at].trim_end_matches(char::is_whitespace);
    let word: String = head
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '\'')
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    (!word.is_empty()).then(|| word.to_ascii_lowercase())
}

/// Whether the lower-case `toward zero` at `at` is the rule's name: nothing
/// stands ahead of it but whitespace, a bracket, a comma or a backtick, or the
/// word ahead of it is one of `NAME_LEADS`, or a gerund naming an operation.
///
/// "truncating toward zero" and "rounding toward zero" name the operation by
/// what it rounds to, as "Round to nearest" does. A gerund of motion is the
/// exception, since what follows it is where something goes: "a tie going
/// toward zero", as `MOTIONS` lists them.
fn toward_zero_names(text: &str, at: usize) -> bool {
    match word_before(text, at) {
        None => true,
        Some(word) => {
            NAME_LEADS.contains(&word.as_str())
                || (word.len() > 4 && word.ends_with("ing") && !MOTIONS.contains(&word.as_str()))
        },
    }
}

/// Whether a clause opens on a subject of its own that no list of rules names.
///
/// Three shapes. A code span that is not a number, since a code span names a
/// thing and a number in one is a value the mode acts on: "`-2.5` goes away
/// from zero" is about the mode, "`arvo-format` sends a tie away from zero" is
/// about a crate. A bare whole number, which is how the registry names a seat:
/// "270 read blind". And a noun from `SUBJECTS`, behind an article or not, in
/// the first two words: "The crate", "The shipped crate", "Seat 270".
///
/// Each gives way where the clause points back with a singular `it`, which is
/// the clause handing what it says to the one thing named before: "Every test
/// shows it sends a tie away from zero" is about the mode, whatever the test
/// is. `they` and `them` point at several things, ties or seats, and never at
/// one mode: "270 read them as going away from zero" stays the seat's. Both are
/// readings of the open pronoun call and stand under its ticket. And a
/// number followed by one of `COUNTED` counts something rather than
/// naming a seat: "5 of the ties go away from zero".
pub(super) fn opens_on_a_subject(clause: &str) -> bool {
    let text = clause.trim_start();
    // FIXME(arvo_s_half_up_reader_reads_a_pronoun_after_a_negated_identi): `it`
    // anywhere in the clause is read as the mode, which is one reading of the
    // open pronoun call; the catalogue holds where it is the subject instead.
    if text
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .any(|w| matches!(w.to_ascii_lowercase().as_str(), "it" | "it's"))
    {
        return false;
    }
    if let Some(span) = text.strip_prefix('`') {
        return !a_number(span.split('`').next().unwrap_or_default());
    }
    let first = text.split(char::is_whitespace).next().unwrap_or_default();
    if !first.is_empty() && first.bytes().all(|b| b.is_ascii_digit()) {
        return !COUNTED.contains(&next_word(&text[first.len() ..]).as_str());
    }
    let words: Vec<String> = text
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .filter(|w| !w.is_empty())
        .take(3)
        .map(str::to_ascii_lowercase)
        .collect();
    let skip = usize::from(
        words
            .first()
            .is_some_and(|w| ARTICLES.contains(&w.as_str())),
    );
    words
        .iter()
        .skip(skip)
        .take(2)
        .any(|w| SUBJECTS.contains(&w.as_str()))
}

/// Whether the lowered `text` holds one of `PERSONAL`, a pronoun that points
/// back at a thing named before rather than at a clause or a stretch of text.
pub(super) fn holds_a_personal_pronoun(text: &str) -> bool {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .any(|w| PERSONAL.contains(&w))
}

/// Whether `clause` names a rule in words, by its owner: "Java's rule",
/// "MATLAB's rounding". The owner is one of `ALIAS_OWNERS`, the parties the
/// governing ruling gives the alias to. Any other owner is no other rule: "op's
/// rule", "the registry's rule" and "arvo's rounding" are the ruled mode under
/// another name, and "its rule" points back at it.
pub(super) fn names_a_rule_in_words(clause: &str) -> bool {
    let lower = clause.to_ascii_lowercase();
    lower.match_indices("'s ").any(|(at, _)| {
        let owner = word_before(&lower, at).unwrap_or_default();
        let owned = next_word(&lower[at + 3 ..]);
        ALIAS_OWNERS.contains(&owner.as_str())
            && (RULE_NOUNS.contains(&owned.as_str()) || owned == "rounding")
    })
}

/// Whether the body of a code span is a number: a digit first, behind a sign or
/// a point where one stands.
fn a_number(body: &str) -> bool {
    body.trim_start_matches(['-', '+', '.'])
        .starts_with(|c: char| c.is_ascii_digit())
}

/// Whether the reading at `r`, in the segment starting at `from`, is a rule's
/// name standing as one: it is spelled there as an entry of `OTHER_NAMES`, and
/// it either opens its segment, past a conjunction, `where` and an article, or
/// has one of `RULE_NOUNS` right behind it.
///
/// The caller asks this only of a segment that does not name the mode.
pub(super) fn a_reading_stands_as_a_name(
    text: &str,
    lower: &str,
    from: usize,
    r: usize,
    reading: &str,
) -> bool {
    let spelled = &text[r .. r + reading.len()];
    if !OTHER_NAMES.contains(&spelled) {
        return false;
    }
    let opens = lower[from .. r]
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
        .filter(|w| !w.is_empty())
        .all(|w| {
            CONJUNCTIONS.contains(&w) || ARTICLES.contains(&w) || matches!(w, "where" | "when")
        });
    let behind = next_word(text[r + reading.len() ..].trim_start_matches('`'));
    opens || RULE_NOUNS.contains(&behind.as_str())
}

#[cfg(test)]
mod tests {
    use super::{a_number, opens_on_a_subject, stands_as_a_name};

    #[test]
    fn a_method_name_followed_by_a_verb_is_the_method_and_by_an_object_is_not() {
        assert!(stands_as_a_name("Round is what goes", 0, "Round"));
        assert!(stands_as_a_name("Ceiling sends a tie", 0, "Ceiling"));
        assert!(!stands_as_a_name("Round every tie", 0, "Round"));
        assert!(!stands_as_a_name("Fix the doc", 0, "Fix"));
        assert!(!stands_as_a_name(" Ceiling aside, a tie", 1, "Ceiling"));
        // Not opening the text, so a possessive or anything else stands ahead.
        assert!(stands_as_a_name("MATLAB's Round the tie", 9, "Round"));
        // Never English, never asked.
        assert!(stands_as_a_name("TowardZero the tie", 0, "TowardZero"));
    }

    #[test]
    fn a_method_name_behind_a_preposition_is_an_imperative_only_where_an_object_follows() {
        assert!(!stands_as_a_name("Round to it and", 0, "Round"));
        assert!(!stands_as_a_name("Round up the tie", 0, "Round"));
        assert!(!stands_as_a_name("Round toward every tie", 0, "Round"));
        assert!(stands_as_a_name("Round to nearest sends", 0, "Round"));
        assert!(stands_as_a_name("Round toward nearest is", 0, "Round"));
        assert!(stands_as_a_name("Round to even goes", 0, "Round"));
    }

    #[test]
    fn toward_zero_behind_an_operations_gerund_is_the_name_and_behind_a_motion_a_direction() {
        assert!(stands_as_a_name(
            "and truncating toward zero sends",
            15,
            "toward zero"
        ));
        assert!(stands_as_a_name(
            "Rounding toward zero after",
            9,
            "toward zero"
        ));
        assert!(!stands_as_a_name(
            "a tie going toward zero",
            11,
            "toward zero"
        ));
        assert!(!stands_as_a_name(
            "keeps moving toward zero",
            13,
            "toward zero"
        ));
        // Four letters ending in `ing` is a noun, not a gerund.
        assert!(!stands_as_a_name("a ring toward zero", 7, "toward zero"));
    }

    #[test]
    fn toward_zero_is_a_name_where_a_noun_phrase_stands() {
        assert!(stands_as_a_name("for toward zero and", 4, "toward zero"));
        assert!(stands_as_a_name("toward zero switches", 0, "toward zero"));
        assert!(stands_as_a_name("is not `toward zero`", 8, "toward zero"));
        assert!(!stands_as_a_name(
            "send a tie toward zero",
            11,
            "toward zero"
        ));
        assert!(!stands_as_a_name(
            "it rounds toward zero",
            10,
            "toward zero"
        ));
    }

    #[test]
    fn a_clause_opens_on_a_subject_where_a_list_has_no_name_for_it() {
        for subject in [
            "`arvo-format` sends a tie",
            "270 read blind",
            "Seat 270 read blind",
            "The crate sends a tie",
            "The shipped crate sends a tie",
        ] {
            assert!(opens_on_a_subject(subject), "{subject}");
        }
        for not in [
            "`-2.5` goes away from zero",
            "`0.5` goes",
            "A tie goes away",
            "Its tie goes",
            "2.5 goes to 3",
            "-3 is away from zero",
            "Either way a tie goes",
            "Every test shows it sends a tie",
            "`arvo-format` asserts it is so",
            "5 of the ties go",
            "3 out of 4 go",
        ] {
            assert!(!opens_on_a_subject(not), "{not}");
        }
    }

    #[test]
    fn a_number_is_a_digit_behind_a_sign_or_a_point() {
        assert!(a_number("-2.5"));
        assert!(a_number(".5"));
        assert!(a_number("3 q"));
        assert!(!a_number("q/2"));
        assert!(!a_number("arvo-format"));
        assert!(!a_number("-"));
    }
}
